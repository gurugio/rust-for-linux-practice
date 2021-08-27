# How to booting rust-for-linux kernel

## booting Ubuntu with QEMU

Download Ubuntu iso file from ubuntu homepage.

Install Qemu
* On Ubuntu, run `apt install qemu`

Run Qemu with Ubuntu iso file
* command: `qemu-system-x86_64 -enable-kvm -smp 8 -m 4096M -hda ubuntu.img -cdrom ubuntu-20.04.2.0-desktop-amd64.iso -boot d`

Install Ubuntu on VM

Booting the VM
* `qemu-system-x86_64 -enable-kvm -cpu host -m 4096M -drive id=d0,file=ubuntu.qcow2,if=none,format=qcow2 -device virtio-blk-pci,drive=d0`

## build rust-for-linux kernel

Setup guest os
- install packages for kernel build
  * For Ubuntu: apt install build-essential git libssl-dev libelf-dev
- install rustup to build rust code

Install clang and llvm
- rust-for-linux kernel is built with LLVM
- `apt install clang lld llvm`
- If installed clang version is <12, you have to install the upper version.
- `apt install clang-12 lld-12`
- check clang version with `clang --version`
- change default version of clang and ld.lld if clang version is not changed
  * `sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-12 100`
  * `sudo update-alternatives --install /usr/bin/ld.lld ld.lld /usr/bin/ld.lld-12 100`

Download kernel code
- download kernel code from https://github.com/Rust-for-Linux/linux

(IMPORTANT)Setup rust build 
- **see https://github.com/Rust-for-Linux/linux/blob/rust/Documentation/rust/quick-start.rst**
- `cd linux` (enter the linux directory)
- `rustup component add rust-src`
- `cargo install --locked --version 0.56.0 bindgen`

Kernel build
- kernel configuration
  * `make x86_64_defconfig`: use a default configuration in the kernel
  * (recommended) `cp /boot/config-5.x ./.config`: copy the configuration file from Ubuntu
- run `make menuconfig`
- **(IMPORTANT) enable `CONFIG_RUST` and `CONFIG_SAMPLES_RUST`**
- make LLVM=1 -j$(nproc)
  * use all CPUs to be used for build
  * If it generates OOM, use less CPUs.
  * for example, `make LLVM=1 -j4`


load sample modules
- `modprobe rust_minimal` : work fine
- build and insmod rust modules
```
make M=samples/rust/ LLVM=1
sudo insmod samples/rust/rust_minimal.ko
```


### Problem solving

If rustc version is <1.54, do 'rustup update'

Kernel build failed with "clang not found error"
- `sudo apt-get install clang`

Kernel build failed with "linker 'ld.lld' not found" error
- sudo apt-get install lld lld-12

Kernel build failed with:
```
*** Compiler is too old.
***   Your Clang version:    10.0.0
***   Minimum Clang version: 10.0.1
```
- `sudo apt-get install clang-12`
- `sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-12 100`
- `sudo update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-10 100`

Kernel build failed with:
```
*** Linker is too old.
***   Your LLD version:    10.0.0
***   Minimum LLD version: 10.0.1
```
- `sudo apt-get install lld-12`
- `sudo update-alternatives --install /usr/bin/lld lld /usr/bin/lld-12 100`
- `ln -s ld.lld-12 ld.lld`


If it failed to build rust modules:
- enable a kernel option `CONFIG_DEBUG_INFO` and build again


# How to setup VisualStudio Code

Download install VisualStudio Code from the homepage

Run VSCode

Click "File" -> "Open Folder" and open the Linux directory

Install rust-analyzer to use "go to definition"
- see https://github.com/Rust-for-Linux/linux/blob/rust/Documentation/rust/quick-start.rst#rust-analyzer
- install a extension "rust-analyzer"
  * There is "Rust" extension but it does not support "go to definition".
- do `make rust-analyzer` in the kernel source directory to create `rust-project.json`
- `go to definition` works for core macros and functions
  * For example, click `File` with right mouse button and select "go to definition".
  * VSCode opens rust/kernel/file.rs and shows `pub struct File` structure.

Build and load rust modules
- edit source and open the terminal (short-key is Ctrl+`)
- build only rust samples: `make M=samples/rust/ LLVM=1`
- load kernel module: `sudo insmod samples/rust/rust_minimal.ko`
- check kernel log: `dmesg` or `tail /var/log/kern.log`


