import Foundation

@_cdecl("apple_log_os_log_entry_parent_activity_identifier")
public func appleLogOSLogEntryParentActivityIdentifier(_ entry: UnsafeMutableRawPointer?) -> UInt64 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.parentActivityIdentifier ?? 0
}
