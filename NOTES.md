# Things I'm learning along the way

## Exact steps for Ubuntu 26.04, 26.10

The quickest and easiest first step is to build this out of tree module for the distribution kernel
you are already running. To do this, you will need to build this module using the same Rust version
that the distribution kernel was built with.

So on Ubuntu, that means builing your module with the Rust version packaged in Ubuntu rather than
the latest and greatest installed with rustup.

From a fresh Ubuntu 26.04 or 26.10 install, you will need to install the following packages,
including the correct `linux-lib-rust-VERSION` package:

```sh
sudo apt install build-essential rust-all pahole linux-lib-rust-`uname -r`
```

Which will install something like `linux-lib-rust-7.0.0-38-generic` or
`linux-lib-rust-7.3.0-9-generic` for the `linux-lib-rust-VERSION` package.

And compile and install the kernel module like this:

```sh
make
sudo insmod rust_out_of_tree.ko
```

Unload the module like this:

```sh
sudo rmmod rust_out_of_tree
```

If you already have a Rust install via rustup, you will get an error if you try to compile this
module because the Rust version does not match what the kernel was built with.  In this case, you
can tell rustup to use the system install like this:

```sh
rustup toolchain link system /usr
rustup default system
```

You can watch the output from the `pr_info!()` macro like this:

```sh
sudo dmesg -w
```

Once you're in the grove, it's handy to have `dmesg -w` running in one terminal and then run this
each time you need to test a change:

```sh
make && sudo rmmod rust_out_of_tree && sudo insmod rust_out_of_tree.ko
```
