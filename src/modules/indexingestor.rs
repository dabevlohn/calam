use chrono::Local;
use error_chain::error_chain;
use reqwest::Client;
use serde::{Deserialize, Serialize};

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    num_docs_for_processing: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    taskid: i32,
    _type: String,
    size: u64,
    finished: bool,
    infected: bool,
    received: i64,
}

impl Document {
    pub fn new(taskid: String, size: u64) -> Self {
        Self {
            // TODO: implement several types
            //
            _type: "file".to_string(),
            taskid: taskid.parse::<i32>().unwrap(),
            size,
            infected: false,
            finished: false,
            received: Local::now().timestamp(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IngestionStatus {
    SUCCESS,
    FAIL,
    QUEUED,
}

pub struct DocIngestor {
    document: Option<Document>,
    indexer: String,
    status: IngestionStatus,
}

impl DocIngestor {
    pub fn new(qwhost: String, qwport: u16, index_name: String) -> Self {
        Self {
            indexer: format!(
                "http://{}:{}/api/v1/{}/ingest?commit=force",
                qwhost, qwport, index_name
            ),
            status: IngestionStatus::QUEUED,
            document: None,
        }
    }
    pub fn attach(&mut self, document: Option<Document>) {
        self.document = document;
    }

    pub async fn send(&mut self) -> Response {
        //let document_body = json!({});
        let response = Client::new().post(&self.indexer).json(&self.document);
        match response.send().await {
            Ok(r) => {
                self.status = IngestionStatus::SUCCESS;
                return r.json::<Response>().await.unwrap();
            }
            Err(_) => {
                self.status = IngestionStatus::FAIL;
                return Response {
                    num_docs_for_processing: 0,
                };
            }
        }
    }
}
