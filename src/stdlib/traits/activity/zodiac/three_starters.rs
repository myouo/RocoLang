use super::super::super::*;

pub trait RocoThreeStartersActivityStdLib: Send {
    fn three_starters_water_query(&mut self) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_query")
    }
    fn three_starters_water_buy(&mut self, _catch_time: i64) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_buy")
    }
    fn three_starters_submit_water_combat(&mut self) -> Result<WaterSourceInfo> {
        unsupported("three_starters::submit_water_combat")
    }
    fn three_starters_submit_water(&mut self) -> Result<WaterSourceInfo> {
        unsupported("three_starters::submit_water")
    }
    fn three_starters_water_claim_gift(&mut self, _catch_time: i64) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_claim_gift")
    }
    fn three_starters_water_query_bag(&mut self) -> Result<WaterSourceInfo> {
        unsupported("three_starters::water_query_bag")
    }
    fn three_starters_fire_query(&mut self) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_query")
    }
    fn three_starters_fire_buy(&mut self, _catch_time: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_buy")
    }
    fn three_starters_fire_start(&mut self, _index: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_start")
    }
    fn three_starters_submit_fire_combat(&mut self, _index: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::submit_fire_combat")
    }
    fn three_starters_submit_fire_direct(&mut self, _index: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::submit_fire_direct")
    }
    fn three_starters_fire_over(&mut self, _index: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_over")
    }
    fn three_starters_fire_claim_gift(&mut self, _catch_time: i64) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_claim_gift")
    }
    fn three_starters_fire_query_bag(&mut self) -> Result<FiresWillInfo> {
        unsupported("three_starters::fire_query_bag")
    }
    fn three_starters_sun_query(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_query")
    }
    fn three_starters_sun_buy(&mut self, _catch_time: i64) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_buy")
    }
    fn three_starters_sun_start_combat(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_start_combat")
    }
    fn three_starters_submit_sun_combat(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::submit_sun_combat")
    }
    fn three_starters_sun_start_collect(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_start_collect")
    }
    fn three_starters_submit_sun(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::submit_sun")
    }
    fn three_starters_sun_claim_gift(&mut self, _catch_time: i64) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_claim_gift")
    }
    fn three_starters_sun_query_bag(&mut self) -> Result<BatheSunInfo> {
        unsupported("three_starters::sun_query_bag")
    }
}
