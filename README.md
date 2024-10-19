# nrl2730-rust

An NRL2730 Server with Rust

#### Looking for a Go version? [Here](https://github.com/FARC-CN/nrl2730-go) it is.
#### We offer a public server with the Go version program: nrl2730.farc.org.cn

```
This program is used to forward UDP packets between clients, the first 20 bytes of the packet header are
    NRL2  4 bytes fixed     "NRL2"
    XX    2 bytes packet    length
    CPUID 7 bytes sending   device serial number
    CPUID 7 bytes receiving device serial number
```

### How to use：

Download, compile, and execute programs

```
# download
git clone https://github.com/FARC-CN/nrl2730-rust

# compile
cd nrl2730-rust
cargo build -r

# run
cd target/release
./nrl2730-rust
```
Note:

After the above program is running, it will receive and process data packets on UDP port 60050. If you need to use other ports, please specify -p [PORT], such as:

```
./nrl2730-rust -p 6000
```

So that you know, the system's firewall must allow UDP port 60050 communication. Common system operations are as follows:

# Thanks
[BG6CQ](https://github.com/bg6cq), provided ideas for the code.

