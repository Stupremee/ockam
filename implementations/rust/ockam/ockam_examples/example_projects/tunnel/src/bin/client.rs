use ockam::{
    route, Context, Entity, Result, SecureChannels, TcpTransport, TrustEveryonePolicy, Vault, TCP,
};

const OUTLET_NAME: &str = "outlet";

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // get the port arugment
    let port = std::env::args()
        .nth(1)
        .expect("provide the local port as the first argument to the binary");

    // create secure listener
    let vault = Vault::create(&ctx)?;
    let mut me = Entity::create(&ctx, &vault)?;
    me.create_secure_channel_listener("secure_listener", TrustEveryonePolicy)?;

    // start tcp transport
    let tcp = TcpTransport::create(&ctx).await?;

    // create outlet for sending messages to the given port
    tcp.create_outlet(OUTLET_NAME, format!("127.0.0.1:{}", port))
        .await?;

    // tell the server that we want to open a new tunnel
    let route = route![(TCP, "127.0.0.1:8000"), "connection_broker"];
    ctx.send(route, OUTLET_NAME.to_string()).await?;
    let new_port = ctx.receive::<u16>().await?;
    println!("Local port {} is now reachable from new port {}", port, new_port);

    Ok(())
}
