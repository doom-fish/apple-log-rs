import Foundation
import OSLog
import os
import AppleLogCShim

private enum BridgeLoggerSeverity: Int32 {
    case `default` = 0
    case trace = 1
    case debug = 2
    case info = 3
    case notice = 4
    case warning = 5
    case error = 6
    case critical = 7
    case fault = 8
}

final class BridgeLoggerBox {
    let logHandle: OSLog
    let logger: Logger

    init(logHandle: OSLog, logger: Logger) {
        self.logHandle = logHandle
        self.logger = logger
    }
}

private func bridgeLoggerLog(
    _ box: BridgeLoggerBox,
    _ severity: Int32,
    _ privacy: Int32,
    _ message: String
) {
    let isPrivate = privacy == 1
    switch BridgeLoggerSeverity(rawValue: severity) ?? .default {
    case .default:
        if isPrivate {
            box.logger.log("\(message, privacy: .private)")
        } else {
            box.logger.log("\(message, privacy: .public)")
        }
    case .trace:
        if isPrivate {
            box.logger.trace("\(message, privacy: .private)")
        } else {
            box.logger.trace("\(message, privacy: .public)")
        }
    case .debug:
        if isPrivate {
            box.logger.debug("\(message, privacy: .private)")
        } else {
            box.logger.debug("\(message, privacy: .public)")
        }
    case .info:
        if isPrivate {
            box.logger.info("\(message, privacy: .private)")
        } else {
            box.logger.info("\(message, privacy: .public)")
        }
    case .notice:
        if isPrivate {
            box.logger.notice("\(message, privacy: .private)")
        } else {
            box.logger.notice("\(message, privacy: .public)")
        }
    case .warning:
        if isPrivate {
            box.logger.warning("\(message, privacy: .private)")
        } else {
            box.logger.warning("\(message, privacy: .public)")
        }
    case .error:
        if isPrivate {
            box.logger.error("\(message, privacy: .private)")
        } else {
            box.logger.error("\(message, privacy: .public)")
        }
    case .critical:
        if isPrivate {
            box.logger.critical("\(message, privacy: .private)")
        } else {
            box.logger.critical("\(message, privacy: .public)")
        }
    case .fault:
        if isPrivate {
            box.logger.fault("\(message, privacy: .private)")
        } else {
            box.logger.fault("\(message, privacy: .public)")
        }
    }
}

@_cdecl("apple_log_logger_create")
public func appleLogLoggerCreate(
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
    let box = BridgeLoggerBox(logHandle: logHandle, logger: Logger(logHandle))
    return retainObject(box)
}

@_cdecl("apple_log_logger_default")
public func appleLogLoggerDefault() -> UnsafeMutableRawPointer {
    retainObject(BridgeLoggerBox(logHandle: .default, logger: Logger()))
}

@_cdecl("apple_log_logger_disabled")
public func appleLogLoggerDisabled() -> UnsafeMutableRawPointer {
    let logHandle = OSLog.disabled
    return retainObject(BridgeLoggerBox(logHandle: logHandle, logger: Logger(logHandle)))
}

@_cdecl("apple_log_logger_from_os_log")
public func appleLogLoggerFromOSLog(
    _ log: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let box: BridgeOSLogBox = takeObject(log) else {
        setError(errorOut, "invalid OSLog handle")
        return nil
    }
    return retainObject(BridgeLoggerBox(logHandle: box.logHandle, logger: Logger(box.logHandle)))
}

@_cdecl("apple_log_logger_release")
public func appleLogLoggerRelease(_ logger: UnsafeMutableRawPointer?) {
    releaseObject(logger)
}

@_cdecl("apple_log_logger_log")
public func appleLogLoggerLog(
    _ logger: UnsafeMutableRawPointer?,
    _ severity: Int32,
    _ privacy: Int32,
    _ message: UnsafePointer<CChar>?
) {
    guard let box: BridgeLoggerBox = takeObject(logger), let message else {
        return
    }
    bridgeLoggerLog(box, severity, privacy, String(cString: message))
}

@_cdecl("apple_log_logger_is_enabled")
public func appleLogLoggerIsEnabled(_ logger: UnsafeMutableRawPointer?, _ level: UInt8) -> Bool {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        return false
    }
    return box.logHandle.isEnabled(type: OSLogType(level))
}

@_cdecl("apple_log_logger_signpost_id")
public func appleLogLoggerSignpostID(_ logger: UnsafeMutableRawPointer?) -> UInt64 {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        return OSSignpostID.invalid.rawValue
    }
    return apple_signpost_id_generate(bridgeLogHandle(box.logHandle))
}

@_cdecl("apple_log_logger_signpost_id_from_pointer")
public func appleLogLoggerSignpostIDFromPointer(
    _ logger: UnsafeMutableRawPointer?,
    _ pointer: UnsafeRawPointer?
) -> UInt64 {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        return OSSignpostID.invalid.rawValue
    }
    return apple_signpost_id_make_with_pointer(bridgeLogHandle(box.logHandle), pointer)
}

@_cdecl("apple_log_logger_signposts_enabled")
public func appleLogLoggerSignpostsEnabled(_ logger: UnsafeMutableRawPointer?) -> Bool {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        return false
    }
    return apple_signpost_enabled(bridgeLogHandle(box.logHandle))
}

@_cdecl("apple_log_logger_signpost_event")
public func appleLogLoggerSignpostEvent(
    _ logger: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?,
    _ message: UnsafePointer<CChar>?
) {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        return
    }
    apple_signpost_event_emit(
        bridgeLogHandle(box.logHandle),
        signpostID,
        name,
        message
    )
}

@_cdecl("apple_log_logger_signpost_interval_begin")
public func appleLogLoggerSignpostIntervalBegin(
    _ logger: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?
) {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        return
    }
    apple_signpost_interval_begin(bridgeLogHandle(box.logHandle), signpostID, name)
}

@_cdecl("apple_log_logger_signpost_animation_interval_begin")
public func appleLogLoggerSignpostAnimationIntervalBegin(
    _ logger: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?
) {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        return
    }
    apple_signpost_animation_interval_begin(bridgeLogHandle(box.logHandle), signpostID, name)
}

@_cdecl("apple_log_logger_signpost_interval_end")
public func appleLogLoggerSignpostIntervalEnd(
    _ logger: UnsafeMutableRawPointer?,
    _ signpostID: UInt64,
    _ name: UnsafePointer<CChar>?
) {
    guard let box: BridgeLoggerBox = takeObject(logger) else {
        return
    }
    apple_signpost_interval_end(bridgeLogHandle(box.logHandle), signpostID, name)
}
