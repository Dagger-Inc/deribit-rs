use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EstimatedExpirationPriceData {
    pub is_estimated: bool,
    pub price: f64,
    pub seconds: f64,
}

#[derive(Debug, Clone)]
pub struct EstimatedExpirationPriceChannel(String);
impl<'de> Deserialize<'de> for EstimatedExpirationPriceChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["estimated_expiration_price", index_name] => {
                Ok(EstimatedExpirationPriceChannel(index_name.to_string()))
            }
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"estimated_expiration_price.{index_name}"
            )),
        }
    }
}
impl Serialize for EstimatedExpirationPriceChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for EstimatedExpirationPriceChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "estimated_expiration_price.{}", self.0)
    }
}
