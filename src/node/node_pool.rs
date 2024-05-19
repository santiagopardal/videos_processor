use std::collections::HashMap;
use crate::consumer::node_creation_error::NodeCreationError;
use crate::node;
use crate::node::node::Node;

pub struct NodePool {
    node: HashMap<u32, Node>
}

impl NodePool {
    pub fn new() -> Self {
        let node: HashMap<u32, Node> = HashMap::new();
        return Self { node };
    }

    pub async fn get_node(&mut self, node_id: &u32) -> Result<&mut Node, NodeCreationError> {
        if !self.node.contains_key(&node_id) {
            let fetched_node: Node = node::api::get_node(node_id).await?;
            self.node.insert(node_id.clone(), fetched_node);
        }

        let node: &mut Node = self.node.get_mut(node_id).unwrap();
        node.connect().await?;

        Ok(node)
    }
}
