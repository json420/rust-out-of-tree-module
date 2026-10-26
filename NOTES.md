# Things I'm learning along the way

## If you want to build on Ubuntu agaist the prebuild Ubuntu kernel

From a fresh Ubuntu 26.04 istall you will need to install the following packages:

```
sudo apt-get install git build-essential rustc pahole linux-lib-rust-7.0.0-38-generic
```

```
sudo apt-get install git build-essential rustc pahole linux-lib-rust-`uname -r`
```
You need to install the correct `linux-lib-rust-VERSION` package:

```
sudo apt-get install linux-lib-rust-`uname -r`
```

Compile and install it like this:

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
