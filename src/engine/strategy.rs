use super::order::*;
use binance::{account::*, model::Kline};
//Trait which is common to all strategy
pub trait Strategy {
    fn execute(& mut self,candle: &Kline,order: &mut Order,account:&Account);
    fn getSymbol(&self)->&String;
}

