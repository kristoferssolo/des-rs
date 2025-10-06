mod args;

use crate::args::{Args, Operation};
use clap::Parser;
use des_lib::Des;

fn main() {
    let args = Args::parse();
    let des = Des::new(args.key.as_64());

    match args.operation {
        Operation::Encrypt => {
            let ciphertext = des.encrypt(args.text.as_64());
            println!("{ciphertext:016X}");
        }
        Operation::Decrypt { output_format } => {
            let plaintext = des.decrypt(args.text.as_64());
            println!("{plaintext:016X}");
        }
    }
}
