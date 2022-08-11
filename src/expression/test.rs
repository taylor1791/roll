use super::*;
use quickcheck_macros::quickcheck;
use std::collections::HashSet;
use std::str::FromStr;

#[quickcheck]
fn constant(seed: u64) -> bool {
    let expression = Expression::from_str("3").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == 3 && expression.to_string() == "3"
}

#[quickcheck]
fn plus(seed: u64) -> bool {
    let expression = Expression::from_str("+2").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == 2 && expression.to_string() == "+2"
}

#[quickcheck]
fn plus_plus(seed: u64) -> bool {
    let expression = Expression::from_str("++9").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == 9 && expression.to_string() == "++9"
}

#[quickcheck]
fn minus(seed: u64) -> bool {
    let expression = Expression::from_str("-1").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == -1 && expression.to_string() == "-1"
}

#[quickcheck]
fn minus_minus(seed: u64) -> bool {
    let expression = Expression::from_str("--127").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == 127 && expression.to_string() == "--127"
}

#[quickcheck]
fn exponent(seed: u64) -> bool {
    let expression = Expression::from_str("-1 ** 3 ** 2").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == -1 && expression.to_string() == "-1 ** 3 ** 2"
}

#[quickcheck]
fn exponent_precedence_minus(seed: u64) -> bool {
    let expression = Expression::from_str("-2**3").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == -8 && expression.to_string() == "-2 ** 3"
}

#[quickcheck]
fn exponent_precedence_dice(seed: u64) -> bool {
    let expression = Expression::from_str("1d4 ** 2").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 1, 16)
        && all_in_range(&rolls, HashSet::from([4]), (1, 1))
        && expression.to_string() == "1d4 ** 2"
}

#[quickcheck]
fn dice(seed: u64) -> bool {
    let expression = Expression::from_str("d4").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 1, 4)
        && all_in_range(&rolls, HashSet::from([4]), (1, 1))
        && expression.to_string() == "1d4"
}

#[quickcheck]
fn n_dice(seed: u64) -> bool {
    let expression = Expression::from_str("3d6").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 3, 18)
        && all_in_range(&rolls, HashSet::from([6]), (3, 3))
        && expression.to_string() == "3d6"
}

#[quickcheck]
fn n_dice_dice(seed: u64) -> bool {
    let expression = Expression::from_str("1d4d6").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 1, 24)
        && all_in_range(&rolls, HashSet::from([4, 6]), (2, 5))
        && expression.to_string() == "1d4d6"
}

#[quickcheck]
fn dice_precedence(seed: u64) -> bool {
    let expression = Expression::from_str("-3d6").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, -18, -3)
        && all_in_range(&rolls, HashSet::from([6]), (3, 3))
        && expression.to_string() == "-3d6"
}

#[quickcheck]
fn difference_literals(seed: u64) -> bool {
    let expression = Expression::from_str("0-1 - 2").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == -3 && expression.to_string() == "0 - 1 - 2"
}

#[quickcheck]
fn sum_literals(seed: u64) -> bool {
    let expression = Expression::from_str("2+3 + 4").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == 9 && expression.to_string() == "2 + 3 + 4"
}

#[quickcheck]
fn plus_mod(seed: u64) -> bool {
    let expression = Expression::from_str("2d8+1").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 3, 17)
        && all_in_range(&rolls, HashSet::from([8]), (2, 2))
        && expression.to_string() == "2d8 + 1"
}

#[quickcheck]
fn minus_mod(seed: u64) -> bool {
    let expression = Expression::from_str("1d2 - 1").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 0, 1)
        && all_in_range(&rolls, HashSet::from([2]), (1, 1))
        && expression.to_string() == "1d2 - 1"
}

#[quickcheck]
fn sum_dice(seed: u64) -> bool {
    let expression = Expression::from_str("1d10+1d12").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 2, 22)
        && all_in_range(&rolls, HashSet::from([10, 12]), (2, 2))
        && expression.to_string() == "1d10 + 1d12"
}

#[quickcheck]
fn left_associative_left_grouping(seed: u64) -> bool {
    let expression = Expression::from_str("(2 - 2) - (1 - 1)").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == 0 && expression.to_string() == "2 - 2 - (1 - 1)"
}

#[quickcheck]
fn left_associative_right_grouping(seed: u64) -> bool {
    let expression = Expression::from_str("1d(20 + 10)").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 1, 30)
        && all_in_range(&rolls, HashSet::from([30]), (1, 1))
        && expression.to_string() == "1d(20 + 10)"
}

#[quickcheck]
fn right_associative_left_grouping(seed: u64) -> bool {
    let expression = Expression::from_str("(-1 ** 3) ** 2").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == 1 && expression.to_string() == "(-1 ** 3) ** 2"
}

#[quickcheck]
fn right_associative_right_grouping(seed: u64) -> bool {
    let expression = Expression::from_str("-1 ** (1 + 1)").unwrap();
    let Evaluand { value, .. } = expression.eval(seed).unwrap();

    value == 1 && expression.to_string() == "-1 ** (1 + 1)"
}

#[quickcheck]
fn trailing_space(seed: u64) -> bool {
    let expression = Expression::from_str("1d20      ").unwrap();
    let Evaluand { rolls, value } = expression.eval(seed).unwrap();

    in_range(value, 1, 20)
        && all_in_range(&rolls, HashSet::from([20]), (1, 1))
        && expression.to_string() == "1d20"
}

fn all_in_range(
    rolls: &HashMap<u64, Vec<u64>>,
    dice: HashSet<u64>,
    n_rolls: (usize, usize),
) -> bool {
    let min_rolls = n_rolls.0;
    let max_rolls = n_rolls.1;

    let n_rolls = rolls.iter().map(|(_, rolls)| rolls.len()).sum::<usize>();

    rolls.iter().all(|(sides, rolls)| {
        dice.contains(sides) && rolls.iter().all(|roll| *roll >= 1 && roll <= sides)
    }) && n_rolls >= min_rolls
        && n_rolls <= max_rolls
}

fn in_range(n: i64, min: i64, max: i64) -> bool {
    n >= min && n <= max
}
