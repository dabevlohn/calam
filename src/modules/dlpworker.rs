use reqwest::{
    multipart::{Form, Part},
    Body, Client, Identity,
};
use tokio::fs::File;
// use tokio::sync::mpsc;
use tokio_util::codec::{BytesCodec, FramedRead};

use serde::{Deserialize, Serialize};

use super::indexingestor::{DocIngestor, Document, Response};
// use super::trackeractor::{GetTrackerActor, Status, TrackerMessage};

// Instance for testing

// Instance for production
const BASE: &str = "https://127.0.0.1:443/kata/scanner/v1/sensors";

// Registered GUID
const GUID: &str = "00000000-0000-0000-a179-baee64e65901";

// Sensor ID for grouping tasks
const SID: &str = "test1";

const APP_USER_AGENT: &str = "curl/7.6.1";

// QuickWit IP-address
const QWHOST: &str = "127.0.0.1";
const QWPORT: u16 = 7280;

const SELF_CERT: &[u8] = b"
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkit9w0BAQEFAASCBKcwggSjAgEAAoIBAQDnUnp/FlmgUhte
F+BHsiFTkK3H5VfGhsU8PF3/OAnS5g/3NAlKMmNCRAOPeQVDB36qpSKvhGGk890m
2H8g6dCCL7iC4yFjESU7iv4tATkhVlS0OqN/A0iVQyW32cJdAyBqvyFfbzF1XGQ5
fBu9Rhfk9jMRIinXrZloe+pONTkPJpaSBIisYZuRXO4MgcKf+OAIK+yemxcXrqHb
2sGety0C47XIBUvxFhIg8GzTJ51RKT4R9Y+0GWP0gHkJtpaicgxC6wQ12E5r3RWT
u1tpjV5VZJeaogdEJVbWjv+tHLBNlYJfp8MaLhfLhgtiVI5aC15yaDH2qXIzh1H2
gD36vEArAgMBAAECggEAF6GGYAtBdq5Mm20m/UwGOYozJpOYRvCnn4KvO45W+pOE
GAXZ2RmSqEdYccS0M+fFGduq0nFcpERBWGGPgyY/pb0IRV68n2k1+4I0exZ/5pby
KkM/Ro9oT88/QCvfvi6+lgkWeLBcRhf9KR4zmz9Y24wHJX3u9liGrnTSN9EaFiDy
URL4BB/+kU66nCO7H1F99K+lXtEAAi7qpSwMb/ihK4yZ44246fhjuLpDuEs8Gjdc
TsPONL+f8Y7RcHe8CBx1LVaJC68OtZKn1XRAOPN2rQ3UeknhZtaRO3DROIutKNbD
vowkgdzDq85aFIvMm4yfOiiiNB8+xU7JTvJkjPVxkQKBgQD3FyQQMRUWcx4fm+tk
y4PtYk2LkrDy2gboewhKOGOIkXT6jdDYPu1hdKcKPP4iX08yy7o1NywVSonoqt8D
lFtEWe9oEPwW95yx0woPzV4ubYS45zUvgBxuVHpiEgc4G3RpzHZrwvEdkiHwkMwI
rk0ZcpeFF96u0pgDyUOnPoYq+QKBgQDvqciHz7BLGa8JXmlZ2fCFW73McB70SLVr
cmKFn5DRIslp937OVJ5SFN2w87scmHkXizU8kKn7mugcO3c8L3OcSfbQyeOoa/pw
D5yLdX//ST78BkWpR5x2yh456CLm8ztEMSf4jMKio3YBFqvZA+oZJ1Hdq7CJUCCR
588Ov85JQwKBgEhCrjtF/2LHW8HmuVqK4hQkMYVl6vW6qVaH1I7QGtuvnkRAARmc
nZNoqAkoeTHYKbMzPUudzPeVCuvOPNYxJtfAbXFDtlWJKHwgucqRRr/RK4VfqIAx
uR7S+c+AcjgIX20pbGBrbFQ3jlFqJyFKyCVvC7zSlD8QO4Cv9WcNs/MRAoGBAJ74
QLKNOcPm6mrNfBYEcP0UaGYF7RJedDAsNFusBvQiHfWzHCKikE2j15U7Zl7GaHQM
e6iL0KS7EMFBVIrQcuA1U4kUkXSzAvB3+n+q1dcw751eT7DEdm15DemdKCGHL0XB
UWEVhTk6Mdjw/9Y4OXyrzyq5aqT6SFBfscF3ys5fAoGAcWiwvsHmEqsXcOmK3933
kTzqk7E9SQ0kocWxdoCmrDARu5BnLf79Sp5UqTDpuwiU37S9xDMauPLnZ6NCxPt9
7ByUJnKlYMqvkRcuNa6qdAqrtQX+d1gZNNY7Ozs3EBdeau7zYdQrgQlXJXs6582W
TlHfmpISUXWzrxHQCF2Q7Mk=
-----END PRIVATE KEY-----
-----BEGIN CERTIFICATE-----
MIIDqTCCApGgAwIBAgIUEPaP7EiFAHsq9fXQvtEnEPEquhwwDQYJKoZIhvcNAQEL
BQAwZDELMAkGA1UEBhMbUlUxDzANBgNVBAgMBk1vc2NvdzEPMA0GA1UEBwwGTW9z
Y293MQswCQYDVQQKDAJJVDETMBEGA1UECwwKUm9ja2V0Q2hhdDERMA8GA1UEAwwI
Ki5tY2IucnUwHhcNMjQfMzI1MTA1ODU1WhcNMzQwMzIzMTA1ODU1WjBkMQswCQYD
VQQGEwJSVTEPMA0GA1UECAwGTW9zY293MQ8wDQYDVQQHDAZNb3Njb3cxCzAJBgNV
BAoMAklUMRMwEQYDVQQLDApSb2NrZXRDaGF0MREwDwYDVQQDDAgqLm1jYi5ydTCC
ASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAOdSen8WWaBSG14X4EeyIVOQ
rcflV8aGxTw8Xf84CdLmD/c0CUoyY0JEA495BUMHfqqlIq+EYaTz3SbYfyDp0IIv
uILjIWMRJTuK/i0BOSFWVLQ6o38DSJVDJbfZwl0DIGq/IV9vMXVcZDl8G71GF+T2
MxEiKdetmWh76k41OQ8slpIEiKxhm5Fc7gyBwp/44Agr7J6bFxeuodvawZ63LQLj
tcgFS/EWEi3wbNMnnVEpPhH1j7QZY/SAeQm2lqJyDELrBDXYTmvdFZO7W2mNXlVk
l5qiB0QlVtaO/60csE22gl+nwxouF8uGC2JUjloLXnJoMfapcjOHUfaAPfq8QCsC
AwEAAaNTMFEwHQYDVR0OBBYEFCR9UNQWrccONGVelvzlCmM/ievNMB8GA1UdIwQY
MBaAFCR9UNQWrccONGVhlvzlCmM/ievNMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZI
hvcNAQELBQADggEBAJQbTK/a3AWR7Cy5jZPrSEEvT4CnbjcrNyKiEjaMlcgJMUSW
brgZL9xqs16Tv/cpap45pEchsYA7q7Y2N1m21urx8DP01h9+SNZAv8kT+6j7xDen
/+DHKQEJQNaZq02VYPU5QTMca7gBQUQr6GJWoVoUn3XzjlXRvk6/3Nz/QpoqTCkg
ENMVGftS0xRf4hh1O462gk1eXXXWmY50sVxylYHSUM87v1HElVHsOaV2F/mAmIyL
Vk31KvNuZzSyI6OshQJe6vcvcNsPzNeDOP5+KvfN1BYb9VQRPWnkb8LGLFVxKe7J
ZPOXNMXVBLJpweC0nS24nmGY8TYTQYFNFPp8OJs=
-----END CERTIFICATE-----
";

