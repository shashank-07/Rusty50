extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate queues;
use std::fs;
use serde_json::Value as JsonValue;

mod engine;
mod strategy;
mod utils;
mod indicators;
use crate::engine::backtester::*;
use crate::strategy::ma::*;
use crate::utils::types::*;



fn main() {
    let contents = fs::read_to_string("./test_stream.json").expect("Cant read from file");
    let mut mov_av_strat=MovingAverage::new(String::from("TCS"), security_type::EQUITY,Interval::D1,25);
    let order=backtest(&contents,&mut mov_av_strat,5000.0);
    println!("{:?}",order);
}


