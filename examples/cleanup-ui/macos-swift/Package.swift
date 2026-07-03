// swift-tools-version: 6.2

import PackageDescription

let package = Package(
    name: "GitAiCleanupUI",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(name: "GitAiCleanupUI", targets: ["GitAiCleanupUI"]),
        .library(name: "GitAiCleanupUILibrary", targets: ["GitAiCleanupUILibrary"])
    ],
    targets: [
        .target(
            name: "CGitAiCore",
            path: "Sources/CGitAiCore",
            publicHeadersPath: "include"
        ),

        .target(
            name: "GitAiCleanupUILibrary",
            dependencies: ["CGitAiCore"],
            path: "Sources/GitAiCleanupUILibrary",
            linkerSettings: [
                .unsafeFlags(["-L", "../../../target/debug"]),
                .linkedLibrary("git_ai_core")
            ]
        ),

        .executableTarget(
            name: "GitAiCleanupUI",
            dependencies: ["GitAiCleanupUILibrary"],
            path: "Sources/GitAiCleanupUI"
        )
    ]
)
