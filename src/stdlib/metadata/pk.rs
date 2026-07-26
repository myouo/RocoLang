use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "pk",
            "invite",
            return_type: "BattleInfo",
            "向指定玩家发起自由 PK 邀请。",
            params: ["target_uin" => "目标玩家 uin。"],
            returns: "战斗邀请信息。接受后战斗由 combat namespace 操作。",
            examples: ["let battle = pk::invite(123456);"]
        ),
        super::stdlib_doc!(
            "pk",
            "accept",
            return_type: "bool",
            "接受当前自由 PK 邀请。",
            params: [],
            returns: "接受成功返回 true。",
            examples: ["pk::accept();"]
        ),
        super::stdlib_doc!(
            "pk",
            "reject",
            return_type: "bool",
            "拒绝当前自由 PK 邀请。",
            params: [],
            returns: "拒绝成功返回 true。",
            examples: ["pk::reject();"]
        ),
        super::stdlib_doc!(
            "pk",
            "cancel_waiting",
            return_type: "ActionResult",
            "取消当前自由 PK 等待。",
            params: [],
            returns: "服务端接受则 ok=true，否则返回业务失败信息。",
            examples: ["let result = pk::cancel_waiting();"]
        ),
    ]
}
