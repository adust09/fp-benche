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

/// Functional state root: iterator-based non-recursive reduce.
pub fn compute_state_root_fp(balances: &[u64]) -> [u8; 32] {
    if balances.is_empty() {
        return [0u8; 32];
    }

    let mut nodes: Vec<[u8; 32]> = balances
        .iter()
        .map(|b| sha256_bytes(&b.to_le_bytes()))
        .collect();

    let target = nodes.len().next_power_of_two();
    nodes.resize(target, [0u8; 32]);

    while nodes.len() > 1 {
        nodes = nodes
            .chunks(2)
            .map(|pair| sha256_pair(&pair[0], &pair[1]))
            .collect();
    }

    nodes[0]
}

/// Hash-free XOR variant for structure-only benchmarking.
pub fn compute_state_root_xor_fp(balances: &[u64]) -> [u8; 32] {
    if balances.is_empty() {
        return [0u8; 32];
    }

    let mut nodes: Vec<[u8; 32]> = balances.iter().map(|b| {
        let mut leaf = [0u8; 32];
        leaf[..8].copy_from_slice(&b.to_le_bytes());
        leaf
    }).collect();

    let target = nodes.len().next_power_of_two();
    nodes.resize(target, [0u8; 32]);

    while nodes.len() > 1 {
        nodes = nodes
            .chunks(2)
            .map(|pair| {
                let mut combined = [0u8; 32];
                for j in 0..32 {
                    combined[j] = pair[0][j] ^ pair[1][j];
                }
                combined
            })
            .collect();
    }

    nodes[0]
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
            let fp = compute_state_root_fp(&balances);
            assert_eq!(imp, fp, "SHA mismatch at size={size}");

            let imp_xor = state_root::compute_state_root_xor(&balances);
            let fp_xor = compute_state_root_xor_fp(&balances);
            assert_eq!(imp_xor, fp_xor, "XOR mismatch at size={size}");
        }
    }
}
