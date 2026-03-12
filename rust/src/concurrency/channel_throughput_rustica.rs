use crossbeam_channel::bounded;
use rustica::datatypes::maybe::Maybe;
use std::thread;

/// Channel throughput with Maybe-wrapped result.
/// Sender uses for_each (same as _fp), receiver uses iter().fold (same as _fp).
/// The only rustica element is Maybe wrapping the result for type-safe representation.
/// This is a control benchmark: channel sync dominates all style differences.
pub fn channel_roundtrip_rustica(n: u64) -> u64 {
    let (tx, rx) = bounded(1024);

    let sender = thread::spawn(move || {
        (0..n).for_each(|i| tx.send(i).expect("send failed"));
    });

    let receiver = thread::spawn(move || {
        let sum = rx.iter().fold(0u64, |acc, val| acc.wrapping_add(val));
        Maybe::some(sum)
    });

    sender.join().expect("sender panicked");
    receiver.join().expect("receiver panicked").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrency::channel_throughput;

    #[test]
    fn should_receive_all_messages() {
        let sum = channel_roundtrip_rustica(100);
        let expected: u64 = (0..100).sum();
        assert_eq!(sum, expected);
    }

    #[test]
    fn should_handle_zero_messages() {
        assert_eq!(channel_roundtrip_rustica(0), 0);
    }

    #[test]
    fn should_match_loop_based_channel() {
        let n = 1000;
        assert_eq!(
            channel_roundtrip_rustica(n),
            channel_throughput::channel_roundtrip(n)
        );
    }
}
