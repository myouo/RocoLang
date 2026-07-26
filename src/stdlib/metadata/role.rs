use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "role",
            "get_cached_scene_roles",
            return_type: "SceneRoleInfo[]",
            "获取当前缓存的场景角色列表。",
            params: [],
            returns: "场景角色数组。",
            examples: ["let roles = role::get_cached_scene_roles();"]
        ),
        super::stdlib_doc!(
            "role",
            "try_change_avatar",
            return_type: "ActionResult",
            "一次性提交九个装扮槽位。",
            params: ["avatar" => "长度必须为 9 的装扮 ID 数组。"],
            returns: "服务端接受则 ok=true，否则返回业务失败信息。",
            examples: ["let result = role::try_change_avatar([1, 2, 3, 4, 5, 6, 7, 8, 9]);"]
        ),
        super::stdlib_doc!(
            "role",
            "try_change_avatar_slot",
            return_type: "ActionResult",
            "更换单个装扮槽位。",
            params: [
                "avatar_position" => "装扮槽位编号。",
                "avatar_id" => "装扮 ID。"
            ],
            returns: "服务端接受则 ok=true，否则返回业务失败信息。",
            examples: ["let result = role::try_change_avatar_slot(1, 12345);"]
        ),
    ]
}
