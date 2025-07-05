mod config;
mod model;

use crate::config::{Deps, Settings};
use crate::model::Payload;
use anyhow::Result;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

async fn handler(assignment: LambdaEvent<Payload>, deps: Deps) -> Result<(), Error> {
    for assignment in assignment.payload.assignments {
        deps.table_client.put_entry(assignment).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time() // Cloudwatch adds timestamp already
        .json()
        .init();

    let settings = Settings::load_config()?;

    let deps = Deps {
        table_client: Arc::new(
            cloud_util::DynamoDbClient::builder()
                .table_name(&settings.table_name)
                .build()
                .await?,
        ),
    };

    run(service_fn(|event| handler(event, deps.clone()))).await?;
    Ok(())
}
