extern crate serde_json;
use std::collections::HashMap;
use super::{strategy::*,order::*};
use serde_json::{Value as JsonValue, json,Map};
use crate::utils::{candle::*,converters::*};
pub fn backtest(raw_data:&String,strat: &mut impl Strategy){
    //Parsing raw data into vector of candles
    let data=match getData(raw_data){
        Ok(vec)=>vec,
        Err(e)=>Vec::new()
    }; 
    
    //Creating a new order struct to manage all the orders of the particular stock
    let mut order=new_Order(0, 0.0,0.0,0.0,5000.0);
    //backtesting startegy by iterating through price data over time
    for price in data{
        strat.execute(&price,&mut order);
    }

    //all of the order stored in the order struct.
    println!("Final quantity spent = {}, and final value  is  {}",order.quantity_buy,order.budget);
}
#[derive(Deserialize, Debug)]
struct Wick {  
    open:String,
    high:String,
    low:String,
    close:String,
    adjusted_close:String,
    volume:String,
    divident_amount:String,
    split_coefficient:String
}


//Function to parse the raw data

pub fn getData(contents:&String) -> Result<Vec<Candle>,&str> {
    
    let res=serde_json::from_str(&contents);
    if res.is_ok(){

        let p:JsonValue= res.unwrap();

        let mut data=Vec::new();
       

        for (key, value) in p["Time Series (Daily)"].as_object().unwrap() {
            
            
            let temp=value.to_string();
            let mut k=temp;
            k=k.replace("1. open", "open");
            k=k.replace("2. high", "high");
            k=k.replace("3. low", "low");
            k=k.replace("4. close", "close");
            k=k.replace("5. adjusted close", "adjusted_close");
            k=k.replace("6. volume", "volume");
            k=k.replace("7. dividend amount", "divident_amount");
            k=k.replace("8. split coefficient", "split_coefficient");


            let value:Wick=serde_json::from_str(&k).unwrap();
        
            let security=new_Candle(
                key.to_string(),
                strTof32(value.open), 
                strTof32(value.high),
                strTof32(value.low),
                strTof32(value.close),
                strTof32(value.adjusted_close),
                strTof32(value.volume),
                strTof32(value.divident_amount),
                strTof32(value.split_coefficient),
                
            );
            
            data.push(security);
        }
        

        Ok(data)
        
    }else{
        Err("Error parsing json object")
    }
    
}