use std::collections::HashMap;
use std::fs;
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

pub struct GetTrackerActor {
    pub sender: mpsc::Sender<TrackerMessage>,
}

impl GetTrackerActor {
    pub async fn send(self, command: Status) -> String {
        let (send, recv) = oneshot::channel();
        let tracker_message = TrackerMessage {
            command,
            respond_to: send,
        };

        let _ = self.sender.send(tracker_message).await;
        match recv.await {
            Ok(state) => return state,
            Err(e) => panic!("{}", e),
        }
    }
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

    fn check_file(&mut self, fileid: String, respond_to: oneshot::Sender<String>) {
        let mut len: u64 = 0;
        match fs::metadata(fileid.as_str()) {
            Ok(x) => len = x.len(),
            Err(e) => println!("wrong path: {}", e),
        }
        let _ = respond_to.send(len.to_string());
    }

    fn update_state(&mut self, fileid: String, respond_to: oneshot::Sender<String>) {
        match self.db.get(fileid.as_str()) {
            None => self.db.insert(fileid, 1),
            Some(val) => self.db.insert(fileid, val + 1),
        };
        let _ = respond_to.send("OK".to_string());
    }

    fn get_all_states(&self, respond_to: oneshot::Sender<String>) {
        let mut buffer = Vec::new();
        for key in self.db.keys() {
            let amount = self.db.get(key).unwrap();
            buffer.push(format!("{}: {};", &key, amount));
        }
        let _ = respond_to.send(buffer.join(""));
    }

    fn handle_message(&mut self, message: TrackerMessage) {
        match message.command {
            Status::NEW(fileid) => {
                println!("new file stream {}", fileid);
                self.update_state(fileid, message.respond_to);
            }
            Status::SAVED(fileid) => {
                println!("file {} saved", fileid);
                self.check_file(fileid, message.respond_to);
                // TODO: start KATA worker here
                //
            }
            Status::SENDTOKT(fileid) => {
                println!("file {} posted to KATA", fileid);
                self.update_state(fileid, message.respond_to);
                // TODO: start SI worker here
                //
            }
            Status::SENDTOSI(fileid) => {
                println!("file {} posted to SearchInform", fileid);
                self.update_state(fileid, message.respond_to);
            }
            Status::GET(fileid) => {
                println!("get file {} status", fileid);
                self.update_state(fileid, message.respond_to);
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
