use super::*;
use crate::pmf::Pmf;
use ibig::{ubig, UBig};
use quickcheck_macros::quickcheck;
use std::collections::HashSet;
use std::str::FromStr;

#[quickcheck]
fn constant(seed: u64) -> bool {
    let expression = Expression::from_str("3").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 3, 3) && expression.to_string() == "3"
}

#[quickcheck]
fn plus(seed: u64) -> bool {
    let expression = Expression::from_str("+2").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 2, 2) && expression.to_string() == "+2"
}

#[quickcheck]
fn plus_plus(seed: u64) -> bool {
    let expression = Expression::from_str("++9").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 9, 9) && expression.to_string() == "++9"
}

#[quickcheck]
fn minus(seed: u64) -> bool {
    let expression = Expression::from_str("-1").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, -1, -1) && expression.to_string() == "-1"
}

#[quickcheck]
fn minus_minus(seed: u64) -> bool {
    let expression = Expression::from_str("--127").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 127, 127) && expression.to_string() == "--127"
}

#[quickcheck]
fn exponent(seed: u64) -> bool {
    let expression = Expression::from_str("-1 ** 3 ** 2").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, -1, -1) && expression.to_string() == "-1 ** 3 ** 2"
}

#[quickcheck]
fn exponent_precedence_minus(seed: u64) -> bool {
    let expression = Expression::from_str("-2**3").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, -8, -8) && expression.to_string() == "-2 ** 3"
}

#[quickcheck]
fn exponent_precedence_dice(seed: u64) -> bool {
    let expression = Expression::from_str("1d4 ** 2").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 1, 16)
        && all_in_range(&rolls, HashSet::from([ubig!(4)]), (1, 1))
        && expression.to_string() == "1d4 ** 2"
}

#[quickcheck]
fn dice(seed: u64) -> bool {
    let expression = Expression::from_str("d4").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 1, 4)
        && all_in_range(&rolls, HashSet::from([ubig!(4)]), (1, 1))
        && expression.to_string() == "1d4"
}

#[quickcheck]
fn n_dice(seed: u64) -> bool {
    let expression = Expression::from_str("3d6").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 3, 18)
        && all_in_range(&rolls, HashSet::from([ubig!(6)]), (3, 3))
        && expression.to_string() == "3d6"
}

#[quickcheck]
fn n_dice_dice(seed: u64) -> bool {
    let expression = Expression::from_str("1d4d6").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 1, 24)
        && all_in_range(&rolls, HashSet::from([ubig!(4), ubig!(6)]), (2, 5))
        && expression.to_string() == "1d4d6"
}

#[quickcheck]
fn dice_precedence(seed: u64) -> bool {
    let expression = Expression::from_str("-3d6").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, -18, -3)
        && all_in_range(&rolls, HashSet::from([ubig!(6)]), (3, 3))
        && expression.to_string() == "-3d6"
}

#[quickcheck]
fn difference_literals(seed: u64) -> bool {
    let expression = Expression::from_str("0-1 - 2").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, -3, -3) && expression.to_string() == "0 - 1 - 2"
}

#[quickcheck]
fn sum_literals(seed: u64) -> bool {
    let expression = Expression::from_str("2+3 + 4").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 9, 9) && expression.to_string() == "2 + 3 + 4"
}

#[quickcheck]
fn plus_mod(seed: u64) -> bool {
    let expression = Expression::from_str("2d8+1").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 3, 17)
        && all_in_range(&rolls, HashSet::from([ubig!(8)]), (2, 2))
        && expression.to_string() == "2d8 + 1"
}

#[quickcheck]
fn minus_mod(seed: u64) -> bool {
    let expression = Expression::from_str("1d2 - 1").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 0, 1)
        && all_in_range(&rolls, HashSet::from([ubig!(2)]), (1, 1))
        && expression.to_string() == "1d2 - 1"
}

