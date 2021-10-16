extern crate serde;
extern crate serde_json;



#[derive(Serialize, Deserialize)]
pub struct Candle{
    pub date:String,
    pub open:f32,
    pub high:f32,
    pub low:f32,
    pub close:f32,
    pub adjusted_close:f32,
    pub volume:f32,
    pub divident_amount:f32,
    pub split_coefficient:f32,
}
pub fn new_Candle(
    date:String,
    open:f32,
    high:f32,
    low:f32,
    close:f32,
    adjusted_close:f32,
    volume:f32,
    divident_amount:f32,
    split_coefficient:f32,
)->Candle{
    Candle{
        date,
        open,
        high,
        low,
        close,
        adjusted_close,
        volume,
        divident_amount,
        split_coefficient
    }
}