use serde::Deserialize;
use serde::Serialize;
/// The identity of a raft log.
/// A term and an index identifies an log globally.
#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogId {
    pub term: u64,
    pub index: u64,
}
