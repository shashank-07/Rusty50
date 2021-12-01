use crate::utils::{types::*,candle::*};
use crate::engine::{strategy::*,order::*};
use crate::indicators::moving_average::*;

use binance::account::{self, Account};
use binance::model::Kline;
pub struct MovingAverageStrategy{
    security: String,
    indicator:MovingAverage
}
impl MovingAverageStrategy{
    pub fn new(
        security:String,
        interval: Interval,
        data_points:usize
        )->MovingAverageStrategy{
            MovingAverageStrategy {
            security,
            indicator:MovingAverage::new(interval,data_points)
        }
    }
  
   
}
impl Strategy for MovingAverageStrategy{
     fn execute(&mut self, candle: &Kline,order: &mut Order,acccount:&Account) {
        

        let ma_val=self.indicator.getValue(candle);
        let close = candle.close.parse::<f32>().unwrap();

        if ma_val>0.0 {
            if close>ma_val {
                match order.buy(candle, 1.0){
                    Err(e)=>(),
                    _=>println!("MA = {}",ma_val)
                }
            }else if close<ma_val {
                match order.sell(candle, 1.0){
                    Err(e)=>(),
                    _=>println!("MA = {}",ma_val)
                }
                
            }  
        }
    }

    fn getSymbol(&self)->&String {
        &self.security
    }
}



