use async_trait::async_trait;
use simple_log::error;
use snarkos_node::Node::Prover;
//use snarkos_node_messages::NodeType;
use snarkos_node_messages::PuzzleRequest;
//use snarkos_node_router::Routing;
use snarkvm::prelude::{Address, Network, PrivateKey, ViewKey};

use crate::node_type::NodeType;
use snarkos_node_router::Routing;
#[async_trait]
pub trait NodeInterface<N: Network>: Routing<N> {
    /// Returns the node type.
    fn node_type(&self) -> NodeType;

    /// Returns the account private key of the node.
    fn private_key(&self) -> &PrivateKey<N>;

    /// Returns the account view key of the node.
    fn view_key(&self) -> &ViewKey<N>;

    /// Returns the account address of the node.
    fn address(&self) -> Address<N>;

    /// Returns `true` if the node is in development mode.
    fn is_dev(&self) -> bool;

    /// Handles OS signals for the node to intercept and perform a clean shutdown.
    /// Note: Only Ctrl-C is supported; it should work on both Unix-family systems and Windows.
    fn handle_signals(&self) {
        let node = self.clone();
        tokio::task::spawn(async move {
            match tokio::signal::ctrl_c().await {
                Ok(()) => {
                    node.shut_down().await;
                    std::process::exit(0);
                }
                Err(error) => error!("tokio::signal::ctrl_c encountered an error: {}", error),
            }
        });
    }

    /// Shuts down the node.
    async fn shut_down(&self);
}
