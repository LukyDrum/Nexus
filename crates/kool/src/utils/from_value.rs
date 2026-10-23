use std::collections::HashMap;

use crate::{language::Value, utils::CloneInner};

pub trait TryFromValue: Sized {
    fn try_from_value(value: Value) -> Option<Self>;
}

impl TryFromValue for Value {
    fn try_from_value(value: Value) -> Option<Self> {
        Some(value.clone())
    }
}

impl TryFromValue for String {
    fn try_from_value(value: Value) -> Option<Self> {
        let Value::String(string) = value else {
            return None;
        };

        Some(string.clone_inner())
    }
}

impl TryFromValue for bool {
    fn try_from_value(value: Value) -> Option<Self> {
        let Value::Bool(value) = value else {
            return None;
        };

        Some(value)
    }
}

impl<T> TryFromValue for Vec<T>
where
    T: TryFromValue,
{
    fn try_from_value(value: Value) -> Option<Self> {
        let Value::Array(array) = value else {
            return None;
        };

        array
            .read()
            .expect("Lock poisoned")
            .iter()
            .map(|value| T::try_from_value(value.clone()))
            .collect()
    }
}

impl<T> TryFromValue for HashMap<String, T>
where
    T: TryFromValue,
{
    fn try_from_value(value: Value) -> Option<Self> {
        let Value::HashMap(map) = value else {
            return None;
        };

        map.read()
            .expect("Lock poisoned")
            .iter()
            .map(|(key, value)| {
                T::try_from_value(value.clone()).map(|value| (key.to_owned(), value))
            })
            .collect()
    }
}

macro_rules! impl_for_num {
    ($num:ty) => {
        impl TryFromValue for $num {
            fn try_from_value(value: Value) -> Option<Self> {
                let Value::Int(value) = value else {
                    return None;
                };

                Some(value.min(<$num>::MAX as i64).max(<$num>::MIN as i64) as $num)
            }
        }
    };
}

impl_for_num!(i32);
impl_for_num!(u32);
impl_for_num!(f32);
impl_for_num!(usize);
