pub struct OrderBookActor {
    pub receiver: mpsc::Receiver<Message>,
    pub sender: mpsc::Sender<TrackerMessage>,
    pub total_invested: f32,
    pub investment_cap: f32,
}

impl OrderBookActor {
    pub(crate) fn new(
        receiver: mpsc::Receiver<Message>,
        sender: mpsc::Sender<TrackerMessage>,
        investment_cap: f32,
    ) -> Self {
        Self {
            receiver,
            sender,
            total_invested: 0.0,
            investment_cap,
        }
    }

    async fn handle_message(&mut self, message: Message) {
        if message.amount + self.total_invested >= self.investment_cap {
            println!(
                "rejecting purchase, total invested: {}",
                self.total_invested
            );
            let _ = message.respond_to.send(0);
        } else {
            self.total_invested += message.amount;
            println!(
                "processing purchase, total invested: {}",
                self.total_invested
            );
            let _ = message.respond_to.send(1);

            let (send, _) = oneshot::channel();
            let tracker_message = TrackerMessage {
                command: Order::BUY(message.ticker, message.amount),
                respond_to: send,
            };

            let _ = self.sender.send(tracker_message).await;
        }
    }

    pub(crate) async fn run(mut self) {
        println!("actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }
}
