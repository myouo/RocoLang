use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "scene",
            "get_current_scene",
            return_type: "int",
            "返回当前场景 ID。",
            params: [],
            returns: "当前场景 ID。",
            examples: ["let scene_id = scene::get_current_scene();"]
        ),
        super::stdlib_doc!(
            "scene",
            "move_to_scene",
            return_type: "bool",
            "提交场景切换并返回服务端确认的场景 ID。",
            params: ["scene_id" => "目标场景 ID。"],
            returns: "服务端确认的场景 ID。",
            examples: ["scene::move_to_scene(72);"]
        ),
        super::stdlib_doc!(
            "scene",
            "try_move_to_scene",
            return_type: "ActionResult",
            "尝试移动到指定场景，失败时返回结构化结果。",
            params: ["scene_id" => "目标场景 ID。"],
            returns: "操作结果。",
            examples: ["let result = scene::try_move_to_scene(72);"]
        ),
        super::stdlib_doc!(
            "scene",
            "get_scene_spirits",
            return_type: "SceneSpiritInfo[]",
            "查询当前场景宠物刷新信息。",
            params: [],
            returns: "当前场景宠物列表。",
            examples: ["let spirits = scene::get_scene_spirits();"]
        ),
        super::stdlib_doc!(
            "scene",
            "try_claim_game_award",
            return_type: "ActionResult",
            "尝试领取当前场景中的旧版场景互动奖励。",
            params: [
                "award_id" => "场景奖励 ID。",
                "condition" => "奖励条件编号。",
                "reward_type" => "奖励类型编号。"
            ],
            returns: "仅允许每日取物表中与当前场景匹配的参数；业务拒绝不会抛错。",
            examples: ["let result = scene::try_claim_game_award(55, 1, 1);"]
        ),
        super::stdlib_doc!(
            "scene",
            "try_mine",
            return_type: "ActionResult",
            "尝试执行一次旧版矿场动作。",
            params: [
                "command" => "矿场动作：1 开始，2 结算。",
                "mining_type" => "矿点类型 1/2/3，必须与当前矿场场景匹配。"
            ],
            returns: "矿场动作结果；业务拒绝不会抛错。",
            examples: ["let result = scene::try_mine(1, 1);"]
        ),
        super::stdlib_doc!(
            "scene",
            "try_challenge_lewei",
            return_type: "ActionResult",
            "尝试执行旧版乐维挑战取物动作。",
            params: [],
            returns: "取物结果；业务拒绝不会抛错。",
            examples: ["let result = scene::try_challenge_lewei();"]
        ),
        super::stdlib_doc!(
            "scene",
            "try_hit_monster",
            return_type: "ActionResult",
            "尝试在场景 110 执行旧版击打怪物取物动作。",
            params: [],
            returns: "取物结果；业务拒绝不会抛错。",
            examples: ["let result = scene::try_hit_monster();"]
        ),
    ]
}
