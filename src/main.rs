extern crate serde_json;
extern crate priority_queue; // not necessary in Rust edition 2018

#[macro_use]
extern crate serde_derive;
extern crate queues;
extern crate lru;

use std::fs;
use std::{thread, time};
use std::sync::atomic::{AtomicBool};

use crate::strategy::{ma::*,sample::*};
use crate::utils::types::*;


use engine::execute::execute;
use lru::LruCache;

use binance::general::General;
use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use binance::market::*;
use binance::account::*;
use binance::config::Config;


mod engine;
mod strategy;
mod utils;
mod indicators;

use dotenv::dotenv;
use std::env;

fn main() {

    dotenv().ok();

    let mut mov_av_strat=MovingAverageStrategy::new(String::from("ethbusd"), Interval::D1,50);    
    
    let account=getAccount(env::var("API_KEY_TEST").unwrap(),env::var("SECRET_KEY_TEST").unwrap(), true);
    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }
    execute(true,&mut mov_av_strat,5000.0,&account);



   
    

}


fn getAccount(api_key:String,secret_key:String,test_net:bool)->Account{
    if test_net{
        let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");

        Binance::new_with_config(Some(api_key.into()), Some(secret_key.into()), &config)
    }else{
        Binance::new(Some(api_key.into()), Some(secret_key.into()))
    }
}








