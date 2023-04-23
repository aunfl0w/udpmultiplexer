use std::net::UdpSocket;
use std::process::ExitCode;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
mod args;
use args::Args;

const MAX_PACKET_SIZE: usize = 65_527;

#[derive(Clone)]
struct DataPacket {
    data: [u8; MAX_PACKET_SIZE],
    len: usize,
}

fn main() -> ExitCode {
    let args = args::parse_args();
    match send(args) {
        Err(error) => {
            println!("Error {:?}", error);
            return ExitCode::FAILURE;
        }
        Ok(_) => {}
    }
    ExitCode::SUCCESS
}

#[allow(unreachable_code)]
fn send(args: Args) -> std::io::Result<()> {
    let socket = UdpSocket::bind(args.listen)?;
    let mut count: u128 = 0;
    let mut queues: Vec<Arc<Mutex<Vec<DataPacket>>>> = Vec::new();
    let signal = Arc::new(Condvar::new());

    for to in args.target {
        let queue = Arc::new(Mutex::new(Vec::<DataPacket>::new()));
        queues.push(queue.clone());
        let signal = signal.clone();
        let to = to.clone();
        thread::Builder::new()
        .name(to.to_string())
        .spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't create outgoing socket");
            println!("Thread {} Starting", to);
            loop {
                let (mut mq, _) = signal
                    .wait_timeout(queue.lock().unwrap(), Duration::from_millis(150))
                    .unwrap();
                if mq.len() > 0 && args.log_backlog {
                    println!("Packet backlog for {} is {}", to, mq.len());
                }
                mq.retain(|data| {
                    socket
                        .send_to(&data.data[0..data.len], &to)
                        .expect(format!("Couldn't send to {}", to).as_str());
                    false
                });
            }
        })?;
    }

    // wakeup threads on timer
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(args.flush_time_ms));
        signal.notify_all();
    });

    loop {
        let mut buf = [0; MAX_PACKET_SIZE];
        let (amt, _src) = socket.recv_from(&mut buf)?;
        let data_packet = DataPacket {
            data: buf,
            len: amt,
        };

        push_message(&queues, data_packet);
        count += 1;
        if count % args.log_count == 0 {
            if args.log_count > 0 {
                println!("{} packets", count);
            }
            if !args.statsd_count_name.eq("") {
                let count_msg = format!("{}:{}|c\n", args.statsd_count_name, count);
                let msg_bytes = count_msg.as_bytes();
                let mut buf = [0; MAX_PACKET_SIZE];
                buf[0..msg_bytes.len()].clone_from_slice(msg_bytes);
                let data_packet = DataPacket {
                    data: buf,
                    len: msg_bytes.len(),
                };
                push_message(&queues, data_packet);
            }
        }
    }

    Ok(())
}

fn push_message(queues: &Vec<Arc<Mutex<Vec<DataPacket>>>>, data_packet: DataPacket) {
    for q in queues {
        q.lock().unwrap().push(data_packet.clone());
    }
}
