# Queries
This directory contains queries for highlighting and analysis based on the grammar.

## Measuring performance
When this library is used for highlighting, any slow query logic will manifest itself on initial
load, and make the user's text editor appear to be sluggish. This makes it important to measure
and optimize the time that each query takes.

This project uses the `criterion` Rust crate to measure query performance over numerous iterations.
Criterion handles the details of running our code and building statistics. To run it:

```
cargo bench
```

This command will run for around 30 seconds and then generate an output report. You can find the
report at `target/criterion/report/index.html`.

## Optimizing performance
If you observe a performance regression, or wish to cause a performance improvement, you can use
the benchmark binary to get more information. One tool that will help you dive deep on hotspots
is [`flamegraph`](https://github.com/flamegraph-rs/flamegraph), which generates reports on the
time spent in various functions.

You can install `flamegraph` from `cargo` with:
```
cargo install flamegraph
```

Depending on your system, you may also have to follow other setup instructions that you can find in
the `flamegraph` README, like installing `linux-perf` through `apt`. I also had to enable perf
events with scary-looking commands like:

```
sudo sysctl -w kernel.perf_event_paranoid=-1
sudo sysctl -w kernel.kptr_restrict=0
```

You should ask your doctor if those are right for you.

Once you've set up `flamegraph` properly, locate the exact unit test binary from the output of the
most recent criterion run (make sure to choose the *most recent one*, or things might not work):
```
$ cargo bench
    Finished bench [optimized] target(s) in 0.02s
     Running unittests ([NOT_THE_ONE_WERE_LOOKING_FOR])

running 2 tests
test tests::test_can_load_grammar ... ignored
test tests::test_can_parse_basic_file ... ignored

test result: ok. 0 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/release/deps/benchmark_queries-ecfa64b9b5e6055c)
Gnuplot not found, using plotters backend
Benchmarking highlights/ts_query_new: Warming up for 3.0000 s
```

In that example, our binary is `target/release/deps/benchmark_queries-[hash]`.

We then run that under `flamegraph` to gather a bunch of `perf` events:
```
BENCHMARK_BINARY=target/release/deps/benchmark_queries-ecfa64b9b5e6055c
flamegraph $BENCHMARK_BINARY --bench
```

At which point, the file at `flamegraph.svg` will present a beautiful (and informative!) picture of
what is causing our delay.
