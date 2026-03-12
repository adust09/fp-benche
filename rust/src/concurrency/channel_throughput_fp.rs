use crossbeam_channel::bounded;
use std::thread;

/// Channel throughput using for_each + fold instead of for loops.
/// Null hypothesis: channel synchronization dominates, so the style
/// difference (for-loop vs iterator combinators) has zero measurable impact.
pub fn channel_roundtrip_fp(n: u64) -> u64 {
    let (tx, rx) = bounded(1024);

    let sender = thread::spawn(move || {
        (0..n).for_each(|i| tx.send(i).expect("send failed"));
    });

    let receiver = thread::spawn(move || {
        rx.iter().fold(0u64, |acc, val| acc.wrapping_add(val))
    });

    sender.join().expect("sender panicked");
    receiver.join().expect("receiver panicked")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrency::channel_throughput;

    #[test]
    fn should_receive_all_messages() {
        let sum = channel_roundtrip_fp(100);
        let expected: u64 = (0..100).sum();
        assert_eq!(sum, expected);
    }

    #[test]
    fn should_handle_zero_messages() {
        assert_eq!(channel_roundtrip_fp(0), 0);
    }

    #[test]
    fn should_match_loop_based_channel() {
        let n = 1000;
        assert_eq!(
            channel_roundtrip_fp(n),
            channel_throughput::channel_roundtrip(n)
        );
    }
}
