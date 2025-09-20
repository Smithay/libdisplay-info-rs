# libdisplay-info-rs

This project contains rust bindings for [`libdisplay-info`](https://gitlab.freedesktop.org/emersion/libdisplay-info/).

From the official libdisplay-info docs:

> EDID and DisplayID library.
> 
> Goals:
> 
> - Provide a set of high-level, easy-to-use, opinionated functions as well as
>   low-level functions to access detailed information.
> - Simplicity and correctness over performance and resource usage.
> - Well-tested and fuzzed.

## Installing libdisplay-info 

You should search for some packages like `libdisplay-info-dev` or `libdisplay-info-devel` with your system package manager, like `apt`.

Alternatively, to build from source and install `libdisplay-info`:
1. Following the build instructions in [`libdisplay-info`](https://gitlab.freedesktop.org/emersion/libdisplay-info/)
   ```shell
   # in libdisplay-info
   git checkout 0.2.0 # as this crate is compatible with libdisplay-info >= 0.1.0, < 0.3.0
   # You should install meson, a build system in pure python.
   # You should be able to use it as long as you have a Python interpreter and pip
   meson setup build/
   ninja -C build/
   ```
2. Run `sudo ninja -C build/ install`, which will by default install related manifest files into `/usr/local/`, for example
   ```shell
   $ sudo ninja -C build/ install
   [sudo] password for user: 
   ninja: Entering directory `build/'
   [0/1] Installing files.
   Installing subdir /home/user/code/libdisplay-info/include/libdisplay-info to /usr/local/include/libdisplay-info
   Installing /home/user/code/libdisplay-info/include/libdisplay-info/dmt.h to /usr/local/include/libdisplay-info
   Installing /home/user/code/libdisplay-info/include/libdisplay-info/gtf.h to /usr/local/include/libdisplay-info
   Installing /home/user/code/libdisplay-info/include/libdisplay-info/info.h to /usr/local/include/libdisplay-info
   Installing /home/user/code/libdisplay-info/include/libdisplay-info/cta.h to /usr/local/include/libdisplay-info
   Installing /home/user/code/libdisplay-info/include/libdisplay-info/cvt.h to /usr/local/include/libdisplay-info
   Installing /home/user/code/libdisplay-info/include/libdisplay-info/displayid.h to /usr/local/include/libdisplay-info
   Installing /home/user/code/libdisplay-info/include/libdisplay-info/edid.h to /usr/local/include/libdisplay-info
   Installing libdisplay-info.so.0.2.0 to /usr/local/lib/riscv64-linux-gnu
   Installing di-edid-decode/di-edid-decode to /usr/local/bin
   Installing /home/user/code/libdisplay-info/build/meson-private/libdisplay-info.pc to /usr/local/lib/riscv64-linux-gnu/pkgconfig  <------ copy this path
   Installing symlink pointing to libdisplay-info.so.0.2.0 to /usr/local/lib/riscv64-linux-gnu/libdisplay-info.so.2
   Installing symlink pointing to libdisplay-info.so.2 to /usr/local/lib/riscv64-linux-gnu/libdisplay-info.so
   ```
3. Set `PKG_CONFIG_PATH` to contain the path to `libdisplay-info.pc`, for example `/usr/local/lib/riscv64-linux-gnu/pkgconfig` as highlighted in the stdout of Step 2.
   ```shell
   export PKG_CONFIG_PATH="/usr/local/lib/riscv64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH"
   ```
