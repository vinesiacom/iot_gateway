use super::{
    entry::{Entry, Value},
    index::Indexes,
    query::QueryResponse,
    Action,
};
use std::{collections::HashMap, error::Error, rc::Rc};

pub struct Measurement {
    pub name: String,
    indexes: Indexes,
}

impl Measurement {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),

            indexes: Indexes::new(),
        }
    }

    pub fn add_entry(
        &mut self,
        timestamp: u64,
        fields: &HashMap<String, Value>,
        tags: &HashMap<String, Value>,
    ) {
        let entry = Entry {
            timestamp: timestamp,
            fields: fields.clone(),
            tags: tags.clone(),
        };

        self.indexes.insert(timestamp, entry);
    }

    pub fn list_entries(&self) -> Vec<Rc<Entry>> {
        self.indexes.values()
    }

    pub fn apply(&self, actions: &Vec<Action>) -> Result<Option<QueryResponse>, Box<dyn Error>> {
        if actions.len() > 0 {
            let mut query_response = actions[0].init(&self.indexes)?;

            for action in actions.iter().skip(1) {
                query_response = action.evaluate(&query_response)?;
            }

            Ok(Some(query_response))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::timedb::{test_helper::create_test_entries, expression::Expression};

    use super::*;

    #[test]
    fn test_measurement_new() {
        let measurement = Measurement::new("test_measurement");

        assert_eq!(measurement.name, "test_measurement");
        // Additional assertions to verify that `indexes` are initialized correctly
    }

    #[test]
    fn test_add_entry() {
        let mut measurement = Measurement::new("test_measurement");
        let fields = HashMap::from([("field1".to_string(), Value::Int(42))]);
        let tags = HashMap::from([("tag1".to_string(), Value::String("value1".to_string()))]);

        measurement.add_entry(123456, &fields, &tags);
        let entries = measurement.list_entries();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].timestamp, 123456);
        assert_eq!(entries[0].fields, fields);
        assert_eq!(entries[0].tags, tags);
    }


    #[test]
    fn test_list_entries() {
        let mut measurement = Measurement::new("test_measurement");
        // Add multiple entries
        let entries = create_test_entries();
        for entry in entries {
            measurement.add_entry(entry.timestamp, &entry.fields, &entry.tags);
        }


        let entries = measurement.list_entries();
        assert_eq!(entries.len(), 1000/* expected number of entries */);
    }

    #[test]
    fn test_apply_actions() {
        let mut measurement = Measurement::new("test_measurement");
        // Populate measurement with entries
        let entries = create_test_entries();
        for entry in entries {
            measurement.add_entry(entry.timestamp, &entry.fields, &entry.tags);
        }

        let start = 1625230000;
        let range = 3*30*24*60*60;
        let end = start + range;
        let sensor_id = "sensor_6".to_string();

        let action = Action::Range(start, Some(end));
        let sensor2 = Action::Filter(Expression::Eq("sensor_id".to_string(), Value::String(sensor_id.clone())));

        let actions = vec![
            action,
            sensor2
            // Other actions
        ];

        // from('measurement')
        //  |> range(from, to)
        //  |> filer((x) => x.sensor_id = 'sensor_6')

        if let Ok(Some(query_response)) = measurement.apply(&actions) {
            // Assertions based on expected outcomes of applying the actions
            print!("Items {} \n", query_response.items.len());

            for entry in query_response.items {
                assert!(entry.timestamp >= start && entry.timestamp <= end);
            
                let sensor_val = entry.get_value("sensor_id");

                match sensor_val {
                    Some(sensor_val) => assert_eq!(sensor_val, &Value::String(sensor_id.clone())),
                    _ => panic!("Incorrect sensor value")
                }
            }

        } else {
            panic!("Action application failed");
        }
    }

}
