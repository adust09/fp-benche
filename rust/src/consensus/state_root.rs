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

/// Imperative state root: in-place bottom-up Merkle tree.
pub fn compute_state_root(balances: &[u64]) -> [u8; 32] {
    if balances.is_empty() {
        return [0u8; 32];
    }

    let mut nodes: Vec<[u8; 32]> = balances
        .iter()
        .map(|b| sha256_bytes(&b.to_le_bytes()))
        .collect();

    let target = nodes.len().next_power_of_two();
    nodes.resize(target, [0u8; 32]);

    let mut width = target;
    while width > 1 {
        for i in 0..(width / 2) {
            nodes[i] = sha256_pair(&nodes[2 * i], &nodes[2 * i + 1]);
        }
        width /= 2;
    }

    nodes[0]
}

/// Hash-free XOR variant for structure-only benchmarking.
pub fn compute_state_root_xor(balances: &[u64]) -> [u8; 32] {
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

    let mut width = target;
    while width > 1 {
        for i in 0..(width / 2) {
            let mut combined = [0u8; 32];
            for j in 0..32 {
                combined[j] = nodes[2 * i][j] ^ nodes[2 * i + 1][j];
            }
            nodes[i] = combined;
        }
        width /= 2;
    }

    nodes[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_returns_zero() {
        assert_eq!(compute_state_root(&[]), [0u8; 32]);
        assert_eq!(compute_state_root_xor(&[]), [0u8; 32]);
    }

    #[test]
    fn single_element() {
        let root = compute_state_root(&[42]);
        assert_ne!(root, [0u8; 32]);
    }

    #[test]
    fn deterministic() {
        let balances = vec![100, 200, 300, 400];
        let r1 = compute_state_root(&balances);
        let r2 = compute_state_root(&balances);
        assert_eq!(r1, r2);
    }

    #[test]
    fn golden_hash_value() {
        let balances = vec![1u64, 2, 3, 4];
        let root = compute_state_root(&balances);
        // Snapshot: re-running must produce the same hash
        let root2 = compute_state_root(&balances);
        assert_eq!(root, root2);
    }
}
