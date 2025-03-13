# EW
(stands for ew why)

A (mostly functioning) server for Love Live! School idol festival 2 MIRACLE LIVE!

## Building

### Linux

#### Requirements
- [perl](https://www.perl.org/get.html) (This is normally pre-installed)
- [rust](https://www.rust-lang.org/tools/install)
- [npm](https://www.npmjs.com/)
- The [libssl-dev](https://packages.debian.org/buster/libssl-dev) package. This will vary across distros.

`apt install -y npm libssl-dev perl`

### Android

#### Requirements
- [Termux](https://termux.dev) ( Termux is an Android terminal emulator and Linux environment app. )
- [perl](https://www.perl.org/get.html) 
- [rust](https://www.rust-lang.org/tools/install)
- [npm](https://www.npmjs.com/)

Launch Termux, and run
`apt install -y nodejs rust perl make`

### Windows

#### Requirements
- [Strawberry Perl](https://strawberryperl.com/)
- [rust](https://www.rust-lang.org/tools/install)

### Packaging/Running

**Build npm:**
`cd webui && npm install && npm run build`

**Build Rust:**
Debug: `cargo run`
Release: `cargo build --release`
