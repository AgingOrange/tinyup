# TinyUp: an `uptime` alternative for e.g. Polybar

> NOTE: I'm pretty new at this. Any feedback is appreciated!

I needed a way to display the uptime of my system in Polybar, where
screen real estate is expensive. `uptime -p` does what I need, but it's not
as concise as I'd like. So, I wrote this.

The only thing it does is provide an alternative to the output of `uptime -p`,
by calculating the difference between the time returned by `uptime --since`
and the current date/time.

## Example output

```bash
$ uptime -p
up 1 day, 2 hours, 3 minutes

$ tinyup
 1d:02:03
```

As you can see, the output of `tinyup` is *much* shorter.

If the prefix character displays as a square, you need to install
a [Nerd Font](https://www.nerdfonts.com/) to be able to see it. It should show a
heartbeat, or you can change it in `main.rs` and recompile.

## Requirements

This program requires the `uptime` command in the `$PATH`, else, it will
complain with File not found.

## Usage

Put this in `polybar/config.ini`:

```ini
[module/uptime]
type = custom/script
exec = ~/.local/bin/tinyup
interval = 60
format-prefix = " "
```

Put "tinyup" in your bar.

For more information, consult Polybar's documentation
for [scripts](https://github.com/polybar/polybar/wiki/Module:-script). 

## Installation

You will need to have Rust installed. The easiest way is to use `rustup` as
described on the [Rust website](https://www.rust-lang.org/tools/install).

After you've installed Rust, clone this project and from its root directory,
use `cargo build --release` to create the binary (`cargo` is part of the
Rust toolchain). Copy the binary to
somewhere in your path, for example using `cp ./target/release/tinywx ~/.
local/bin/`.
