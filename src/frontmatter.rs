use serde_yaml as yaml;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Frontmatter<T> {
    map: HashMap<String, T>,
}

impl<T> Frontmatter<T>
where
    for<'a> T: serde::de::Deserialize<'a> + serde::Serialize,
{
    pub fn new(f: &mut String) -> Result<Frontmatter<T>, yaml::Error> {
        let f = f.split("---").collect::<Vec<&str>>()[1];
        let m = match yaml::from_str::<HashMap<String, T>>(&f) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
        Ok(Frontmatter { map: m })
    }
    pub fn get(&self, key: &str) -> Option<&T> {
        self.map.get(key)
    }
    pub fn set(&mut self, key: &str, value: T) {
        self.map.insert(String::from(key), value);
    }
    pub fn to_string(&self) -> Result<String, yaml::Error> {
        Ok(format!("---\n{}---\n\n", yaml::to_string(&self.map)?))
    }
    pub fn to_map(&self) -> &HashMap<String, T> {
        &self.map
    }
}

pub fn to_yaml_value<T>(value: Vec<T>, json_to_string: bool) -> yaml::Value
where
    T: ToString + std::fmt::Display,
{
    if value.len() >= 2 {
        yaml::Value::Sequence(
            value
                .iter()
                // This causes a recursion limit, which I'm not caring on fixing
                // for now knowing the scope of this project as a hole.
                // .map(|v| to_yaml_value(vec![v; 1], json_to_string))
                .map(|v| yaml::Value::String(v.to_string()))
                .collect::<Vec<yaml::Value>>(),
        );
    }

    let value = &value[0].to_string();

    match value.to_lowercase().as_str() {
        "null" | "~" => return yaml::Value::Null,
        "true" | "yes" => return yaml::Value::Bool(true),
        "false" | "no" => return yaml::Value::Bool(false),
        _ => (),
    }

    if let Ok(v) = value.parse::<u64>() {
        return yaml::Value::Number(v.into());
    }
    if let Ok(v) = value.parse::<i64>() {
        return yaml::Value::Number(v.into());
    }
    if let Ok(v) = value.parse::<f64>() {
        return yaml::Value::Number(v.into());
    }

    match yaml::from_str::<serde_yaml::Value>(value) {
        Ok(v) => {
            if json_to_string {
                return yaml::Value::String(String::from(value));
            } else {
                return v;
            }
        }
        Err(_) => (),
    }

    yaml::Value::String(String::from(value))
}
