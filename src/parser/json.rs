use crate::utils::masks::*;
use rand::{thread_rng, Rng};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn process_json_value(
    value: &Value,
    frequency_maps: &mut Vec<HashMap<String, usize>>,
    example_maps: &mut Vec<HashMap<String, String>>,
    grain: &str,
    prefix: String,
    column_names: &mut HashMap<String, usize>,
    remove_array_numbers: bool,
    pathdepth: usize,
    current_depth: usize,
) {
    match value {
        Value::Object(map) => {
            if current_depth < pathdepth {
                for (key, value) in map.iter() {
                    let full_key = if prefix.is_empty() {
                        key.to_string()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    process_json_value(
                        value,
                        frequency_maps,
                        example_maps,
                        grain,
                        full_key,
                        column_names,
                        remove_array_numbers,
                        pathdepth + 1,
                        current_depth,
                    );
                }
            }
        }
        Value::Array(values) => {
            for (idx, value) in values.iter().enumerate() {
                let full_key = if remove_array_numbers {
                    format!("{}[]", prefix)
                } else {
                    format!("{}[{}]", prefix, idx)
                };
                process_json_value(
                    value,
                    frequency_maps,
                    example_maps,
                    grain,
                    full_key,
                    column_names,
                    remove_array_numbers,
                    pathdepth + 1,
                    current_depth,
                );
            }
        }
        _ => {
            let value_str = value.to_string();
            let masked_value = mask_value(&value_str, grain);
            let idx = column_names.entry(prefix.clone()).or_insert_with(|| {
                let new_idx = frequency_maps.len();
                frequency_maps.push(HashMap::new());
                example_maps.push(HashMap::new());
                new_idx
            });

            let count = frequency_maps[*idx]
                .entry(masked_value.clone())
                .or_insert(0);
            *count += 1;

            // Reservoir sampling
            let mut rng = thread_rng();
            if rng.gen::<f64>() < 1.0 / (*count as f64) {
                example_maps[*idx].insert(masked_value.clone(), value_str);
            }
        }
    }
}

pub(crate) fn process_json_line(
    line: &str,
    frequency_maps: &mut Vec<HashMap<String, usize>>,
    example_maps: &mut Vec<HashMap<String, String>>,
    grain: &str,
    column_names: &mut HashMap<String, usize>,
    pathdepth: usize,
    remove_array_numbers: bool,
) {
    if let Ok(json_value) = serde_json::from_str::<Value>(line) {
        process_json_value(
            &json_value,
            frequency_maps,
            example_maps,
            grain,
            String::new(),
            column_names,
            remove_array_numbers,
            pathdepth,
            0,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_process_json_value() {
        let mut frequency_maps = vec![];
        let mut example_maps = vec![];
        let mut column_names = HashMap::new();

        let json_value = json!({
            "foo": {
                "bar": "baz",
                "qux": [1, 2, 3],
            },
            "hello": ["world", "foo", "bar"],
            "value": 42,
        });

        process_json_value(
            &json_value,
            &mut frequency_maps,
            &mut example_maps,
            "test",
            String::new(),
            &mut column_names,
            true,
            10,
            0,
        );

        assert_eq!(column_names.len(), 4);

        let foo_idx = column_names.get("foo.bar").unwrap();
        assert_eq!(frequency_maps[*foo_idx].get("\"aaa\"").unwrap(), &1);
        assert_eq!(
            example_maps[*foo_idx].get("\"aaa\""),
            Some(&"\"baz\"".to_string())
        );

        let hello_idx = column_names.get("hello[]").unwrap();
        assert_eq!(frequency_maps[*hello_idx].get("\"aaaaa\"").unwrap(), &1);

        let value_idx = column_names.get("value").unwrap();
        assert_eq!(frequency_maps[*value_idx].get("99").unwrap(), &1);
        assert_eq!(example_maps[*value_idx].get("99"), Some(&"42".to_string()));
    }

    #[test]
    fn test_process_json_line() {
        let mut frequency_maps: Vec<HashMap<String, usize>> = Vec::new();
        let mut example_maps: Vec<HashMap<String, String>> = Vec::new();
        let mut column_names: HashMap<String, usize> = HashMap::new();

        process_json_line(
            r#"{"foo": {"bar": [1,2,3], "baz": {"qux": "hello"}}}"#,
            &mut frequency_maps,
            &mut example_maps,
            "XXXX",
            &mut column_names,
            3,
            true,
        );

        // Check that frequency_maps has been updated correctly
        assert_eq!(
            frequency_maps,
            vec![
                hashmap! {"9".to_string() => 3},
                hashmap! {"\"aaaaa\"".to_string() => 1}
            ]
        );
    }
}
