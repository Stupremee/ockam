pub mod json_proto;

pub use json_proto::*;

use serde::{Deserialize, Serialize};
pub type TTL = usize;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Lease {
    value: String,
    ttl: TTL,
    issued: usize,
}

impl Lease {
    pub fn new<S: ToString>(value: S, ttl: usize, issued: usize) -> Self {
        Lease {
            value: value.to_string(),
            ttl,
            issued,
        }
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn ttl(&self) -> TTL {
        self.ttl
    }

    pub fn issued(&self) -> usize {
        self.issued
    }

    pub fn invalid(&self) -> bool {
        self.value.is_empty() || self.ttl == 0 || self.issued == 0
    }

    pub fn is_valid(&self, now: usize) -> bool {
        !self.invalid() && self.issued + self.ttl > now
    }
}
