use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!(
            "reclaim_goods", "query_goods", return_type: "ReclaimGoodsListResult",
            "查询指定回收分类中的可回收物品。只能在非战斗状态使用。",
            params: ["goods_type" => "服务端回收物分类。"],
            returns: "物品列表、余额、安全码状态和服务端提示。",
            examples: ["let goods = reclaim_goods::query_goods(1);"]
        ),
        super::stdlib_doc!(
            "reclaim_goods", "sell_goods", return_type: "ReclaimGoodsSellResult",
            "出售一种物品。只能在非战斗状态使用。",
            params: ["goods_type" => "服务端回收物分类。", "item_id" => "物品 ID。", "count" => "出售数量。"],
            returns: "出售后的余额和服务端提示。",
            examples: ["let result = reclaim_goods::sell_goods(1, 1001, 2);"]
        ),
        super::stdlib_doc!(
            "reclaim_goods", "query_eggs", return_type: "ReclaimGoodsEggListResult",
            "查询可回收的宠物蛋。只能在非战斗状态使用。",
            params: [], returns: "宠物蛋列表、余额和安全码状态。",
            examples: ["let eggs = reclaim_goods::query_eggs();"]
        ),
        super::stdlib_doc!(
            "reclaim_goods", "sell_egg", return_type: "ReclaimGoodsSellResult",
            "出售一个由来源账号、宠物 ID 和捕获时间唯一确定的宠物蛋。只能在非战斗状态使用。",
            params: ["host_uin" => "宠物蛋来源账号 UIN。", "spirit_id" => "宠物 ID。", "catch_time" => "捕获时间。"],
            returns: "出售后的余额和服务端提示。",
            examples: ["let result = reclaim_goods::sell_egg(123, 19, 456);"]
        ),
        super::stdlib_doc!(
            "reclaim_goods", "sell_goods_batch", return_type: "ReclaimGoodsSellResult",
            "批量出售最多 15 组查询得到的物品。只能在非战斗状态使用。",
            params: ["goods_type" => "服务端回收物分类。", "items" => "ReclaimGoodsItem 数组。"],
            returns: "出售后的余额和服务端提示。",
            examples: ["let goods = reclaim_goods::query_goods(1); let result = reclaim_goods::sell_goods_batch(1, goods.items);"]
        ),
        super::stdlib_doc!(
            "reclaim_goods", "sell_eggs_batch", return_type: "ReclaimGoodsSellResult",
            "批量出售最多 15 个查询得到的宠物蛋。只能在非战斗状态使用。",
            params: ["eggs" => "ReclaimGoodsEgg 数组。"],
            returns: "出售后的余额和服务端提示。",
            examples: ["let eggs = reclaim_goods::query_eggs(); let result = reclaim_goods::sell_eggs_batch(eggs.eggs);"]
        ),
        super::stdlib_doc!(
            "reclaim_goods", "query_prices", return_type: "ReclaimGoodsPriceResult",
            "查询指定回收分类的物品单价。只能在非战斗状态使用。",
            params: ["goods_type" => "服务端回收物分类；农场收获物使用 7。"],
            returns: "物品单价列表和安全码状态。",
            examples: ["let prices = reclaim_goods::query_prices(7);"]
        ),
    ]
}
