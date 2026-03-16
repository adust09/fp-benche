use crate::consensus::types::{isqrt, Validator, BASE_REWARD_FACTOR};

/// Functional epoch processing: iterator chain producing a new Vec.
pub fn process_epoch_fp(validators: &[Validator], balances: &[u64]) -> Vec<u64> {
    let total_active_balance: u64 = validators
        .iter()
        .filter(|v| v.active && !v.slashed)
        .map(|v| v.effective_balance)
        .sum();

    if total_active_balance == 0 {
        return balances.to_vec();
    }

    let sqrt_total = isqrt(total_active_balance);

    validators
        .iter()
        .zip(balances.iter())
        .map(|(v, &bal)| {
            let base_reward = v.effective_balance * BASE_REWARD_FACTOR / sqrt_total;
            if v.index % 7 != 0 {
                bal + base_reward
            } else {
                bal.saturating_sub(base_reward)
            }
        })
        .collect()
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

            let fp_balances = process_epoch_fp(&validators, &initial);

            assert_eq!(imp_balances, fp_balances, "mismatch at seed={seed}");
        }
    }
}
