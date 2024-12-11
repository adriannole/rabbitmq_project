use amiquip::{Connection, Channel, QueueDeclareOptions, Publish, Result};
use std::env;
use std::io::{self, Write};

fn send_message(channel: &Channel, message: &str) -> Result<()> {
    let queue = channel.queue_declare("test_queue", QueueDeclareOptions::default())?;
    queue.publish(Publish::new(message.as_bytes(), "test_queue"))?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::insecure_open("amqp://user:password@localhost:5672")?;
    let channel = conn.open_channel(None)?;

    loop {
        print!("Ingrese el mensaje a enviar (o 'exit' para salir): ");
        io::stdout().flush()?;
        
        let mut message = String::new();
        io::stdin().read_line(&mut message)?;

        let message = message.trim();
        if message == "exit" {
            break;
        }

        send_message(&channel, message)?;
        println!("Mensaje enviado: {}", message);
    }

    Ok(())
}
