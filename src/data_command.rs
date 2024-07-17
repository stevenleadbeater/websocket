use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, PartialEq)]
pub struct DataCommand<T> {
    pub payload: T,
    pub command_type: CommandType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd)]
pub enum CommandType {
    Create,
    Update,
    Delete,
}
