use ockam::{
    Context, Entity, Lease, LeaseProtocolRequest, LeaseProtocolResponse, Result, Routed,
    SecureChannels, TcpTransport, TrustEveryonePolicy, Vault, Worker,
};
use std::time::SystemTime;
use tracing::info;

pub struct Provisioner {}

#[ockam::worker]
impl Worker for Provisioner {
    type Message = String;
    type Context = Context;

    async fn handle_message(
        &mut self,
        ctx: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        let reply = msg.return_route();
        let json = msg.body();

        info!("Request JSON: {}", json);

        let request = LeaseProtocolRequest::from_json(json.as_str()).unwrap();

        info!("Lease Request: {:#?}", request);

        let local_token =
            std::env::var("INFLUX_TOKEN").expect("Please set INFLUX_TOKEN to a valid token");

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;

        let lease = Lease::new(local_token, 1_000_000, now);
        let response = LeaseProtocolResponse::success(lease);
        ctx.send(reply, response.as_json()).await
    }
}

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    // TCP
    let tcp = TcpTransport::create(&ctx).await?;
    tcp.listen("127.0.0.1:4100").await?;

    // Virtual Token Lease Manager
    ctx.start_worker("lease_manager", Provisioner {}).await?;

    // Secure Channel
    let vault = Vault::create(&ctx)?;
    let mut provisioner = Entity::create(&ctx, &vault)?;
    provisioner.create_secure_channel_listener("secure_channel", TrustEveryonePolicy)?;

    Ok(())
}
