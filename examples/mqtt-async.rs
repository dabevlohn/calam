use tokio::{task, time};

use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;

/*
 * This macro annotation indicates that we are using the tokio runtime,
 * where current_thread means our asynchronous code will run in a single-threaded context.
 */
#[tokio::main(flavor = "current_thread")]
/*
 * This is the main function of the program, which is an asynchronous function. In this function,
 * we first initialize an MQTT client and set connection options.
 * Then we create an asynchronous client and an event loop, and call the requests function in a task.
 * Finally, we poll events through the event loop and handle them.
 */
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    pretty_env_logger::init();
    // color_backtrace::install();

    // Set MQTT connection options
    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // Created an asynchronous MQTT client and event loop
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    /*
     * Created an asynchronous task containing a closure.
     * Inside the closure, it first calls requests(client).await;
     * to perform message publishing and subscription operations,
     * then sleeps the task for 3 seconds using
     * time::sleep(Duration::from_secs(3)).await;
     */
    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    loop {
        // Waits for and retrieves the next event in the event loop.
        let event = eventloop.poll().await;
        // Performs pattern matching on the retrieved event to determine its type
        match &event {
            Ok(v) => {
                println!("Event = {v:?}");
            }
            Err(e) => {
                println!("Error = {e:?}");
                return Ok(());
            }
        }
    }
}

/*
 * This is an asynchronous function for publishing and subscribing to messages. In this function,
 * we subscribe to a topic and loop through sending messages from 1 to 10,
 * one message per second. Finally, we sleep for 120 seconds to handle subsequent events.
 */
async fn requests(client: AsyncClient) {
    /*
     * Used to subscribe to a specific topic ("hello/world") on the MQTT server,
     * specifying the Quality of Service (QoS) as AtMostOnce, indicating at most
     * once message delivery.
     */
    client
        .subscribe("files/todo", QoS::AtMostOnce)
        .await
        .unwrap();

    /*
     * Send 10 messages to the "hello/world" topic, with the length
     * of each message increasing from 1 to 10, with an interval of
     * 1 second. Each message has a Quality of Service (QoS) of ExactlyOnce.
     */
    for i in 1..=1000 {
        client
            .publish("files/todo", QoS::ExactlyOnce, false, i.to_string())
            .await
            .unwrap();

        time::sleep(Duration::from_millis(10)).await;
    }

    time::sleep(Duration::from_secs(120)).await;
}
