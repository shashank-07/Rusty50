use binance::account::{self, Account};
use binance::model::Kline;

use crate::utils::types::*;
use crate::engine::{strategy::*,order::*};
use crate::indicators::{moving_average::*,price_trend::*,support_resistance::*};
pub struct SampleStrategy{
    security: String,
    ind_sr:SupportResistance,
    ind_ma:MovingAverage
}
impl SampleStrategy{

    pub fn new(
        security:String,
        )->SampleStrategy{

            SampleStrategy {
            security,
            ind_sr:SupportResistance::new(3,10000),
            ind_ma:MovingAverage::new(Interval::D1,50)

        }
    }
  
   
}
impl Strategy for SampleStrategy{
     fn execute(&mut self, candle:&Kline,order: &mut Order,acccount:&Account) {

        let strength=&3;
        self.ind_sr.update(candle);

        let price=candle.close.parse::<f32>().unwrap() as i32;
        if self.ind_sr.support_set.contains(&price){
            if self.ind_sr.support_set.get(&price).unwrap()>strength{
                    match order.buy(candle,1.0){
                        Err(e)=>(),
                        _=>()
                    }
            }
        }
        if self.ind_sr.resistance_set.contains(&price) {
            if self.ind_sr.resistance_set.get(&price).unwrap()>&1{
                match order.sell(candle,1.0){
                    Err(e)=>(),
                    _=>()
                }
            }
        }
    }

    fn getSymbol(&self)->&String {
        &self.security
    }
}



