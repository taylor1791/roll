use super::ibig::{serialize_ibig, serialize_opt_ibig};
use ibig::IBig;
use roll::pmf;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct Pmf {
    #[serde(serialize_with = "serialize_pmf")]
    pmf: pmf::Pmf<IBig>,
    statistics: Statistics,
}

impl Pmf {
    pub fn new(pmf: pmf::Pmf<IBig>) -> Self {
        let min = pmf.iter().next().map(|outcome| outcome.value.clone());
        let mean = pmf.expected_value();
        let max = pmf.iter().last().map(|outcome| outcome.value.clone());

        Self {
            pmf,
            statistics: Statistics { min, mean, max },
        }
    }
}

#[derive(Serialize)]
struct Statistics {
    #[serde(serialize_with = "serialize_opt_ibig")]
    min: Option<IBig>,
    mean: f64,
    #[serde(serialize_with = "serialize_opt_ibig")]
    max: Option<IBig>,
}

#[derive(Serialize)]
struct Outcome<'a> {
    #[serde(serialize_with = "serialize_ibig")]
    value: &'a IBig,
    p: f64,
}

fn serialize_pmf<S>(pmf: &pmf::Pmf<IBig>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeSeq;

    let iter = pmf.iter();

    let mut seq = serializer.serialize_seq(iter.size_hint().1)?;
    for outcome in iter {
        seq.serialize_element(&Outcome {
            value: &outcome.value,
            p: outcome.p,
        })?;
    }
    seq.end()
}
