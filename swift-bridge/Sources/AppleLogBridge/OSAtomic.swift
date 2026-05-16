import Foundation
import AppleLogCShim

final class BridgeOSAtomicI32Box {
    var value: Int32

    init(_ value: Int32) {
        self.value = value
    }
}

final class BridgeOSAtomicI64Box {
    var value: Int64

    init(_ value: Int64) {
        self.value = value
    }
}

final class BridgeOSAtomicQueueBox {
    let raw: UnsafeMutableRawPointer?

    init(raw: UnsafeMutableRawPointer?) {
        self.raw = raw
    }

    deinit {
        apple_os_atomic_queue_destroy(raw)
    }
}

final class BridgeOSAtomicFifoQueueBox {
    let raw: UnsafeMutableRawPointer?

    init(raw: UnsafeMutableRawPointer?) {
        self.raw = raw
    }

    deinit {
        apple_os_atomic_fifo_queue_destroy(raw)
    }
}

private func withUInt32Pointer<T>(
    _ box: BridgeOSAtomicI32Box,
    _ body: (UnsafeMutablePointer<UInt32>) -> T
) -> T {
    withUnsafeMutablePointer(to: &box.value) { pointer in
        body(UnsafeMutableRawPointer(pointer).assumingMemoryBound(to: UInt32.self))
    }
}

@_cdecl("apple_log_os_atomic_i32_new")
public func appleLogOSAtomicI32New(_ value: Int32) -> UnsafeMutableRawPointer {
    retainObject(BridgeOSAtomicI32Box(value))
}

@_cdecl("apple_log_os_atomic_i32_release")
public func appleLogOSAtomicI32Release(_ atomic: UnsafeMutableRawPointer?) {
    releaseObject(atomic)
}

@_cdecl("apple_log_os_atomic_i32_load")
public func appleLogOSAtomicI32Load(_ atomic: UnsafeMutableRawPointer?) -> Int32 {
    let box: BridgeOSAtomicI32Box? = takeObject(atomic)
    return box?.value ?? 0
}

@_cdecl("apple_log_os_atomic_i32_store")
public func appleLogOSAtomicI32Store(_ atomic: UnsafeMutableRawPointer?, _ value: Int32) {
    let box: BridgeOSAtomicI32Box? = takeObject(atomic)
    box?.value = value
}

@_cdecl("apple_log_os_atomic_i32_add")
public func appleLogOSAtomicI32Add(_ atomic: UnsafeMutableRawPointer?, _ amount: Int32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_add32(amount, &box.value)
}

@_cdecl("apple_log_os_atomic_i32_add_barrier")
public func appleLogOSAtomicI32AddBarrier(_ atomic: UnsafeMutableRawPointer?, _ amount: Int32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_add32_barrier(amount, &box.value)
}

@_cdecl("apple_log_os_atomic_i32_increment")
public func appleLogOSAtomicI32Increment(_ atomic: UnsafeMutableRawPointer?) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_increment32(&box.value)
}

@_cdecl("apple_log_os_atomic_i32_increment_barrier")
public func appleLogOSAtomicI32IncrementBarrier(_ atomic: UnsafeMutableRawPointer?) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_increment32_barrier(&box.value)
}

@_cdecl("apple_log_os_atomic_i32_decrement")
public func appleLogOSAtomicI32Decrement(_ atomic: UnsafeMutableRawPointer?) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_decrement32(&box.value)
}

@_cdecl("apple_log_os_atomic_i32_decrement_barrier")
public func appleLogOSAtomicI32DecrementBarrier(_ atomic: UnsafeMutableRawPointer?) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_decrement32_barrier(&box.value)
}

