// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Peer, Router, ALEO_MAXIMUM_FORK_DEPTH};
use snarkos_node_messages::{
    ChallengeRequest,
    ChallengeResponse,
    Data,
    Disconnect,
    DisconnectReason,
    Message,
    MessageCodec,
    MessageTrait,
    NodeType,
    Ping,
    RawStatus,
    Status,
};
use snarkos_node_tcp::{ConnectionSide, Tcp, P2P};
use snarkvm::prelude::{error, Block, FromBytes, Header, Network};

use anyhow::{bail, Result};
use futures::SinkExt;
use std::{io, net::SocketAddr};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

impl<N: Network> P2P for Router<N> {
    /// Returns a reference to the TCP instance.
    fn tcp(&self) -> &Tcp {
        &self.tcp
    }
}

impl<N: Network> Router<N> {
    /// Performs the handshake protocol.
    pub async fn handshake(
        &self,
        peer_addr: SocketAddr,
        stream: &mut TcpStream,
        peer_side: ConnectionSide,
    ) -> io::Result<()> {
        // Construct the stream.
        let mut framed = Framed::new(stream, MessageCodec::<N>::default());

        // Ensure the peer is allowed to connect.
        if let Err(forbidden_message) = self.ensure_peer_is_allowed(peer_addr) {
            return Err(error(format!("{forbidden_message}")));
        }
        debug!("Received a connection request from '{peer_addr}'");

        /* Step 1: Send the challenge request. */

        // Send a challenge request to the peer.
        let message = Message::<N>::ChallengeRequest(ChallengeRequest {
            version: Message::<N>::VERSION,
            fork_depth: ALEO_MAXIMUM_FORK_DEPTH,
            node_type: self.node_type,
            status: self.status.get(),
            listener_port: self.local_ip().port(),
        });
        trace!("Sending '{}-A' to '{peer_addr}'", message.name());
        framed.send(message).await?;

        /* Step 2: Receive the challenge request. */

        // Listen for the challenge request message.
        let challenge_request = match framed.try_next().await? {
            // Received the challenge request message, proceed.
            Some(Message::ChallengeRequest(data)) => data,
            // Received a disconnect message, abort.
            Some(Message::Disconnect(reason)) => return Err(error(format!("'{peer_addr}' disconnected: {reason:?}"))),
            // Received an unexpected message, abort.
            _ => return Err(error(format!("'{peer_addr}' did not send a challenge request"))),
        };
        trace!("Received '{}-B' from '{peer_addr}'", challenge_request.name());

        // Verify the challenge request. If a disconnect reason was returned, send the disconnect message and abort.
        if let Some(reason) = self.verify_challenge_request(peer_addr, &challenge_request) {
            trace!("Sending 'Disconnect' to '{peer_addr}'");
            framed.send(Message::Disconnect(Disconnect { reason: reason.clone() })).await?;
            return Err(error(format!("Dropped '{peer_addr}' for reason: {reason:?}")));
        }

        /* Step 3: Send the challenge response. */

        // TODO (howardwu): Make this step more efficient (by not deserializing every time).
        // Retrieve the genesis block header.
        let genesis_header = *Block::<N>::from_bytes_le(N::genesis_bytes()).map_err(|e| error(e.to_string()))?.header();
        // Send the challenge response.
        let message = Message::ChallengeResponse(ChallengeResponse { header: Data::Object(genesis_header) });
        trace!("Sending '{}-B' to '{peer_addr}'", message.name());
        framed.send(message).await?;

        /* Step 4: Receive the challenge response. */

        // Listen for the challenge response message.
        let challenge_response = match framed.try_next().await? {
            // Received the challenge response message, proceed.
            Some(Message::ChallengeResponse(data)) => data,
            // Received a disconnect message, abort.
            Some(Message::Disconnect(reason)) => return Err(error(format!("'{peer_addr}' disconnected: {reason:?}"))),
            // Received an unexpected message, abort.
            _ => return Err(error(format!("'{peer_addr}' did not send a challenge response"))),
        };
        trace!("Received '{}-A' from '{peer_addr}'", challenge_response.name());

        // Verify the challenge response. If a disconnect reason was returned, send the disconnect message and abort.
        if let Some(reason) = self.verify_challenge_response(peer_addr, challenge_response, genesis_header).await {
            trace!("Sending 'Disconnect' to '{peer_addr}'");
            framed.send(Message::Disconnect(Disconnect { reason: reason.clone() })).await?;
            return Err(error(format!("Dropped '{peer_addr}' for reason: {reason:?}")));
        }

        /* Step 5: Add the peer to the router. */

        // Prepare the peer.
        let peer_ip = match peer_side {
            // The peer initiated the connection.
            ConnectionSide::Initiator => SocketAddr::new(peer_addr.ip(), challenge_request.listener_port),
            // This node initiated the connection.
            ConnectionSide::Responder => peer_addr,
        };
        let peer_version = challenge_request.version;
        let peer_type = challenge_request.node_type;
        let peer_status = RawStatus::from_status(challenge_request.status);

        // Construct the peer.
        let peer = Peer::new(peer_side, peer_ip, peer_version, peer_type, peer_status);
        // Insert the connected peer in the router.
        self.insert_connected_peer(peer, peer_addr);
        info!("Connected to '{peer_ip}'");

        /* Step 6: Send the first ping. */

        // Send the first `Ping` message to the peer.
        let message = Message::Ping(Ping {
            version: Message::<N>::VERSION,
            fork_depth: ALEO_MAXIMUM_FORK_DEPTH,
            node_type: self.node_type,
            status: self.status.get(),
        });
        trace!("Sending '{}' to '{peer_ip}'", message.name());
        framed.send(message).await?;

        Ok(())
    }

