use crate::language::Value;

pub trait TryFromValue: Sized {
    fn try_from_value(value: &Value) -> Option<Self>;
}

impl TryFromValue for Value {
    fn try_from_value(value: &Value) -> Option<Self> {
        Some(value.clone())
    }
}

macro_rules! impl_for_num {
    ($num:ty) => {
        impl TryFromValue for $num {
            fn try_from_value(value: &Value) -> Option<Self> {
                let Value::Int(value) = value else {
                    return None;
                };

                Some(*value.min($num::MAX as i64).max($num::MIN as i64) as $num)
            }
        }
    };
}

impl_for_num!(f32);
