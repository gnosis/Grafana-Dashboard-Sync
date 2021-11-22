use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use reqwest::{Client, StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Copy a Grafana dashboard from one instance to another.")]
struct Opt {
    /// Path to config file.
    #[structopt(long, parse(from_os_str))]
    config: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    source_url: String,
    source_api_key: String,
    source_dashboard_uid: String,
    destination_url: String,
    destination_api_key: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    let config = std::fs::read_to_string(opt.config).context("read config file")?;
    let config: Config = toml::de::from_str(&config).context("parse config")?;
    let client = Client::new();
    let api_staging = Api {
        client: client.clone(),
        base: config.source_url.parse().context("source url")?,
        access_key: config.source_api_key,
    };
    let api_prod = Api {
        client: client.clone(),
        base: config.destination_url.parse().context("destination url")?,
        access_key: config.destination_api_key,
    };
    let source = api_staging
        .get_dashboard(&config.source_dashboard_uid)
        .await
        .context("get dashboard from source instance")?
        .ok_or_else(|| anyhow!("dashboard not found in source instance"))?;
    create_or_update(source, &api_prod)
        .await
        .context("create or update")?;
    Ok(())
}

async fn create_or_update(source: GetDashboardResponse, dest_api: &Api) -> Result<()> {
    let dashboard = match dest_api
        .get_dashboard(&source.dashboard.uid)
        .await
        .context("query dashboard from destination instance")?
    {
        Some(existing) => {
            println!(
                "Destination instance dashboard with matching uid exists at version {}. Updating.",
                existing.dashboard.version
            );
            let mut source = source.dashboard;
            source.id = existing.dashboard.id;
            source.version = existing.dashboard.version;
            source
        }
        None => {
            println!("No destination instance dashboard with matching uid exists. Creating.");
            let mut source = source.dashboard;
            source.id = None;
            source.version = 0;
            source
        }
    };
    let response = dest_api
        .post_dashboard(&PostDashboardRequest { dashboard })
        .await
        .context("write dashboard to destination instance")?;
    let mut destination_url = dest_api.base.clone();
    destination_url.set_path(&response.url);
    println!(
        "Created or updated destination instance dashboard with id {}, version {}: {}",
        response.id, response.version, destination_url
    );
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Dashboard {
    id: Option<u32>,
    uid: String,
    version: u32,
    #[serde(flatten)]
    rest: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct GetDashboardResponse {
    dashboard: Dashboard,
}

#[derive(Debug, Serialize)]
struct PostDashboardRequest {
    dashboard: Dashboard,
}

#[derive(Debug, Deserialize)]
struct PostDashboardResponse {
    id: u32,
    uid: String,
    url: String,
    version: u32,
}

struct Api {
    client: Client,
    base: Url,
    access_key: String,
}

impl Api {
    fn authorization(&self) -> String {
        format!("Bearer {}", self.access_key)
    }

    async fn get_dashboard(&self, uid: &str) -> Result<Option<GetDashboardResponse>> {
        let mut url = self.base.clone();
        url.set_path(&format!("/api/dashboards/uid/{}", uid));
        let request = self
            .client
            .get(url)
            .header("Authorization", self.authorization());
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
        match status {
            StatusCode::NOT_FOUND => Ok(None),
            StatusCode::OK => serde_json::from_str(&body).context(body),
            _ => Err(anyhow!("status: {}, body: {}", status, body)),
        }
    }

    async fn post_dashboard(
        &self,
        request: &PostDashboardRequest,
    ) -> Result<PostDashboardResponse> {
        let mut url = self.base.clone();
        url.set_path("/api/dashboards/db");
        let request = self
            .client
            .post(url)
            .json(&request)
            .header("Authorization", self.authorization());
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
        match status {
            StatusCode::OK | StatusCode::CREATED => serde_json::from_str(&body).context(body),
            _ => Err(anyhow!("status: {}, body: {}", status, body)),
        }
    }
}
