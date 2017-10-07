use uuid::Uuid;
use chrono::NaiveDate;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;
use error::ConvertError;


/// Generic value storage 32 byte in size
/// Some contains the same value container, but the variant is more
/// important for type hinting and view presentation hinting purposes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Value {
    Nil, // no value
    Bool(bool),

    Tinyint(i8),
    Smallint(i16),
    Int(i32),
    Bigint(i64),

    Float(f32),
    Double(f64),

    Blob(Vec<u8>),
    Text(String),
    Str(&'static str),

    Uuid(Uuid),
    Date(NaiveDate),
    Timestamp(DateTime<Utc>),
}

impl Value {
    #[doc(hidden)]
    /// this is for debugging pupose only
    #[allow(unused)]
    fn get_type_name(&self) -> &'static str {
        match *self {
            Value::Nil => "Nil",
            Value::Bool(_) => "bool",
            Value::Tinyint(_) => "i8",
            Value::Smallint(_) => "i16",
            Value::Int(_) => "i32",
            Value::Bigint(_) => "i64",
            Value::Float(_) => "f32",
            Value::Double(_) => "f64",
            Value::Blob(_) => "Vec<u8>",
            Value::Text(_) => "String",
            Value::Str(_) => "&'static str",
            Value::Uuid(_) => "Uuid",
            Value::Date(_) => "NaiveDate",
            Value::Timestamp(_) => "DateTime",
        }
    }
}




macro_rules! impl_from {
    ($ty:ty, $variant: ident) => {
        /// Owned types
        impl From<$ty> for Value {
            fn from(f: $ty) -> Self{
                Value::$variant(f)
            }
        }

        /// For borrowed types
        impl<'a> From<&'a $ty> for Value {
            fn from(f: &'a $ty) -> Self{
                Value::$variant(f.to_owned())
            }
        }

        /// for borrowed option types
        impl<'a> From<&'a Option<$ty>> for Value {
            fn from(f: &'a Option<$ty>) -> Self{
                match *f{
                    Some(ref f) => From::from(f), 
                    None => Value::Nil,
                }
            }
        }
    }
}

impl_from!(bool, Bool);
impl_from!(i8, Tinyint);
impl_from!(i16, Smallint);
impl_from!(i32, Int);
impl_from!(i64, Bigint);
impl_from!(f32, Float);
impl_from!(f64, Double);
impl_from!(Vec<u8>, Blob);
impl_from!(String, Text);
impl_from!(&'static str, Str);
impl_from!(Uuid, Uuid);
impl_from!(NaiveDate, Date);
impl_from!(DateTime<Utc>, Timestamp);

macro_rules! impl_tryfrom {
    ($ty: ty, $ty_name: tt, $($variant: ident),*) => {
        /// try from to owned
        impl<'a> TryFrom<&'a Value> for $ty {
            type Error = ConvertError;

            fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
                match *value {
                    $(Value::$variant(ref v) => Ok(v.to_owned() as $ty),
                    )*
                    _ => Err(ConvertError::NotSupported(value.get_type_name().to_string(), $ty_name.into())),
                }
            }
        }

        /// try from to Option<T>
        impl<'a> TryFrom<&'a Value> for Option<$ty> {
            type Error = ConvertError;

            fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
                match *value {
                    Value::Nil => Ok(None),
                    _ => TryFrom::try_from(value).map(|v|Some(v)), 
                }
            }
        }
    }
}

impl_tryfrom!(bool, "bool", Bool);
impl_tryfrom!(i8, "i8", Tinyint);
impl_tryfrom!(i16, "i16", Tinyint, Smallint);
impl_tryfrom!(i32, "i32", Tinyint, Smallint, Int);
impl_tryfrom!(i64, "i64", Tinyint, Smallint, Int, Bigint);
impl_tryfrom!(f32, "f32", Float);
impl_tryfrom!(f64, "f64", Float, Double);
impl_tryfrom!(Vec<u8>, "Vec<u8>", Blob);
impl_tryfrom!(String, "String", Text);
impl_tryfrom!(&'static str, "&'static str", Str);
impl_tryfrom!(Uuid, "Uuid", Uuid);
impl_tryfrom!(NaiveDate, "NaiveDate", Date);
impl_tryfrom!(DateTime<Utc>, "DateTime<Utc>", Timestamp);

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    use chrono::offset::Utc;

    #[test]
    fn data_sizes() {
        assert_eq!(32, size_of::<Value>());
        assert_eq!(24, size_of::<Vec<u8>>());
        assert_eq!(24, size_of::<String>());
        assert_eq!(12, size_of::<DateTime<Utc>>());
        assert_eq!(4, size_of::<NaiveDate>());
        assert_eq!(16, size_of::<Uuid>());
    }

    #[test]
    fn test_types() {
        let _: Value = 127i8.into();
        let _: Value = 2222i16.into();
        let _: Value = 4444i32.into();
        let _: Value = 10000i64.into();
        let _v1: Value = 1.0f32.into();
        let _v2: Value = 100.0f64.into();
        let _v3: Value = Utc::now().into();
        let _v7: Value = Utc::today().naive_utc().into();
        let _v4: Value = "hello world!".into();
        let _v5: Value = "hello world!".to_string().into();
        let _v6: Value = vec![1u8, 2, 255, 3].into();
    }
}
