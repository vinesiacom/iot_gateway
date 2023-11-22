use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use candid::CandidType;
use serde::Deserialize;

#[derive(Clone, CandidType, Deserialize)]
pub struct Entry {
    pub timestamp: u64,
    pub fields: HashMap<String, String>,
    pub tags: HashMap<String, String>,
}

#[derive(Clone, CandidType, Deserialize)]
pub enum Expression {
    TimestampEq(u64),
    TimestampGt(u64),
    TimestampLt(u64),
    FieldEq(String, String),
    FieldContainsKey(String),
    TagEq(String, String),
    TagContainsKey(String),

    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),

    TagFilter(Vec<String>),   // Keep only these tags
    FieldFilter(Vec<String>), // Keep only these fields
}

impl Expression {
    pub fn evaluate(&self, entry: &mut Entry) -> bool {
        match self {
            Expression::TimestampEq(val) => entry.timestamp == *val,
            Expression::TimestampGt(val) => entry.timestamp > *val,
            Expression::TimestampLt(val) => entry.timestamp < *val,
            Expression::FieldEq(key, val) => entry.fields.get(key).map_or(false, |v| v == val),
            Expression::FieldContainsKey(key) => entry.fields.contains_key(key),
            Expression::TagEq(key, val) => entry.tags.get(key).map_or(false, |v| v == val),
            Expression::TagContainsKey(key) => entry.tags.contains_key(key),

            Expression::And(left, right) => left.evaluate(entry) && right.evaluate(entry),
            Expression::Or(left, right) => left.evaluate(entry) || right.evaluate(entry),

            Expression::TagFilter(keep_tags) => {
                entry.tags.retain(|k, _| keep_tags.contains(k));
                true // Always return true as this is a filtering operation, not a condition
            },
            Expression::FieldFilter(keep_fields) => {
                entry.fields.retain(|k, _| keep_fields.contains(k));
                true // Always return true as this is a filtering operation, not a condition
            },
        }
    }
}

#[derive(Clone, CandidType, Deserialize)]
pub enum AggregateFunction {
    Mean,
    Max,
    Min,
    Sum,
}

#[derive(Clone, CandidType, Deserialize)]
pub enum Action {
    Range(u64, Option<u64>), //start and optional end of range in timestamp
    Filter(Expression),      //filter the results using expression
    AggregateWindow(String, AggregateFunction), //Aggregate window specification, function to use
}

