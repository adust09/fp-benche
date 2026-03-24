use crate::consensus::types::{DepositData, Withdrawal};
use rustica::pvec::PersistentVector;

/// Rustica balance updates: PersistentVector update chain.
/// Processing order: deposits → withdrawals → slashings (fixed).
pub fn process_balance_updates_rustica(
    balances: PersistentVector<u64>,
    deposits: &[DepositData],
    withdrawals: &[Withdrawal],
    slash_indices: &[usize],
) -> PersistentVector<u64> {
    // Phase 1: deposits
    let after_deposits = deposits.iter().fold(balances, |acc, d| {
        let current = *acc.get(d.validator_index).unwrap();
        acc.update(d.validator_index, current + d.amount)
    });

    // Phase 2: withdrawals
    let after_withdrawals = withdrawals.iter().fold(after_deposits, |acc, w| {
        let current = *acc.get(w.validator_index).unwrap();
        acc.update(w.validator_index, current.saturating_sub(w.amount))
    });

    // Phase 3: slashings
    slash_indices.iter().fold(after_withdrawals, |acc, &idx| {
        let current = *acc.get(idx).unwrap();
        acc.update(idx, current - current / 32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consensus::balance_updates;
    use crate::consensus::types::*;

    #[test]
    fn should_match_imperative() {
        for seed in [42, 123, 7777] {
            let validators = generate_validators(500, seed);
            let initial: Vec<u64> = validators.iter().map(|v| v.effective_balance).collect();

            let deposits = generate_deposits(50, 500, seed + 100);
            let withdrawals = generate_withdrawals(30, &initial, &deposits, seed + 200);
            let slash_indices = generate_slash_indices(20, 500, seed + 300);

            let mut imp = initial.clone();
            balance_updates::process_balance_updates(
                &mut imp,
                &deposits,
                &withdrawals,
                &slash_indices,
            );

            let pv = PersistentVector::from_slice(&initial);
            let result =
                process_balance_updates_rustica(pv, &deposits, &withdrawals, &slash_indices);
            let result_vec: Vec<u64> = (0..result.len()).map(|i| *result.get(i).unwrap()).collect();

            assert_eq!(imp, result_vec, "mismatch at seed={seed}");
        }
    }
}
