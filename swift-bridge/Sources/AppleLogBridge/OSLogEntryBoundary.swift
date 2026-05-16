import Foundation

@_cdecl("apple_log_os_log_entry_is_boundary")
public func appleLogOSLogEntryIsBoundary(_ entry: UnsafeMutableRawPointer?) -> Bool {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.kind == 3
}
