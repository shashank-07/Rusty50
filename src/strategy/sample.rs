use binance::account::{self, Account};
use binance::model::Kline;

use crate::utils::types::*;
use crate::engine::{strategy::*,order::*};
use crate::indicators::{moving_average::*,price_trend::*,support_resistance::*};
pub struct SampleStrategy{
    security: String,
    indicator_sr:SupportResistance,
    indicator_ma:MovingAverage
}
impl SampleStrategy{

    pub fn new(
        security:String,
        )->SampleStrategy{

            SampleStrategy {
            security,
            indicator_sr:SupportResistance::new(3,10000),
            indicator_ma:MovingAverage::new(Interval::D1,50)

        }
    }
  
   
}
impl Strategy for SampleStrategy{
     fn execute(&mut self, candle:&Kline,order: &mut Order,acccount:&Account) {

        // let strength=&3;
        // self.indicator_sr.update(candle);
        // let ma_val=self.indicator_ma.getValue(candle);

        // let price=candle.close as i32;
        // if self.indicator_sr.support_set.contains(&price){
        //     if self.indicator_sr.support_set.get(&price).unwrap()>strength && candle.close>ma_val {
        //             order.buy(candle,1.0);
        //     }
        // }
        // if self.indicator_sr.resistance_set.contains(&price) {
        //     if self.indicator_sr.resistance_set.get(&price).unwrap()>strength && candle.close<ma_val{
        //         println!("{}",order.last_bought);
        //         order.sell(candle,1.0);
        //     }
        // }
    }

    fn getSymbol(&self)->&String {
        &self.security
    }
}



