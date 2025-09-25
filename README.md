# ami-rs

Tired of whoami. Try out these alternatives. Written in **Rust** 🦀

credits to https://www.reddit.com/user/Kat1eWolf/ for the idea

---

![Overview](screenshots/overview.png)

---

## Setup Guide

To install ami-rs on your system, follow these steps:

### Use installation script

```bash
wget https://raw.githubusercontent.com/ShadowNetter-Official/ami-rs/main/install.sh && sh install.sh
```

### Or build and install manually

### 1. Clone This Repo

```bash
git clone https://github.com/ShadowNetter-Official/ami-rs
cd ami-rs
```
### 2. Build and install

```bash
cargo build --release
cp target/release/whereami ~/.cargo/bin/
cp target/release/whatami ~/.cargo/bin/
cp target/release/whyami ~/.cargo/bin/
cp target/release/howami ~/.cargo/bin/
cp target/release/whenami ~/.cargo/bin/
```

---

## Removing

You can remove ami-rs with:

```bash
rm ~/.cargo/bin/whereami
rm ~/.cargo/bin/whatami
rm ~/.cargo/bin/whyami
rm ~/.cargo/bin/whenami
rm ~/.cargo/bin/howami
```

---

### Done :)

I hope you like it

### MIT License

Copyright (c) 2025 ShadowNetter-Official

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
