use ockam::{
    route, Address, Context, Entity, Identity, Result, SecureChannels, TcpTransport,
    TrustEveryonePolicy, Vault, TCP,
};
use ockam_influxdb::{InfluxClient, LeaseManager, LeaseWorker};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // TCP
    let _tcp = TcpTransport::create(&ctx).await?;

    // Secure Channel stuff
    let vault = Vault::create(&ctx)?;

    // Token Lease Manager stuff

    let secure_channel_route = route![(TCP, "localhost:4100"), "secure_channel"];
    let secure_channel = client.create_secure_channel(secure_channel_route, TrustEveryonePolicy)?;

    // Client application stuff
    let api_url = "http://127.0.0.1:8086";
    let org = "ockam";
    let bucket = "ockam-bucket";

    let lease_manager_route = route![secure_channel, "lease_manager"];
    let org_id = "ockam".to_string();
    // TODO: wire up provisioner route and org to Entity options
    let mut client = Entity::create(&ctx, &vault)?;

    let leased_token = client.get_lease(org)?;
    let client = InfluxClient::new(api_url, org, bucket, leased_token.value());
    client.send_metrics().await;
    ctx.stop().await?;
    Ok(())
}
