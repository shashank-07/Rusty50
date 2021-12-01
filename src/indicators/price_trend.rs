// Shows weather market is in uptrend or downtrend

use crate::utils::candle::*;
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;
use chrono::{NaiveDate, Datelike, Weekday};

pub struct PriceTrend{
    time_frames:Vec<usize>,
    dates_passed:Vec<String>,
    high_prices:Vec<PriorityQueue<String,OrderedFloat<f32>>>,
    low_prices:Vec<PriorityQueue<String,OrderedFloat<f32>>>,
}

impl PriceTrend{
    pub fn new(time_frames:Vec<usize>)->PriceTrend{
        let mut high_prices=Vec::new();
        let mut low_prices=Vec::new();
        for timeFrame in 0..time_frames.len() {
           let high_q:PriorityQueue<String,OrderedFloat<f32>>=PriorityQueue::new();
           let low_q:PriorityQueue<String,OrderedFloat<f32>>=PriorityQueue::new();
           high_prices.push(high_q);
           low_prices.push(low_q);
        }
        
        PriceTrend{

            time_frames,
            dates_passed:Vec::new(),
            high_prices,
            low_prices,
        }
    }

 
   
    pub fn upTrend(&mut self,candle:&Candle)->Vec<bool>{
        let mut res = vec![true; self.time_frames.len()];
        let mut ind=0;
        for size in self.time_frames.iter_mut(){
            let top:f32=match self.high_prices[ind].peek(){
                Some(v)=>v.1.into_inner()*1.0,
                None=> 0.0,
            };
    
            let bottom =match self.low_prices[ind].peek(){
                Some(v)=>v.1.into_inner()*-1.0,
                None=> 0.0,
            };
            
            if top-candle.close>candle.close-bottom {
                res[ind]=false;
            }
            
            if self.dates_passed.len()>= *size {
                let ref_date=NaiveDate::parse_from_str(&self.dates_passed[self.dates_passed.len()-*size], "%Y-%m-%d").unwrap();
                while true {
                    let start=NaiveDate::parse_from_str("1970-1-1", "%Y-%m-%d").unwrap();
                    let top:NaiveDate=match self.high_prices[ind].peek(){
                        Some(v)=>NaiveDate::parse_from_str(v.0, "%Y-%m-%d").unwrap(),
                        None=> NaiveDate::parse_from_str("1970-1-1", "%Y-%m-%d").unwrap(),
                    };
                    if start==top {
                        panic!("Cannot parse date");
                    }else if top<=ref_date {
                        self.high_prices[ind].pop().unwrap();
                    }else{
                        break;
                    }        
                }
                while false {
                    let start=NaiveDate::parse_from_str("1970-1-1", "%Y-%m-%d").unwrap();
                    let top:NaiveDate=match self.low_prices[ind].peek(){
                        Some(v)=>NaiveDate::parse_from_str(v.0, "%Y-%m-%d").unwrap(),
                        None=> NaiveDate::parse_from_str("1970-1-1", "%Y-%m-%d").unwrap(),
                    };
                    if start==top {
                        panic!("Cannot parse date");
                    }else if top<=ref_date {
                        self.high_prices[ind].pop().unwrap();
                    }else{
                        break;
                    }        
                }
                
            }
            self.high_prices[ind].push(candle.date.clone(),OrderedFloat(candle.close));
            self.low_prices[ind].push(candle.date.clone(),OrderedFloat(-1.0*candle.close));
            ind+=1;
        }
        self.dates_passed.push(candle.date.clone());
       
        res
    }
    

}


