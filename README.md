
Starter repo for doing the [Maelstrom challenges]([text](https://fly.io/dist-sys/1/)) in rust.

## Getting Started

Install maelstrom's dependencies.

On Mac:
```bash
brew install openjdk graphviz gnuplot
```

On Ubuntu/Debian:
```bash
apt install openjdk-17-jdk graphviz gnuplot
```

Install Maelstrom:
```bash
./install_maelstrom.sh
```

To run your solution, build:
```bash
cargo build
```

and then run the binary with the maelstrom client:
```bash
./maelstrom/maelstrom test -w echo --bin ./target/debug/rust-maelstrom-boilerplate --node-count 1 --time-limit 10 --log-stderr
```

The `-w` flag specifies the workload to use. The starter code is an implementation of the `echo` workload, for others you will need to set this flag accordingly.

## Helpful Links

You can find docs for the maelstrom client [here](https://docs.rs/maelstrom-node/0.1.6/maelstrom/). The repo is at https://github.com/sitano/maelstrom-rust-node, where you can read source code and examples.