use binance::model::{KlineSummary, Kline};

pub fn strTof32(str:String)->f32{
    let f:f32=str.parse().unwrap();
    f
}
pub fn strToi32(str:String)->i32{
    let i:i32=str.parse().unwrap();
    i
}

pub fn KlineSummaryToKline(kline:KlineSummary,symbol:String)->Kline{
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
