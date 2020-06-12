# packet-capture
![Rust](https://github.com/telumo/packet-capture/workflows/Rust/badge.svg)


```sh
# Ethernetの名前を調べる
# for linux
$ ip addr
# for mac
$ ifconfig

# ビルド
$ cargo build

# 実行
$ sudo ./target/debug/packet-capture <eth-name>
```