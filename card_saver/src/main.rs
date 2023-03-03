mod card;
mod data;

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};
use card::YuGiOhCard;
use data::*;
use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    println!("どもはじみまして！忍者くろすべし !");
    println!("今なんか言った？？");

    let mut client = Client::connect(
        "postgresql://admin:admin1234@localhost:5432/postgres",
        NoTls,
    )?;

    // create tables
    create_tables(&mut client)?;

    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").unwrap();

    // Create a channel
    let channel = connection.open_channel(None).unwrap();

    // Declare a queue
    let queue = channel
        .queue_declare(
            "YuGiOh-Processed-Queue",
            QueueDeclareOptions {
                durable: false,
                exclusive: false,
                auto_delete: false,
                arguments: Default::default(),
            },
        )
        .unwrap();

    // Start consuming messages
    let consumer = queue.consume(ConsumerOptions::default()).unwrap();
    let limit = 12244;
    let mut counter = 0;
    println!("starting insertion -> ");
    for (_, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                if counter == limit {
                    println!("\nfinished inserting.");
                    break;
                };
                let payload = std::str::from_utf8(&delivery.body).unwrap();
                let my_message: YuGiOhCard = serde_json::from_str(payload).unwrap();

                if counter % 10 == 0 {
                    print!("");
                }
                insert_card_into_db(&mut client, &my_message)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
        counter += 1;
    }

    let cards_in_database = get_entries_count(&mut client)?;
    println!("Finished creating {} cards.", cards_in_database);

    Ok(())
}
