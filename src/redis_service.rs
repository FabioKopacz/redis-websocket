extern crate redis;
use redis::Commands;

fn add_new_price(price: i32) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    println!("Saving price: {price}");

    let _: () = con.set("bitcoin", price)?;

    Ok(())
}

fn get_bitcoin_price() -> redis::RedisResult<i32> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let price: i32 = con.get("bitcoin")?;
    Ok(price)
}

fn main() {
    add_new_price(40).expect("Error");
    let price = get_bitcoin_price().expect("Error");
    println!("Recovered price: {price}");
}
