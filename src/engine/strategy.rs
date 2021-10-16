use crate::utils::candle::*;
use super::order::*;
//Trait which is common to all strategy
pub trait Strategy {
    fn execute(& mut self,candle: &Candle,order: &mut Order);
}

