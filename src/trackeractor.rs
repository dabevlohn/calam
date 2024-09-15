use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Clone)]
pub enum Status {
    NEW(String),
    SAVED(String),
    SENDTOKT(String),
    SENDTOSI(String),
    GET(String),
    GETALL,
}

pub struct TrackerMessage {
    pub command: Status,
    pub respond_to: oneshot::Sender<String>,
}

pub struct TrackerActor {
    pub receiver: mpsc::Receiver<TrackerMessage>,
    pub db: HashMap<String, i8>,
}

impl TrackerActor {
    pub fn new(receiver: mpsc::Receiver<TrackerMessage>) -> Self {
        Self {
            receiver,
            db: HashMap::new(),
        }
    }

    fn update_state(&mut self, fileid: String) {
        match self.db.get(fileid.as_str()) {
            None => self.db.insert(fileid, 1),
            Some(val) => self.db.insert(fileid, val + 1),
        };
    }

    fn get_all_states(&self, respond_to: oneshot::Sender<String>) {
        let mut buffer = Vec::new();
        for key in self.db.keys() {
            let amount = self.db.get(key).unwrap();
            buffer.push(format!("{}: {};", &key, amount));
        }
        buffer.push("\n".to_string());
        let _ = respond_to.send(buffer.join(""));
    }

    fn handle_message(&mut self, message: TrackerMessage) {
        match message.command {
            Status::NEW(fileid) => {
                println!("new file stream {}", fileid);
                self.update_state(fileid);
            }
            Status::SAVED(fileid) => {
                println!("file {} saved", fileid);
                self.update_state(fileid);
                // TODO: start KATA worker here
                //
            }
            Status::SENDTOKT(fileid) => {
                println!("file {} posted to KATA", fileid);
                self.update_state(fileid);
                // TODO: start SI worker here
                //
            }
            Status::SENDTOSI(fileid) => {
                println!("file {} posted to SearchInform", fileid);
                self.update_state(fileid);
            }
            Status::GET(fileid) => {
                println!("get file {} status", fileid);
                self.update_state(fileid);
            }
            Status::GETALL => {
                println!("get all files status");
                self.get_all_states(message.respond_to);
            }
        }
    }

    pub async fn run(mut self) {
        println!("tracker actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}
