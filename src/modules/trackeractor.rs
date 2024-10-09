use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FmtResult};
use ::std::io::Error as IoError;
use reqwest::{
    multipart::{Form, Part},
    Body, Client,
};
use std::collections::HashMap;
use std::fs;
use tokio::fs::File;
use tokio::sync::{mpsc, oneshot};
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Debug, Clone)]
pub enum Status {
    New(String),
    Saved(String),
    SendToKt(String),
    GotFromKt(String),
    GetAll,
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
            Ok(state) => state,
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

    async fn check_file(&mut self, fileid: String, respond_to: oneshot::Sender<String>) {
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
        let _ = respond_to.send("Status updated".to_string());
    }

    fn get_all_states(&self, respond_to: oneshot::Sender<String>) {
        let mut buffer = Vec::new();
        for key in self.db.keys() {
            let amount = self.db.get(key).unwrap();
            buffer.push(format!("{}: {};", &key, amount));
        }
        let _ = respond_to.send(buffer.join(""));
    }

    async fn handle_message(&mut self, message: TrackerMessage) {
        match message.command {
            Status::New(fileid) => {
                println!("new file stream {}", fileid);
                self.update_state(fileid, message.respond_to);
            }
            Status::Saved(fileid) => {
                println!("file {} saved", fileid);
                self.check_file(fileid.to_owned(), message.respond_to).await;
            }
            Status::SendToKt(fileid) => {
                println!("file {} posted to KATA", fileid);
                self.update_state(fileid.to_owned(), message.respond_to);
            }
            Status::GotFromKt(fileid) => {
                println!("file {} scanned by KATA", fileid);
                self.update_state(fileid.to_owned(), message.respond_to);
            }
            Status::GetAll => {
                println!("get all files status");
                self.get_all_states(message.respond_to);
            }
        }
    }

    pub async fn run(mut self) {
        println!("tracker actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    async fn send_file_for_checking(
        &self,
        endpoint: &str,
        fpath: String,
    ) -> Result<String, TrackerError> {
        let file = File::open(fpath).await?;

        let stream = FramedRead::new(file, BytesCodec::new());
        let stream_body = Body::wrap_stream(stream);

        let stream_part = Part::stream(stream_body);
        let form = Form::new().part("file", stream_part);

        let response = self.client.post(endpoint).multipart(form).send().await?;

        let result = response.text().await?;

        Ok(result)
    }
}

#[derive(Debug)]
pub struct TrackerError(pub String);

impl Display for TrackerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "MyError: {}", self.0)
    }
}

impl Error for TrackerError {}

impl From<IoError> for TrackerError {
    fn from(err: IoError) -> Self {
        TrackerError(format!("{} ({})", err, err.kind()))
    }
}

impl From<reqwest::Error> for TrackerError {
    fn from(err: reqwest::Error) -> Self {
        TrackerError(err.to_string())
    }
}