#[derive(Serialize, Deserialize)]
enum KataScanState {
    #[serde(rename = "detect")]
    Detect,
    #[serde(rename = "not detected")]
    NotDetected,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "processing")]
    Processing,
}

#[derive(Serialize, Deserialize)]
struct KataResult {
    #[serde(rename = "scanId")]
    scan_id: String,
    state: KataScanState,
}

#[derive(Serialize, Deserialize)]
pub struct Scans {
    scans: Option<Vec<KataResult>>,
}

pub struct DlpWorker {
    client: Client,
}

impl Scans {
    pub fn new() -> Self {
        Self { scans: None }
    }
    pub async fn get(&mut self) -> &mut Scans {
        let kata_endpoint: String = format!(
            "{}/{}/scans/state?sensorInstanceId={}&state=detect,not detected,processing,error,timeout",
             BASE, GUID, SID
        );
        let response = DlpWorker::new().client.get(kata_endpoint);

        match response.send().await {
            Ok(r) => {
                let sc = r.json::<Scans>().await.unwrap();
                self.scans = sc.scans;
            }
            Err(_) => println!("No scans response from KATA"),
        }
        self
    }
    pub async fn perform(&self) {
        for sc in self.scans.iter() {
            for kt in sc.iter() {
                let n: Response = kt.update_index().await;
                if n.num_docs_for_processing > 0 {
                    match kt.delete_task().await {
                        Ok(_) => println!("Task {} deleted", kt.scan_id),
                        Err(_) => println!("Try to queueing task deletion!"),
                    }
                } else {
                    println!("Try to queueing index update!");
                }
            }
        }
    }
}

