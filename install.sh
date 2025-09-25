#!/bin/bash

echo
echo "ami-rs"
echo "by ShadowNetter"
echo
echo "cloning into repo..."
git clone https://github.com/ShadowNetter-Official/ami-rs
cd ami-rs
echo "done"
echo "installing..."
cargo build --release
#whereami
cp target/release/whereami ~/.cargo/bin/
#whatami
cp target/release/whatami ~/.cargo/bin/
#whyami
cp target/release/whyami ~/.cargo/bin/
#howami
cp target/release/howami ~/.cargo/bin/
#whenami
cp target/release/whenami ~/.cargo/bin/
echo "done"
echo
echo "to uninstall do:"
echo "rm ~/.cargo/bin/whereami"
echo "rm ~/.cargo/bin/whatami"
echo "rm ~/.cargo/bin/whyami"
echo "rm ~/.cargo/bin/whenami"
echo "rm ~/.cargo/bin/howami"
echo
