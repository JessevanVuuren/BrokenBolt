use serde::{Deserialize, Deserializer, de};

pub fn str_to_f64<'de, D: Deserializer<'de>>(ty: D) -> Result<f64, D::Error> {
    let str_num: String = de::Deserialize::deserialize(ty)?;
    str_num.parse::<f64>().map_err(de::Error::custom)
}