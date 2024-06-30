use bevy::utils::HashMap;
use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug)]
pub struct OrderFile {
    pub models: Vec<String>,
    pub model_locations: Vec<[f32; 3]>,
    pub audio: Vec<String>,
    pub audio_locations: Vec<[f32; 3]>,
}

impl<'de> Deserialize<'de> for OrderFile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrderFileVisitor;
        impl<'de> Visitor<'de> for OrderFileVisitor {
            type Value = OrderFile;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct OrderFile")
            }
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut models: Vec<String> = vec![];
                let mut model_locations: Vec<[f32; 3]> = vec![];
                let mut audio: Vec<String> = vec![];
                let mut audio_locations: Vec<[f32; 3]> = vec![];
                while let Some(entry) = map.next_entry::<&str, Vec<HashMap<&str, [f32; 3]>>>()? {
                    match entry.0 {
                        "models" => {
                            entry.1.iter().for_each(|object_location| {
                                object_location.iter().for_each(|(object, location)| {
                                    models.push(String::from(*object));
                                    model_locations.push(*location);
                                });
                            });
                        }
                        "audio" => {
                            entry.1.iter().for_each(|object_location| {
                                object_location.iter().for_each(|(object, location)| {
                                    audio.push(String::from(*object));
                                    audio_locations.push(*location)
                                });
                            });
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                return Ok(OrderFile {
                    models: models,
                    model_locations: model_locations,
                    audio: audio,
                    audio_locations: audio_locations,
                });
            }
        }
        return deserializer.deserialize_map(OrderFileVisitor);
    }
}
