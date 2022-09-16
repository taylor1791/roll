use ibig::UBig;
use num_traits::{One, Zero};
use std::collections::HashMap;

#[derive(Default)]
pub struct Combinations {
    pascal: HashMap<usize, UBig>,
}

impl Combinations {
    pub fn probability_dice_roll_sum(&mut self, sum: usize, n_dice: usize, sides: usize) -> f64 {
        // References:
        //  * https://math.stackexchange.com/questions/2304799/probabilies-of-rolling-n-dice-to-add-up-to-a-specific-sum
        //  * https://github.com/carlosvega/DiceProbabilities
        //  * https://www.omnicalculator.com/statistics/dice
        //  * https://marvelvietnam.com/top2/bai-viet/dice-from-wolfram-mathworld/2478703015
        self.dice_roll_sum_count(sum, n_dice as usize, sides)
            .to_f64()
            / (UBig::from(sides)).pow(n_dice).to_f64()
    }

    pub fn dice_roll_sum_count(&mut self, sum: usize, n_dice: usize, sides: usize) -> UBig {
        if n_dice == 0 || sides == 0 {
            return UBig::one();
        }

        let mut count = UBig::zero();
        let bound = (sum - n_dice) / sides;

        // The summation is split between addition and subtraction to avoid overflow below zero.
        for i in (0..=bound).step_by(2) {
            count += self.choose(n_dice, i) * self.choose(sum - 1 - i * sides, n_dice - 1);
        }

        for i in (1..=bound).step_by(2) {
            count -= self.choose(n_dice, i) * self.choose(sum - 1 - i * sides, n_dice - 1);
        }

        count
    }

    pub fn choose(&mut self, n: usize, r: usize) -> UBig {
        if r > n {
            return Zero::zero();
        }

        // The number of combinations is symmetric, but combinations are easier to compute it from
        // the "smaller" side.
        if r > (n / 2) {
            self._choose(n, n - r)
        } else {
            self._choose(n, r)
        }
    }

