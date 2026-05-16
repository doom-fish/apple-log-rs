use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let atomic32 = OSAtomicI32::new(10);
    let atomic64 = OSAtomicI64::new(100);
    let queue = OSAtomicQueue::new()?;
    let fifo = OSAtomicFifoQueue::new()?;

    println!("add32={} inc64={}", atomic32.add(5), atomic64.increment());
    println!("cas32={} cas64={}", atomic32.compare_and_swap(15, 20), atomic64.compare_and_swap(101, 200));
    queue.enqueue(7);
    fifo.enqueue(9);
    println!("queue={:?} fifo={:?}", queue.dequeue(), fifo.dequeue());
    Ok(())
}
