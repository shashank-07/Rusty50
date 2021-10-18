use crate::utils::candle::*;

#[derive(Debug)]
pub struct Order{
    pub quantity_buy:i32, //quantity of the share bought
    pub profit:f32, //the selling price of the shares
    pub loss:f32, //the cost price
    pub net:f32, //the actual profit
    pub budget:f32, //budget of the order
}
impl Order{
    pub fn buy(&mut self,candle:&Candle,percentage:f32){
        //make sure percentage of order is between 0 and 1, partial orders allowed (i.e, u dont need to spend ur full budget all at once)
        assert_eq!(percentage<=1.0&&percentage>0.0, true, "Invalid percentage: {} , must be between 0 and 1", percentage);
        if self.budget*percentage>=candle.close {
            let budget=self.budget*percentage;
            let quantity=budget/candle.close;
    
            self.quantity_buy+=quantity as i32;
            self.budget-=quantity.floor()*candle.close;
            self.loss+=quantity.floor()*candle.close;
            println!("BOUGHT {} shares AT {} at {}",quantity as i32,candle.close,candle.date);
        }
        

    }
    pub fn sell(&mut self,candle:&Candle,percentage:f32){
        assert_eq!(percentage<=1.0&&percentage>0.0, true, "Invalid percentage: {} , must be between 0 and 1", percentage);
        if self.quantity_buy>0 {
            let quantity=self.quantity_buy as f32*percentage;
            let sell_price=quantity.floor()*candle.close;
            self.quantity_buy-=quantity as i32;
            self.profit+=sell_price;
            self.budget+=sell_price;
            self.net=self.profit-self.loss;
            println!("SOLD {} shares AT {} at {}",quantity as i32,candle.close,candle.date);
        }
        

    }
}

pub fn new_Order(
    quantity_buy:i32,
    profit:f32,
    loss:f32,
    net:f32,
    budget:f32
)->Order{
    Order{
        quantity_buy,
        profit,
        loss,
        net,
        budget
    }
}