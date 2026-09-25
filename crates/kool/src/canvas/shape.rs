use std::collections::HashMap;

use crate::{canvas::Point, language::Value};

pub const SHAPE_KEY: &str = "shape";
pub const RADIUS_KEY: &str = "radius";
pub const WIDTH_KEY: &str = "width";
pub const HEIGHT_KEY: &str = "height";
pub const POINTS_KEY: &str = "points";

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Circle {
        center: Point,
        radius: f32,
    },
    Rectangle {
        center: Point,
        width: f32,
        height: f32,
    },
    Polygon {
        points: Vec<Point>,
    }
}

impl Shape {
    pub fn from_value(value: Value) -> Option<Self> {
        let Value::HashMap(map) = value else {
            return None;
        };
        let map = map.read().expect("Lock poisoned");

        let Some(Value::String(shape_name)) = get_from_map(&map, SHAPE_KEY) else {
            return None;
        };

        match shape_name.as_str().to_lowercase().as_str() {
            "circle" => {

            }
        }
    }
}

fn get_from_map<'a>(map: &'a HashMap<Value, Value>, key: &str) -> Option<&'a Value> {
    map.get(&Value::new_string(key))
}

fn get_f32_from_map(map: &HashMap<Value, Value>, key: &str) -> Option<f32> {
    let Value::Int(value) = map.get(&Value::new_string(key))? else {
        return None;
    };
    Some(*value as f32)
}

fn get_point_from_map(map: &HashMap<Value, Value>, key: &str) -> Option<f32> {
    let Value::Int(value) = map.get(&Value::new_string(key))? else {
        return None;
    };
    Some(*value as f32)
}
