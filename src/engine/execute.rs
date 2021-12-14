use binance::model::{KlineSummary, Kline};
use binance::{websockets::*, account::*};
use binance::api::*;

use binance::market::*;

use std::sync::atomic::{AtomicBool};
use crate::{engine::strategy::*};
use crate::utils::converters;
use super::order::*;


pub fn execute(backtest:bool,strat: &mut impl Strategy,budget:f32,account:&Account){
    if backtest{
        let market: Market = Binance::new(None, None);
        let mut order=Order::new(0.0,budget);
        let mut last_price=0.0;
        
        match market.get_klines(strat.getSymbol().to_uppercase(), "1d", 1000,None, None) {
            Ok(klines) => {   
                match klines {
                    binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                        for kline in klines{
                            println!("{}",kline.close);
                            last_price=kline.close;
                            let candle=converters::KlineSummaryToKline(kline,strat.getSymbol().to_string());
                            strat.execute(&candle, &mut order, account);
                        }
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        println!("{:?}  ",order);
        println!("net value = {}$",(order.quantity_buy as f64*last_price+order.budget as f64) as i32);

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