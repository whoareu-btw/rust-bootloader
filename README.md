# rust-bootloader
vibe coded bootloader by gemini

## NOTE
This bootloader only boot `bzImage` and `initramfs.img`, still aiming for custom initramfs instead full linux distro, you can modify it if you want.


### How to setup rustup?

```
$ rustup-init -y
$ source $HOME/.cargo/env
$ rustup override set nightly
$ rustup component add rust-src
```

### Add these component before compiling

```
$ mkdir .cargo # inside rust-bootloader directory
$ nvim .cargo/config.toml
```

```
[build]
target = "x86_64-unknown-uefi"

[unstable]
build-std = ["core", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]
```

And then compile with:

```
$ cargo build --release
```

### How to test it?

```
# Inside rust-bootloader directory

$ mkdir -p esp/EFI/BOOT

$ cp target/x86_64-unknown-uefi/release/bootloader.efi esp/EFI/BOOT/BOOTX64.EFI

$ qemu-system-x86_64 -bios /where-ovmf-directory/OVMF_CODE.fd -drive format=raw,file=fat:rw:esp -m 1024
```
