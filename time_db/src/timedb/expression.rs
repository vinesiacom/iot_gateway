use std::{collections::HashSet, rc::Rc};

use candid::CandidType;
use serde::Deserialize;

use super::{
    entry::{Entry, Value},
    query::QueryResponse,
};

#[derive(Clone, CandidType, Deserialize)]
pub enum Expression {
    Eq(String, Value), // Field or tag name, Equal
    Gt(String, Value), // Field or tag name, Greater than
    Lt(String, Value), // Field or tag name, Lower than
    Ge(String, Value), // Field or tag name, Greater or equal
    Le(String, Value), // Field or tag name, Lower or equal

    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),

    TagFilter(Vec<String>),   // Keep only these tags
    FieldFilter(Vec<String>), // Keep only these fields
}

impl Expression {
    pub fn evaluate(&self, entry: &Rc<Entry>) -> bool {
        match self {
        Expression::Eq(field, expected_value) => entry.get_value(field).map_or(false, |v| v == expected_value),
            Expression::Gt(field, expected_value) => {
                let val = entry.get_value(field);
                match val {
                    Some(val) => Expression::compare_gt(&val, expected_value),
                    _ => false,
                }
            }
            Expression::Lt(field, expected_value) => {
                let val = entry.get_value(field);
                match val {
                    Some(val) => Expression::compare_lt(&val, expected_value),
                    _ => false,
                }
            }
            Expression::Ge(field, expected_value) => {
                let val = entry.get_value(field);
                match val {
                    Some(val) => Expression::compare_ge(&val, expected_value),
                    _ => false,
                }
            }
            Expression::Le(field, expected_value) => {
                let val = entry.get_value(field);
                match val {
                    Some(val) => Expression::compare_le(&val, expected_value),
                    _ => false,
                }
            }

            Expression::TagFilter(_) => true,
            Expression::FieldFilter(_) => true,
            Expression::And(left, right) => left.evaluate(entry) && right.evaluate(entry),
            Expression::Or(left, right) => left.evaluate(entry) || right.evaluate(entry),
            Expression::Not(val) => !val.evaluate(entry),
        }
    }

    pub fn filter(&self, query: &mut QueryResponse) {
        match self {
            Expression::TagFilter(keep_tags) => {
                query.tags = Expression::merge_unique(&query.tags, keep_tags)
            }
            Expression::FieldFilter(keep_fields) => {
                query.fields = Expression::merge_unique(&query.tags, keep_fields)
            }
            _ => {}
        }
    }

    fn merge_unique(vec1: &Vec<String>, vec2: &Vec<String>) -> Vec<String> {
        let mut unique_set: HashSet<String> = HashSet::new();

        unique_set.extend(vec1.clone());
        unique_set.extend(vec2.clone());

        unique_set.into_iter().collect()
    }

    fn compare_gt(val1: &Value, val2: &Value) -> bool {
        match (val1, val2) {
            (Value::Int(v1), Value::Int(v2)) => v1 > v2,
            (Value::Int(v1), Value::Float(v2)) => (*v1 as f32) > *v2,
            (Value::Float(v1), Value::Int(v2)) => *v1 > (*v2 as f32),
            (Value::Float(v1), Value::Float(v2)) => *v1 > *v2,
            (Value::UInt(v1), Value::UInt(v2)) => v1 > v2,
            // Other type comparisons as needed
            _ => false, // Return false for non-comparable types or unhandled combinations
        }
    }

    fn compare_ge(val1: &Value, val2: &Value) -> bool {
        match (val1, val2) {
            (Value::Int(v1), Value::Int(v2)) => v1 >= v2,
            (Value::Int(v1), Value::Float(v2)) => (*v1 as f32) >= *v2,
            (Value::Float(v1), Value::Int(v2)) => *v1 >= (*v2 as f32),
            (Value::Float(v1), Value::Float(v2)) => *v1 >= *v2,
            (Value::UInt(v1), Value::UInt(v2)) => v1 >= v2,
            _ => false,
        }
    }

    fn compare_lt(val1: &Value, val2: &Value) -> bool {
        match (val1, val2) {
            (Value::Int(v1), Value::Int(v2)) => v1 < v2,
            (Value::Int(v1), Value::Float(v2)) => (*v1 as f32) < *v2,
            (Value::Float(v1), Value::Int(v2)) => *v1 < (*v2 as f32),
            (Value::Float(v1), Value::Float(v2)) => *v1 < *v2,
            (Value::UInt(v1), Value::UInt(v2)) => v1 < v2,
            _ => false,
        }
    }

    fn compare_le(val1: &Value, val2: &Value) -> bool {
        match (val1, val2) {
            (Value::Int(v1), Value::Int(v2)) => v1 <= v2,
            (Value::Int(v1), Value::Float(v2)) => (*v1 as f32) <= *v2,
            (Value::Float(v1), Value::Int(v2)) => *v1 <= (*v2 as f32),
            (Value::Float(v1), Value::Float(v2)) => *v1 <= *v2,
            (Value::UInt(v1), Value::UInt(v2)) => v1 <= v2,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc};

    use super::*;

    #[test]
    fn test_eq_expression() {
        let entry = Rc::new(Entry {
            timestamp: 123456,
            fields: HashMap::from([
                ("temperature".to_string(), Value::Int(25)),
                // ... other fields ...
            ]),
            tags: HashMap::new(),
        });

        let expr = Expression::Eq("temperature".to_string(), Value::Int(25));
        assert!(expr.evaluate(&entry));

        let expr_not_equal = Expression::Eq("temperature".to_string(), Value::Int(30));
        assert!(!expr_not_equal.evaluate(&entry));
    }

    #[test]
    fn test_gt_expression() {
        let entry = Rc::new(Entry {
            timestamp: 123456,
            fields: HashMap::from([
                ("temperature".to_string(), Value::Int(25)),
                ("temperature_f".to_string(), Value::Float(25.0)),
                // ... other fields ...
            ]),
            tags: HashMap::new(),
        });

        let expr_int = Expression::Gt("temperature".to_string(), Value::Int(24));
        assert!(expr_int.evaluate(&entry));

        let expr_float = Expression::Gt("temperature".to_string(), Value::Float(24.0));
        assert!(expr_float.evaluate(&entry));

        let expr_int_2 = Expression::Gt("temperature_f".to_string(), Value::Int(24));
        assert!(expr_int_2.evaluate(&entry));

        let expr_float_2 = Expression::Gt("temperature_f".to_string(), Value::Float(24.0));
        assert!(expr_float_2.evaluate(&entry));

        let expr_not_equal = Expression::Eq("temperature".to_string(), Value::Int(30));
        assert!(!expr_not_equal.evaluate(&entry));
    }


    #[test]
    fn test_tag_filter() {
        let entry = Entry {
            timestamp: 123456,
            fields: HashMap::from([
                ("temperature".to_string(), Value::Int(25)),
                ("temperature_f".to_string(), Value::Float(25.0)),
                // ... other fields ...
            ]),
            tags: HashMap::new(),
        };

        let mut query_response = QueryResponse::new();
        query_response.items = vec![Rc::new(entry)];

        let tag_filter = Expression::TagFilter(vec!["temperature".to_string()]);
        tag_filter.filter(&mut query_response);

        assert_eq!(query_response.tags.len(), 1);
    }
    // Additional test cases...
}