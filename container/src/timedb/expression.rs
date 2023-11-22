use std::collections::HashSet;

use candid::CandidType;
use serde::Deserialize;

use super::{
    entry::{Entry, Value},
    query::QueryResponse,
};

#[derive(Clone, CandidType, Deserialize)]
pub enum Expression {
    // TimestampEq(u64),
    // TimestampGt(u64),
    // TimestampLt(u64),
    // FieldEq(String, String),
    // FieldContainsKey(String),
    // TagEq(String, String),
    // TagContainsKey(String),
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
    pub fn evaluate(&self, entry: &Entry) -> bool {
        match self {
            Expression::Eq(field, expected_value) => entry
                .fields
                .get(field)
                .map_or(false, |v| v == expected_value),
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
            // ... other cases ...
            // Expression::TagFilter(keep_tags) => {
            //     // Logic for TagFilter
            // },
            // Expression::FieldFilter(keep_fields) => {
            //     // Logic for FieldFilter
            // },
            // Expression::TimestampEq(val) => entry.timestamp == *val,
            // Expression::TimestampGt(val) => entry.timestamp > *val,
            // Expression::TimestampLt(val) => entry.timestamp < *val,
            // Expression::FieldEq(key, val) => entry.fields.get(key).map_or(false, |v| v == val),
            // Expression::FieldContainsKey(key) => entry.fields.contains_key(key),
            // Expression::TagEq(key, val) => entry.tags.get(key).map_or(false, |v| v == val),
            // Expression::TagContainsKey(key) => entry.tags.contains_key(key),
            Expression::And(left, right) => left.evaluate(entry) && right.evaluate(entry),
            Expression::Or(left, right) => left.evaluate(entry) || right.evaluate(entry),
            Expression::Not(val) => !val.evaluate(entry),
            // _ => true,
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
