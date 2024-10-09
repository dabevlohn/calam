use rumqttc::{Client, LastWill, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

/*
 * This is the main function of the program. In this function, we initialize an MQTT client,
 * set connection options and last will message. Then we create a client and a connection,
 * and call the publish function in a new thread. Next, we use connection.iter()
 * method to iterate through the notifications in the connection and handle each notification.
 */
fn main() {
    // Initialize the logger
    pretty_env_logger::init();

    // Set MQTT connection options and last will message
    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    let will = LastWill::new("files/done", "999", QoS::AtMostOnce, false);
    mqttoptions
        .set_keep_alive(Duration::from_secs(5))
        .set_last_will(will);
    // Create MQTT client and connection, and call the publish function in a new thread
    let (client, mut connection) = Client::new(mqttoptions, 10);
    thread::spawn(move || publish(client));

    // Iterate through the notifications in the connection and handle each notification
    for (i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(notif) => {
                println!("{i}. Notification = {notif:?}");
            }
            Err(error) => {
                println!("{i}. Notification = {error:?}");
                return;
            }
        }
    }

    println!("Done with the stream!!");
}

/*
 * This is a helper function for publishing MQTT messages. In this function, we first sleep
 * for one second, then subscribe to a topic. Then we loop and send ten messages with lengths
 * ranging from 0 to 9, with each message's QoS being at least once.
 */
fn publish(client: Client) {
    // Wait for one second before subscribing to a topic
    // thread::sleep(Duration::from_secs(1));
    // client.subscribe("files/done", QoS::AtMostOnce).unwrap();

    // Send ten messages with lengths ranging from 0 to 9, each message's QoS being at least once
    for i in 0..1000_usize {
        let payload = i.to_string();
        let topic = "files/todo".to_string();
        let qos = QoS::AtLeastOnce;

        client.publish(topic, qos, false, payload).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}
