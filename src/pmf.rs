use crate::iter_func::IterFunc;
use ibig::IBig;
use std::cmp::Ord;

#[derive(Clone, Debug)]
pub struct Pmf<A> {
    values: Vec<Outcome<A>>,
}

#[derive(Clone, Debug)]
pub struct Outcome<A> {
    pub p: f64,
    pub value: A,
}

#[derive(Clone, Debug)]
pub struct PmfIterator<'a, A> {
    slice_iter: std::slice::Iter<'a, Outcome<A>>,
}

impl<A> Pmf<A>
where
    A: Ord,
{
    pub fn constant(value: A) -> Self {
        Self {
            values: vec![Outcome { value, p: 1.0 }],
        }
    }

    pub fn from_mass_function<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (f64, A)>,
    {
        let values = iter
            .into_iter()
            .map(|(p, value)| Outcome { p, value })
            .sorted_by(|left, right| left.value.cmp(&right.value))
            .group_by(
                |outcome| &outcome.value,
                |outcome, values| Outcome {
                    p: values.iter().fold(outcome.p, |left, right| left + right.p),
                    value: outcome.value,
                },
            )
            .collect::<Vec<Outcome<A>>>();

        let mut pmf = Self { values };
        pmf.normalize();

        pmf
    }

    pub fn iter(&self) -> PmfIterator<A> {
        PmfIterator {
            slice_iter: self.values.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn map<F, B>(&mut self, f: F) -> Pmf<B>
    where
        F: Fn(&A) -> B,
        B: Ord,
    {
        let new_values = self
            .values
            .iter()
            .map(|Outcome { value, p }| Outcome {
                value: f(value),
                p: *p,
            })
            .sorted_by(|left, right| left.value.cmp(&right.value))
            .collect();

        Pmf { values: new_values }
    }

    pub fn cartesian_product<F, B, C>(&self, right: &Pmf<B>, f: F) -> Pmf<C>
    where
        F: Fn(&A, &B) -> C,
        B: Clone + Ord,
        Pmf<C>: std::iter::FromIterator<(f64, C)>,
    {
        self.iter()
            .cartesian_product_by(right.iter(), |l, r| (l.p * r.p, f(&l.value, &r.value)))
            .collect()
    }

    fn normalize(&mut self) {
        let normalizing_constant: f64 = self.values.iter().map(|Outcome { p, .. }| p).sum();

        if normalizing_constant == 0.0 && !self.values.is_empty() {
            self.values = vec![];
        } else if normalizing_constant != 1.0 {
            self.values
                .iter_mut()
                .map(|outcome| outcome.p /= normalizing_constant)
                .for_each(drop)
        }
    }
}

impl Pmf<IBig> {
    pub fn expected_value(&self) -> f64 {
        self.values.iter().fold(0.0, |mean, outcome| {
            mean + outcome.value.to_f64() * outcome.p
        })
    }
}

impl<'a, A> Iterator for PmfIterator<'a, A> {
    type Item = &'a Outcome<A>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice_iter.next()
    }
}

impl<A> FromIterator<(f64, A)> for Pmf<A>
where
    A: Ord,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (f64, A)>,
    {
        Pmf::from_mass_function(iter)
    }
}

#[cfg(test)]
impl<A> quickcheck::Arbitrary for Pmf<A>
where
    A: quickcheck::Arbitrary + Ord,
{
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Pmf::from_mass_function(
            Vec::<A>::arbitrary(g)
                .into_iter()
                .map(|value| (Into::<f64>::into(u8::arbitrary(g)) / 255.0_f64, value)),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ibig::ibig;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn mapping_over_constant(n: i8) -> bool {
        let n = Into::<i16>::into(n);

        let pmf = Pmf::constant(n).map(|n| -n);
        let outcomes = pmf.iter().collect::<Vec<_>>();

        outcomes.len() == 1 && outcomes[0].value == -n
    }

    #[quickcheck]
    fn are_normalized(a: Pmf<u8>) -> TestResult {
        if a.is_empty() {
            return TestResult::discard();
        }

        let p = a.iter().map(|outcome| outcome.p).sum::<f64>();

        TestResult::from_bool(float_eq::float_eq!(p, 1.0, abs <= 0.000001))
    }

    #[quickcheck]
    fn cartesian_product_preserves_normalization(a: Pmf<u8>, b: Pmf<u8>) -> TestResult {
        if a.is_empty() || b.is_empty() {
            return TestResult::discard();
        }

        let pmf = a.cartesian_product(&b, |a, b| Into::<u16>::into(*a) + Into::<u16>::into(*b));
        let p = pmf.iter().map(|outcome| outcome.p).sum::<f64>();

        TestResult::from_bool(float_eq::float_eq!(p, 1.0, abs <= 0.000001))
    }

    #[test]
    fn expected_value_roulette() {
        let pmf = [(1.0 / 38.0, ibig!(36)), (37.0 / 38.0, ibig!(-1))]
            .into_iter()
            .collect::<Pmf<_>>();

        float_eq::assert_float_eq!(pmf.expected_value(), -1.0 / 38.0, abs <= 0.000001);
    }
}
