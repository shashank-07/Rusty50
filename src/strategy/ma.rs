use crate::utils::{types::*,candle::*};
use crate::engine::{strategy::*,order::*};
use crate::indicators::moving_average::*;

use binance::account::{self, Account};
use binance::model::Kline;

use ta::indicators::SimpleMovingAverage as Sma;
use ta::DataItem;
use ta::Next;
pub struct MovingAverageStrategy{
    security: String,
    ind:Sma
}
impl MovingAverageStrategy{
    pub fn new(
        security:String,
        interval: Interval,
        data_points:usize
        )->MovingAverageStrategy{
            MovingAverageStrategy {
            security,
            ind:Sma::new(data_points).unwrap()
        }
    }
  
   
}
impl Strategy for MovingAverageStrategy{
     fn execute(&mut self, candle: &Kline,order: &mut Order,acccount:&Account) {
        
        let dt = DataItem::builder()
            .open(candle.open.parse::<f64>().unwrap())
            .high(candle.high.parse::<f64>().unwrap())
            .low(candle.low.parse::<f64>().unwrap())
            .close(candle.close.parse::<f64>().unwrap())
            .volume(candle.volume.parse::<f64>().unwrap())
            .build()
            .unwrap();
        let ma_val = self.ind.next(&dt);
        let close = candle.close.parse::<f64>().unwrap();
        if ma_val>0.0 {
            if close>ma_val {
                match order.buy(candle, 1.0){
                    Err(e)=>(),
                    _=>()
                }
            }else if close<ma_val {
                match order.sell(candle, 1.0){
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



