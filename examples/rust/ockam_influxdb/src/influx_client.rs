use crate::lease_manager::LeaseManager;
use rand::random;
use reqwest::header::{HeaderMap, HeaderValue};

// A user's special Influx integration
pub struct InfluxClient {
    api_url: String,
    org: String,
    bucket: String,
    leased_token: String,
}

impl InfluxClient {
    pub fn new(api_url: &str, org: &str, bucket: &str, leased_token: &str) -> Self {
        InfluxClient {
            api_url: api_url.to_string(),
            org: org.to_string(),
            bucket: bucket.to_string(),
            leased_token: leased_token.to_string(),
        }
    }

    pub async fn send_metrics(&self) {
        let url = format!(
            "{}/api/v2/write?org={}&bucket={}&precision=s",
            self.api_url, self.org, self.bucket
        );

        let mut headers = HeaderMap::new();
        let token = format!("Token {}", self.leased_token);

        headers.insert(
            "Authorization",
            HeaderValue::from_str(token.as_str()).unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        for i in 0..10 {
            let data = random::<usize>() % 10_000;

            let metric = format!("metrics,env=test r{}={}", i, data);
            let resp = client.post(url.clone()).body(metric).send().await.unwrap();
            assert!(resp.status().is_success())
        }
    }
}
