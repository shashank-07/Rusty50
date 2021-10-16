use std::collections::HashMap;

use crate::utils::{types::*,candle::*};
use crate::engine::{strategy::*,order::*};
use crate::indicators::moving_average::*;
pub struct MovingAverage{
    security: String,
    sec_type: security_type,
    indicator:moving_average
}
impl MovingAverage{
    pub fn getName(&self)->&String{
        &self.security
    }
    pub fn new(
        security:String,
        sec_type:security_type,
        interval: Interval,
        data_points:usize
        )->MovingAverage{
        MovingAverage {
            security,
            sec_type,
            indicator:moving_average::new(interval,data_points)
        }
    }
  
   
}
impl Strategy for MovingAverage{
     fn execute(&mut self, candle: &Candle,order: &mut Order) {
        

        let ma_val=self.indicator.getValue(candle);
        if(ma_val>0.0){
            if(candle.close>ma_val){
                order.buy(candle, 1.0);
            }else if(candle.close<ma_val){
                order.sell(candle, 1.0);
                
            }  
        }
    }
}



