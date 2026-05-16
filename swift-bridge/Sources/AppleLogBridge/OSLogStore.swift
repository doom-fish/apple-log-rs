import Foundation
import OSLog
import os

final class BridgeOSLogMessageComponent {
    let formatSubstring: String
    let placeholder: String
    let argumentCategory: Int32
    let dataValue: Data?
    let doubleValue: Double
    let int64Value: Int64
    let numberValue: NSNumber?
    let stringValue: String?
    let uint64Value: UInt64

    init(_ component: OSLogMessageComponent) {
        formatSubstring = component.formatSubstring
        placeholder = component.placeholder
        argumentCategory = Int32(component.argumentCategory.rawValue)
        dataValue = component.argumentDataValue
        doubleValue = component.argumentDoubleValue
        int64Value = component.argumentInt64Value
        numberValue = component.argumentNumberValue
        stringValue = component.argumentStringValue
        uint64Value = component.argumentUInt64Value
    }
}

final class BridgeOSLogEntrySnapshot {
    let kind: Int32
    let composedMessage: String
    let dateSecondsSince1970: Double
    let storeCategory: Int32
    let activityIdentifier: UInt64?
    let process: String?
    let processIdentifier: Int32?
    let sender: String?
    let threadIdentifier: UInt64?
    let category: String?
    let components: [BridgeOSLogMessageComponent]
    let formatString: String?
    let subsystem: String?
    let level: Int32?
    let signpostIdentifier: UInt64?
    let signpostName: String?
    let signpostType: Int32?
    let parentActivityIdentifier: UInt64?

    init(entry: OSLogEntry) {
        composedMessage = entry.composedMessage
        dateSecondsSince1970 = entry.date.timeIntervalSince1970
        storeCategory = Int32(entry.storeCategory.rawValue)

        if let processEntry = entry as? OSLogEntryFromProcess {
            activityIdentifier = UInt64(processEntry.activityIdentifier)
            process = processEntry.process
            processIdentifier = Int32(processEntry.processIdentifier)
            sender = processEntry.sender
            threadIdentifier = processEntry.threadIdentifier
        } else {
            activityIdentifier = nil
            process = nil
            processIdentifier = nil
            sender = nil
            threadIdentifier = nil
        }

        if let payloadEntry = entry as? OSLogEntryWithPayload {
            category = payloadEntry.category
            components = payloadEntry.components.map(BridgeOSLogMessageComponent.init)
            formatString = payloadEntry.formatString
            subsystem = payloadEntry.subsystem
        } else {
            category = nil
            components = []
            formatString = nil
            subsystem = nil
        }

        if let logEntry = entry as? OSLogEntryLog {
            kind = 1
            level = Int32(logEntry.level.rawValue)
            signpostIdentifier = nil
            signpostName = nil
            signpostType = nil
            parentActivityIdentifier = nil
        } else if let signpostEntry = entry as? OSLogEntrySignpost {
            kind = 2
            level = nil
            signpostIdentifier = signpostEntry.signpostIdentifier
            signpostName = signpostEntry.signpostName
            signpostType = Int32(signpostEntry.signpostType.rawValue)
            parentActivityIdentifier = nil
        } else if entry is OSLogEntryBoundary {
            kind = 3
            level = nil
            signpostIdentifier = nil
            signpostName = nil
            signpostType = nil
            parentActivityIdentifier = nil
        } else if let activityEntry = entry as? OSLogEntryActivity {
            kind = 4
            level = nil
            signpostIdentifier = nil
            signpostName = nil
            signpostType = nil
            parentActivityIdentifier = UInt64(activityEntry.parentActivityIdentifier)
        } else {
            kind = 0
            level = nil
            signpostIdentifier = nil
            signpostName = nil
            signpostType = nil
            parentActivityIdentifier = nil
        }
    }
}

final class BridgeOSLogStoreBox {
    let store: OSLogStore

    init(store: OSLogStore) {
        self.store = store
    }
}

final class BridgeOSLogPositionBox {
    let position: OSLogPosition

    init(position: OSLogPosition) {
        self.position = position
    }
}

final class BridgeOSLogEntryListBox {
    let entries: [BridgeOSLogEntrySnapshot]

    init(entries: [BridgeOSLogEntrySnapshot]) {
        self.entries = entries
    }
}

private func bridgePredicate(_ predicate: UnsafePointer<CChar>?) -> NSPredicate? {
    guard let predicate else {
        return nil
    }
    return NSPredicate(format: String(cString: predicate))
}

@_cdecl("apple_log_os_log_store_local")
public func appleLogOSLogStoreLocal(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        return retainObject(BridgeOSLogStoreBox(store: try OSLogStore.local()))
    } catch {
        setError(errorOut, error.localizedDescription)
        return nil
    }
}

@_cdecl("apple_log_os_log_store_create")
public func appleLogOSLogStoreCreate(
    _ scope: Int32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    do {
        let resolvedScope: OSLogStore.Scope
        switch scope {
        case 0:
            resolvedScope = .system
        case 1:
            resolvedScope = .currentProcessIdentifier
        default:
            setError(errorOut, "invalid OSLogStore scope")
            return nil
        }
        return retainObject(BridgeOSLogStoreBox(store: try OSLogStore(scope: resolvedScope)))
    } catch {
        setError(errorOut, error.localizedDescription)
        return nil
    }
}

