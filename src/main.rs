use clap::Parser;

use pacinfo::args::Args;

fn main() {
    let args = Args::parse();
    match pacinfo::run(args.pkgnames, args.dbpath, args.repos) {
        Ok(output) => {
            println!("{output}");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Critical failure - pacinfo has stopped working");
            eprintln!("Reason: {}", e);
            std::process::exit(1);
        }
    }
}
