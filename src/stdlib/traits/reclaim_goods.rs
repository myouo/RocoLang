use super::*;

pub trait RocoReclaimGoodsStdLib: Send {
    fn reclaim_goods_query_goods(&mut self, _goods_type: i64) -> Result<ReclaimGoodsListResult> {
        unsupported("reclaim_goods::query_goods")
    }

    fn reclaim_goods_sell_goods(
        &mut self,
        _goods_type: i64,
        _item_id: i64,
        _count: i64,
    ) -> Result<ReclaimGoodsSellResult> {
        unsupported("reclaim_goods::sell_goods")
    }

    fn reclaim_goods_query_eggs(&mut self) -> Result<ReclaimGoodsEggListResult> {
        unsupported("reclaim_goods::query_eggs")
    }

    fn reclaim_goods_sell_egg(
        &mut self,
        _host_uin: i64,
        _spirit_id: i64,
        _catch_time: i64,
    ) -> Result<ReclaimGoodsSellResult> {
        unsupported("reclaim_goods::sell_egg")
    }

    fn reclaim_goods_sell_goods_batch(
        &mut self,
        _goods_type: i64,
        _items: Vec<ReclaimGoodsItem>,
    ) -> Result<ReclaimGoodsSellResult> {
        unsupported("reclaim_goods::sell_goods_batch")
    }

    fn reclaim_goods_sell_eggs_batch(
        &mut self,
        _eggs: Vec<ReclaimGoodsEgg>,
    ) -> Result<ReclaimGoodsSellResult> {
        unsupported("reclaim_goods::sell_eggs_batch")
    }

    fn reclaim_goods_query_prices(&mut self, _goods_type: i64) -> Result<ReclaimGoodsPriceResult> {
        unsupported("reclaim_goods::query_prices")
    }
}
