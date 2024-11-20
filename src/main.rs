// Jackson Coxson

use std::u64;

use api::MissionaryApi;
use chrono::{Duration, Utc};
use info::MissionaryInfo;

mod api;
mod info;
mod kic;
mod stats;
mod timeline;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let username = std::env::var("CHURCH_USERNAME").expect("No username in env var!");
    let password = std::env::var("CHURCH_PASSWORD").expect("No password in env var!");

    let api = MissionaryApi::new(username, password);

    println!("Getting user info...");
    let info = match MissionaryInfo::from_cache("cache/info.json") {
        Ok(i) => i,
        Err(_) => {
            println!("Getting info from church servers");
            api.fetch_user_details("cache/info.json")
                .await
                .expect("Failed to get user info");
            MissionaryInfo::from_cache("cache/info.json").expect("Failed to read info.json")
        }
    };
    println!(
        "Logged in as {} {} in the {}",
        info.first, info.last, info.mission_name
    );

    // Fetch or load key indicators
    println!("Fetching key indicators...");
    let kics_path = "cache/kics.json";
    if std::path::Path::new(kics_path).exists() {
        println!("Key indicators found in cache. Loading...");
    } else {
        println!("Key indicators not found in cache. Fetching from API...");
        api.fetch_key_indicators(&info, kics_path)
            .await
            .expect("Failed to fetch key indicators");
    }

    // Extract prosAreaIds
    println!("Extracting prosAreaIds from kics.json...");
    let pros_area_ids = kic::extract_pros_area_ids(kics_path)
        .await
        .expect("Failed to extract prosAreaIds");

    println!(
        "Found {} prosAreaIds: {:?}",
        pros_area_ids.len(),
        pros_area_ids
    );

    let mut people = Vec::new();

    for area in pros_area_ids {
        match api.fetch_commands(&area.to_string(), &info).await {
            Ok(mut p) => {
                people.append(&mut p);
            }
            Err(e) => {
                println!("Failed to fetch area {area}: {e:?}");
            }
        }
    }

    println!("Collected {} people", people.len());

    let now = Utc::now();
    let now_timestamp = now.timestamp() as u64 * 1000;

    // Subtract 6 months (approx. 6 * 30 days = 180 days)
    let six_months_ago = now - Duration::days(180);
    let six_months_ago_timestamp = six_months_ago.timestamp() as u64 * 1000;

    let twelve_ago = now - Duration::days(365);
    let twelve_ago_timestamp = twelve_ago.timestamp() as u64 * 1000;
    println!("LAST 6 MONTHS STATS");
    stats::print_stats(people, twelve_ago_timestamp, six_months_ago_timestamp);
}