@_cdecl("apple_log_os_log_store_from_url")
public func appleLogOSLogStoreFromURL(
    _ path: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let path else {
        setError(errorOut, "path is required")
        return nil
    }
    do {
        let url = URL(fileURLWithPath: String(cString: path))
        return retainObject(BridgeOSLogStoreBox(store: try OSLogStore(url: url)))
    } catch {
        setError(errorOut, error.localizedDescription)
        return nil
    }
}

@_cdecl("apple_log_os_log_store_release")
public func appleLogOSLogStoreRelease(_ store: UnsafeMutableRawPointer?) {
    releaseObject(store)
}

@_cdecl("apple_log_os_log_store_position_date")
public func appleLogOSLogStorePositionDate(
    _ store: UnsafeMutableRawPointer?,
    _ secondsSince1970: Double
) -> UnsafeMutableRawPointer? {
    guard let box: BridgeOSLogStoreBox = takeObject(store) else {
        return nil
    }
    let position = box.store.position(date: Date(timeIntervalSince1970: secondsSince1970))
    return retainObject(BridgeOSLogPositionBox(position: position))
}

@_cdecl("apple_log_os_log_store_position_since_end")
public func appleLogOSLogStorePositionSinceEnd(
    _ store: UnsafeMutableRawPointer?,
    _ seconds: Double
) -> UnsafeMutableRawPointer? {
    guard let box: BridgeOSLogStoreBox = takeObject(store) else {
        return nil
    }
    let position = box.store.position(timeIntervalSinceEnd: seconds)
    return retainObject(BridgeOSLogPositionBox(position: position))
}

@_cdecl("apple_log_os_log_store_position_since_latest_boot")
public func appleLogOSLogStorePositionSinceLatestBoot(
    _ store: UnsafeMutableRawPointer?,
    _ seconds: Double
) -> UnsafeMutableRawPointer? {
    guard let box: BridgeOSLogStoreBox = takeObject(store) else {
        return nil
    }
    let position = box.store.position(timeIntervalSinceLatestBoot: seconds)
    return retainObject(BridgeOSLogPositionBox(position: position))
}

@_cdecl("apple_log_os_log_position_release")
public func appleLogOSLogPositionRelease(_ position: UnsafeMutableRawPointer?) {
    releaseObject(position)
}

@_cdecl("apple_log_os_log_store_get_entries")
public func appleLogOSLogStoreGetEntries(
    _ store: UnsafeMutableRawPointer?,
    _ options: UInt,
    _ position: UnsafeMutableRawPointer?,
    _ predicate: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let storeBox: BridgeOSLogStoreBox = takeObject(store) else {
        setError(errorOut, "invalid OSLogStore handle")
        return nil
    }
    let positionBox: BridgeOSLogPositionBox? = takeObject(position)
    do {
        let entries = try storeBox.store.getEntries(
            with: OSLogEnumerator.Options(rawValue: options),
            at: positionBox?.position,
            matching: bridgePredicate(predicate)
        )
        let snapshots = Array(entries).map(BridgeOSLogEntrySnapshot.init)
        return retainObject(BridgeOSLogEntryListBox(entries: snapshots))
    } catch {
        setError(errorOut, error.localizedDescription)
        return nil
    }
}

@_cdecl("apple_log_os_log_entry_list_release")
public func appleLogOSLogEntryListRelease(_ list: UnsafeMutableRawPointer?) {
    releaseObject(list)
}

@_cdecl("apple_log_os_log_entry_list_count")
public func appleLogOSLogEntryListCount(_ list: UnsafeMutableRawPointer?) -> Int {
    let box: BridgeOSLogEntryListBox? = takeObject(list)
    return box?.entries.count ?? 0
}

@_cdecl("apple_log_os_log_entry_list_get")
public func appleLogOSLogEntryListGet(
    _ list: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let box: BridgeOSLogEntryListBox = takeObject(list), index >= 0, index < box.entries.count else {
        return nil
    }
    return retainObject(box.entries[index])
}

@_cdecl("apple_log_os_log_entry_release")
public func appleLogOSLogEntryRelease(_ entry: UnsafeMutableRawPointer?) {
    releaseObject(entry)
}

@_cdecl("apple_log_os_log_entry_kind")
public func appleLogOSLogEntryKind(_ entry: UnsafeMutableRawPointer?) -> Int32 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.kind ?? 0
}

@_cdecl("apple_log_os_log_entry_copy_composed_message")
public func appleLogOSLogEntryCopyComposedMessage(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return copyOptionalCString(snapshot?.composedMessage)
}

@_cdecl("apple_log_os_log_entry_get_date_seconds")
public func appleLogOSLogEntryGetDateSeconds(_ entry: UnsafeMutableRawPointer?) -> Double {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.dateSecondsSince1970 ?? 0
}