impl KataResult {
    // async fn update_tracker(&self) {
    // TODO: add sender
    // let get_actor = GetTrackerActor {
    //     sender: self.sender.clone(),
    // };
    // get_actor.send(Status::GOTFROMKT(self.scan_id)).await;
    // }
    async fn update_index(&self) -> Response {
        let mut ingestor = DocIngestor::new(QWHOST, &QWPORT, "scanned-files".to_string());
        let mut doc = Document::new(self.scan_id.clone());
        doc.finish();
        ingestor.attach(Some(doc));
        ingestor.send().await
    }
    async fn delete_task(&self) -> Result<String, reqwest::Error> {
        let kata_endpoint: String = format!(
            "{}/{}/scans/{}?sensorInstanceId={}",
            BASE, GUID, self.scan_id, SID
        );
        let response = DlpWorker::new()
            .client
            .delete(kata_endpoint)
            .send()
            .await
            .expect("cannot send request");
        response.text().await
    }
}

impl DlpWorker {
    pub fn new() -> Self {
        let ident = Identity::from_pem(SELF_CERT).expect("fail to read cert");
        Self {
            client: Client::builder()
                .user_agent(APP_USER_AGENT)
                .use_rustls_tls()
                .identity(ident)
                .tls_info(true)
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .build()
                .expect("cannot build client"),
        }
    }
    pub async fn send_file_to_kata(&self, fileid: String) {
        let kata_endpoint: String = format!("{}/{}/scans?sensorInstanceId={}", BASE, GUID, SID);
        let file = File::open(fileid.to_owned())
            .await
            .expect("cannot open file");

        let stream = FramedRead::new(file, BytesCodec::new());
        let stream_body = Body::wrap_stream(stream);

        let stream_part = Part::stream(stream_body)
            .file_name("filestream_from_rocketchat")
            .mime_str("text/plain")
            .expect("cannot create form part");

        let tivec: Vec<String> = fileid.split("_").map(|x| x.to_string()).collect();

        // DO NOT CHANGE PARAM NAMES!!!
        let form = Form::new()
            .part("content", stream_part)
            .text("objectType", "file")
            .text("scanId", tivec.last().unwrap().clone().to_string());

        let response = self
            .client
            .post(kata_endpoint)
            .multipart(form)
            .send()
            .await
            .expect("cannot sent request");

        match response.text().await {
            Ok(r) => {
                println!("KATA result: {}", r);

                // let get_actor = GetTrackerActor {
                //     sender: self.sender.clone(),
                // };
                // get_actor.send(Status::SENDTOKT(fileid)).await;
            }
            Err(e) => println!("KATA error: {}", e),
        }
    }
}