pub enum EntryCollection<'a> {
    Vec(&'a Vec<Entry>),
    Map(&'a BTreeMap<u64, Entry>),
}

impl Action {
    pub fn evaluate<'a>(&self, entries: EntryCollection<'a>) -> Vec<Entry> {
        match self {
            Action::Range(start, end) => {
                match entries {
                    EntryCollection::Map(map) => {
                        map.range(start..=&end.unwrap_or(u64::MAX))
                           .map(|(_, entry)| entry.clone())
                           .collect()
                    },
                    EntryCollection::Vec(vec) => {
                        vec.iter()
                           .filter(|entry| entry.timestamp >= *start && entry.timestamp <= end.unwrap_or(u64::MAX))
                           .cloned()
                           .collect()
                    }
                }
            },

            Action::Filter(expression) => {
                let mut filtered_entries = Vec::new();
                let entries_iter = match entries {
                    EntryCollection::Map(map) => map.values().cloned().collect(),
                    EntryCollection::Vec(vec) => vec.clone(),
                };
                for entry in entries_iter.iter() {
                    let mut item = entry.clone();
                    if expression.evaluate(&mut item) {
                        filtered_entries.push(item);
                    }
                }
                filtered_entries
            },
            Action::AggregateWindow(aggregate_function, window_size_str) => {
                let entries_vec: Vec<Entry> = match entries {
                    EntryCollection::Map(map) => map.values().cloned().collect(),
                    EntryCollection::Vec(vec) => vec.clone(),
               
                };
                Action::aggregate_entries(entries_vec, &aggregate_function, window_size_str)
            }
        }
    }

    pub fn aggregate_entries(
        entries: Vec<Entry>,
        window_size_str: &str,
        aggregate_function: &AggregateFunction,
    ) -> Vec<Entry> {
        let window_size = match Action::parse_window_size(window_size_str) {
            Some(size) => size,
            None => return Vec::new(), // Return empty if the window size is invalid
        };

        let mut sorted_entries = entries;
        sorted_entries.sort_by_key(|entry| entry.timestamp);

        let mut windowed_results = Vec::new();
        let mut window_fields: HashMap<String, Vec<i32>> = HashMap::new();
        let mut current_window_start = None;

        for entry in sorted_entries {
            let timestamp = entry.timestamp;
            current_window_start = current_window_start.or(Some(timestamp));
            let window_end = current_window_start.unwrap() + window_size;

            if timestamp >= window_end {
                let aggregated_fields = window_fields
                    .iter()
                    .map(|(field, values)| {
                        let aggregated_value = match aggregate_function {
                            AggregateFunction::Mean => Action::mean(values),
                            AggregateFunction::Max => Action::max(values),
                            AggregateFunction::Min => Action::min(values),
                            AggregateFunction::Sum => Action::sum(values),
                        };
                        (field.clone(), aggregated_value.to_string())
                    })
                    .collect();

                windowed_results.push(Entry {
                    timestamp: current_window_start.unwrap(),
                    fields: aggregated_fields,
                    tags: HashMap::new(),
                });

                current_window_start = Some(timestamp);
                window_fields.clear();
            }

            for (field, value_str) in &entry.fields {
                if let Ok(value) = value_str.parse::<i32>() {
                    window_fields
                        .entry(field.clone())
                        .or_insert_with(Vec::new)
                        .push(value);
                }
            }
        }
        // Process the last window
        if !window_fields.is_empty() {
            let aggregated_fields = window_fields
                .iter()
                .map(|(field, values)| {
                    let aggregated_value = match aggregate_function {
                        AggregateFunction::Mean => Action::mean(values),
                        AggregateFunction::Max => Action::max(values),
                        AggregateFunction::Min => Action::min(values),
                        AggregateFunction::Sum => Action::sum(values),
                    };
                    (field.clone(), aggregated_value.to_string())
                })
                .collect();

            windowed_results.push(Entry {
                timestamp: current_window_start.unwrap(),
                fields: aggregated_fields,
                tags: HashMap::new(),
            });
        }

        windowed_results
    }

    fn parse_window_size(window_size_str: &str) -> Option<u64> {
        let len = window_size_str.len();
        let (num_str, unit) = window_size_str.split_at(len - 1);
        let num = num_str.parse::<u64>().ok()?;

        match unit {
            "s" => Some(num),                // seconds
            "m" => Some(num * 60),           // minutes
            "h" => Some(num * 60 * 60),      // hours
            "d" => Some(num * 60 * 60 * 24), // days
            _ => None,
        }
    }

    fn mean(values: &[i32]) -> i32 {
        if values.is_empty() {
            return 0;
        }
        let sum: i32 = values.iter().sum();
        sum / values.len() as i32
    }

    fn max(values: &[i32]) -> i32 {
        *values.iter().max().unwrap_or(&0)
    }

    fn min(values: &[i32]) -> i32 {
        *values.iter().min().unwrap_or(&0)
    }

    fn sum(values: &[i32]) -> i32 {
        values.iter().sum()
    }
}

#[derive(Clone, CandidType, Deserialize)]
pub struct Measurement {
    name: String,
    entries: BTreeMap<u64, Entry>,
}

impl Measurement {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: BTreeMap::new(),
        }
    }

    pub fn add_entry(
        &mut self,
        timestamp: u64,
        fields: &HashMap<String, String>,
        tags: &HashMap<String, String>,
    ) {
        let entry = Entry {
            timestamp: timestamp,
            fields: fields.clone(),
            tags: tags.clone(),
        };

        self.entries.insert(timestamp, entry);
    }

    pub fn list_entries(&self) -> Vec<Entry> {
        self.entries.values().cloned().collect()
    }

    // pub fn query_entries(&self, expression: Expression) -> Vec<Entry> {
    //     self.entries
    //         .values()
    //         .cloned()
    //         .into_iter()
    //         .filter(|entry| expression.evaluate(entry))
    //         .collect()
    // }

    pub fn apply(&self, actions: &Vec<Action>) -> Vec<Entry> {
        let mut output: Vec<Entry> = vec![];

        for action in actions {
            let entries_collection = match &action {
                Action::Range(_, _) => EntryCollection::Map(&self.entries),
                _ => EntryCollection::Vec(&output),
            };
    
            let result = action.evaluate(entries_collection);
    
            // Update current_entries based on the action's output
            output = result
        }
    
        output
    }
}

pub struct TimeDb {
    db: BTreeMap<String, Measurement>,
}

impl TimeDb {
    pub fn new() -> Self {
        Self {
            db: BTreeMap::new(),
        }
    }

    pub fn get_measurement(&mut self, name: &str) -> &mut Measurement {
        if !self.db.contains_key(name) {
            let m = Measurement::new(name);
            self.db.insert(name.to_string(), m);
        }

        self.db.get_mut(name).unwrap()
    }
}

thread_local! {
    pub static DB: Rc<RefCell<TimeDb>> =  Rc::new(RefCell::new(TimeDb::new()));
}
