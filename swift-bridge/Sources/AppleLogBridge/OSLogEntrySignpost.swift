import Foundation

@_cdecl("apple_log_os_log_entry_signpost_identifier")
public func appleLogOSLogEntrySignpostIdentifier(_ entry: UnsafeMutableRawPointer?) -> UInt64 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.signpostIdentifier ?? 0
}

@_cdecl("apple_log_os_log_entry_copy_signpost_name")
public func appleLogOSLogEntryCopySignpostName(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return copyOptionalCString(snapshot?.signpostName)
}

@_cdecl("apple_log_os_log_entry_signpost_type")
public func appleLogOSLogEntrySignpostType(_ entry: UnsafeMutableRawPointer?) -> Int32 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.signpostType ?? -1
}
