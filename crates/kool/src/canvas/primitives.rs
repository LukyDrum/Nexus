use std::ops::Add;

use crate::language::Value;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Point {
    pub fn from_value(value: &Value) -> Option<Self> {
        let Value::Array(array) = value else {
            return None;
        };
        let array = array.read().expect("Lock poisoned");

        if array.len() != 2 {
            return None;
        }

        let Value::Int(x) = array[0] else {
            return None;
        };
        let Value::Int(y) = array[1] else {
            return None;
        };

        Some(Self {
            x: x as f32,
            y: y as f32,
        })
    }
}

impl Line {
    pub fn from_value(value: Value) -> Option<Self> {
        let Value::Array(array) = value else {
            return None;
        };
        let array = array.read().expect("Lock poisoned");

        if array.len() != 2 {
            return None;
        }

        let start = Point::from_value(&array[0])?;
        let end = Point::from_value(&array[1])?;

        Some(Line { start, end })
    }
}
