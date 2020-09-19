# AppSer

Sample Elixir application that parses and reduce a huge JSON object with 10M elements. This library contains two implementations one in pure Elixir and another as a NIF powered by Rust.

## Results

```
❯ mix run bench.exs
Compiling NIF crate :app_ser_nif (native/app_ser_nif)...
    Finished release [optimized] target(s) in 0.02s
Operating System: Linux
CPU Information: Intel(R) Core(TM) i7-4720HQ CPU @ 2.60GHz
Number of Available Cores: 8
Available memory: 31.25 GB
Elixir 1.10.4
Erlang 23.0.3

Benchmark suite executing with the following configuration:
warmup: 2 s
time: 1 min
memory time: 0 ns
parallel: 1
inputs: none specified
Estimated total run time: 2.07 min

Benchmarking AppSer...
Benchmarking AppSerNif...

Name                ips        average  deviation         median         99th %
AppSerNif          2.04         0.49 s     ±2.97%         0.49 s         0.54 s
AppSer           0.0829        12.07 s    ±10.84%        11.48 s        14.41 s

Comparison:
AppSerNif          2.04
AppSer           0.0829 - 24.64x slower +11.58 s
```

## Tests

`mix test`

## Sample data

To generate data for benchmarking, `python gen.py | jq > values.json `
