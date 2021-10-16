//Moving average indicator calculates mpoving average of the last 50 candles, returns 0 if period is less than 50.

//the latest candles is stored in queue so when we are going to 51st candle 1st candle gets popped out
use crate::utils::{candle::*, types::*};


use queues::*;

pub struct moving_average{
    interval: Interval,
    data_points:usize,
    value:f32,
    prices:Queue<f32>,
}

impl moving_average{
    pub fn new(
        interval: Interval,
        data_points:usize,
    )->moving_average{
        moving_average{
            interval,
            data_points,
            value:0.0,
            prices:queue![]
        }
    }
    pub fn getValue(&mut self,candle:&Candle)->f32{
        let mut res=0.0;
        if(self.prices.size()<self.data_points){
            self.prices.add(candle.close);
            self.value+=candle.close;
            
            
        }else{
            res=self.value/(self.prices.size() as f32);
            match self.prices.remove(){
                Ok(v)=>{
                    self.value-=v;

                },
                Err(e)=>{
                    println!("queue empty ig {}",e);
                    return 0.0;
                }
            }
            self.prices.add(candle.close);
         
            self.value+=candle.close;
            

        }
        res
    }
}