@_cdecl("apple_log_os_atomic_i32_or")
public func appleLogOSAtomicI32Or(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_or32(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_or_barrier")
public func appleLogOSAtomicI32OrBarrier(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_or32_barrier(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_or_orig")
public func appleLogOSAtomicI32OrOrig(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_or32_orig(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_or_orig_barrier")
public func appleLogOSAtomicI32OrOrigBarrier(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_or32_orig_barrier(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_and")
public func appleLogOSAtomicI32And(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_and32(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_and_barrier")
public func appleLogOSAtomicI32AndBarrier(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_and32_barrier(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_and_orig")
public func appleLogOSAtomicI32AndOrig(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_and32_orig(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_and_orig_barrier")
public func appleLogOSAtomicI32AndOrigBarrier(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_and32_orig_barrier(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_xor")
public func appleLogOSAtomicI32Xor(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_xor32(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_xor_barrier")
public func appleLogOSAtomicI32XorBarrier(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_xor32_barrier(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_xor_orig")
public func appleLogOSAtomicI32XorOrig(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_xor32_orig(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_xor_orig_barrier")
public func appleLogOSAtomicI32XorOrigBarrier(_ atomic: UnsafeMutableRawPointer?, _ mask: UInt32) -> Int32 {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return 0 }
    return withUInt32Pointer(box) { apple_os_atomic_xor32_orig_barrier(mask, $0) }
}

@_cdecl("apple_log_os_atomic_i32_compare_and_swap")
public func appleLogOSAtomicI32CompareAndSwap(
    _ atomic: UnsafeMutableRawPointer?,
    _ oldValue: Int32,
    _ newValue: Int32
) -> Bool {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return false }
    return apple_os_atomic_compare_and_swap32(oldValue, newValue, &box.value)
}

@_cdecl("apple_log_os_atomic_i32_compare_and_swap_barrier")
public func appleLogOSAtomicI32CompareAndSwapBarrier(
    _ atomic: UnsafeMutableRawPointer?,
    _ oldValue: Int32,
    _ newValue: Int32
) -> Bool {
    guard let box: BridgeOSAtomicI32Box = takeObject(atomic) else { return false }
    return apple_os_atomic_compare_and_swap32_barrier(oldValue, newValue, &box.value)
}

@_cdecl("apple_log_os_atomic_i64_new")
public func appleLogOSAtomicI64New(_ value: Int64) -> UnsafeMutableRawPointer {
    retainObject(BridgeOSAtomicI64Box(value))
}

@_cdecl("apple_log_os_atomic_i64_release")
public func appleLogOSAtomicI64Release(_ atomic: UnsafeMutableRawPointer?) {
    releaseObject(atomic)
}

@_cdecl("apple_log_os_atomic_i64_load")
public func appleLogOSAtomicI64Load(_ atomic: UnsafeMutableRawPointer?) -> Int64 {
    let box: BridgeOSAtomicI64Box? = takeObject(atomic)
    return box?.value ?? 0
}

@_cdecl("apple_log_os_atomic_i64_store")
public func appleLogOSAtomicI64Store(_ atomic: UnsafeMutableRawPointer?, _ value: Int64) {
    let box: BridgeOSAtomicI64Box? = takeObject(atomic)
    box?.value = value
}

@_cdecl("apple_log_os_atomic_i64_add")
public func appleLogOSAtomicI64Add(_ atomic: UnsafeMutableRawPointer?, _ amount: Int64) -> Int64 {
    guard let box: BridgeOSAtomicI64Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_add64(amount, &box.value)
}

@_cdecl("apple_log_os_atomic_i64_add_barrier")
public func appleLogOSAtomicI64AddBarrier(_ atomic: UnsafeMutableRawPointer?, _ amount: Int64) -> Int64 {
    guard let box: BridgeOSAtomicI64Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_add64_barrier(amount, &box.value)
}

@_cdecl("apple_log_os_atomic_i64_increment")
public func appleLogOSAtomicI64Increment(_ atomic: UnsafeMutableRawPointer?) -> Int64 {
    guard let box: BridgeOSAtomicI64Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_increment64(&box.value)
}

@_cdecl("apple_log_os_atomic_i64_increment_barrier")
public func appleLogOSAtomicI64IncrementBarrier(_ atomic: UnsafeMutableRawPointer?) -> Int64 {
    guard let box: BridgeOSAtomicI64Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_increment64_barrier(&box.value)
}

@_cdecl("apple_log_os_atomic_i64_decrement")
public func appleLogOSAtomicI64Decrement(_ atomic: UnsafeMutableRawPointer?) -> Int64 {
    guard let box: BridgeOSAtomicI64Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_decrement64(&box.value)
}

@_cdecl("apple_log_os_atomic_i64_decrement_barrier")
public func appleLogOSAtomicI64DecrementBarrier(_ atomic: UnsafeMutableRawPointer?) -> Int64 {
    guard let box: BridgeOSAtomicI64Box = takeObject(atomic) else { return 0 }
    return apple_os_atomic_decrement64_barrier(&box.value)
}

@_cdecl("apple_log_os_atomic_i64_compare_and_swap")
public func appleLogOSAtomicI64CompareAndSwap(
    _ atomic: UnsafeMutableRawPointer?,
    _ oldValue: Int64,
    _ newValue: Int64
) -> Bool {
    guard let box: BridgeOSAtomicI64Box = takeObject(atomic) else { return false }
    return apple_os_atomic_compare_and_swap64(oldValue, newValue, &box.value)
}

@_cdecl("apple_log_os_atomic_i64_compare_and_swap_barrier")
public func appleLogOSAtomicI64CompareAndSwapBarrier(
    _ atomic: UnsafeMutableRawPointer?,
    _ oldValue: Int64,
    _ newValue: Int64
) -> Bool {
    guard let box: BridgeOSAtomicI64Box = takeObject(atomic) else { return false }
    return apple_os_atomic_compare_and_swap64_barrier(oldValue, newValue, &box.value)
}

@_cdecl("apple_log_os_atomic_queue_new")
public func appleLogOSAtomicQueueNew(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let raw = apple_os_atomic_queue_create() else {
        setError(errorOut, "failed to allocate OSAtomic queue")
        return nil
    }
    return retainObject(BridgeOSAtomicQueueBox(raw: raw))
}

@_cdecl("apple_log_os_atomic_queue_release")
public func appleLogOSAtomicQueueRelease(_ queue: UnsafeMutableRawPointer?) {
    releaseObject(queue)
}

@_cdecl("apple_log_os_atomic_queue_enqueue")
public func appleLogOSAtomicQueueEnqueue(_ queue: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let box: BridgeOSAtomicQueueBox = takeObject(queue) else { return }
    apple_os_atomic_queue_enqueue_value(box.raw, value)
}

@_cdecl("apple_log_os_atomic_queue_dequeue")
public func appleLogOSAtomicQueueDequeue(
    _ queue: UnsafeMutableRawPointer?,
    _ valueOut: UnsafeMutablePointer<UInt>?
) -> Bool {
    guard let box: BridgeOSAtomicQueueBox = takeObject(queue) else { return false }
    return apple_os_atomic_queue_dequeue_value(box.raw, valueOut)
}

@_cdecl("apple_log_os_atomic_fifo_queue_new")
public func appleLogOSAtomicFifoQueueNew(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let raw = apple_os_atomic_fifo_queue_create() else {
        setError(errorOut, "failed to allocate OSAtomic FIFO queue")
        return nil
    }
    return retainObject(BridgeOSAtomicFifoQueueBox(raw: raw))
}

@_cdecl("apple_log_os_atomic_fifo_queue_release")
public func appleLogOSAtomicFifoQueueRelease(_ queue: UnsafeMutableRawPointer?) {
    releaseObject(queue)
}

@_cdecl("apple_log_os_atomic_fifo_queue_enqueue")
public func appleLogOSAtomicFifoQueueEnqueue(_ queue: UnsafeMutableRawPointer?, _ value: UInt) {
    guard let box: BridgeOSAtomicFifoQueueBox = takeObject(queue) else { return }
    apple_os_atomic_fifo_enqueue_value(box.raw, value)
}

@_cdecl("apple_log_os_atomic_fifo_queue_dequeue")
public func appleLogOSAtomicFifoQueueDequeue(
    _ queue: UnsafeMutableRawPointer?,
    _ valueOut: UnsafeMutablePointer<UInt>?
) -> Bool {
    guard let box: BridgeOSAtomicFifoQueueBox = takeObject(queue) else { return false }
    return apple_os_atomic_fifo_dequeue_value(box.raw, valueOut)
}
