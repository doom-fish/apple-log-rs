#import <Foundation/Foundation.h>
#import <OSLog/OSLog.h>

NS_ASSUME_NONNULL_BEGIN

NSPredicate * _Nullable ALXTryMakePredicate(
    NSString *format,
    NSError * _Nullable * _Nullable error
);

NSArray<OSLogEntry *> * _Nullable ALXTryCollectEntries(
    OSLogStore *store,
    OSLogEnumeratorOptions options,
    OSLogPosition * _Nullable position,
    NSPredicate * _Nullable predicate,
    NSUInteger limit,
    NSError * _Nullable * _Nullable error
);

NS_ASSUME_NONNULL_END
