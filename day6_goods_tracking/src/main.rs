use day6_goods_tracking::consumers;
use futures::prelude::*;
use libp2p::swarm::{keep_alive, NetworkBehaviour, Swarm, SwarmEvent};
use libp2p::{identity, ping, Multiaddr, PeerId};
use std::error::Error;
/// A system to build goods and food tracking
/// 
/// Important actors: 
///     - Consumers
///     - Merchants
///     - Sellers
/// 
/// The Merchants and the sellers commumicate using a P2P connections

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    
    let consumer1 = consumers::Consumer::new(
        1234456,
        "John".to_owned(), 
        "Achanya".to_owned(),
        "Behind yoruba mosque".to_owned());

    let consumer1_name = consumer1.get_name();

    println!("Hello, world!: {}", consumer1_name);
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    let transport = libp2p::development_transport(local_key).await?;

    let behaviour = Behaviour::default();

    let mut swarm = Swarm::with_async_std_executor(transport, behaviour, local_peer_id);

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {}
        }
    }
}

/// Our network behaviour.
///
/// For illustrative purposes, this includes the [`KeepAlive`](behaviour::KeepAlive) behaviour so a continuous sequence of
/// pings can be observed.
#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}