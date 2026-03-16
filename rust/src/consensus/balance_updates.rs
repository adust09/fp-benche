use crate::consensus::types::{DepositData, Withdrawal};

/// Imperative balance updates: in-place mutation.
/// Processing order: deposits → withdrawals → slashings (fixed).
pub fn process_balance_updates(
    balances: &mut Vec<u64>,
    deposits: &[DepositData],
    withdrawals: &[Withdrawal],
    slash_indices: &[usize],
) {
    for d in deposits {
        balances[d.validator_index] += d.amount;
    }

    for w in withdrawals {
        balances[w.validator_index] = balances[w.validator_index].saturating_sub(w.amount);
    }

    for &idx in slash_indices {
        balances[idx] -= balances[idx] / 32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consensus::types::*;

    #[test]
    fn should_handle_empty_ops() {
        let mut balances = vec![100, 200, 300];
        process_balance_updates(&mut balances, &[], &[], &[]);
        assert_eq!(balances, vec![100, 200, 300]);
    }

    #[test]
    fn should_apply_deposits() {
        let mut balances = vec![100, 200];
        let deposits = vec![DepositData {
            validator_index: 0,
            amount: 50,
        }];
        process_balance_updates(&mut balances, &deposits, &[], &[]);
        assert_eq!(balances, vec![150, 200]);
    }

    #[test]
    fn should_apply_slashings() {
        let mut balances = vec![3200];
        process_balance_updates(&mut balances, &[], &[], &[0]);
        assert_eq!(balances, vec![3200 - 3200 / 32]);
    }

    #[test]
    fn conservation_law() {
        let seed = 42u64;
        let count = 1000;
        let validators = generate_validators(count, seed);
        let initial: Vec<u64> = validators.iter().map(|v| v.effective_balance).collect();

        let deposits = generate_deposits(50, count, seed + 100);
        let withdrawals = generate_withdrawals(30, &initial, &deposits, seed + 200);
        let slash_indices = generate_slash_indices(20, count, seed + 300);

        let sum_before: u128 = initial.iter().map(|&b| b as u128).sum();
        let deposit_sum: u128 = deposits.iter().map(|d| d.amount as u128).sum();

        let mut balances = initial;
        // Compute expected withdrawal and slash amounts before mutation
        let mut temp = balances.clone();
        for d in &deposits {
            temp[d.validator_index] += d.amount;
        }
        let mut actual_withdrawal: u128 = 0;
        for w in &withdrawals {
            let deducted = temp[w.validator_index].min(w.amount);
            actual_withdrawal += deducted as u128;
            temp[w.validator_index] = temp[w.validator_index].saturating_sub(w.amount);
        }
        let mut slash_amount: u128 = 0;
        for &idx in &slash_indices {
            slash_amount += (temp[idx] / 32) as u128;
            temp[idx] -= temp[idx] / 32;
        }

        process_balance_updates(&mut balances, &deposits, &withdrawals, &slash_indices);
        let sum_after: u128 = balances.iter().map(|&b| b as u128).sum();

        assert_eq!(
            sum_after,
            sum_before + deposit_sum - actual_withdrawal - slash_amount
        );
    }
}
