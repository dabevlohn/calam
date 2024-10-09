use chrono::Local;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub num_docs_for_processing: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    taskid: i32,
    _type: String,
    size: u64,
    finished: Option<i64>,
    infected: bool,
    received: i64,
}

impl Document {
    pub fn new(taskid: String) -> Self {
        Self {
            // TODO: implement several types
            //
            _type: "file".to_string(),
            taskid: taskid.parse::<i32>().unwrap(),
            size: 0,
            infected: false,
            received: Local::now().timestamp(),
            finished: None,
        }
    }
    pub fn update_size(&mut self, size: String) {
        self.size = size.parse::<u64>().unwrap();
    }
    pub fn finish(&mut self) {
        self.finished = Some(Local::now().timestamp());
    }
}

#[derive(Debug, Clone)]
pub enum IngestionStatus {
    Success,
    Fail,
    Queued,
}

pub struct DocIngestor {
    document: Option<Document>,
    indexer: String,
    status: IngestionStatus,
}

impl DocIngestor {
    pub fn new(qwhost: &str, qwport: &u16, index_name: String) -> Self {
        Self {
            indexer: format!(
                "http://{}:{}/api/v1/{}/ingest?commit=force",
                qwhost, qwport, index_name
            ),
            status: IngestionStatus::Queued,
            document: None,
        }
    }
    pub fn attach(&mut self, document: Option<Document>) {
        self.document = document;
    }

    pub async fn send(&mut self) -> Response {
        let response = Client::new().post(&self.indexer).json(&self.document);
        match response.send().await {
            Ok(r) => {
                self.status = IngestionStatus::Success;
                r.json::<Response>().await.unwrap()
            }
            Err(_) => {
                self.status = IngestionStatus::Fail;
                Response {
                    num_docs_for_processing: 0,
                }
            }
        }
    }
}
