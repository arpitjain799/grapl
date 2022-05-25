use derive_dynamic_node::{
    GraplStaticId,
    NodeDescription,
};
use rust_proto::graph_descriptions::*;

#[derive(Debug, Clone, PartialEq, Hash, NodeDescription, GraplStaticId)]
struct IpV6Address {
    #[grapl(static_id, immutable)]
    address: String,
}

impl IIpV6AddressNode for IpV6AddressNode {
    fn get_mut_dynamic_node(&mut self) -> &mut NodeDescription {
        &mut self.dynamic_node
    }

    fn get_dynamic_node(&self) -> &NodeDescription {
        &self.dynamic_node
    }
}
