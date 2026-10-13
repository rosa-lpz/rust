# Rust - Installation

## Using rustup (Recommended)

It looks like you’re running macOS, Linux, or another Unix-like OS. To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions. See ["Other Installation Methods"](https://forge.rust-lang.org/infra/other-installation-methods.html) if you are on Windows.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



## Notes about Rust installation

### Getting started

If you're just getting started with Rust and would like a more detailed walk-through, see our [getting started](https://rust-lang.org/learn/get-started) page.      

### Toolchain management with `rustup`

  Rust is installed and managed by the  [`rustup`](https://rust-lang.github.io/rustup/)  tool. Rust has a 6-week  [     rapid release process   ](https://github.com/rust-lang/rfcs/blob/master/text/0507-release-channels.md) and supports a  [great   number of platforms](https://forge.rust-lang.org/release/platform-support.html), so there are many builds of Rust available at  any time. `rustup` manages these builds in a consistent way  on every platform that Rust supports, enabling installation of Rust  from the beta and nightly release channels as well as support for  additional cross-compilation targets.

  If you've installed `rustup` in the past, you can update  your installation by running `rustup update`.

  For more information see the  [   `rustup` documentation](https://rust-lang.github.io/rustup/).

### Configuring the `PATH` environment variable

  In the Rust development environment, all tools are installed to the      `~/.cargo/bin`     directory, and this is where you will find the Rust toolchain,  including `rustc`, `cargo`, and `rustup`.

  Accordingly, it is customary for Rust developers to include this  directory in their  [   `PATH` environment variable](https://en.wikipedia.org/wiki/PATH_(variable)). During installation  `rustup` will attempt to configure the `PATH`.  Because of differences between platforms, command shells, and bugs in  `rustup`, the modifications to `PATH` may not  take effect until the console is restarted, or the user is logged out,  or it may not succeed at all.

  If, after installation, running `rustc --version` in the  console fails, this is the most likely reason.

### Uninstall Rust

  If at any point you would like to uninstall Rust, you can run  `rustup self uninstall`.  We'll miss you though!





# References

* https://rust-lang.org/tools/install/