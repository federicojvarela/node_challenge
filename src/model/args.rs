use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The address of the node to reach to.
    /// `dig seed.bitcoin.sipa.be +short` may provide a fresh list of nodes.
    #[arg(short, long, default_value = "165.22.213.4:8333")]
    pub remote_address: String,

    /// The address of this local node.
    /// This address doesn't matter much as it will be ignored by the bitcoind node
    #[arg(short, long, default_value = "0.0.0.0:0")]
    pub local_address: String,
}
