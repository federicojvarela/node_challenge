use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The address of the node to reach to.
    #[arg(short, long, default_value = "165.22.213.4:8333")]
    pub remote_address: String,

    /// The address of this local node.
    #[arg(short, long, default_value = "0.0.0.0:0")]
    pub local_address: String,
}