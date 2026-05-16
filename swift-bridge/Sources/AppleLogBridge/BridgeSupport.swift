import Darwin
import Foundation
import OSLog
import os

func retainObject(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

func takeObject<T: AnyObject>(_ raw: UnsafeMutableRawPointer?) -> T? {
    guard let raw else {
        return nil
    }
    return Unmanaged<T>.fromOpaque(raw).takeUnretainedValue()
}

func releaseObject(_ raw: UnsafeMutableRawPointer?) {
    guard let raw else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(raw).release()
}

func copyCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

func copyOptionalCString(_ string: String?) -> UnsafeMutablePointer<CChar>? {
    guard let string else {
        return nil
    }
    return copyCString(string)
}

func setError(_ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?, _ message: String) {
    errorOut?.pointee = copyCString(message)
}

func copyDataBuffer(
    _ data: Data?,
    _ lengthOut: UnsafeMutablePointer<Int>?
) -> UnsafeMutableRawPointer? {
    guard let data else {
        lengthOut?.pointee = 0
        return nil
    }
    let byteCount = data.count
    guard let buffer = malloc(byteCount) else {
        lengthOut?.pointee = 0
        return nil
    }
    data.copyBytes(to: buffer.assumingMemoryBound(to: UInt8.self), count: byteCount)
    lengthOut?.pointee = byteCount
    return buffer
}

func bridgeLogHandle(_ log: OSLog) -> UnsafeMutableRawPointer {
    Unmanaged.passUnretained(log as AnyObject).toOpaque()
}

@_cdecl("apple_log_string_free")
public func appleLogStringFree(_ string: UnsafeMutablePointer<CChar>?) {
    free(string)
}

@_cdecl("apple_log_bytes_free")
public func appleLogBytesFree(_ bytes: UnsafeMutableRawPointer?) {
    free(bytes)
}
