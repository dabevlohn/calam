pub struct BuyOrder {
    pub order: String,
    pub ticker: String,
    pub amount: f32,
    pub sender: mpsc::Sender<Message>,
}

impl BuyOrder {
    pub(crate) fn new(ticker: String, amount: f32, sender: mpsc::Sender<Message>) -> Self {
        Self {
            order: "BUY".to_string(),
            ticker,
            amount,
            sender,
        }
    }

    pub(crate) async fn send(self) {
        let (send, recv) = oneshot::channel();
        let message = Message {
            order: self.order,
            amount: self.amount,
            ticker: self.ticker,
            respond_to: send,
        };
        let _ = self.sender.send(message).await;
        match recv.await {
            Ok(outcome) => println!("here is the outcome: {}", outcome),
            Err(e) => println!("{}", e),
        }
    }
}
