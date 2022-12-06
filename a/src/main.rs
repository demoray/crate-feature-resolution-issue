use clap::Parser;

#[derive(Parser, Debug)]
struct Config {
    /// The first argument
    a: String,
    /// The second argument
    b: u64,
}

fn main() {
    let config = Config::parse();
    println!("config: {:?}", config);
}
