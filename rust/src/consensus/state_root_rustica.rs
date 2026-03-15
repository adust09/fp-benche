use rustica::pvec::PersistentVector;
use sha2::{Digest, Sha256};

fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn sha256_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

/// Rustica state root: index-walk pairwise rebuild using PersistentVector.
pub fn compute_state_root_rustica(balances: &[u64]) -> [u8; 32] {
    if balances.is_empty() {
        return [0u8; 32];
    }

    let leaves: Vec<[u8; 32]> = balances
        .iter()
        .map(|b| sha256_bytes(&b.to_le_bytes()))
        .collect();

    let target = leaves.len().next_power_of_two();
    let mut padded = leaves;
    padded.resize(target, [0u8; 32]);

    let mut pv = PersistentVector::from_slice(&padded);

    let mut width = pv.len();
    while width > 1 {
        let half = width / 2;
        let new_nodes: Vec<[u8; 32]> = (0..half)
            .map(|i| {
                let left = pv.get(2 * i).unwrap();
                let right = pv.get(2 * i + 1).unwrap();
                sha256_pair(left, right)
            })
            .collect();
        pv = PersistentVector::from_slice(&new_nodes);
        width = half;
    }

    *pv.get(0).unwrap()
}

/// Hash-free XOR variant for structure-only benchmarking.
pub fn compute_state_root_xor_rustica(balances: &[u64]) -> [u8; 32] {
    if balances.is_empty() {
        return [0u8; 32];
    }

    let leaves: Vec<[u8; 32]> = balances
        .iter()
        .map(|b| {
            let mut leaf = [0u8; 32];
            leaf[..8].copy_from_slice(&b.to_le_bytes());
            leaf
        })
        .collect();

    let target = leaves.len().next_power_of_two();
    let mut padded = leaves;
    padded.resize(target, [0u8; 32]);

    let mut pv = PersistentVector::from_slice(&padded);

    let mut width = pv.len();
    while width > 1 {
        let half = width / 2;
        let new_nodes: Vec<[u8; 32]> = (0..half)
            .map(|i| {
                let left = pv.get(2 * i).unwrap();
                let right = pv.get(2 * i + 1).unwrap();
                let mut combined = [0u8; 32];
                for j in 0..32 {
                    combined[j] = left[j] ^ right[j];
                }
                combined
            })
            .collect();
        pv = PersistentVector::from_slice(&new_nodes);
        width = half;
    }

    *pv.get(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consensus::state_root;

    #[test]
    fn should_match_imperative() {
        for size in [1, 2, 4, 7, 16, 100, 1024] {
            let balances: Vec<u64> = (1..=size as u64).collect();
            let imp = state_root::compute_state_root(&balances);
            let rustica = compute_state_root_rustica(&balances);
            assert_eq!(imp, rustica, "SHA mismatch at size={size}");

            let imp_xor = state_root::compute_state_root_xor(&balances);
            let rustica_xor = compute_state_root_xor_rustica(&balances);
            assert_eq!(imp_xor, rustica_xor, "XOR mismatch at size={size}");
        }
    }
}
