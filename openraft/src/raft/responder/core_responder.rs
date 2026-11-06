use crate::RaftTypeConfig;
use crate::impls::ProgressResponder;
use crate::raft::ClientWriteResult;
use crate::raft::MembershipCompletion;
use crate::raft::responder::Responder;
use crate::type_config::alias::LogIdOf;
use crate::type_config::alias::WriteResponderOf;

/// The responder used in RaftCore.
///
/// RaftCore use this responder to send response to the caller.
/// It forwards ordinary writes or maps a conditional membership completion at the same apply boundary.
pub(crate) enum CoreResponder<C>
where C: RaftTypeConfig
{
    Progress(ProgressResponder<C, ClientWriteResult<C>>),
    MembershipCompletion(ProgressResponder<C, MembershipCompletion<C>>),
    UserDefined(WriteResponderOf<C>),
}

impl<C> Responder<C, ClientWriteResult<C>> for CoreResponder<C>
where C: RaftTypeConfig
{
    fn on_commit(&mut self, log_id: LogIdOf<C>) {
        match self {
            Self::Progress(responder) => responder.on_commit(log_id),
            Self::MembershipCompletion(responder) => responder.on_commit(log_id),
            Self::UserDefined(responder) => responder.on_commit(log_id),
        }
    }

    fn on_complete(self, res: ClientWriteResult<C>) {
        match self {
            Self::Progress(responder) => responder.on_complete(res),
            Self::MembershipCompletion(responder) => responder.on_complete(match res {
                Ok(response) => MembershipCompletion::Applied(response),
                Err(error) => MembershipCompletion::WriteError(error),
            }),
            Self::UserDefined(responder) => responder.on_complete(res),
        }
    }
}
