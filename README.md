# udpmultiplexer
Multiplex UDP packets to multiple destinations with StatsD support.

# Usage

```
Multiplex incoming udp packets on listen host:port to N target host:port locations

Usage: udpmultiplexer [OPTIONS] --listen <LISTEN> --target <TARGET>

Options:
      --listen <LISTEN>
          Listen host:port use if ip address or 0.0.0.0 for all
      --target <TARGET>
          Target host:port can repeat
  -l, --log-count <LOG_COUNT>
          Log count of incoming messages every log-count messages [default: 0]
      --log-backlog
          Log count of packet backlog
  -s, --statsd-count-name <STATSD_COUNT_NAME>
          Add statsd count of packets sent [default: ]
  -f, --flush-time-ms <FLUSH_TIME_MS>
          Duration to wait to flush threads in ms.  Can be used to tune throughput [default: 10]
  -h, --help
          Print help
  -V, --version
          Print version

```