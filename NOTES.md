# Things I'm learning along the way

## If you want to build on Ubuntu agaist the prebuild Ubuntu kernel

You need to install the correct `linux-lib-rust-VERSION` package:

```
sudo apt-get install linux-lib-rust-`uname -r`
```

Compile and install t like this:

```
make
sudo make modules_install
sudo insmod /lib/modules/`uname -r`/updates/rust_out_of_tree.ko
```

Then to unload the module like this:

```
sudo rmmod rust_out_of_tree
```
