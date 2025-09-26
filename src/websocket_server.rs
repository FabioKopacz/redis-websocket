use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

use redis::{Client, Commands, Connection, RedisResult};

fn get_bitcoin_price(con: &mut Connection) -> redis::RedisResult<i32> {
    let price: i32 = con.get("bitcoin")?;
    Ok(price)
}

fn add_new_price(con: &mut Connection, price: i32) -> redis::RedisResult<()> {
    println!("Saving price: {price}");

    let _: () = con.set("bitcoin", price)?;

    Ok(())
}

fn make_redis_instance() -> RedisResult<Connection> {
    let client = Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    Ok(con)
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");
    println!("WebSocket server running on ws://127.0.0.1:8080");

    //let mut connection = make_redis_instance().unwrap();

    // let price = 40;
    // add_new_price(&mut connection, price).unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Error during WebSocket handshake");

            println!("Novo cliente conectado!");

            let client = Client::open("redis://127.0.0.1/").unwrap();
            let mut connection = client.get_connection().expect("error");
            // Divide o stream em sender e receiver
            let (mut write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        if msg.is_text() {
                            let price: i32 = connection.get("bitcoin").unwrap();

                            println!("{price}");
                            if let Err(e) =
                                write.send(Message::Text(price.to_string().into())).await
                            {
                                eprintln!("Erro ao enviar mensagem: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Erro ao receber mensagem: {}", e);
                        break;
                    }
                }
            }

            println!("Cliente desconectado!");
        });
    }
}
