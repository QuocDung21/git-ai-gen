// swift-tools-version: 5.10

import PackageDescription

let package = Package(
    name: "GitAiMacApp",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(name: "GitAiMacApp", targets: ["GitAiMacApp"]),
        .library(name: "GitAiMacLogic", targets: ["GitAiMacLogic"])
    ],
    targets: [
        .target(
            name: "CGitAiCore",
            path: "CGitAiCore",
            publicHeadersPath: "include"
        ),
        .target(
            name: "GitAiMacLogic",
            dependencies: ["CGitAiCore"],
            path: "git-ai-app",
            exclude: ["App"],
            sources: [
                "Features/Cleanup/Models/CleanupModels.swift",
                "Features/Cleanup/Services/RustCleanupBridge.swift",
                "Features/Cleanup/Views/CleanupBroadScanWarningView.swift",
                "Features/Cleanup/Views/CleanupPreview.swift",
                "Features/Cleanup/Views/CleanupResultsView.swift",
                "Features/Cleanup/Views/CleanupSettingsView.swift",
                "Features/Cleanup/Views/CleanupView.swift"
            ],
            linkerSettings: [
                .unsafeFlags(["-L", "../../../target/debug", "-L", "../../../target/release"]),
                .linkedLibrary("git_ai_core")
            ]
        ),
        .executableTarget(
            name: "GitAiMacApp",
            dependencies: ["GitAiMacLogic"],
            path: "git-ai-app/App"
        ),
        .testTarget(
            name: "LauncherLogicTests",
            dependencies: ["GitAiMacLogic"],
            path: "LauncherLogicTests"
        ),
    ]
)
