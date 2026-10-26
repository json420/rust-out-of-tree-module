# Things I'm learning along the way

## If you want to build on Ubuntu agaist the prebuild Ubuntu kernel

You need to install the correct `linux-lib-rust-VERSION` package:

```
sudo apt-get install linux-lib-rust-`uname -r`
```