use binance::model::{KlineSummary, Kline};
use binance::{websockets::*, account::*};
use binance::api::*;

use binance::market::*;

use std::sync::atomic::{AtomicBool};
use crate::{engine::strategy::*};

use super::order::*;


pub fn execute(backtest:bool,strat: &mut impl Strategy,budget:f32,account:&Account){
    if backtest{
        let market: Market = Binance::new(None, None);
        let mut order=Order::new(0.0,budget);
        let mut last_price=0.0;
        
        match market.get_klines(strat.getSymbol().to_uppercase(), "1d", 365, None, None) {
            Ok(klines) => {   
                match klines {
                    binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                        for kline in klines{
                            last_price=kline.close;
                            let candle=convertToCandle(kline,strat.getSymbol().to_string());
                            strat.execute(&candle, &mut order, account);
                        }
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        println!("{:?}  ",order);
        println!("net value = {}$",(order.quantity_buy as f64*last_price) as i32);

    }else{
        let keep_running = AtomicBool::new(true); // Used to control the event loop
        let kline: String = format!("{}", strat.getSymbol().to_owned()+"@kline_1m");
        
        //Initialize new order
        let mut order=Order::new(0.0,budget);
        println!("{}",kline);
        let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
            match event {
                WebsocketEvent::Kline(kline_event) => {
                    let candle=kline_event.kline;
                    println!("{}",candle.close);

                    //Strategy gets executed
                    strat.execute(&candle, &mut order, account);
                
                    //Update the positions

                },
                _ => (),
            };
            Ok(())
        });
    
        web_socket.connect(&kline).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            match e {
              err => {
                 println!("Error: {:?}", err);
              }
            }
         }
         web_socket.disconnect().unwrap();
    }
    

}

pub fn convertToCandle(kline:KlineSummary,symbol:String)->Kline{
    Kline{
        start_time:kline.open_time,
        end_time:0,
        symbol:symbol,
        interval:String::from("1d"),
        first_trade_id:0,
        last_trade_id:0,
        open:kline.open.to_string(),
        close:kline.close.to_string(),
        high:kline.high.to_string(),
        low:kline.low.to_string(),
        volume:kline.volume.to_string(),
        number_of_trades: kline.number_of_trades as i32,
        is_final_bar: false,
        quote_volume: kline.quote_asset_volume.to_string(),
        active_buy_volume: kline.taker_buy_base_asset_volume.to_string(),
        active_volume_buy_quote: kline.quote_asset_volume.to_string(),
        ignore_me: String::from("ignored")
    }
}