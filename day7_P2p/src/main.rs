// The goal of this session is to build a ping clone using a p2p network
use futures::prelude::*;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult};
use libp2p::swarm::{keep_alive, NetworkBehaviour, Swarm, SwarmEvent};
use libp2p::{
    development_transport, identity,
    swarm::{SwarmBuilder},
    PeerId,
};
use libp2p::{ping, Multiaddr};
use std::{env, error::Error, time::Duration};

const BOOTNODES: [&str; 4] = [
    "QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
    "QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
    "QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
    "QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
];


#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    // we define a transport protocol to be used
    // configures how bytes are sent over the network

    let transport = libp2p::development_transport(local_key).await?;

    // we define a network behaiviour
    // We configure what bytes are sent over the network    

    // let behaviour = Behaviour::default();
    

    // let mut swarm = Swarm::with_async_std_executor(transport, behaviour, local_peer_id);

    // Create a swarm to manage peers and events.
    let mut swarm = {
        // Create a Kademlia behaviour.
        let mut cfg = KademliaConfig::default();
        cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = MemoryStore::new(local_peer_id);
        let mut behaviour = Kademlia::with_config(local_peer_id, store, cfg);

        // Add the bootnodes to the local routing table. `libp2p-dns` built
        // into the `transport` resolves the `dnsaddr` when Kademlia tries
        // to dial these nodes.
        for peer in &BOOTNODES {
            behaviour.add_address(&peer.parse()?, "/dnsaddr/bootstrap.libp2p.io".parse()?);
        }

        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
    };

    // Order Kademlia to search for a peer.
    let to_search = env::args()
        .nth(1)
        .map(|p| p.parse())
        .transpose()?
        .unwrap_or_else(PeerId::random);

    println!("Searching for the closest peers to {to_search}");
    swarm.behaviour_mut().get_closest_peers(to_search);

    // swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    // if let Some(addr) = std::env::args().nth(1) {
    //     let remote: Multiaddr = addr.parse()?;
    //     swarm.dial(remote)?;
    //     println!("Dialed {addr}")
    // }

    loop {
        let event = swarm.select_next_some().await;
        if let SwarmEvent::Behaviour(KademliaEvent::OutboundQueryProgressed {
            result: QueryResult::GetClosestPeers(result),
            ..
        }) = event
        {
            match result {
                Ok(ok) => {
                    if !ok.peers.is_empty() {
                        println!("Query finished with closest peers: {:#?}", ok.peers)
                    } else {
                        // The example is considered failed as there
                        // should always be at least 1 reachable peer.
                        println!("Query finished with no closest peers.")
                    }
                }
                Err(GetClosestPeersError::Timeout { peers, .. }) => {
                    if !peers.is_empty() {
                        println!("Query timed out with closest peers: {peers:#?}")
                    } else {
                        // The example is considered failed as there
                        // should always be at least 1 reachable peer.
                        println!("Query timed out with no closest peers.");
                    }
                }
            };

            break;
        }
    }

    Ok(())

    // loop {
    //     match swarm.select_next_some().await {
    //         SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
    //         SwarmEvent::Behaviour(event) => println!("{event:?}"),
    //         _ => {}
    //     }
    // }
}