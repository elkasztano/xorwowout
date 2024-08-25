# Xorwowout

Test program for some pseudo random number generators from the
[xorwowgen](https://github.com/elkasztano/xorwow) crate.

## Clone and compile

_The Rust toolchain is required._

```bash
git clone https://github.com/elkasztano/xorwowout
cd xorwowout
cargo build --release
```

## Usage

The program is intended to be used in conjunction with a PRNG test
tool that accepts data from `stdin`, like
[Dieharder](https://linux.die.net/man/1/dieharder).

### Examples

Navigate to your 'xorwowout' directory.

Inspect some pseudorandom data from the `xorwow128::LargeWrap`
generator:

```bash
target/release/xorwowout LargeWrap | hexdump -n 512
```

Test the `xorwow64::XorB` generator, view and save the result:

```bash
subject="XorB"
target/release/xorwowout $subject | dieharder -a -g 200 | tee ${subject}_result
```

Create file in binary format:

```bash
target/release/xorwowout LargeXor | dd bs=1M count=1 iflag=fullblock of=LargeXor_binary
```

Show help text:

```bash
target/release/xorwowout --help
```

`target/release/xorwowout ` _can be replaced with_ `cargo run --release -- `_._

## Note

If you print the output from the program directly, you may easily mess up your terminal completely, similar to just typing `cat /dev/urandom`. It is therefore recommended to pipe the data to an appropriate program as shown above.
