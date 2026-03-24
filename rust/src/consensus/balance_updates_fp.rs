use crate::consensus::types::{DepositData, Withdrawal};

/// Functional balance updates: iterator pipelines producing new Vecs.
/// Processing order: deposits → withdrawals → slashings (fixed).
pub fn process_balance_updates_fp(
    balances: &[u64],
    deposits: &[DepositData],
    withdrawals: &[Withdrawal],
    slash_indices: &[usize],
) -> Vec<u64> {
    // Phase 1: deposits
    let after_deposits = deposits.iter().fold(balances.to_vec(), |mut acc, d| {
        acc[d.validator_index] += d.amount;
        acc
    });

    // Phase 2: withdrawals
    let after_withdrawals = withdrawals.iter().fold(after_deposits, |mut acc, w| {
        acc[w.validator_index] = acc[w.validator_index].saturating_sub(w.amount);
        acc
    });

    // Phase 3: slashings
    slash_indices.iter().fold(after_withdrawals, |mut acc, &idx| {
        acc[idx] -= acc[idx] / 32;
        acc
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

            let fp = process_balance_updates_fp(&initial, &deposits, &withdrawals, &slash_indices);

            assert_eq!(imp, fp, "mismatch at seed={seed}");
        }
    }
}
