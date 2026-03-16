use crate::consensus::types::{isqrt, Validator, BASE_REWARD_FACTOR};
use rustica::pvec::PersistentVector;

/// Rustica epoch processing: PersistentVector map producing a new PersistentVector.
pub fn process_epoch_rustica(
    validators: &[Validator],
    balances: PersistentVector<u64>,
) -> PersistentVector<u64> {
    let total_active_balance: u64 = validators
        .iter()
        .filter(|v| v.active && !v.slashed)
        .map(|v| v.effective_balance)
        .sum();

    if total_active_balance == 0 {
        return balances;
    }

    let sqrt_total = isqrt(total_active_balance);

    let mut result = balances;
    for (i, v) in validators.iter().enumerate() {
        let base_reward = v.effective_balance * BASE_REWARD_FACTOR / sqrt_total;
        let bal = *result.get(i).unwrap();
        let new_bal = if v.index % 7 != 0 {
            bal + base_reward
        } else {
            bal.saturating_sub(base_reward)
        };
        result = result.update(i, new_bal);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consensus::epoch_processing;
    use crate::consensus::types::generate_validators;

    #[test]
    fn should_match_imperative() {
        for seed in [42, 123, 7777] {
            let validators = generate_validators(200, seed);
            let initial: Vec<u64> = validators.iter().map(|v| v.effective_balance).collect();

            let mut imp_balances = initial.clone();
            epoch_processing::process_epoch(&validators, &mut imp_balances);

            let pv = PersistentVector::from_slice(&initial);
            let rustica_result = process_epoch_rustica(&validators, pv);

            let rustica_vec: Vec<u64> = (0..rustica_result.len())
                .map(|i| *rustica_result.get(i).unwrap())
                .collect();

            assert_eq!(imp_balances, rustica_vec, "mismatch at seed={seed}");
        }
    }
}
