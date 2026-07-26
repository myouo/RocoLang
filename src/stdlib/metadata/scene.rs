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
    ]
}
