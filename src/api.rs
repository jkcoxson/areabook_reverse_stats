// Jackson Coxson

use base64::Engine;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use flate2::read::GzDecoder;
use reqwest::Client;
use std::collections::HashMap;
use std::io::Read;
use tokio::fs::{self, File};
use tokio::io::{self, AsyncWriteExt};

use crate::info::MissionaryInfo;
use crate::timeline::{CommandsResponse, Person, TimelineEntry, TimelineEventKind};

pub struct MissionaryApi {
    username: String,
    password: String,
}

impl MissionaryApi {
    /// Creates a new instance of the API client.
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    /// Builds the Base64-encoded authorization header.
    fn get_auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.username, self.password);
        const CUSTOM_ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
            &base64::alphabet::URL_SAFE,
            base64::engine::general_purpose::NO_PAD,
        );
        format!("Basic {}", CUSTOM_ENGINE.encode(credentials))
    }

    /// Fetches user details and saves the result as `info.json`.
    pub async fn fetch_user_details(
        &self,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://missionary.churchofjesuschrist.org/ws/areabook/v5.2/user/details";

        let client = Client::new();
        let response = client
            .get(url)
            .header("Accept-Encoding", "gzip")
            .header("Accept-Language", "en-US")
            .header("Authorization", self.get_auth_header())
            .header("client-version", "6.20.0")
            .header("Connection", "Keep-Alive")
            .header("device-type", "Android")
            .header("forceNoDataGuard", "false")
            .header("forceUseDataGuard", "false")
            .header("group-name", "SRG")
            .header("Host", "missionary.churchofjesuschrist.org")
            .header("lastSync", "")
            .header("skipMtcMentorAccess", "false")
            .header(
                "User-Agent",
                "Area Book 6.20.0 (620037). Android Android SDK built for x86_64. Android 14 (api 34) 81c9cf2ff6e4bdee",
            )
            .send()
            .await?
            .bytes()
            .await?;

        let gz_path = format!("{}.gz", output_path);
        let mut gz_file = File::create(&gz_path).await?;
        gz_file.write_all(&response).await?;

        // Decompress GZIP file
        Self::decompress_gzip(&gz_path, output_path).await?;

        Ok(())
    }

    /// Decompresses a GZIP file to the given output path.
    async fn decompress_gzip(input_path: &str, output_path: &str) -> io::Result<()> {
        let gz_file = fs::read(input_path).await?;
        let mut decoder = GzDecoder::new(&gz_file[..]);

        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;

        let mut decompressed_file = File::create(output_path).await?;
        decompressed_file.write_all(&decompressed_data).await?;

        // Remove the GZIP file after decompression
        fs::remove_file(input_path).await?;
        Ok(())
    }

    pub async fn fetch_key_indicators(
        &self,
        info: &MissionaryInfo,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://missionary.churchofjesuschrist.org/ws/areabook/v5.2/key-indicators";

        let client = Client::new();
        let response = client
            .get(url)
            .header("Accept-Encoding", "gzip")
            .header("Accept-Language", "en-US")
            .header("Authorization", self.get_auth_header())
            .header("client-version", "6.20.0")
            .header("Connection", "Keep-Alive")
            .header("device-type", "Android")
            .header("forceNoDataGuard", "false")
            .header("forceUseDataGuard", "false")
            .header("group-name", "SRG")
            .header("Host", "missionary.churchofjesuschrist.org")
            .header("lastSync", "")
            .header("missionId", info.mission_id.to_string())
            .header("prosAreaId", info.pros_area_id.to_string())
            .header("prosAreaName", &info.pros_area_name)
            .header("syncGuid", "e2b16e2a-98d7-4637-af6c-fdd9e135d731")
            .header("syncStrategy", "F")
            .header(
                "User-Agent",
                "Area Book 6.20.0 (620037). Android Android SDK built for x86_64. Android 14 (api 34) 81c9cf2ff6e4bdee",
            )
            .send()
            .await?
            .bytes()
            .await?;

        let gz_path = format!("{}.gz", output_path);
        let mut gz_file = File::create(&gz_path).await?;
        gz_file.write_all(&response).await?;

        // Decompress GZIP file
        Self::decompress_gzip(&gz_path, output_path).await?;

        Ok(())
    }

    /// Reads the commands for an area from the cache or fetches it new
    pub async fn fetch_commands(
        &self,
        area: &str,
        info: &MissionaryInfo,
    ) -> Result<Vec<Person>, Box<dyn std::error::Error>> {
        // Read from the cache
        let data = match fs::read_to_string(format!("cache/commands/{}.json", area)).await {
            Ok(d) => d,
            Err(_) => {
                let url = "https://missionary.churchofjesuschrist.org/ws/areabook/v5.2/commands";

                let client = Client::new();
                let response = client
                    .get(url)
                    .header("Accept-Encoding", "gzip")
                    .header("Accept-Language", "en-US")
                    .header("Authorization", self.get_auth_header())
                    .header("client-version", "6.20.0")
                    .header("Connection", "Keep-Alive")
                    .header("device-type", "Android")
                    .header("forceNoDataGuard", "false")
                    .header("forceUseDataGuard", "false")
                    .header("group-name", "SRG")
                    .header("Host", "missionary.churchofjesuschrist.org")
                    .header("lastSync", "")
                    .header("missionId", info.mission_id.to_string())
                    .header("prosAreaId", area)
                    .header("prosAreaName", &info.pros_area_name)
                    .header("syncGuid", "e2b16e2a-98d7-4637-af6c-fdd9e135d731")
                    .header("syncStrategy", "F")
                    .header(
                        "User-Agent",
                        "Area Book 6.20.0 (620037). Android Android SDK built for x86_64. Android 14 (api 34) 81c9cf2ff6e4bdee",
                    )
                    .send()
                    .await?
                    .bytes()
                    .await?;
                let gz_path = format!("cache/commands/{area}.gz");
                let mut gz_file = File::create(&gz_path).await?;
                gz_file.write_all(&response).await?;

                // Decompress GZIP file
                Self::decompress_gzip(&gz_path, &format!("cache/commands/{area}.json")).await?;
                fs::read_to_string(format!("cache/commands/{}.json", area)).await?
            }
        };

        let res: CommandsResponse = serde_json::from_str(&data)?;

        let mut people_map = HashMap::new();
        let person_events = res
            .person_events
            .iter()
            .map(|x| (&x.event_id, &x.person_id))
            .collect::<HashMap<&String, &String>>();

        for people in res.contacts {
            if people.status == 40 {
                // member
                continue;
            }

            people_map.insert(
                people.id,
                Person {
                    name: format!(
                        "{}{}",
                        people.first_name.unwrap_or_default(),
                        people.last_name.unwrap_or_default()
                    ),
                    language: people.preferred_language_id,
                    area: res.pros_area_name.clone(),
                    timeline: Vec::new(),
                },
            );
        }

        for c in res.events {
            if c.start_time.is_none() {
                continue;
            }
            if let Some(person_id) = person_events.get(&c.id) {
                if let Some(p) = people_map.get_mut(*person_id) {
                    p.timeline.push(TimelineEntry {
                        time: c.start_time.unwrap(),
                        details: c.report.unwrap_or_default(),
                        kind: TimelineEventKind::Event,
                    })
                }
            }
        }

        for c in res.person_drops {
            if let Some(p) = people_map.get_mut(&c.person_id) {
                p.timeline.push(TimelineEntry {
                    time: c.drop_date,
                    details: c.created_by.unwrap_or_default(),
                    kind: TimelineEventKind::Drop,
                })
            }
        }

        for c in res.person_resets {
            if let Some(p) = people_map.get_mut(&c.person_id) {
                p.timeline.push(TimelineEntry {
                    time: c.reset_date,
                    details: c.created_by.unwrap_or_default(),
                    kind: TimelineEventKind::Reset,
                })
            }
        }

        for c in res.sacrament_attendance {
            let naive_date =
                NaiveDate::parse_from_str(&c.date, "%Y-%m-%d").expect("Invalid date format");
            let ndt = NaiveDateTime::new(naive_date, NaiveTime::default());
            let datetime = Utc.from_utc_datetime(&ndt);
            let timestamp = datetime.timestamp() as u64;

            if let Some(person_id) = person_events.get(&c.id) {
                if let Some(p) = people_map.get_mut(*person_id) {
                    p.timeline.push(TimelineEntry {
                        time: timestamp,
                        details: String::new(),
                        kind: TimelineEventKind::Reset,
                    })
                }
            }
        }

        let mut people = people_map.into_values().collect::<Vec<Person>>();
        people
            .iter_mut()
            .for_each(|x| x.timeline.sort_by(|a, b| a.time.cmp(&b.time)));

        Ok(people)
    }
}
