use std::cmp::Ordering;

pub trait IterFunc: Iterator {
    fn cartesian_product_by<'a, 'b, J, F, A, B, C>(
        mut self,
        right: J,
        f: F,
    ) -> CartesianProductBy<'a, Self, J, F, A>
    where
        Self: Iterator<Item = &'a A> + Sized,
        B: 'b,
        J: Clone + Iterator<Item = &'b B>,
        F: Fn(&'a A, &'b B) -> C,
    {
        let current_left = self.next();
        let original_right = right;
        let right = original_right.clone();

        CartesianProductBy {
            left: self,
            current_left,
            right,
            original_right,
            f,
        }
    }

    fn group_by<A, F, K, G, B>(mut self, key: F, aggregator: G) -> GroupBy<Self, A, F, G>
    where
        Self: Iterator<Item = A> + Sized,
        F: Fn(&A) -> &K,
        G: Fn(A, Vec<A>) -> B,
        K: PartialEq,
    {
        let next = self.next();

        GroupBy {
            iter: self,
            next,
            key,
            aggregator,
        }
    }

    fn sorted_by<F>(self, f: F) -> std::vec::IntoIter<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut v = Vec::from_iter(self);

        v.sort_by(f);
        v.into_iter()
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct CartesianProductBy<'a, I, J, F, A> {
    left: I,
    current_left: Option<&'a A>,
    right: J,
    original_right: J,
    f: F,
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct GroupBy<I, A, F, G> {
    iter: I,
    next: Option<A>,
    key: F,
    aggregator: G,
}

impl<T> IterFunc for T where T: Iterator {}

impl<'a, 'b, I, J, F, A, B, C> Iterator for CartesianProductBy<'a, I, J, F, A>
where
    B: 'b,
    I: Iterator<Item = &'a A>,
    J: Clone + Iterator<Item = &'b B>,
    F: Fn(&'a A, &'b B) -> C,
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        let right = match self.right.next() {
            Some(right) => right,
            None => {
                self.current_left = self.left.next();
                self.right = self.original_right.clone();

                match self.right.next() {
                    None => return None,
                    Some(right) => right,
                }
            }
        };

        let left = match &self.current_left {
            Some(left) => left,
            None => return None,
        };

        Some((self.f)(left, right))
    }
}

impl<I, A, F, K, G, B> Iterator for GroupBy<I, A, F, G>
where
    I: Iterator<Item = A> + Sized,
    F: Fn(&A) -> &K,
    G: Fn(A, Vec<A>) -> B,
    K: PartialEq,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = std::mem::take(&mut self.next) {
            let mut values = vec![];
            let key = (self.key)(&value);

            loop {
                self.next = self.iter.next();

                match std::mem::take(&mut self.next) {
                    Some(value) if (self.key)(&value) == key => values.push(value),
                    Some(value) => {
                        self.next = Some(value);
                        break;
                    }
                    None => break,
                }
            }

            return Some((self.aggregator)(value, values));
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_cartesian_product_by_cardinality(a: Vec<u8>, b: Vec<u8>) -> bool {
        let ds = a.iter().cartesian_product_by(b.iter(), |_, _| ());

        ds.count() == a.len() * b.len()
    }

    #[quickcheck]
    fn test_sorted_by(a: Vec<i8>) -> bool {
        let mut sorted = a.clone();
        sorted.sort_unstable();

        let sorted_by = a.into_iter().sorted_by(|a, b| a.cmp(b)).collect::<Vec<_>>();

        equal(&sorted_by, &sorted)
    }

    #[quickcheck]
    fn test_group_by(mut data: Vec<u8>) -> bool {
        data.sort_unstable();

        let count = &data
            .iter()
            .group_by(|x| *x, |_, values| values.len() + 1)
            .sum::<usize>();

        *count == data.len()
    }

    fn equal<A>(a: &[A], b: &[A]) -> bool
    where
        A: PartialEq,
    {
        a.iter().zip(b.iter()).filter(|(a, b)| a == b).count() == a.len()
    }
}
