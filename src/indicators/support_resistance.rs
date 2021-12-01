// Shows weather market is in uptrend or downtrend

use lru::LruCache;

use crate::utils::candle::*;

#[derive(PartialEq)]
enum CandleColor{
    RED,
    GREEN,
    NONE
}
pub struct SupportResistance{
    pub support_set:LruCache<i32,i32>,
    pub resistance_set:LruCache<i32,i32>,
    current_color:CandleColor,
    last_open:i32,
    last_close:i32,
    threshold:i32,

}

impl SupportResistance{
    pub fn new(threshold:i32,period:usize)->SupportResistance{        
        SupportResistance{
            support_set:LruCache::new(period),
            resistance_set:LruCache::new(period),
            current_color:CandleColor::NONE,
            last_open:0,
            last_close:0,
            threshold
        }
    }

 
   
    pub fn update(&mut self,candle:&Candle){
       let mut current_color=CandleColor::NONE;
       if candle.close>candle.open {
           current_color=CandleColor::GREEN;
       }else if candle.close<candle.open{
           current_color=CandleColor::RED;
       }
       if self.current_color!=current_color && self.current_color!=CandleColor::NONE && current_color != CandleColor::NONE{
           if current_color== CandleColor::GREEN{
               //update support
                self.update_set(true);
              
               

           }
           if current_color == CandleColor::RED{
               //Update resistance
               self.update_set(false);
           }
       }
       self.last_close=candle.close as i32;
       self.last_open= candle.open as i32;
       self.current_color=current_color;
      
    }

    pub fn update_set(&mut self,support:bool){
       
        if support{
            let low=self.last_close-(self.last_close*self.threshold/100);
            let high =self.last_close+(self.last_close*self.threshold/100); 
            println!("{}, {}",low,high);
    
            for price in low..high+1{
                let mut count =0;
                match self.support_set.get(&price){
                    Some(p)=>{
                        count = p+1;
                    },
                    None=>{
                        count=1;
                        
                        self.support_set.put(price,0);
                        
                        match self.resistance_set.get(&price){
                            Some(r_p)=>{
                                count=*r_p;
                                self.resistance_set.pop(&price);
                            },
                            None =>()
                        }
                    }
                }
            
                let x = self.support_set.get_mut(&price).unwrap();
                *x = count;
             

            }
        }else{
            let low=self.last_close-(self.last_close*self.threshold/100);
            let high =self.last_close+(self.last_close*self.threshold/100); 
            println!("{}, {}",low,high);
    
            for price in low..high+1{
                let mut count =0;

                match self.resistance_set.get(&price){
                    Some(p)=>{
                       count = p+1;
                    },
                    None=>{
                       count=1;
                
                       self.resistance_set.put(price,0);
                    
                       match self.support_set.get(&price){
                           Some(r_p)=>{
                               count=*r_p;
                               self.support_set.pop(&price);
                           },
                           None =>()
                       }
                    }
                }
               
                let x = self.resistance_set.get_mut(&price).unwrap();
                *x = count;
            }
        }
       
    }
}


