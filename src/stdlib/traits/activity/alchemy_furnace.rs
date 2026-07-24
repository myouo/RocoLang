use super::super::*;

pub trait RocoAlchemyActivityStdLib: Send {
    fn alchemy_furnace_monkey_cultivation_query(&mut self) -> Result<MonkeyCultivationInfo> {
        unsupported("alchemy_furnace::monkey_cultivation_query")
    }
    fn alchemy_furnace_submit_monkey_cultivation_default(
        &mut self,
    ) -> Result<MonkeyCultivationInfo> {
        unsupported("alchemy_furnace::submit_monkey_cultivation_default")
    }
    fn alchemy_furnace_submit_monkey_cultivation_pills(
        &mut self,
        _dragon_tiger: bool,
        _cultivate_origin: bool,
        _nine_turn: bool,
    ) -> Result<MonkeyCultivationInfo> {
        unsupported("alchemy_furnace::submit_monkey_cultivation_pills")
    }
    fn alchemy_furnace_monkey_cultivation_claim_gift(&mut self) -> Result<MonkeyCultivationInfo> {
        unsupported("alchemy_furnace::monkey_cultivation_claim_gift")
    }
    fn alchemy_furnace_monkey_evo_query(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_query")
    }
    fn alchemy_furnace_submit_monkey_evo_combat(
        &mut self,
        _fight_type: i64,
    ) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::submit_monkey_evo_combat")
    }
    fn alchemy_furnace_monkey_evo_give_up(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_give_up")
    }
    fn alchemy_furnace_submit_monkey_evo_default(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::submit_monkey_evo_default")
    }
    fn alchemy_furnace_submit_monkey_evo_pills(
        &mut self,
        _dragon_tiger: bool,
        _cultivate_origin: bool,
        _nine_turn: bool,
    ) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::submit_monkey_evo_pills")
    }
    fn alchemy_furnace_monkey_evo_query_bag(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_query_bag")
    }
    fn alchemy_furnace_monkey_evo_evolve(&mut self, _catch_time: i64) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_evolve")
    }
    fn alchemy_furnace_monkey_evo_claim_gift(&mut self) -> Result<MonkeyEvoInfo> {
        unsupported("alchemy_furnace::monkey_evo_claim_gift")
    }
    fn alchemy_furnace_raging_fire_query(&mut self) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_query")
    }
    fn alchemy_furnace_submit_raging_fire_stone(&mut self, _count: i64) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::submit_raging_fire_stone")
    }
    fn alchemy_furnace_submit_raging_fire_combat(
        &mut self,
        _target: i64,
    ) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::submit_raging_fire_combat")
    }
    fn alchemy_furnace_raging_fire_buy(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_buy")
    }
    fn alchemy_furnace_raging_fire_query_bag(&mut self) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_query_bag")
    }
    fn alchemy_furnace_raging_fire_claim_gift(
        &mut self,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<RagingFireInfo> {
        unsupported("alchemy_furnace::raging_fire_claim_gift")
    }
}
