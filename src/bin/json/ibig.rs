use ibig::IBig;
use serde::{Serialize, Serializer};

const MIN_SAFE_INTEGER: i64 = -9007199254740991;
const MAX_SAFE_INTEGER: i64 = 9007199254740991;

#[derive(Serialize)]
pub struct IBigSerializer<'a>(#[serde(serialize_with = "serialize_ibig")] &'a IBig);

impl<'a> IBigSerializer<'a> {
    pub fn new(n: &'a IBig) -> Self {
        Self(n)
    }
}

pub fn serialize_ibig<S>(n: &IBig, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if n < &IBig::from(MIN_SAFE_INTEGER) || n > &IBig::from(MAX_SAFE_INTEGER) {
        serializer.serialize_str(&n.to_string())
    } else {
        // Unwrap is safe because MIN_SAFE_INTEGER .. MAX_SAFE_INTEGER always fits in an i64.
        serializer.serialize_i64(n.to_string().parse().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ibig::IBig;
    use serde_json::{json, to_value, Value};

    #[derive(Serialize)]
    struct IBigSerializer(#[serde(serialize_with = "serialize_ibig")] IBig);

    fn serialize(n: IBig) -> Result<Value, serde_json::Error> {
        to_value(IBigSerializer(n))
    }

    #[test]
    fn test_safe_positive_number() {
        let n = IBig::from(25478_u64);
        let json = serialize(n);

        assert_eq!(json.unwrap(), json!(25478));
    }

    #[test]
    fn test_unsafe_positive_number() {
        let n = "2547817133237982226615393".parse().unwrap();
        let json = serialize(n);

        assert_eq!(json.unwrap(), json!("2547817133237982226615393"));
    }

    #[test]
    fn test_safe_negative_number() {
        let n = IBig::from(-9_i64);
        let json = serialize(n);

        assert_eq!(json.unwrap(), json!(-9));
    }

    #[test]
    fn test_unsafe_negative_number() {
        let n = "-53982029029059266071596725977".parse().unwrap();
        let json = serialize(n);

        assert_eq!(json.unwrap(), json!("-53982029029059266071596725977"));
    }
}
