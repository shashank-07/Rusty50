extern crate serde_json;
use super::{strategy::*,order::*};
use serde_json::Value as JsonValue;
use crate::utils::{candle::*,converters::*};
pub fn backtest(raw_data:&String,strat: &mut impl Strategy,budget:f32)->Order{
    //Parsing raw data into vector of candles
    let data=match get_data(raw_data){
        Ok(vec)=>vec,
        Err(e)=>{
            println!("{}",e);
            Vec::new()
        }
    }; 
    
    //Creating a new order struct to manage all the orders of the particular stock
    let mut order=Order::new(0.0,budget);
    
    //backtesting startegy by iterating through price data over time
    // for price in data{
    //     strat.execute(&price,&mut order);
    //     order.value=order.budget+(order.quantity_buy as f32*price.close);
    // }
    order
    //all of the order stored in the order struct.
    
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

pub fn get_data(contents:&String) -> Result<Vec<Candle>,&str> {
    
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
        
            let security=Candle::new(
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

#[cfg(test)]

mod tests{
    
    
    #[test]
    fn one_result(){
       
    }

    #[test]
    fn case_insensitive(){
       

    }

}

