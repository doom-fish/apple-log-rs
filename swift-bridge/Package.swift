// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "AppleLogBridge",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "AppleLogBridge",
            type: .static,
            targets: ["AppleLogBridge"])
    ],
    targets: [
        .target(
            name: "AppleLogBridge",
            dependencies: ["AppleLogCShim", "AppleLogObjCBridge"],
            path: "Sources/AppleLogBridge"),
        .target(
            name: "AppleLogObjCBridge",
            path: "Sources/AppleLogObjCBridge",
            publicHeadersPath: "include"),
        .target(
            name: "AppleLogCShim",
            path: "Sources/AppleLogCShim",
            publicHeadersPath: "include",
            cSettings: [
                .unsafeFlags(["-fmodules", "-Wno-deprecated-declarations"])
            ])
    ]
)
