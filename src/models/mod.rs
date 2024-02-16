use serde::{Deserialize, Serialize};

pub mod thread;
pub mod post;
pub mod user;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Timestamp {
    pub secs_since_epoch: u32,
    pub nanos_since_epoch: u32,
}