#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Replays a ledger of integer moves to its closing balance, or None at the move a balance would break.
///
/// ```
/// assert_eq!(mrlysolo::replay(&[5, -2, -3]), Some(0));
/// assert_eq!(mrlysolo::replay(&[5, -6]), None);
/// ```
pub fn replay(moves: &[i64]) -> Option<i64> {
    let mut balance: i64 = 0;
    for &delta in moves {
        balance = balance.checked_add(delta)?;
        if balance < 0 {
            return None;
        }
    }
    Some(balance)
}

#[cfg(test)]
mod tests {
    #[test]
    fn refuses_the_breaking_move() {
        assert_eq!(super::replay(&[]), Some(0));
        assert_eq!(super::replay(&[3, -1, 4]), Some(6));
        assert_eq!(super::replay(&[-1]), None);
        assert_eq!(super::replay(&[i64::MAX, 1]), None);
    }
}
