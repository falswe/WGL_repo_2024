pub type NodeId = u8;

#[derive(Debug)]
pub struct SourceRoutingHeader {
    pub hops: Vec<NodeId>,
}
