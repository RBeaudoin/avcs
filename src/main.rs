mod cartridge;

use cartridge::Cartridge;
use clap::{Arg, Command};

fn main() {
    let args = Command::new("avcs")
        .about("Atari 2600 emulator")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("rom").help("ROM file to load").required(true))
        .get_matches();

    if let Some(rom) = args.get_one::<String>("rom") {
        let bytes = match std::fs::read(rom) {
            Ok(bytes) => bytes,
            Err(error) => {
                eprintln!("Failed to read ROM '{rom}': {error}");
                std::process::exit(1);
            }
        };
        let cartridge = match Cartridge::new(bytes) {
            Ok(cartridge) => cartridge,
            Err(cartridge::CartridgeError::EmptyRom) => {
                eprintln!("ROM is zero bytes!!");
                std::process::exit(1);
            }
        };
        println!("Loaded {} bytes", cartridge.size_bytes());
    };
}
