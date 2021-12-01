use binance::model::Kline;


#[derive(Debug)]
pub struct Order{
    pub quantity_buy:f32, //quantity of the crypto bought
    pub budget:f32,
    pub last_bought:f32,
    pub value:f32//budget of the order
}
impl Order{
    pub fn new(
        quantity_buy:f32,
        budget:f32
    )->Order{
        Order{
            quantity_buy,
            budget,
            last_bought:0.0,
            value:budget
        }
    }
    pub fn buy(&mut self,candle:&Kline,percentage:f32)->Result<&str, &str>{
        //make sure percentage of order is between 0 and 1, partial orders allowed (i.e, u dont need to spend ur full budget all at once)
        assert_eq!(percentage<=1.0&&percentage>0.0, true, "Invalid percentage: {} , must be between 0 and 1", percentage);
        let price = candle.close.parse::<f32>().unwrap();
        if self.budget*percentage>=price {

            let budget=self.budget*percentage;
            let quantity=budget/price;

            self.quantity_buy+=quantity;
            self.budget-=quantity*price;

            self.last_bought=price;
            println!("BOUGHT {} shares AT {} at {}",quantity as i32,price,candle.start_time);
            Ok("Success")
            
        }else{
            Err("Did not buy")
        }
        

    }
    pub fn sell(&mut self,candle:&Kline,percentage:f32)->Result<&str, &str>{
        assert_eq!(percentage<=1.0&&percentage>0.0, true, "Invalid percentage: {} , must be between 0 and 1", percentage);
        let price = candle.close.parse::<f32>().unwrap();
        if self.quantity_buy>0.0 {
            
            let quantity=self.quantity_buy as f32*percentage;
            let sell_price=quantity*price;

            self.quantity_buy-=quantity;
            self.budget+=sell_price;

            println!("SOLD {} shares AT {} at {}",quantity as i32,price,candle.start_time);
            Ok("Success")
            
        }else{
            Err("Did not sell")
        }        

    }
}

