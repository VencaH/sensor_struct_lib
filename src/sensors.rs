use serde::{Deserialize, Serialize};
use serde_json::{to_string,to_vec, from_slice};

use std::collections::HashMap;
use std::str::Bytes;
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SensorDefinition {
    pub sensor_id: String,
    pub sensor_type: String,
    pub component: String,
}

impl TryInto<String> for SensorDefinition {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        to_string(&self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct SensorData {
    pub sensor_id: String,
    pub date: DateTime<Utc>,
    pub measures: Vec<Measure>,
}

impl SensorData {
    #[must_use]
    pub fn get_formatted_measures(&self) -> HashMap<String, String> {
        self.measures.iter().map(|measure| {
            match measure {
                Measure::Humidity(value) => {
                    ("humidity".to_string(), format!("{value:.1} %"))
                }
                Measure::Temperature(value) => {
                    ("temperature".to_string(), format!("{value:.1} °C"))
                }
                Measure::Unknown(value) => {
                    ("unknown".to_string(), format!("{value:.1} °C"))
                }
            }
        }).collect::<HashMap<String, String>>()
    }
}


impl TryFrom<&Bytes> for SensorData {
    type Error = serde_json::Error;

    fn try_from(value: &Bytes) -> Result<Self, Self::Error> {
        from_slice(value.into())
    }
}

impl TryInto<Vec<u8>> for SensorData {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        to_vec(&self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq, Clone)]
pub enum Measure {
    Temperature(f32),
    Humidity(f32),
    Unknown(f32),
}

impl Default for Measure {
    fn default() -> Self {
        Self::Unknown(0f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_data_get_formatted_measures() {
        // This test was written by chatGPT 3.5
        let data = SensorData {
            sensor_id: "sensor-1".to_string(),
            date: Utc::now(),
            measures: vec![
                Measure::Temperature(25.0),
                Measure::Humidity(50.0),
                Measure::Unknown(10.0),
            ],
        };
        let formatted_measures = data.get_formatted_measures();
        let expected_result: HashMap<String, String> = [
            ("temperature".to_string(), "25.0 °C".to_string()),
            ("humidity".to_string(), "50.0 %".to_string()),
            ("unknown".to_string(), "10.0 °C".to_string()),
        ]
            .iter()
            .cloned()
            .collect();
        assert_eq!(formatted_measures, expected_result);
    }


    #[test]
    fn test_deserialize() {
        let raw = r#"{"sensor_id": "esp8266_tmphum_001", "date": "2023-04-15T19:55:06.973482577+00:00", "measures": [{"Temperature": 23.170000076293945}, {"Humidity": 56.197265625}]}"#;
        let deserialized = serde_json::from_str::<SensorData>(raw);
        println!("{deserialized:?}");
    }
}
