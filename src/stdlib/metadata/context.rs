use super::{registered, StdlibFunctionContext, StdlibFunctionRegistration};

pub fn stdlib_function_context(module: &str, name: &str) -> Option<StdlibFunctionContext> {
    registered::FUNCTIONS
        .iter()
        .find(|registration| registration.module == module && registration.name == name)
        .map(classify_function_context)
}

pub(super) fn classify_function_context(
    registration: &StdlibFunctionRegistration,
) -> StdlibFunctionContext {
    let module = registration.module;
    let name = registration.name;

    if module == "combat" {
        return match name {
            "use_skill"
            | "try_use_skill"
            | "use_item"
            | "try_use_item"
            | "change_spirit"
            | "try_change_spirit"
            | "escape"
            | "try_escape"
            | "get_my_hp"
            | "get_my_max_hp"
            | "get_rival_hp"
            | "get_rival_max_hp"
            | "get_my_pp"
            | "get_current_round"
            | "get_actions"
            | "get_state"
            | "get_lineup"
            | "get_my_spirit_info"
            | "get_rival_spirit_info"
            | "can_use_skill"
            | "can_use_item"
            | "can_change_to_spirit"
            | "can_capture" => StdlibFunctionContext::ActiveCombat,
            "wait_next_action"
            | "wait_round_end"
            | "get_history"
            | "try_use_skill_and_wait"
            | "try_change_spirit_and_wait"
            | "try_escape_and_wait" => StdlibFunctionContext::CombatActiveOrTerminal,
            "get_action_snapshot" | "is_finished" => StdlibFunctionContext::Any,
            _ => StdlibFunctionContext::OutOfCombat,
        };
    }

    if module == "scene" {
        return match name {
            "get_current_scene" | "get_scene_spirits" => StdlibFunctionContext::Any,
            _ => StdlibFunctionContext::OutOfCombat,
        };
    }

    if module == "spirit" && name == "take_pushed_drops" {
        return StdlibFunctionContext::Any;
    }

    if module == "role" && name == "get_items" {
        return StdlibFunctionContext::OutOfCombat;
    }

    if matches!(
        module,
        "combat_result"
            | "combat_status"
            | "game"
            | "lookup"
            | "daily_memory"
            | "personality"
            | "profile"
            | "remote_state"
            | "role"
            | "session"
            | "spirit_book"
            | "system"
            | "weather"
    ) {
        return StdlibFunctionContext::Any;
    }

    StdlibFunctionContext::OutOfCombat
}
