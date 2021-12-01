use binance::{websockets::*, account::*};
use std::sync::atomic::{AtomicBool};
use crate::{engine::strategy::*};

use super::order::*;
pub fn execute(strat: &mut impl Strategy,budget:f32,account:&Account){
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