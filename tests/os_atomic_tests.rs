use apple_log::prelude::*;

#[test]
fn os_atomic_smoke() {
    let atomic32 = OSAtomicI32::new(5);
    assert_eq!(atomic32.add(1), 6);
    assert!(atomic32.compare_and_swap(6, 10));
    assert_eq!(atomic32.or(0b0011), 0b1011);

    let atomic64 = OSAtomicI64::new(10);
    assert_eq!(atomic64.increment(), 11);
    assert!(atomic64.compare_and_swap(11, 20));

    let queue = OSAtomicQueue::new().expect("queue");
    queue.enqueue(7);
    assert_eq!(queue.dequeue(), Some(7));

    let fifo = OSAtomicFifoQueue::new().expect("fifo");
    fifo.enqueue(9);
    assert_eq!(fifo.dequeue(), Some(9));
}
