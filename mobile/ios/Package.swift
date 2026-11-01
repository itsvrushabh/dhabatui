// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "DhabaTUILive",
    platforms: [
        .iOS(.v15),
        .macOS(.v12)
    ],
    products: [
        .library(name: "DhabaTUILive", targets: ["DhabaTUILive"]),
    ],
    targets: [
        .target(
            name: "DhabaTUILive",
            path: "DhabaTUILive",
            exclude: ["Info.plist"]
        ),
    ]
)
