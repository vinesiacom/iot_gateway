#[cfg(test)]
// mod tests {
    use crate::timedb::{entry::Value, Entry};
#[cfg(test)]
use rand::Rng;
#[cfg(test)]
use std::collections::HashMap; // Assuming you are using the 'rand' crate for random data generation

#[cfg(test)]
pub fn create_test_entries() -> Vec<Entry> {
        let mut entries = Vec::new();
        let total_entries = 1000;
        let year_in_seconds = 365 * 24 * 60 * 60;
        let interval = year_in_seconds / total_entries;

        let mut rng = rand::thread_rng(); // Random number generator

        for i in 0..total_entries {
            let timestamp = i as u64 * interval + 1625230000;
            let entry = Entry {
                timestamp,
                fields: HashMap::from([
                    ("temperature".to_string(), Value::Int(rng.gen_range(15..30))),
                    ("humidity".to_string(), Value::Int(rng.gen_range(30..60))),
                ]),
                tags: HashMap::from([
                    (
                        "sensor_id".to_string(),
                        Value::String(format!("sensor_{}", rng.gen_range(1..10))),
                    ),
                    (
                        "location".to_string(),
                        Value::String("test_location".to_string()),
                    ),
                ]),
            };
            entries.push(entry);
        }

        entries
    }
// }
