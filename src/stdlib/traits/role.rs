use super::*;

pub trait RocoRoleStdLib: Send {
    fn get_items(&mut self) -> Result<Vec<BagItemInfo>> {
        unsupported("role::get_items")
    }

    fn try_change_avatar(&mut self, _avatar: Vec<i64>) -> Result<ActionResult> {
        unsupported("role::try_change_avatar")
    }

    fn try_change_avatar_slot(
        &mut self,
        _avatar_position: i64,
        _avatar_id: i64,
    ) -> Result<ActionResult> {
        unsupported("role::try_change_avatar_slot")
    }
}
