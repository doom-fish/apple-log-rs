import Foundation
import OSLog
import os
import AppleLogCShim

@_cdecl("apple_log_os_signpost_id_null")
public func appleLogOSSignpostIDNull() -> UInt64 {
    OSSignpostID.null.rawValue
}

@_cdecl("apple_log_os_signpost_id_invalid")
public func appleLogOSSignpostIDInvalid() -> UInt64 {
    OSSignpostID.invalid.rawValue
}

@_cdecl("apple_log_os_signpost_id_exclusive")
public func appleLogOSSignpostIDExclusive() -> UInt64 {
    OSSignpostID.exclusive.rawValue
}

@_cdecl("apple_log_os_signpost_id_generate")
public func appleLogOSSignpostIDGenerate(_ log: UnsafeMutableRawPointer?) -> UInt64 {
    guard let box: BridgeOSLogBox = takeObject(log) else {
        return OSSignpostID.invalid.rawValue
    }
    return OSSignpostID(log: box.logHandle).rawValue
}

@_cdecl("apple_log_os_signpost_id_make_with_pointer")
public func appleLogOSSignpostIDMakeWithPointer(
    _ log: UnsafeMutableRawPointer?,
    _ pointer: UnsafeRawPointer?
) -> UInt64 {
    guard let box: BridgeOSLogBox = takeObject(log) else {
        return OSSignpostID.invalid.rawValue
    }
    return apple_signpost_id_make_with_pointer(bridgeLogHandle(box.logHandle), pointer)
}
