import Foundation
import OSLog
import os

final class BridgeOSLogBox {
    let logHandle: OSLog
    let subsystem: String?
    let category: String?

    init(logHandle: OSLog, subsystem: String?, category: String?) {
        self.logHandle = logHandle
        self.subsystem = subsystem
        self.category = category
    }
}

@_cdecl("apple_log_os_log_create")
public func appleLogOSLogCreate(
    _ subsystem: UnsafePointer<CChar>?,
    _ category: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let subsystem, let category else {
        setError(errorOut, "subsystem and category are required")
        return nil
    }
    let subsystemString = String(cString: subsystem)
    let categoryString = String(cString: category)
    let box = BridgeOSLogBox(
        logHandle: OSLog(subsystem: subsystemString, category: categoryString),
        subsystem: subsystemString,
        category: categoryString
    )
    return retainObject(box)
}

@_cdecl("apple_log_os_log_default")
public func appleLogOSLogDefault() -> UnsafeMutableRawPointer {
    retainObject(BridgeOSLogBox(logHandle: .default, subsystem: nil, category: nil))
}

@_cdecl("apple_log_os_log_disabled")
public func appleLogOSLogDisabled() -> UnsafeMutableRawPointer {
    retainObject(BridgeOSLogBox(logHandle: .disabled, subsystem: nil, category: nil))
}

@_cdecl("apple_log_os_log_release")
public func appleLogOSLogRelease(_ log: UnsafeMutableRawPointer?) {
    releaseObject(log)
}

@_cdecl("apple_log_os_log_is_enabled")
public func appleLogOSLogIsEnabled(_ log: UnsafeMutableRawPointer?, _ level: UInt8) -> Bool {
    guard let box: BridgeOSLogBox = takeObject(log) else {
        return false
    }
    return box.logHandle.isEnabled(type: OSLogType(level))
}

@_cdecl("apple_log_os_log_signposts_enabled")
public func appleLogOSLogSignpostsEnabled(_ log: UnsafeMutableRawPointer?) -> Bool {
    guard let box: BridgeOSLogBox = takeObject(log) else {
        return false
    }
    return box.logHandle.signpostsEnabled
}

@_cdecl("apple_log_os_log_copy_subsystem")
public func appleLogOSLogCopySubsystem(_ log: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let box: BridgeOSLogBox? = takeObject(log)
    return copyOptionalCString(box?.subsystem)
}

@_cdecl("apple_log_os_log_copy_category")
public func appleLogOSLogCopyCategory(_ log: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let box: BridgeOSLogBox? = takeObject(log)
    return copyOptionalCString(box?.category)
}