    /// Ensure the peer is allowed to connect.
    fn ensure_peer_is_allowed(&self, peer_ip: SocketAddr) -> Result<()> {
        // Ensure the peer IP is not this node.
        if self.is_local_ip(&peer_ip) {
            bail!("Dropping connection request from '{peer_ip}' (attempted to self-connect)")
        }
        // Ensure the node does not surpass the maximum number of peer connections.
        if self.number_of_connected_peers() >= self.max_connected_peers() {
            bail!("Dropping connection request from '{peer_ip}' (maximum peers reached)")
        }
        // Ensure the node is not already connected to this peer.
        if self.is_connected(&peer_ip) {
            bail!("Dropping connection request from '{peer_ip}' (already connected)")
        }
        // Ensure the peer is not restricted.
        if self.is_restricted(&peer_ip) {
            bail!("Dropping connection request from '{peer_ip}' (restricted)")
        }
        // Ensure the peer is not spamming connection attempts.
        if !peer_ip.ip().is_loopback() {
            // Add this connection attempt and retrieve the number of attempts.
            let num_attempts = self.cache.insert_inbound_connection(peer_ip.ip(), Self::RADIO_SILENCE_IN_SECS as i64);
            // Ensure the connecting peer has not surpassed the connection attempt limit.
            if num_attempts > Self::MAXIMUM_CONNECTION_FAILURES {
                // Restrict the peer.
                self.insert_restricted_peer(peer_ip);
                bail!("Dropping connection request from '{peer_ip}' (tried {num_attempts} times)")
            }
        }
        Ok(())
    }

    /// Verifies the given challenge request. Returns a disconnect reason if the request is invalid.
    fn verify_challenge_request(&self, peer_addr: SocketAddr, message: &ChallengeRequest) -> Option<DisconnectReason> {
        // Retrieve the components of the challenge request.
        let &ChallengeRequest { version, fork_depth, node_type, status: peer_status, listener_port: _ } = message;

        // Ensure the message protocol version is not outdated.
        if version < Message::<N>::VERSION {
            warn!("Dropping {peer_addr} on version {version} (outdated)");
            return Some(DisconnectReason::OutdatedClientVersion);
        }

        // Ensure the maximum fork depth is correct.
        if fork_depth != ALEO_MAXIMUM_FORK_DEPTH {
            warn!("Dropping {peer_addr} for an incorrect maximum fork depth of {fork_depth}");
            return Some(DisconnectReason::InvalidForkDepth);
        }

        // If this node is not a beacon node and is syncing, the peer is a beacon node, and this node is ahead, proceed to disconnect.
        if self.node_type != NodeType::Beacon && self.status.is_syncing() && node_type == NodeType::Beacon {
            warn!("Dropping {peer_addr} as this node is ahead");
            return Some(DisconnectReason::YouNeedToSyncFirst);
        }

        // If this node is a beacon node, the peer is not a beacon node and is syncing, and the peer is ahead, proceed to disconnect.
        if self.node_type == NodeType::Beacon && node_type != NodeType::Beacon && peer_status == Status::Syncing {
            warn!("Dropping {peer_addr} as this node is ahead");
            return Some(DisconnectReason::INeedToSyncFirst);
        }

        // TODO (howardwu): Remove this after Phase 2.
        if !self.is_dev
            && self.node_type.is_validator()
            && node_type.is_beacon()
            && peer_addr.ip().to_string() != "159.65.195.225"
        {
            warn!("Dropping {peer_addr} for an invalid node type of {node_type}");
            return Some(DisconnectReason::ProtocolViolation);
        }

        None
    }

    /// Verifies the given challenge response. Returns a disconnect reason if the response is invalid.
    async fn verify_challenge_response(
        &self,
        peer_addr: SocketAddr,
        message: ChallengeResponse<N>,
        genesis_header: Header<N>,
    ) -> Option<DisconnectReason> {
        // Retrieve the components of the challenge response.
        let ChallengeResponse { header } = message;

        // Perform the deferred non-blocking deserialization of the block header.
        let block_header = match header.deserialize().await {
            Ok(block_header) => block_header,
            Err(_) => {
                warn!("Handshake with {peer_addr} failed (cannot deserialize the block header)");
                return Some(DisconnectReason::InvalidChallengeResponse);
            }
        };

        // Verify the challenge response, by checking that the block header matches.
        if block_header != genesis_header {
            warn!("Handshake with {peer_addr} failed (incorrect block header)");
            return Some(DisconnectReason::InvalidChallengeResponse);
        }

        None
    }
}
