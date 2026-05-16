import Foundation
import OSLog
import os
import AppleLogCShim

final class BridgeOSSignposterBox {
    let logHandle: OSLog
    let signposter: OSSignposter

    init(logHandle: OSLog, signposter: OSSignposter) {
        self.logHandle = logHandle
        self.signposter = signposter
    }
}

@_cdecl("apple_log_os_signposter_create")
public func appleLogOSSignposterCreate(
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
    let logHandle = OSLog(subsystem: subsystemString, category: categoryString)
    let box = BridgeOSSignposterBox(logHandle: logHandle, signposter: OSSignposter(logHandle: logHandle))
    return retainObject(box)
}

@_cdecl("apple_log_os_signposter_default")
public func appleLogOSSignposterDefault() -> UnsafeMutableRawPointer {
    let logHandle = OSLog.default
    return retainObject(BridgeOSSignposterBox(logHandle: logHandle, signposter: OSSignposter()))
}

@_cdecl("apple_log_os_signposter_disabled")
public func appleLogOSSignposterDisabled() -> UnsafeMutableRawPointer {
    let logHandle = OSLog.disabled
    return retainObject(BridgeOSSignposterBox(logHandle: logHandle, signposter: .disabled))
}

@_cdecl("apple_log_os_signposter_from_os_log")
public func appleLogOSSignposterFromOSLog(
    _ log: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let box: BridgeOSLogBox = takeObject(log) else {
        setError(errorOut, "invalid OSLog handle")
        return nil
    }
    return retainObject(
        BridgeOSSignposterBox(logHandle: box.logHandle, signposter: OSSignposter(logHandle: box.logHandle))
    )
}

@_cdecl("apple_log_os_signposter_from_logger")
public func appleLogOSSignposterFromLogger(
    _ logger: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        setError(errorOut, "invalid Logger handle")
        return nil
    }
    return retainObject(
        BridgeOSSignposterBox(logHandle: box.logHandle, signposter: OSSignposter(logger: box.logger))
    )
}

@_cdecl("apple_log_os_signposter_release")
public func appleLogOSSignposterRelease(_ signposter: UnsafeMutableRawPointer?) {
    releaseObject(signposter)
}

@_cdecl("apple_log_os_signposter_is_enabled")
public func appleLogOSSignposterIsEnabled(_ signposter: UnsafeMutableRawPointer?) -> Bool {
    guard let box: BridgeOSSignposterBox = takeObject(signposter) else {
        return false
    }
    return box.signposter.isEnabled
}

@_cdecl("apple_log_os_signposter_make_signpost_id")
public func appleLogOSSignposterMakeSignpostID(_ signposter: UnsafeMutableRawPointer?) -> UInt64 {
    guard let box: BridgeOSSignposterBox = takeObject(signposter) else {
        return OSSignpostID.invalid.rawValue
    }
    return box.signposter.makeSignpostID().rawValue
}

@_cdecl("apple_log_os_signposter_make_signpost_id_from_pointer")
public func appleLogOSSignposterMakeSignpostIDFromPointer(
    _ signposter: UnsafeMutableRawPointer?,
    _ pointer: UnsafeRawPointer?
) -> UInt64 {
    guard let box: BridgeOSSignposterBox = takeObject(signposter) else {
        return OSSignpostID.invalid.rawValue
    }
    return apple_signpost_id_make_with_pointer(bridgeLogHandle(box.logHandle), pointer)
}

@_cdecl("apple_log_os_signposter_emit_event")
public func appleLogOSSignposterEmitEvent(
    _ signposter: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ message: UnsafePointer<CChar>?
) {
    guard let box: BridgeOSSignposterBox = takeObject(signposter) else {
        return
    }
    apple_signpost_event_emit(bridgeLogHandle(box.logHandle), signpostID, name, message)
}

@_cdecl("apple_log_os_signposter_begin_interval")
public func appleLogOSSignposterBeginInterval(
    _ signposter: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ message: UnsafePointer<CChar>?
) {
    guard let box: BridgeOSSignposterBox = takeObject(signposter) else {
        return
    }
    apple_signpost_event_emit(bridgeLogHandle(box.logHandle), signpostID, name, message)
    apple_signpost_interval_begin(bridgeLogHandle(box.logHandle), signpostID, name)
}

@_cdecl("apple_log_os_signposter_begin_animation_interval")
public func appleLogOSSignposterBeginAnimationInterval(
    _ signposter: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ message: UnsafePointer<CChar>?
) {
    guard let box: BridgeOSSignposterBox = takeObject(signposter) else {
        return
    }
    apple_signpost_event_emit(bridgeLogHandle(box.logHandle), signpostID, name, message)
    apple_signpost_animation_interval_begin(bridgeLogHandle(box.logHandle), signpostID, name)
}

@_cdecl("apple_log_os_signposter_end_interval")
public func appleLogOSSignposterEndInterval(
    _ signposter: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ message: UnsafePointer<CChar>?
) {
    guard let box: BridgeOSSignposterBox = takeObject(signposter) else {
        return
    }
    apple_signpost_interval_end(bridgeLogHandle(box.logHandle), signpostID, name)
    apple_signpost_event_emit(bridgeLogHandle(box.logHandle), signpostID, name, message)
}
