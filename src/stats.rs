// Jackson Coxson

use std::collections::HashMap;

use csv::Writer;

use crate::timeline::{Person, TimelineEventKind};

const MIN_TIME: u64 = 12 * 60 * 60 * 1000;
const MILLIS_IN_DAY: f64 = 1000.0 * 60.0 * 60.0 * 24.0;

// Stats to print:
// Average time between contacts

pub fn print_stats(people: Vec<Person>, start: u64, end: u64) {
    println!(
        "{} days between contacts",
        average_contact_time(&people, start, end) / MILLIS_IN_DAY
    );
}

fn average_contact_time(people: &Vec<Person>, start: u64, end: u64) -> f64 {
    let mut times = Vec::new();
    let mut areas: HashMap<&String, Vec<u64>> = HashMap::new();

    for person in people {
        // println!("----- {} ------", person.name);
        let mut dropped = false;
        let mut pivot = None;

        for e in person.timeline.iter() {
            match e.kind {
                TimelineEventKind::Event => {
                    if dropped {
                        continue;
                    }
                    if e.time < start || e.time > end {
                        continue;
                    }
                    // println!("-- {}", e.details);
                    if let Some(pivot) = pivot {
                        let time_difference = e.time - pivot;
                        if time_difference > MIN_TIME {
                            times.push(time_difference);
                            if let Some(l) = areas.get_mut(&person.area) {
                                l.push(time_difference);
                            } else {
                                areas.insert(&person.area, vec![time_difference]);
                            }
                        }
                    }
                    pivot = Some(e.time);
                }
                TimelineEventKind::Drop => {
                    // println!("--- DROP");
                    dropped = true;
                }
                TimelineEventKind::Reset => {
                    // println!("--- RESET");
                    dropped = false;
                    pivot = None;
                }
                TimelineEventKind::Sacrament => {}
            }
        }
    }

    let areas = areas
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                (v.iter().sum::<u64>() as f64 / v.len() as f64) / MILLIS_IN_DAY,
            )
        })
        .collect::<HashMap<&String, f64>>();

    println!("{areas:#?}");
    export_to_csv(&areas, "export.csv").unwrap();

    let sum = times.iter().sum::<u64>() as f64;
    sum / times.len() as f64
}

fn export_to_csv(
    data: &HashMap<&String, f64>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::from_path(file_path)?;

    // Write the header
    writer.write_record(["Key", r#"Value"#])?;

    // Write each key-value pair
    for (key, value) in data {
        writer.write_record([key, &value.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}
