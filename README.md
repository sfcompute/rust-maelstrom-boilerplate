
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