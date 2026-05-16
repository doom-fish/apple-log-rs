import Foundation
import AppleLogCShim

final class BridgeOSActivityBox {
    let raw: UnsafeMutableRawPointer?
    let owned: Bool

    init(raw: UnsafeMutableRawPointer?, owned: Bool) {
        self.raw = raw
        self.owned = owned
    }

    deinit {
        if owned {
            apple_activity_release(raw)
        }
    }
}

@_cdecl("apple_log_os_activity_create")
public func appleLogOSActivityCreate(
    _ description: UnsafePointer<CChar>?,
    _ parent: UnsafeMutableRawPointer?,
    _ flags: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let parentBox: BridgeOSActivityBox? = takeObject(parent)
    let raw = apple_activity_create(description, parentBox?.raw, flags)
    guard raw != nil else {
        setError(errorOut, "failed to create OSActivity")
        return nil
    }
    return retainObject(BridgeOSActivityBox(raw: raw, owned: true))
}

@_cdecl("apple_log_os_activity_start")
public func appleLogOSActivityStart(
    _ description: UnsafePointer<CChar>?,
    _ flags: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let raw = apple_activity_start(description, flags)
    guard raw != nil else {
        setError(errorOut, "failed to start OSActivity")
        return nil
    }
    return retainObject(BridgeOSActivityBox(raw: raw, owned: true))
}

@_cdecl("apple_log_os_activity_current")
public func appleLogOSActivityCurrent() -> UnsafeMutableRawPointer {
    retainObject(BridgeOSActivityBox(raw: apple_activity_current(), owned: false))
}

@_cdecl("apple_log_os_activity_none")
public func appleLogOSActivityNone() -> UnsafeMutableRawPointer {
    retainObject(BridgeOSActivityBox(raw: apple_activity_none(), owned: false))
}

@_cdecl("apple_log_os_activity_release")
public func appleLogOSActivityRelease(_ activity: UnsafeMutableRawPointer?) {
    releaseObject(activity)
}

@_cdecl("apple_log_os_activity_get_identifier")
public func appleLogOSActivityGetIdentifier(
    _ activity: UnsafeMutableRawPointer?,
    _ parentOut: UnsafeMutablePointer<UInt64>?
) -> UInt64 {
    let box: BridgeOSActivityBox? = takeObject(activity)
    return apple_activity_get_identifier(box?.raw, parentOut)
}

@_cdecl("apple_log_os_activity_apply")
public func appleLogOSActivityApply(
    _ activity: UnsafeMutableRawPointer?,
    _ context: UnsafeMutableRawPointer?,
    _ function: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
) {
    guard let function else {
        return
    }
    let box: BridgeOSActivityBox? = takeObject(activity)
    apple_activity_apply_f(box?.raw, context, function)
}

@_cdecl("apple_log_os_activity_scope_enter")
public func appleLogOSActivityScopeEnter(_ activity: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let box: BridgeOSActivityBox? = takeObject(activity)
    return apple_activity_scope_enter_alloc(box?.raw)
}

@_cdecl("apple_log_os_activity_scope_leave")
public func appleLogOSActivityScopeLeave(_ state: UnsafeMutableRawPointer?) {
    apple_activity_scope_leave_free(state)
}

@_cdecl("apple_log_os_activity_label_useraction")
public func appleLogOSActivityLabelUserAction(_ label: UnsafePointer<CChar>?) {
    apple_activity_label_useraction(label)
}

@_cdecl("apple_log_os_activity_set_breadcrumb")
public func appleLogOSActivitySetBreadcrumb(_ name: UnsafePointer<CChar>?) {
    apple_activity_set_breadcrumb(name)
}

@_cdecl("apple_log_os_activity_end")
public func appleLogOSActivityEnd(_ activity: UnsafeMutableRawPointer?) {
    let box: BridgeOSActivityBox? = takeObject(activity)
    apple_activity_end(box?.raw)
}

@_cdecl("apple_log_os_activity_get_active_identifiers")
public func appleLogOSActivityGetActiveIdentifiers(_ parentOut: UnsafeMutablePointer<UInt64>?) -> UInt64 {
    apple_activity_get_identifiers(parentOut)
}
