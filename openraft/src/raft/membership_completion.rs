use std::fmt;
use std::sync::Arc;

use crate::RaftTypeConfig;
use crate::errors::ClientWriteError;
use crate::raft::ClientWriteResponse;
use crate::type_config::alias::StoredMembershipOf;

/// The outcome of conditionally completing the Core's current joint membership.
pub enum MembershipCompletion<C: RaftTypeConfig> {
    /// The effective membership was uniform when Core handled the request. No entry was proposed.
    /// The committed membership is returned separately: uniform does not imply committed.
    Unchanged {
        effective: Arc<StoredMembershipOf<C>>,
        committed: Arc<StoredMembershipOf<C>>,
    },
    /// Core completed its recorded joint target, and the uniform entry has been applied.
    Applied(ClientWriteResponse<C>),
    /// The membership write failed, with the same semantics as an ordinary membership write.
    WriteError(ClientWriteError<C>),
}

impl<C> fmt::Debug for MembershipCompletion<C>
where
    C: RaftTypeConfig,
    C::R: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unchanged { effective, committed } => {
                f.debug_struct("Unchanged").field("effective", effective).field("committed", committed).finish()
            }
            Self::Applied(response) => f.debug_tuple("Applied").field(response).finish(),
            Self::WriteError(error) => f.debug_tuple("WriteError").field(error).finish(),
        }
    }
}