    fn _choose(&mut self, n: usize, r: usize) -> UBig {
        if r == 0 {
            return One::one();
        }

        // The cantor pairing function. Provides a performance improvement over using
        // `(usize, usize)` as an index.
        let pair = (n + r) * (n + r + 1) / 2 + r;

        if let Some(value) = self.pascal.get(&pair) {
            return value.clone();
        }

        let result = n * self._choose(n - 1, r - 1) / r;

        // Store every third row of pascals triangle. These parameters were hand selected based on
        // the benchmarks; heap access are not free.
        if n % 3 == 2 {
            self.pascal.insert(pair, result.clone());
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_choose_definition(n: u8, r: u8) -> TestResult {
        let n = usize::from(n);
        let r = usize::from(r);

        if r > n {
            return TestResult::discard();
        }

        TestResult::from_bool(choose(n, r) == factorial(n) / factorial(r) / factorial(n - r))
    }

    #[quickcheck]
    fn test_choose_symmetry_identity(n: u8, r: u8) -> TestResult {
        let n = usize::from(n);
        let r = usize::from(r);

        if r > n {
            return TestResult::discard();
        }

        TestResult::from_bool(choose(n, r) == choose(n, n - r))
    }

    #[quickcheck]
    fn test_choose_absorption(n: u8, r: u8) -> TestResult {
        let n = usize::from(n);
        let r = usize::from(r);

        if r > n || n == 0 || r == 0 {
            return TestResult::discard();
        }

        TestResult::from_bool(choose(n, r) == n * choose(n - 1, r - 1) / r)
    }

    #[quickcheck]
    fn test_choose_addition(n: u8, r: u8) -> TestResult {
        let n = usize::from(n);
        let r = usize::from(r);

        if r > n || n == 0 || r == 0 {
            return TestResult::discard();
        }

        TestResult::from_bool(choose(n, r) == choose(n - 1, r) + choose(n - 1, r - 1))
    }

    #[quickcheck]
    fn test_choose_upper_index_summation(n: u8, r: u8) -> TestResult {
        let n = usize::from(n);
        let r = usize::from(r);

        if r > n {
            return TestResult::discard();
        }

        let mut sum = UBig::zero();
        for i in 0..=n {
            sum += choose(i, r);
        }

        TestResult::from_bool(sum == choose(n + 1, r + 1))
    }

    #[quickcheck]
    fn test_choose_sum_nth_pascal_row(n: u8) -> TestResult {
        let n = usize::from(n);

        let mut sum = UBig::zero();
        for i in 0..=n {
            sum += choose(n, i);
        }

        TestResult::from_bool(UBig::from(2_u8).pow(n) == sum)
    }

    #[quickcheck]
    fn test_count_sum_symmetry(sum: u8, n_dice: u8, sides: u8) -> TestResult {
        let sum = usize::from(sum);
        let n_dice = usize::from(n_dice);
        let sides = usize::from(sides);

        let max = n_dice * sides;
        if sum < n_dice || sum > max {
            return TestResult::discard();
        }

        let mut comb = Combinations::default();
        TestResult::from_bool(
            comb.dice_roll_sum_count(sum, n_dice, sides)
                == comb.dice_roll_sum_count(max - sum + n_dice, n_dice, sides),
        )
    }

    #[quickcheck]
    // This test is similar to the sum_total_rolls benchmark. The benchmark ensures this test is
    // reasonably fast.
    fn test_count_sum_sum(n_dice: u8, sides: u8) -> TestResult {
        let min = Into::<usize>::into(n_dice);
        let max = Into::<usize>::into(n_dice) * Into::<usize>::into(sides);

        // Discard large results that are "too slow."
        if max > 64 * 64 {
            return TestResult::discard();
        }

        let mut total_rolls = UBig::zero();
        let mut comb = Combinations::default();
        for i in min..=max {
            total_rolls += comb.dice_roll_sum_count(i, Into::<usize>::into(n_dice), sides.into());
        }

        TestResult::from_bool(total_rolls == UBig::from(sides).pow(Into::<usize>::into(n_dice)))
    }

    #[test]
    fn test_dee_twenty() {
        let mut comb = Combinations::default();

        for i in 1..=20 {
            assert_eq!(comb.probability_dice_roll_sum(i, 1, 20), 1.0 / 20.0);
        }
    }

    #[test]
    fn test_two_dee_ten() {
        let mut comb = Combinations::default();

        for i in 2_u8..=20 {
            let count = if i > 11 { 21 - i } else { i - 1 };

            assert_eq!(
                comb.probability_dice_roll_sum(Into::<usize>::into(i), 2, 10),
                Into::<f64>::into(count) / 100.0
            );
        }
    }

    #[test]
    fn test_three_dee_six() {
        let mut comb = Combinations::default();

        assert_eq!(comb.probability_dice_roll_sum(3, 3, 6), 1.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(4, 3, 6), 3.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(5, 3, 6), 6.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(6, 3, 6), 10.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(7, 3, 6), 15.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(8, 3, 6), 21.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(9, 3, 6), 25.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(10, 3, 6), 27.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(11, 3, 6), 27.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(12, 3, 6), 25.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(13, 3, 6), 21.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(14, 3, 6), 15.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(15, 3, 6), 10.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(16, 3, 6), 6.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(17, 3, 6), 3.0 / 216.0);
        assert_eq!(comb.probability_dice_roll_sum(18, 3, 6), 1.0 / 216.0);
    }

    fn choose(n: usize, r: usize) -> UBig {
        let mut comb = Combinations::default();
        comb.choose(n, r)
    }

    fn factorial(n: usize) -> UBig {
        let product = One::one();
        if n == 0 {
            return product;
        }

        n * factorial(n - 1)
    }
}
