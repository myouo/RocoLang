use super::*;

/// Free player-versus-player invitation and waiting APIs.
pub trait RocoPkStdLib: Send {
    fn pk_invite(&mut self, _target_uin: i64) -> Result<BattleInfo> {
        unsupported("pk::invite")
    }

    fn pk_accept(&mut self) -> Result<bool> {
        unsupported("pk::accept")
    }

    fn pk_reject(&mut self) -> Result<bool> {
        unsupported("pk::reject")
    }

    fn pk_cancel_waiting(&mut self) -> Result<ActionResult> {
        unsupported("pk::cancel_waiting")
    }
}
