use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_array, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("move_to_scene", move |scene_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.move_to_scene(scene_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("try_move_to_scene", move |scene_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.try_move_to_scene(scene_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_current_scene", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_current_scene().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_scene_spirits", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_scene_spirits()
                .map(|spirits| to_array(&spirits))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "try_claim_game_award",
            move |award_id: i64, condition: i64, reward_type: i64| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.try_claim_game_award(award_id, condition, reward_type)
                    .map_err(to_rhai_error)
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("try_mine", move |command: i64, mining_type: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.try_mine(command, mining_type).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("try_challenge_lewei", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.try_challenge_lewei().map_err(to_rhai_error)
        });
    }
    {
        module.set_native_fn("try_hit_monster", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.try_hit_monster().map_err(to_rhai_error)
        });
    }
}
