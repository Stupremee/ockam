// This node starts a tcp listener, a secure channel listener, and an echoer worker.
// It then runs forever waiting for messages.

use hello_ockam::Echoer;
use ockam::{Context, Entity, Result, SecureChannels, TcpTransport, TrustEveryonePolicy, Vault};

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    ctx.start_worker("echoer", Echoer).await?;

    // Initialize the TCP Transport.
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a TCP listener and wait for incoming connections.
    tcp.listen("127.0.0.1:4000").await?;

    // Create a Vault to safely store secret keys for Bob.
    let vault = Vault::create(&ctx)?;

    // Create an Entity to represent Bob.
    let mut bob = Entity::create(&ctx, &vault)?;

    // Create a secure channel listener for Bob that will wait for requests to
    // initiate an Authenticated Key Exchange.
    bob.create_secure_channel_listener("bob_listener", TrustEveryonePolicy)?;

    // Don't call ctx.stop() here so this node runs forever.
    Ok(())
}
