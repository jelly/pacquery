use clap::Parser;

use pacquery::args::Args;

fn main() {
    let args = Args::parse();
    match pacquery::run(args.pkgnames, args.dbpath, args.repos) {
        Ok(output) => {
            println!("{output}");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Critical failure - pacquery has stopped working");
            eprintln!("Reason: {}", e);
            std::process::exit(1);
        }
    }
}
