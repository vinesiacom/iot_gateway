use std::{collections::HashMap, error::Error, rc::Rc};

use candid::CandidType;
use serde::Deserialize;

use super::{
    aggregate::AggregateFunction,
    entry::{Entry, Value},
    expression::Expression,
    index::Indexes,
    query::QueryResponse,
};

#[derive(Clone, CandidType, Deserialize)]
pub enum Action {
    Range(u64, Option<u64>), //start and optional end of range in timestamp
    Filter(Expression),      //filter the results using expression
    AggregateWindow(String, AggregateFunction), //Aggregate window specification, function to use
}

impl Action {
    pub fn init(&self, indexes: &Indexes) -> Result<QueryResponse, Box<dyn Error>> {
        let mut query_response = QueryResponse::new();

        match self {
            Action::Range(start, end) => {
                let range = indexes
                    .main_index
                    .range(start..=&end.unwrap_or(u64::MAX))
                    .map(|(_, entry)| entry.clone())
                    .collect();

                query_response.items = range;
            }

            Action::Filter(expression) => {
                let entries = indexes.values();
                query_response.items = entries;
                expression.filter(&mut query_response);
            }
            Action::AggregateWindow(aggregate_function, window_size_str) => {
                let entries = indexes.values();
                query_response.items =
                    Action::aggregate_entries(&entries, &aggregate_function, window_size_str)?;
            }
        };

        Ok(query_response)
    }

    pub fn evaluate(
        &self,
        query_response: &QueryResponse,
    ) -> Result<QueryResponse, Box<dyn Error>> {
        let mut output = query_response.clone();

        match self {
            Action::Range(start, end) => {
                output.items = output
                    .items
                    .iter()
                    .filter(|entry| {
                        entry.timestamp >= *start && entry.timestamp <= end.unwrap_or(u64::MAX)
                    })
                    .cloned()
                    .collect();
            }

            Action::Filter(expression) => {
                expression.filter(&mut output);
            }
            Action::AggregateWindow(aggregate_function, window_size_str) => {
                output.items =
                    Action::aggregate_entries(&output.items, &aggregate_function, window_size_str)?;
            }
        };

        Ok(output)
    }

    pub fn aggregate_entries(
        entries: &Vec<Rc<Entry>>,
        window_size_str: &str,
        aggregate_function: &AggregateFunction,
    ) -> Result<Vec<Rc<Entry>>, Box<dyn Error>> {
        let window_size = match Action::parse_window_size(window_size_str) {
            Some(size) => size,
            None => return Ok(Vec::new()), // Return empty if the window size is invalid
        };

        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by_key(|entry| entry.timestamp);

        let mut windowed_results: Vec<Rc<Entry>> = Vec::new();
        let mut window_fields: HashMap<String, Vec<Value>> = HashMap::new(); //Contains a list of fields with their values for every entry in current window
        let mut current_window_start = None;

        for entry in sorted_entries {
            let timestamp = entry.timestamp;
            current_window_start = current_window_start.or(Some(timestamp));
            let window_end = current_window_start.unwrap() + window_size;

            if timestamp >= window_end {
                let mut aggregated_fields = HashMap::new();

                for (field, values) in window_fields.iter() {
                    let aggregated_value = match aggregate_function {
                        AggregateFunction::Mean => AggregateFunction::mean(values)?,
                        AggregateFunction::Max => AggregateFunction::max(values)?,
                        AggregateFunction::Min => AggregateFunction::min(values)?,
                        AggregateFunction::Sum => AggregateFunction::sum(values)?,
                    };
                    aggregated_fields.insert(field.clone(), aggregated_value);
                }

                windowed_results.push(Rc::new(Entry {
                    timestamp: current_window_start.unwrap(),
                    fields: aggregated_fields,
                    tags: HashMap::new(),
                }));

                current_window_start = Some(timestamp);
                window_fields.clear();
            }

            //Fill window_fields with all the field names that are there
            for (field, value) in &entry.fields {
                window_fields
                    .entry(field.clone())
                    .or_insert_with(Vec::new)
                    .push(value.clone());
            }
        }

        // Process the last window
        if !window_fields.is_empty() {
            let mut aggregated_fields = HashMap::new();

            for (field, values) in window_fields.iter() {
                let aggregated_value = match aggregate_function {
                    AggregateFunction::Mean => AggregateFunction::mean(values)?,
                    AggregateFunction::Max => AggregateFunction::max(values)?,
                    AggregateFunction::Min => AggregateFunction::min(values)?,
                    AggregateFunction::Sum => AggregateFunction::sum(values)?,
                };
                aggregated_fields.insert(field.clone(), aggregated_value);
            }

            windowed_results.push(Rc::new(Entry {
                timestamp: current_window_start.unwrap(),
                fields: aggregated_fields,
                tags: HashMap::new(),
            }));
        }

        Ok(windowed_results)
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
}
