#import "AppleLogObjCBridge.h"

static NSError *ALXErrorWithMessage(NSInteger code, NSString *message) {
    return [NSError errorWithDomain:@"AppleLogObjCBridge"
                               code:code
                           userInfo:@{NSLocalizedDescriptionKey: message}];
}

static NSError *ALXErrorFromException(NSException *exception) {
    NSString *reason = exception.reason ?: exception.name;
    return ALXErrorWithMessage(1, reason);
}

NSPredicate * _Nullable ALXTryMakePredicate(
    NSString *format,
    NSError * _Nullable * _Nullable error
) {
    @try {
        return [NSPredicate predicateWithFormat:format argumentArray:@[]];
    } @catch (NSException *exception) {
        if (error != NULL) {
            *error = ALXErrorFromException(exception);
        }
        return nil;
    }
}

NSArray<OSLogEntry *> * _Nullable ALXTryCollectEntries(
    OSLogStore *store,
    OSLogEnumeratorOptions options,
    OSLogPosition * _Nullable position,
    NSPredicate * _Nullable predicate,
    NSUInteger limit,
    NSError * _Nullable * _Nullable error
) {
    NSMutableArray<OSLogEntry *> *entries = [[NSMutableArray alloc] init];
    @try {
        NSError *enumeratorError = nil;
        OSLogEnumerator *enumerator = [store entriesEnumeratorWithOptions:options
                                                                 position:position
                                                                predicate:predicate
                                                                    error:&enumeratorError];
        if (enumerator == nil) {
            if (error != NULL) {
                *error = enumeratorError ?: ALXErrorWithMessage(2, @"OSLogStore rejected the query");
            }
            return nil;
        }
        while (entries.count < limit) {
            @autoreleasepool {
                OSLogEntry *entry = [enumerator nextObject];
                if (entry == nil) {
                    break;
                }
                [entries addObject:entry];
            }
        }
    } @catch (NSException *exception) {
        if (error != NULL) {
            *error = ALXErrorFromException(exception);
        }
        return nil;
    }
    return entries;
}
