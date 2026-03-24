use crate::consensus::types::{isqrt, Validator, BASE_REWARD_FACTOR};

/// Imperative epoch processing: for-loop with in-place balance mutation.
pub fn process_epoch(validators: &[Validator], balances: &mut Vec<u64>) {
    let total_active_balance: u64 = validators
        .iter()
        .filter(|v| v.active && !v.slashed)
        .map(|v| v.effective_balance)
        .sum();

    if total_active_balance == 0 {
        return;
    }

    let sqrt_total = isqrt(total_active_balance);

    for (i, v) in validators.iter().enumerate() {
        let base_reward = v.effective_balance * BASE_REWARD_FACTOR / sqrt_total;
        let participating = v.index % 7 != 0;
        if participating {
            balances[i] += base_reward;
        } else {
            balances[i] = balances[i].saturating_sub(base_reward);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consensus::types::generate_validators;

    #[test]
    fn should_handle_empty() {
        let validators = vec![];
        let mut balances = vec![];
        process_epoch(&validators, &mut balances);
        assert!(balances.is_empty());
    }

    #[test]
    fn should_process_small_set() {
        let validators = generate_validators(100, 42);
        let mut balances: Vec<u64> = validators.iter().map(|v| v.effective_balance).collect();
        let original_sum: u64 = balances.iter().sum();
        process_epoch(&validators, &mut balances);
        // Balances should have changed
        let new_sum: u64 = balances.iter().sum();
        assert_ne!(original_sum, new_sum);
    }

    #[test]
    fn golden_value_test() {
        let validators = generate_validators(10, 42);
        let mut balances: Vec<u64> = validators.iter().map(|v| v.effective_balance).collect();
        process_epoch(&validators, &mut balances);
        // Snapshot the result for cross-variant comparison
        let expected = balances.clone();

        // Re-run should produce same result from same input
        let mut balances2: Vec<u64> = validators.iter().map(|v| v.effective_balance).collect();
        process_epoch(&validators, &mut balances2);
        assert_eq!(expected, balances2);
    }
}
