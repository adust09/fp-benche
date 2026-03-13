use crossbeam_channel::{bounded, Receiver, Sender};
use std::thread;

/// Send n messages through a bounded crossbeam channel.
/// Returns the sum of received values (forces complete consumption).
pub fn channel_roundtrip(n: u64) -> u64 {
    let (tx, rx): (Sender<u64>, Receiver<u64>) = bounded(1024);

    let sender = thread::spawn(move || {
        for i in 0..n {
            tx.send(i).expect("send failed");
        }
    });

    let receiver = thread::spawn(move || {
        let mut sum = 0u64;
        for val in rx {
            sum = sum.wrapping_add(val);
        }
        sum
    });

    sender.join().expect("sender panicked");
    receiver.join().expect("receiver panicked")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_receive_all_messages() {
        let sum = channel_roundtrip(100);
        let expected: u64 = (0..100).sum();
        assert_eq!(sum, expected);
    }

    #[test]
    fn should_handle_zero_messages() {
        assert_eq!(channel_roundtrip(0), 0);
    }
}