@_cdecl("apple_log_os_log_entry_get_store_category")
public func appleLogOSLogEntryGetStoreCategory(_ entry: UnsafeMutableRawPointer?) -> Int32 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.storeCategory ?? 0
}

@_cdecl("apple_log_os_log_entry_get_activity_identifier")
public func appleLogOSLogEntryGetActivityIdentifier(_ entry: UnsafeMutableRawPointer?) -> UInt64 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.activityIdentifier ?? 0
}

@_cdecl("apple_log_os_log_entry_copy_process")
public func appleLogOSLogEntryCopyProcess(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return copyOptionalCString(snapshot?.process)
}

@_cdecl("apple_log_os_log_entry_get_process_identifier")
public func appleLogOSLogEntryGetProcessIdentifier(_ entry: UnsafeMutableRawPointer?) -> Int32 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.processIdentifier ?? -1
}

@_cdecl("apple_log_os_log_entry_copy_sender")
public func appleLogOSLogEntryCopySender(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return copyOptionalCString(snapshot?.sender)
}

@_cdecl("apple_log_os_log_entry_get_thread_identifier")
public func appleLogOSLogEntryGetThreadIdentifier(_ entry: UnsafeMutableRawPointer?) -> UInt64 {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.threadIdentifier ?? 0
}

@_cdecl("apple_log_os_log_entry_copy_category")
public func appleLogOSLogEntryCopyCategory(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return copyOptionalCString(snapshot?.category)
}

@_cdecl("apple_log_os_log_entry_copy_format_string")
public func appleLogOSLogEntryCopyFormatString(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return copyOptionalCString(snapshot?.formatString)
}

@_cdecl("apple_log_os_log_entry_copy_subsystem")
public func appleLogOSLogEntryCopySubsystem(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return copyOptionalCString(snapshot?.subsystem)
}

@_cdecl("apple_log_os_log_entry_component_count")
public func appleLogOSLogEntryComponentCount(_ entry: UnsafeMutableRawPointer?) -> Int {
    let snapshot: BridgeOSLogEntrySnapshot? = takeObject(entry)
    return snapshot?.components.count ?? 0
}

@_cdecl("apple_log_os_log_entry_component_get")
public func appleLogOSLogEntryComponentGet(
    _ entry: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let snapshot: BridgeOSLogEntrySnapshot = takeObject(entry), index >= 0, index < snapshot.components.count else {
        return nil
    }
    return retainObject(snapshot.components[index])
}

@_cdecl("apple_log_os_log_message_component_release")
public func appleLogOSLogMessageComponentRelease(_ component: UnsafeMutableRawPointer?) {
    releaseObject(component)
}

@_cdecl("apple_log_os_log_message_component_copy_format_substring")
public func appleLogOSLogMessageComponentCopyFormatSubstring(_ component: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogMessageComponent? = takeObject(component)
    return copyOptionalCString(snapshot?.formatSubstring)
}

@_cdecl("apple_log_os_log_message_component_copy_placeholder")
public func appleLogOSLogMessageComponentCopyPlaceholder(_ component: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogMessageComponent? = takeObject(component)
    return copyOptionalCString(snapshot?.placeholder)
}

@_cdecl("apple_log_os_log_message_component_get_argument_category")
public func appleLogOSLogMessageComponentGetArgumentCategory(_ component: UnsafeMutableRawPointer?) -> Int32 {
    let snapshot: BridgeOSLogMessageComponent? = takeObject(component)
    return snapshot?.argumentCategory ?? 0
}

@_cdecl("apple_log_os_log_message_component_get_double")
public func appleLogOSLogMessageComponentGetDouble(_ component: UnsafeMutableRawPointer?) -> Double {
    let snapshot: BridgeOSLogMessageComponent? = takeObject(component)
    return snapshot?.doubleValue ?? 0
}

@_cdecl("apple_log_os_log_message_component_get_int64")
public func appleLogOSLogMessageComponentGetInt64(_ component: UnsafeMutableRawPointer?) -> Int64 {
    let snapshot: BridgeOSLogMessageComponent? = takeObject(component)
    return snapshot?.int64Value ?? 0
}

@_cdecl("apple_log_os_log_message_component_copy_string")
public func appleLogOSLogMessageComponentCopyString(_ component: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    let snapshot: BridgeOSLogMessageComponent? = takeObject(component)
    return copyOptionalCString(snapshot?.stringValue)
}

@_cdecl("apple_log_os_log_message_component_get_uint64")
public func appleLogOSLogMessageComponentGetUInt64(_ component: UnsafeMutableRawPointer?) -> UInt64 {
    let snapshot: BridgeOSLogMessageComponent? = takeObject(component)
    return snapshot?.uint64Value ?? 0
}

@_cdecl("apple_log_os_log_message_component_copy_data")
public func appleLogOSLogMessageComponentCopyData(
    _ component: UnsafeMutableRawPointer?,
    _ lengthOut: UnsafeMutablePointer<Int>?
) -> UnsafeMutableRawPointer? {
    let snapshot: BridgeOSLogMessageComponent? = takeObject(component)
    return copyDataBuffer(snapshot?.dataValue, lengthOut)
}
