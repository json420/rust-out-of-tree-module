# Things I'm learning along the way

## Exact steps for Ubuntu 26.04, 26.10

The quickest and easiest first step is to build this out of tree module for the distribution kernel
you are already running. To do this, you will need to build this module using the same Rust version
that the distribution kernel was built with.

So on Ubuntu, that means builing your module with the Rust version packaged in Ubuntu rather than
the latest and greatest installed with rustup.

From a fresh Ubuntu 26.04 or 26.10 install, you will need to install the following packages,
including the correct `linux-lib-rust-VERSION` package:

```
sudo apt install git build-essential rust-all pahole linux-lib-rust-`uname -r`
```

Which will install something like `linux-lib-rust-7.0.0-38-generic` or
`linux-lib-rust-7.3.0-9-generic`for the `linux-lib-rust-VERSION` package.

Then clone this repo:

```
git clone https://github.com/Rust-for-Linux/rust-out-of-tree-module.git
cd rust-out-of-tree-module
```

And compile and install the kernel module like this:

```
make
sudo insmod rust_out_of_tree.ko
```

Unload the module like this:

```
sudo rmmod rust_out_of_tree
```

If you already have a Rust install via rustup, you will get an error if you try to compile this
module because the Rust version does not match what the kernel was built with.  In this case, you
can tell rustup to use the system install like this:

```
rustup toolchain link system /usr
rustup default system
```

You can watch the output from the `pr_info!()` macro like this:

```
sudo dmesg -w
```

Once you're in the grove, it's handy to have `dmesg -w` running in one terminal and then run this
each time you need to test a change:

```
make && sudo rmmod rust_out_of_tree && sudo insmod rust_out_of_tree.ko
```
