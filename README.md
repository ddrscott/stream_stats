# Stream Stats

Output statistics about data from stdin while redirecting the data to stdout.
The statistics are bytes read, bytes read per second, lines read, lines read per
second and total seconds.

```sh
# Throughput of random generator:
stream_stats < /dev/random | stream_stats > /dev/null
# =>  8.0 sec | 106496 kb [ 13305.2/s ] | 426873 lines [ 53332/s ]

# Throughput of inflating a zip file:
gunzip -c /path/to/file.gz | stream_stats > /path/to/unzipped
# =>  8.0 sec | 106496 kb [ 13305.2/s ] | 426873 lines [ 53332/s ]

# How many logs lines are generated:
tail -f /var/log/*.log | stream_stats
# => Thu Mar  8 07:55:49.516 SC: <airportd[179]> airportdProcessSystemConfigurationEvent: ...
# => Fri Mar  9 04:46:32.738 <kernel> GTK:
# => Fri Mar  9 04:46:32.738 [00000000] 76 32 D2 8B 48 91 05 73 67 E2 35 1C 02 EE D0 BF
# => Fri Mar  9 04:46:32.738 <kernel> installGTK: GTK installed
# 26.0 sec | 16 kb [ 0.6/s ] | 194 lines [ 7/s ]
```
## Practical Uses

### HTTP Request Rate
```sh
# with log output
/var/log/httpd/access.log | stream_stats

# without log output
/var/log/httpd/access.log | stream_stats > /dev/null
```

### General Log Activity
```sh
# with log output
tail -f /var/log/*.log | stream_stats
```

### Ideas?!?

Let us know how you're using this tool!

## Installation

Install to local system

    $ cargo install stream_stats

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ddrscott/stream_stats.

## License

The crate is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
