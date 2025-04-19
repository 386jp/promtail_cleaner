use std::time::Duration;
use anyhow::Result;
use dotenv::dotenv;
use futures::stream::iter;
use tokio::fs;
use tokio::fs::metadata;
use tokio::fs::File;
use futures::StreamExt;
use tokio::signal::unix::{signal, SignalKind};

use crate::promtail::config::read_promtail_config;
use crate::promtail::position::read_positions_file;

mod promtail;


async fn is_same_file_size(file_path: String, file_size: u64) -> Result<bool> {
    match File::open(&file_path).await {
        Ok(_) => {
            let metadata = metadata(&file_path).await?;
            Ok(metadata.len() == file_size)
        },
        Err(e) => {
            log::error!("Error on inspecting log file size: {} {}", file_path, e);
            Ok(false)
        }
    }
}

async fn delete_file(file_path: String) -> Result<()> {
    match fs::remove_file(&file_path).await {
        Ok(_) => {
            log::info!("Log file deleted successfully: {}", file_path);
            Ok(())
        },
        Err(e) => {
            log::error!("Failed to delete log file: {}. Error: {}", file_path, e);
            Ok(())
        }
    }
}

async fn inspect_log_file(file_path: String, file_size: u64) -> Result<()> {
    if is_same_file_size(file_path.clone(), file_size).await? {
        log::debug!("Log file size is same, able to delete: {}", file_path);
        delete_file(file_path).await?;
    } else {
        log::debug!("Log file size is different from promtail position file, ignored: {}", file_path);
    }
    Ok(())
}


async fn inspect_position_file(position_file_path: String) -> Result<()> {
    log::debug!("Running inspection tasks...");
    log::debug!("Position file path: {}", position_file_path);

    let position_data = read_positions_file(&position_file_path).await?;
    log::debug!("Position data: {:?}", position_data.positions);

    let concurrency_limit: Option<usize> = None;

    iter(position_data.positions.iter())
    .for_each_concurrent(concurrency_limit, |(key, value)| {
        async move {
            let _ = inspect_log_file(key.clone(), value.clone()).await;
        }
    })
    .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting application...");

    let inspection_interval = std::env::var("INSPECTION_INTERVAL_SEC")
        .unwrap_or_else(|_| "".to_string())
        .parse::<u64>()
        .unwrap_or(300);

    log::info!("Inspection interval: {} seconds", inspection_interval);

    let mut interval = tokio::time::interval(Duration::from_secs(inspection_interval));
    let promtail_config = read_promtail_config().await?;
    let promtail_position_file = promtail_config.positions.filename.clone();
    log::info!("Promtail position file: {}", promtail_position_file.clone());

    log::info!("Application started!");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = inspect_position_file(promtail_position_file.clone()).await {
                    log::error!("Error occurred while performing task: {:?}", e);
                    std::process::exit(1);
                }
            }
            _ = async {
                let mut sigterm = signal(SignalKind::terminate()).expect("Failed to set up SIGTERM handler");
                sigterm.recv().await;
            } => {
                log::info!("Received SIGTERM. Stopping application...");
                break;
            }
        }
    }

    log::info!("Application stopped.");
    Ok(())
}
