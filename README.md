# rust-bootloader
vibe coded bootloader by gemini

## UPDATE
This bootloader now can boot alpine linux. For do that, see the `virt-manager` tutorial.

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

$ qemu-system-x86_64 -bios /where-ovmf-directory/OVMF_CODE.fd -drive format=raw,file=fat:rw:esp -m 1024 # for this, use legacy branch
```

If use virti-manager with alpine
```
# mkdir -p /boot/efi/EFI/lazyboot
# cp /boot/vmlinuz-lts /boot/efi/EFI/lazyboot/
# cp /boot/initramfs-lts /boot/efi/EFI/lazyboot/
```
