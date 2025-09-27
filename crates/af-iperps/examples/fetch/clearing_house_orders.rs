use af_iperps::graphql::GraphQlClientExt as _;
use af_sui_types::Address;
use clap::Parser;
use color_eyre::Result;
use futures::TryStreamExt as _;
use sui_gql_client::queries::GraphQlClientExt as _;
use sui_gql_client::reqwest::ReqwestClient;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "https://sui-testnet.mystenlabs.com/graphql")]
    rpc: String,

    #[arg(long, default_value_t = Address::from_hex_unwrap(
        b"0x4264c07a42f9d002c1244e43a1f0fa21c49e4a25c7202c597b8476ef6bb57113",
    ))]
    ch: Address,

    #[arg(long)]
    asks: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let Args { rpc, ch, asks } = Args::parse();
    let client = ReqwestClient::new(reqwest::Client::default(), rpc.to_owned());

    let package = client.object_type(ch).await?.address;

    tokio::pin!(
        let stream = client.clearing_house_orders(package, ch, None, asks);
    );

    while let Some((order_id, order)) = stream.try_next().await? {
        println!("Order ID: {order_id}");
        println!("{order}");
    }
    Ok(())
}
