use core::fmt;

#[derive(Hash, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum NodeStatus {
    StartingNode,
    IntermediateNode,
    EndingNode,
    NotANode,
}

impl Default for NodeStatus {
    fn default() -> Self {
        NodeStatus::IntermediateNode
    }
}

impl fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeStatus::StartingNode => write!(f, "{}", "Starting Node"),
            NodeStatus::IntermediateNode => write!(f, "{}", "Intermediate Node"),
            NodeStatus::EndingNode => write!(f, "{}", "Ending Node"),
            NodeStatus::NotANode => write!(f, "{}", "Not a Node"),
        }
    }
}
