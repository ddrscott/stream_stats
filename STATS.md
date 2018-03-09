Code for these various attempts in the branch `various_attempts`

# Cat

Baseline timing for reading input and writing to stdout.

## `cat`

```sh
time cat /tmp/lines_5m.csv > /dev/null
# => 0.01s user 0.26s system 99% cpu 0.277 total
```

## ./veedrac

https://codereview.stackexchange.com/questions/94941/simple-cat-in-rust

```sh
rustc -O veedrac.rs
time ./veedrac /tmp/lines_5m.csv > /dev/null
# => 0.01s user 0.26s system 99% cpu 0.277 total
```

## Stack Overflow - fill_buf

```sh
./so_37079342 < /tmp/lines_5m.csv > /dev/null
# => 0.02s user 0.27s system 99% cpu 0.293 total
```

# Line Count

Timings for performing line counts.

## `wc -l` - Word Count
```sh
time wc -l < /tmp/lines_5m.csv
# => 5000000
# => 0.86s user 0.26s system 99% cpu 1.121 total
```

## Ruby with counters
```sh
time ./exe/stream_count /tmp/lines_5m.csv > /dev/null
# => 1,409,473,763 bytes [ 595,876 kb/sec ] | 5,000,000 lines [ 2,164,557 lines/sec ]
# => 1.63s user 1.10s system 94% cpu 2.896 total
```

## Crystal with counters
```sh
time ./stream_count < /tmp/lines_5m.csv > /dev/null
# => 1,376,439 kb [ 1,376,439 kb/sec ] | 5,000,002 lines [ 5,000,002 lines/sec ]
# => 0.92s user 0.49s system 98% cpu 1.431 total
```

## Node stream-stat
https://github.com/peterwmwong/stream-stat

```sh
time node index-pipe.js < /tmp/lines_5m.csv > /dev/null
# => 1,405,353,984 bytes [ 686,208 kb/sec ] | 4,986,019 lines [ 2,493,009 lines/sec ]
# => 2.17s user 0.95s system 110% cpu 2.812 total
```

# Rust `io.lines()`

This does not return the correct byte count.

```sh
time ./lines < /tmp/lines_5m.csv > /dev/null
# => num_lines: 5000000, num_bytes: 1404473763
# => 0.75s user 0.34s system 99% cpu 1.100 total
```

## Rust `io.read_line()`

```sh
time ./read_line < /tmp/lines_5m.csv > /dev/null
# => num_lines: 5000000, num_bytes: 1409473763
# => 0.48s user 0.32s system 99% cpu 0.800 total
```

## Rust `stream_stats` with static shared memory
Commit: `84d83e4`
```sh
time ./target/release/stream_stats < /tmp/lines_5m.csv > /dev/null
# =>  0.8 sec | 1376439 kb [ 1704.7 kb/sec ] | 5000000 lines [ 6341091 lines/sec ]
# => 0.47s user 0.32s system 99% cpu 0.794 total
```

## Rust `stream_stats` with AtomicUsize
Commit: `b83183d`
```sh
time ./target/release/stream_stats < /tmp/lines_5m.csv > /dev/null
# => 0.8 sec | 1376439 kb [ 1623148.5/s ] | 5000000 lines [ 5896187/s ]
# => 0.52s user 0.33s system 99% cpu 0.854 total
```