#[quickcheck]
fn sum_dice(seed: u64) -> bool {
    let expression = Expression::from_str("1d10+1d12").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 2, 22)
        && all_in_range(&rolls, HashSet::from([ubig!(10), ubig!(12)]), (2, 2))
        && expression.to_string() == "1d10 + 1d12"
}

#[quickcheck]
fn left_associative_left_grouping(seed: u64) -> bool {
    let expression = Expression::from_str("(2 - 2) - (1 - 1)").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 0, 0) && expression.to_string() == "2 - 2 - (1 - 1)"
}

#[quickcheck]
fn left_associative_right_grouping(seed: u64) -> bool {
    let expression = Expression::from_str("1d(20 + 10)").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 1, 30)
        && all_in_range(&rolls, HashSet::from([ubig!(30)]), (1, 1))
        && expression.to_string() == "1d(20 + 10)"
}

#[quickcheck]
fn right_associative_left_grouping(seed: u64) -> bool {
    let expression = Expression::from_str("(-1 ** 3) ** 2").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 1, 1) && expression.to_string() == "(-1 ** 3) ** 2"
}

#[quickcheck]
fn right_associative_right_grouping(seed: u64) -> bool {
    let expression = Expression::from_str("-1 ** (1 + 1)").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 1, 1) && expression.to_string() == "-1 ** (1 + 1)"
}

#[quickcheck]
fn trailing_space(seed: u64) -> bool {
    let expression = Expression::from_str("1d20      ").unwrap();
    let pmf = pmf(&expression).unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(&pmf, value, 1, 20)
        && all_in_range(&rolls, HashSet::from([UBig::from(20_u8)]), (1, 1))
        && expression.to_string() == "1d20"
}

#[test]
fn unexpected_end_empty_string() {
    let expression = Expression::from_str("");

    assert!(expression
        .unwrap_err()
        .to_string()
        .contains("Unexpected end of input."));
}

#[test]
fn unexpected_end_expression() {
    let expression = Expression::from_str("10d");

    assert!(expression
        .unwrap_err()
        .to_string()
        .contains("Unexpected end of input."));
}

#[test]
fn unexpected_token_start() {
    let expression = Expression::from_str("cs");

    assert!(expression
        .unwrap_err()
        .to_string()
        .contains("Unexpected token at position 1."),);
}

#[test]
fn unexpected_token_in_subexpression() {
    let expression = Expression::from_str("1d(1+)");

    assert!(expression
        .unwrap_err()
        .to_string()
        .contains("Unexpected token at position 6."),);
}

#[test]
fn unexpected_token_erroneous_character() {
    let expression = Expression::from_str("3d6    e");

    assert!(expression
        .unwrap_err()
        .to_string()
        .contains("Unexpected token at position 8."),);
}

fn all_in_range(
    rolls: &HashMap<UBig, Vec<UBig>>,
    dice: HashSet<UBig>,
    n_rolls: (usize, usize),
) -> bool {
    let min_rolls = n_rolls.0;
    let max_rolls = n_rolls.1;

    let n_rolls = rolls.iter().map(|(_, rolls)| rolls.len()).sum::<usize>();

    rolls.iter().all(|(sides, rolls)| {
        dice.contains(sides) && rolls.iter().all(|roll| *roll >= ubig!(1) && roll <= sides)
    }) && n_rolls >= min_rolls
        && n_rolls <= max_rolls
}

fn in_range(pmf: &Pmf<IBig>, n: IBig, min: i64, max: i64) -> bool {
    let min = IBig::from(min);
    let max = IBig::from(max);

    let pmf_values_in_range = pmf.iter().fold(true, |in_range, outcome| {
        in_range && outcome.value >= min && n <= max
    });

    pmf_values_in_range && n >= min && n <= max
}

fn pmf(expression: &Expression) -> Result<Pmf<IBig>, anyhow::Error> {
    let mut combinations = Combinations::default();

    super::pmf::pmf(expression, &mut combinations)
}
