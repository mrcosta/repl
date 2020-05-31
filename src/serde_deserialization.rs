use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Skippable<T> {
    None,
    Some(T),
}

impl<T> Skippable<T> {
    pub fn is_some(&self) -> bool {
        match *self {
            Skippable::Some(_) => true,
            Skippable::None => false,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        match self {
            Skippable::Some(x) => Some(f(x)),
            Skippable::None => None,
        }
    }

    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            Skippable::Some(ref x) => Some(x),
            Skippable::None => None,
        }
    }
}

impl<T> Default for Skippable<T> {
    fn default() -> Self {
        Skippable::None
    }
}

impl<'de, T> Deserialize<'de> for Skippable<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Skippable::Some(T::deserialize(deserializer)?))
    }
}

#[test]
fn empty_hash_map_deserializes_as_an_empty_hash_map() {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct MyStruct {
        my_map: HashMap<String, isize>,
    }

    let json = r#"{ "my_map": null }"#;

    let my_struct = serde_json::from_str::<MyStruct>(json).unwrap();
    assert_eq!(
        my_struct,
        MyStruct {
            my_map: hashmap! {},
        }
    );
}

#[test]
fn optional_hash_map_deserializes_as_none() {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct MyStruct {
        my_map: HashMap<String, isize>,
    }

    let json = r#"{ }"#;

    let my_struct = serde_json::from_str::<MyStruct>(json).unwrap();
    // assert_eq!(my_struct, MyStruct { my_map:  });
}

#[test]
#[ignore]
fn not_sent_hash_map_should_deserializes_as_an_empty_hash_map_but_throws_error() {
    // called `Result::unwrap()` on an `Err` value: Error("missing field `my_map`", line: 1, column: 3)
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct MyStruct {
        my_map: HashMap<String, isize>,
    }

    let json = r#"{ }"#;

    serde_json::from_str::<MyStruct>(json).unwrap();
}

#[test]
fn not_sent_hash_map_deserializes_as_skippable_none() {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct MyStruct {
        #[serde(default)]
        my_map: Skippable<HashMap<String, isize>>,
    }

    let json = r#"{ }"#;

    let my_struct = serde_json::from_str::<MyStruct>(json).unwrap();
    assert_eq!(
        my_struct,
        MyStruct {
            my_map: Skippable::None,
        }
    );
}

#[test]
fn not_sent_hash_map_deserializes_as_skippable_none_test_null() {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct MyStruct {
        #[serde(default)]
        my_map: Skippable<HashMap<String, isize>>,
    }

    let json = r#"{ "my_map": null }"#;

    let my_struct = serde_json::from_str::<MyStruct>(json).unwrap();
    assert_eq!(
        my_struct,
        MyStruct {
            my_map: Skippable::None,
        }
    );
}

#[test]
fn not_sent_hash_map_deserializes_as_optional_none() {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct MyStruct {
        my_map: Option<String>,
    }

    let json = r#"{  }"#;

    let my_struct = serde_json::from_str::<MyStruct>(json).unwrap();
    assert_eq!(my_struct, MyStruct { my_map: None });
}

#[test]
fn null_sent_hash_map_deserializes_as_optional_none() {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct MyStruct {
        my_map: Option<String>,
    }

    let json = r#"{ "my_hap": null }"#;

    let my_struct = serde_json::from_str::<MyStruct>(json).unwrap();
    assert_eq!(my_struct, MyStruct { my_map: None });
}
