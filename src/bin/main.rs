use clap::Parser;
use recheck::compute_recheck_hash;

#[derive(Parser)]
#[command(name = "recheck")]
#[command(about = "Hash reproductible vérifié", long_about = None)]
struct Cli {
    path: String,
}

fn main() {
    let cli = Cli::parse();

    match compute_recheck_hash(&cli.path) {
        Ok(hash) => println!("Hash reproductible : {}", hash),
        Err(e) => eprintln!("Erreur : {}", e),
    }
}
