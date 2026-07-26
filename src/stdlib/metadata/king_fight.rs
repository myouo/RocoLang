use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![super::stdlib_doc!(
        "king_fight",
        "cancel_match",
        return_type: "ActionResult",
        "取消当前王者争夺战匹配等待。",
        params: [],
        returns: "服务端接受则 ok=true，否则返回业务失败信息。",
        examples: ["let result = king_fight::cancel_match();"]
    )]
}
