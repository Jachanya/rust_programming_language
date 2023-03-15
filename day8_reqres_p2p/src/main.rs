/* I am going to tweak the LibP2p library for the request and response functionality */
use futures::StreamExt;

use libp2p::{
    development_transport, identity,
    swarm::{SwarmBuilder, SwarmEvent},
    PeerId,
    core::multiaddr
};

// In order to use request response, we import important modules from libp2p

// use libp2p::request_response::{
//     Codec, Behaviour, Config
// };

use std::{error::Error, time::Duration};

// Swarm class contain the state of the network as a whole. We control
// the entire behaviour of lib p2p using the Swarm. The Swarm struct contains 
// all active and pending connections to remotes and manages the state of all
// substreams.

// To initialize the Swarm we need three things, The identity of the local 
// node, An implementation of the Transport trait. We use the transport to
// communicate with other nodes based on their address. Finally, we the 
// network behaviour trait. The behaviour trait defines how the swarm should
// behave once it connects to a node.

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // here we are trying to create a random key
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    // configuring our transport layer to be tcp

    let transport = libp2p::development_transport(local_key).await?;

    // In this section, we want to explore the request response behaviour in
    // a p2p network

    Ok(())
}
