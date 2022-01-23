use log::{debug, info};
use std::time::Instant;

struct FetchService {
    url: &'static str,
    client: reqwest::Client,
}

impl FetchService {
    fn new(url: &'static str) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    async fn fetch(&self) -> Result<serde_json::Value, reqwest::Error> {
        info!("Fething data from {}", self.url);
        let now = Instant::now();
        let resp: serde_json::Value = self.client.get(self.url).send().await?.json().await?;
        debug!("Took {}ms to fetch data", now.elapsed().as_millis());

        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();

    let url = "https://api.chainweb.com/chainweb/0.0/mainnet01/cut";
    let fetch_service = FetchService::new(url);

    // Two request in parallel
    let results = tokio::try_join!(fetch_service.fetch(), fetch_service.fetch());
    match results {
        Ok((_first, _second)) => {
            // println!("{:#?}", &_first);
            // println!("{:#?}", &_second);
        }
        Err(err) => {
            println!("Fetching failed; error = {}", err);
        }
    }

    Ok(())
}
