//Moving average indicator calculates mpoving average of the last n (n = period) candles, returns 0 if period is less than 50.

//the latest candles is stored in queue so when we are going to n+1 candle 1st candle gets popped out
use crate::utils::{candle::*, types::*};


use binance::model::Kline;
use queues::*;

pub struct MovingAverage{
    interval: Interval,
    period:usize,
    value:f32,
    prices:Queue<f32>,
}

impl MovingAverage{
    pub fn new(
        interval: Interval,
        period:usize,
    )->MovingAverage{
        MovingAverage{
            interval,
            period,
            value:0.0,
            prices:queue![]
        }
    }
    pub fn getValue(&mut self,candle:&Kline)->f32{
        let mut res=0.0;
        let close = candle.close.parse::<f32>().unwrap();

        if self.prices.size()<self.period {
            self.prices.add(close).unwrap();
            self.value+=close;
            
            
        }else{
            res=self.value/(self.prices.size() as f32);
            match self.prices.remove(){
                Ok(v)=>{
                    self.value-=v;

                },
                Err(e)=>{
                    println!("queue empty {}",e);
                    return 0.0;
                }
            }
            self.prices.add(close).unwrap();
         
            self.value+=close;
            

        }
        res
    }
}
