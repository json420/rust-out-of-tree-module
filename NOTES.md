# Things I'm learning along the way

## Exact steps for Ubuntu 26.04, 26.10

The quickest and easist first step is to build an out of tree module for the distribution kernel
you are already running. To do this, you will need to build your module using the same rust version
the distribution kernel was built with.

So on Ubuntu, that means builing your module with the rust version packaged in Ubuntu rather than
the latest and greatest installed with rustup.

From a fresh Ubuntu 26.04 install you will need to install the following packages, including the
correct `linux-lib-rust-VERSION` package:

```
sudo apt-get install git build-essential rust-all pahole linux-lib-rust-`uname -r`
```

Which will install something like `linux-lib-rust-7.0.0-38-generic` for the
`linux-lib-rust-VERSION` package.

Then compile and install the out of kernel module like this:

```
make
sudo insmod rust_out_of_tree.ko
```

Unload the module like this:

```
sudo rmmod rust_out_of_tree
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
