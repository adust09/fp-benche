use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub const MAX_EFFECTIVE_BALANCE: u64 = 32_000_000_000; // 32 ETH in Gwei
pub const BASE_REWARD_FACTOR: u64 = 64;

#[derive(Debug, Clone, PartialEq)]
pub struct Validator {
    pub index: u32,
    pub effective_balance: u64,
    pub slashed: bool,
    pub active: bool,
}

pub struct DepositData {
    pub validator_index: usize,
    pub amount: u64,
}

pub struct Withdrawal {
    pub validator_index: usize,
    pub amount: u64,
}

pub fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as u64;
    loop {
        let x1 = (x + n / x) / 2;
        if x1 >= x {
            return x;
        }
        x = x1;
    }
}

pub fn generate_validators(count: usize, seed: u64) -> Vec<Validator> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count)
        .map(|i| Validator {
            index: i as u32,
            effective_balance: rng.gen_range(16_000_000_000..=MAX_EFFECTIVE_BALANCE),
            slashed: rng.gen_ratio(1, 100),
            active: rng.gen_ratio(95, 100),
        })
        .collect()
}

pub fn generate_balances(count: usize, seed: u64) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count)
        .map(|_| rng.gen_range(16_000_000_000..=MAX_EFFECTIVE_BALANCE))
        .collect()
}

pub fn generate_deposits(count: usize, validator_count: usize, seed: u64) -> Vec<DepositData> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count)
        .map(|_| DepositData {
            validator_index: rng.gen_range(0..validator_count),
            amount: rng.gen_range(1_000_000..=1_000_000_000),
        })
        .collect()
}

pub fn generate_withdrawals(
    count: usize,
    balances: &[u64],
    deposits: &[DepositData],
    seed: u64,
) -> Vec<Withdrawal> {
    // Compute available balance per validator (initial + deposits)
    let mut available = balances.to_vec();
    for d in deposits {
        available[d.validator_index] += d.amount;
    }

    let mut rng = StdRng::seed_from_u64(seed);
    (0..count)
        .map(|_| {
            let idx = rng.gen_range(0..balances.len());
            let max_amount = available[idx].min(1_000_000_000);
            let amount = if max_amount > 0 {
                rng.gen_range(1..=max_amount)
            } else {
                0
            };
            available[idx] = available[idx].saturating_sub(amount);
            Withdrawal {
                validator_index: idx,
                amount,
            }
        })
        .collect()
}

pub fn generate_slash_indices(count: usize, validator_count: usize, seed: u64) -> Vec<usize> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices = Vec::with_capacity(count);
    let mut used = vec![false; validator_count];
    while indices.len() < count && indices.len() < validator_count {
        let idx = rng.gen_range(0..validator_count);
        if !used[idx] {
            used[idx] = true;
            indices.push(idx);
        }
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustica::pvec::PersistentVector;

    #[test]
    fn api_smoke_test() {
        let data = vec![1u64, 2, 3, 4, 5];
        let pv = PersistentVector::from_slice(&data);
        assert_eq!(pv.len(), 5);
        assert_eq!(*pv.get(0).unwrap(), 1);
        let pv2 = pv.update(2, 99);
        assert_eq!(*pv2.get(2).unwrap(), 99);
        assert_eq!(*pv.get(2).unwrap(), 3); // original unchanged
    }

    #[test]
    fn isqrt_known_values() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(100), 10);
        assert_eq!(isqrt(1_000_000), 1000);
    }
}
