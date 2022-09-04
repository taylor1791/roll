use ibig::IBig;

pub struct JsonSafeIBigSerializer<'a>(pub &'a IBig);

const MIN_SAFE_INTEGER: i64 = -9007199254740991;
const MAX_SAFE_INTEGER: i64 = 9007199254740991;

impl<'a> serde::Serialize for JsonSafeIBigSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0 < &IBig::from(MIN_SAFE_INTEGER) || self.0 > &IBig::from(MAX_SAFE_INTEGER) {
            serializer.serialize_str(&self.0.to_string())
        } else {
            // Unwrap is safe because MIN_SAFE_INTEGER .. MAX_SAFE_INTEGER always fits in an i64.
            serializer.serialize_i64(self.0.to_string().parse().unwrap())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ibig::IBig;
    use serde_json::{json, to_value};

    #[test]
    fn test_safe_positive_number() {
        let n = IBig::from(25478_u64);
        let json = to_value(&JsonSafeIBigSerializer(&n));

        assert_eq!(json.unwrap(), json!(25478));
    }

    #[test]
    fn test_unsafe_positive_number() {
        let n = "2547817133237982226615393".parse().unwrap();
        let json = to_value(&JsonSafeIBigSerializer(&n));

        assert_eq!(json.unwrap(), json!("2547817133237982226615393"));
    }

    #[test]
    fn test_safe_negative_number() {
        let n = IBig::from(-9_i64);
        let json = to_value(&JsonSafeIBigSerializer(&n));

        assert_eq!(json.unwrap(), json!(-9));
    }

    #[test]
    fn test_unsafe_negative_number() {
        let n = "-53982029029059266071596725977".parse().unwrap();

        let json = to_value(&JsonSafeIBigSerializer(&n));

        assert_eq!(json.unwrap(), json!("-53982029029059266071596725977"));
    }
}
