mod generator;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short, default_value_t = 1)]
    count: usize,
    #[arg(long, default_value_t = 'G')]
    chain_id: char,
}

fn main() {
    let args = Args::parse();

    for i in 0..args.count {
        let acc = generator::generate(args.chain_id as u8);

        println!("# {}", i + 1);
        println!("address: {}", acc.address);
        println!("public key: {}", acc.public_key);
        println!("private key: {}", acc.private_key);
        println!("seed: {}", acc.seed);
        println!("---------------------------------------------------");
    }
}
