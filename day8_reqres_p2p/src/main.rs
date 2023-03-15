/* I am going to tweak the LibP2p library for the request and response functionality */
use futures::StreamExt;
use futures::channel::{mpsc, oneshot};
use futures::{prelude::*, select};

use libp2p::{
    core::{
        upgrade::{read_length_prefixed, write_length_prefixed, ProtocolName},
        Multiaddr,
    },
    identity,
    swarm::{SwarmBuilder, SwarmEvent},
    PeerId,
    request_response::{self, codec, ProtocolSupport, RequestId, ResponseChannel},
};

use async_std::io;
use async_trait::async_trait;

// In order to use request response, we import important modules from libp2p

// use libp2p::request_response::{
//     Codec, Behaviour, Config
// };

use std::{error::Error, time::Duration, iter};

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

    let request_response = request_response::Behaviour::new(
        JsonExchangeCodec(),
        iter::once((JsonExchangeProtocol(), ProtocolSupport::Full)),
        Default::default(),
    );

    // Build the Swarm, connecting the lower layer transport logic with the
    // higher layer network behaviour logic.
    let mut swarm = SwarmBuilder::with_async_std_executor(
        transport,
        request_response,
        local_peer_id,
    )
    .build();

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Listen on all interfaces and whatever port the OS assigns.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    
    // Kick it off.
    loop {
        select! {
            line = stdin.select_next_some() => {},
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {address:?}");
                },
                SwarmEvent::Behaviour(request_response::Event::Message {peer, message}) => {
                    println!("{}", peer);
                    },
                _ => todo!()
            }
            
        }
    }
}


// We have to implement Codec trait for our request response
#[derive(Debug, Clone)]
struct JsonExchangeProtocol();

#[derive(Clone)]
struct JsonExchangeCodec();

#[derive(Debug, Clone, PartialEq, Eq)]
struct JsonRequest(String);

#[derive(Debug, Clone, PartialEq, Eq)]
struct JsonResponse(Vec<u8>);

impl ProtocolName for JsonExchangeProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/json-exchange/1".as_bytes()
    }
}


// libp2p::request_response::codec::Codec

#[async_trait]
impl codec::Codec for JsonExchangeCodec {
    type Protocol = JsonExchangeProtocol;
    type Request = JsonRequest;
    type Response = JsonResponse;

    async fn read_request<T>(
        &mut self,
        _: &JsonExchangeProtocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, 1_000_000).await?;

        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        Ok(JsonRequest(String::from_utf8(vec).unwrap()))
    }

    async fn read_response<T>(
        &mut self,
        _: &JsonExchangeProtocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, 500_000_000).await?; // update transfer maximum

        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        Ok(JsonResponse(vec))
    }

    async fn write_request<T>(
        &mut self,
        _: &JsonExchangeProtocol,
        io: &mut T,
        JsonRequest(data): JsonRequest,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, data).await?;
        io.close().await?;

        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &JsonExchangeProtocol,
        io: &mut T,
        JsonResponse(data): JsonResponse,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, data).await?;
        io.close().await?;

        Ok(())
    }
}