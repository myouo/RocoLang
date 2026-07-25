use super::{field, StdlibFieldDoc};

pub(super) fn doc(type_name: &str) -> Option<(&'static str, Vec<StdlibFieldDoc>)> {
    match type_name {
        "ReclaimGoodsItem" => Some((
            "可回收物品。",
            vec![
                field("item_id", "int", "物品 ID。"),
                field("count", "int", "持有数量。"),
                field("unit_price", "int", "当前单价。"),
                field("previous_price", "int", "原单价。"),
                field("status", "int", "服务端状态。"),
            ],
        )),
        "ReclaimGoodsEgg" => Some((
            "可回收宠物蛋的唯一标识。",
            vec![
                field("spirit_id", "int", "宠物 ID。"),
                field("catch_time", "int", "捕获时间。"),
                field("host_uin", "int", "来源账号 UIN。"),
            ],
        )),
        "ReclaimGoodsPrice" => Some((
            "回收物单价。",
            vec![
                field("item_id", "int", "物品 ID。"),
                field("unit_price", "int", "回收单价。"),
            ],
        )),
        "ReclaimGoodsListResult" => Some((
            "可回收物品查询结果。",
            vec![
                field("balance", "RocoOptionalI64", "响应包含的余额。"),
                field("tips", "string", "服务端提示。"),
                field("safe_code_open", "bool", "是否开启安全码。"),
                field("safe_code_required", "bool", "本次操作是否需要安全码。"),
                field("items", "ReclaimGoodsItem[]", "可回收物品。"),
            ],
        )),
        "ReclaimGoodsEggListResult" => Some((
            "可回收宠物蛋查询结果。",
            vec![
                field("balance", "RocoOptionalI64", "响应包含的余额。"),
                field("safe_code_open", "bool", "是否开启安全码。"),
                field("safe_code_required", "bool", "本次操作是否需要安全码。"),
                field("eggs", "ReclaimGoodsEgg[]", "可回收宠物蛋。"),
            ],
        )),
        "ReclaimGoodsPriceResult" => Some((
            "回收价格查询结果。",
            vec![
                field("safe_code_open", "bool", "是否开启安全码。"),
                field("safe_code_required", "bool", "本次操作是否需要安全码。"),
                field("items", "ReclaimGoodsPrice[]", "物品单价。"),
            ],
        )),
        "ReclaimGoodsSellResult" => Some((
            "回收出售结果。",
            vec![
                field("balance", "RocoOptionalI64", "出售后的余额。"),
                field("tips", "string", "服务端提示。"),
            ],
        )),
        _ => None,
    }
}
