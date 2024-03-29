#[derive(Debug, clap::Parser)]
#[clap(name = "pacquery", about, author)]
pub struct Args {
    /// List of input packages
    #[arg(required = true)]
    pub pkgnames: Vec<String>,

    /// Repositories
    #[arg(
        default_value = "core,extra,multilib",
        long,
        use_value_delimiter = true
    )]
    pub repos: Vec<String>,

    /// The path to the pacman database, default ( /var/lib/pacman )
    #[arg(long)]
    pub dbpath: Option<String>,
}
