# rusty-time

Rusty-time is a command-line tool that helps you keep track of your daily timesheets. Simply log your time as you work, and get a condensed report when you are done!

![rusty-time](https://raw.githubusercontent.com/rideron89/rusty-time/master/screenshot.png)

## Usage

Starting the program is as easy as running the release script:

```bash
$ ./bin/rusty-time
```

## Build

You will need to make sure you have Rust installed if you plan on building the source code.

##### MacOS and Linux

Run these commands to install Rust and setup your environment:

```bash
$ curl https://sh.rustup.rs -sSf | sh    # download and install
$ source $HOME/.cargo/env                # enable Rust without having to log back in
```

##### Windows

You will need to download `rustup-init.exe` from [https://rustup.rs/](https://rustup.rs/).

More detailed instructions can be found at the official docs: [https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html).

---

Once you have Rust installed, you can build and run the application with [Cargo](https://github.com/rust-lang/cargo)

```bash
$ cargo run
```