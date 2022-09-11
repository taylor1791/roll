use super::ibig::{serialize_ibig, IBigSerializer};
use ibig::{IBig, UBig};
use roll::expression;
use serde::{Serialize, Serializer};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Evaluand(#[serde(with = "EvaluandSerializer")] pub expression::Evaluand);

impl Evaluand {
    pub fn new(expression: expression::Evaluand) -> Self {
        Self(expression)
    }
}

#[derive(Serialize)]
#[serde(remote = "expression::Evaluand")]
struct EvaluandSerializer {
    #[serde(serialize_with = "serialize_rolls")]
    rolls: HashMap<UBig, Vec<UBig>>,
    #[serde(serialize_with = "serialize_ibig")]
    value: IBig,
}

fn serialize_rolls<S>(rolls: &HashMap<UBig, Vec<UBig>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeMap;

    let mut map = serializer.serialize_map(Some(rolls.len()))?;
    for (sides, rolls) in rolls {
        map.serialize_entry(&format!("d{}", sides), &RollsSerializer(rolls))?;
    }
    map.end()
}

struct RollsSerializer<'a>(&'a Vec<UBig>);

impl<'a> serde::Serialize for RollsSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for roll in self.0 {
            seq.serialize_element(&IBigSerializer::new(&roll.into()))?;
        }
        seq.end()
    }
}
