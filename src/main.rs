use rand_core::{SeedableRng, RngCore};
use xorwowgen::xorwow128::*;
use xorwowgen::xorwow64::*;
use std::env;
use std::io::{self, Write, Error, ErrorKind};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(a) => {
            match a.as_str() {
                "LargeWrap" => {
                    let mut rng = LargeWrap::from_entropy();
                    rng2stdout(&mut rng)?;
                },
                "LargeXor" => {
                    let mut rng = LargeXor::from_entropy();
                    rng2stdout(&mut rng)?;
                },
                "XorA" => {
                    let mut rng = XorA::from_entropy();
                    rng2stdout(&mut rng)?;
                },
                "XorB" => {
                    let mut rng = XorB::from_entropy();
                    rng2stdout(&mut rng)?;
                }
                "WrapA" => {
                    let mut rng = WrapA::from_entropy();
                    rng2stdout(&mut rng)?;
                }
                "WrapB" => {
                    let mut rng = WrapB::from_entropy();
                    rng2stdout(&mut rng)?;
                },

                "help" => show_help(args.get(0)),
                "-h" => show_help(args.get(0)),
                "--help" => show_help(args.get(0)),
                
                _ => {
                    eprintln!("\x1b[0;7m unknown generator \x1b[0m\n\
                        type '{} help' to show help text",
                        args.get(0).unwrap_or(&"".to_string()));
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
            }
        },
        None => {
            eprintln!("\x1b[0;7m please provide an argument \x1b[0m");
            return Err(Error::from(ErrorKind::InvalidInput));
        }
    }

    Ok(())
}

fn rng2stdout<T: RngCore>(rng: &mut T) -> std::io::Result<()> {
    let mut buf = [0u8; 4];

    loop {
        rng.fill_bytes(&mut buf);
        match io::stdout().write_all(&buf) {
            Ok(()) => {},
            Err(_) => {
                eprintln!("\x1b[0;1;7m PIPE FINISH \x1b[0m");
                break;
            },
        }
    }
    Ok(())
}

fn show_help(bin_name: Option<&String>) {
    let alt = "".to_string();
    let bn = bin_name.unwrap_or(&alt);
    eprintln!("USAGE: {} rng_to_test\n\n\
        Supported rng's:\n\
        LargeWrap, LargeXor, XorA, XorB, WrapA, WrapB",
        bn);
}
