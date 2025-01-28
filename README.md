
Starter repo for doing the [Maelstrom challenges]([text](https://fly.io/dist-sys/1/)) in rust.

## Getting Started

Install maelstrom's dependencies:
```bash
brew install openjdk graphviz gnuplot # Mac
apt install openjdk-17-jdk graphviz gnuplot # Ubuntu/Debian
```

Install Maelstrom:
```bash
./install_maelstrom.sh
```

To run your solution:
```bash
cargo build
./maelstrom/maelstrom test -w echo --bin ./target/debug/rust-maelstrom-boilerplate --node-count 1 --time-limit 10 --log-stderr
```

## Helpful Links

You can find docs for the maelstrom client [here]([https://github.com/jepsen-io/maelstrom/blob/master/doc/client-usage.md](https://docs.rs/maelstrom-node/0.1.6/maelstrom/)). The repo is at [text](https://github.com/sitano/maelstrom-rust-node), where you can read source code and examples.