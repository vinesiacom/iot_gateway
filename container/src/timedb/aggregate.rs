use candid::CandidType;
use serde::Deserialize;

use std::boxed::Box;
use std::error::Error;

use super::entry::Value;

#[derive(Clone, CandidType, Deserialize)]
pub enum AggregateFunction {
    Mean,
    Max,
    Min,
    Sum,
}

impl AggregateFunction {
    pub fn mean(values: &Vec<Value>) -> Result<Value, Box<dyn Error>> {
        if values.is_empty() {
            return Ok(Value::None);
        }

        let sum_result: Result<Vec<f64>, _> = values
            .iter()
            .map(|value| match value {
                Value::Int(v) => Ok(*v as f64),
                Value::UInt(v) => Ok(*v as f64),
                Value::Float(v) => Ok(*v as f64),
                _ => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Non-numeric value",
                ))),
            })
            .collect();

        let sum_vec = sum_result?;
        let sum: f64 = sum_vec.iter().sum();
        let mean = sum / values.len() as f64;

        Ok(Value::Float(mean as f32)) // Assuming the mean should be returned as a float
    }

    pub fn max(values: &[Value]) -> Result<Value, Box<dyn Error>> {
        if values.is_empty() {
            return Ok(Value::None);
        }

        let max_value = values
            .iter()
            .filter_map(|value| match value {
                Value::Int(v) => Some(*v as f64),
                Value::UInt(v) => Some(*v as f64),
                Value::Float(v) => Some(*v as f64),
                _ => None,
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        max_value.map_or(
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Non-numeric value",
            ))),
            |v| Ok(Value::Float(v as f32)),
        )
    }

    pub fn min(values: &[Value]) -> Result<Value, Box<dyn Error>> {
        if values.is_empty() {
            return Ok(Value::None);
        }

        let min_value = values
            .iter()
            .filter_map(|value| match value {
                Value::Int(v) => Some(*v as f64),
                Value::UInt(v) => Some(*v as f64),
                Value::Float(v) => Some(*v as f64),
                _ => None,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        min_value.map_or(
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Non-numeric value",
            ))),
            |v| Ok(Value::Float(v as f32)),
        )
    }

    pub fn sum(values: &[Value]) -> Result<Value, Box<dyn Error>> {
        if values.is_empty() {
            return Ok(Value::None);
        }

        let sum_result: Result<Vec<f64>, _> = values
            .iter()
            .map(|value| match value {
                Value::Int(v) => Ok(*v as f64),
                Value::UInt(v) => Ok(*v as f64),
                Value::Float(v) => Ok(*v as f64),
                _ => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Non-numeric value",
                ))),
            })
            .collect();

        let sum_vec = sum_result?;
        let sum: f64 = sum_vec.iter().sum();

        Ok(Value::Float(sum as f32))
    }
}
