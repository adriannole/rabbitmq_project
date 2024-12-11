use amiquip::{Connection, Channel, QueueDeclareOptions, Consumer, Result};
use std::thread;
use std::time::Duration;

fn consume_messages(channel: &Channel) -> Result<()> {
    let queue = channel.queue_declare("test_queue", QueueDeclareOptions::default())?;
    let consumer = queue.consume(amiquip::ConsumerOptions::default())?;

    println!("Esperando mensajes...");
    for message in consumer.receiver() {
        match message {
            Ok(delivery) => {
                println!("Mensaje recibido: {}", String::from_utf8_lossy(&delivery.data));
                delivery.ack(nowait = false)?;
            }
            Err(err) => {
                eprintln!("Error al recibir el mensaje: {}", err);
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::insecure_open("amqp://user:password@localhost:5672")?;
    let channel = conn.open_channel(None)?;

    consume_messages(&channel)?;
    Ok(())
}
