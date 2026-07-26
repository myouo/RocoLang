use super::StdlibFunctionDetails;

pub fn docs() -> Vec<StdlibFunctionDetails> {
    vec![
        super::stdlib_doc!("multi_evolution", "fire_claim_reward", return_type: "MultiEvolutionRewardsInfo", "领取火系多元进化奖励。", params: [], returns: "返回实际获得的奖励。", examples: ["let info = multi_evolution::fire_claim_reward();"]),
        super::stdlib_doc!("multi_evolution", "fire_evolve", return_type: "MultiEvolutionElementEvolveResult", "执行火系多元进化。", params: ["target_index" => "配置中的进化目标序号。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。", "item_count" => "消耗的增压剂数量，范围 0..=3。", "fire_score" => "火焰小游戏获得的火力，范围 1..=100。"], returns: "返回新宠物 ID 和服务端进化结果。", examples: ["let info = multi_evolution::fire_evolve(1, 100, 1, 0, 100);"]),
        super::stdlib_doc!("multi_evolution", "fire_query_booster_item_count", return_type: "MultiEvolutionBoosterItemInfo", "查询火系多元进化增压剂数量。", params: [], returns: "返回增压剂物品 ID 和拥有数量。", examples: ["let info = multi_evolution::fire_query_booster_item_count();"]),
        super::stdlib_doc!("multi_evolution", "fire_query_candidates", return_type: "MultiEvolutionCandidatesInfo", "查询火系多元进化候选宠物。", params: ["target_index" => "配置中的进化目标序号。"], returns: "返回候选宠物列表。", examples: ["let info = multi_evolution::fire_query_candidates(1);"]),
        super::stdlib_doc!("multi_evolution", "fire_query_reward_available", return_type: "MultiEvolutionRewardAvailabilityInfo", "查询火系多元进化奖励是否可领取。", params: [], returns: "返回奖励可领取状态。", examples: ["let info = multi_evolution::fire_query_reward_available();"]),
        super::stdlib_doc!("multi_evolution", "grass_first_evolve", return_type: "MultiEvolutionGrassStageResult", "执行草系多元进化第一阶段。", params: ["target_index" => "配置中的进化目标序号。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。", "sunlight" => "投入的小游戏阳光数量。"], returns: "返回第一阶段提交结果。", examples: ["let info = multi_evolution::grass_first_evolve(1, 100, 1, 20);"]),
        super::stdlib_doc!("multi_evolution", "grass_query_candidates", return_type: "MultiEvolutionCandidatesInfo", "查询草系多元进化候选宠物。", params: ["target_index" => "配置中的进化目标序号。"], returns: "返回候选宠物列表。", examples: ["let info = multi_evolution::grass_query_candidates(1);"]),
        super::stdlib_doc!("multi_evolution", "grass_second_evolve", return_type: "MultiEvolutionGrassEvolveResult", "执行草系多元进化第二阶段。", params: ["target_index" => "配置中的进化目标序号。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。", "sunlight" => "投入的小游戏阳光数量。"], returns: "返回新宠物 ID。", examples: ["let info = multi_evolution::grass_second_evolve(1, 100, 1, 20);"]),
        super::stdlib_doc!("multi_evolution", "water_evolve", return_type: "MultiEvolutionElementEvolveResult", "执行水系多元进化。", params: ["target_index" => "配置中的进化目标序号。", "spirit_id" => "宠物 ID。", "catch_time" => "用于定位宠物实例的捕获时间。"], returns: "返回新宠物 ID 和服务端进化结果。", examples: ["let info = multi_evolution::water_evolve(1, 100, 1);"]),
        super::stdlib_doc!("multi_evolution", "water_query_candidates", return_type: "MultiEvolutionCandidatesInfo", "查询水系多元进化候选宠物。", params: ["target_index" => "配置中的进化目标序号。"], returns: "返回候选宠物列表。", examples: ["let info = multi_evolution::water_query_candidates(1);"]),
    ]
}
