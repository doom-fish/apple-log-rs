import Foundation

@_cdecl("apple_log_os_log_entry_level")
public func appleLogOSLogEntryLevel(_ entry: UnsafeMutableRawPointer?) -> Int32 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.level ?? -1
}
