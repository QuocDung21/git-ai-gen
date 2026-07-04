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
            exclude: [
                "Assets.xcassets",
                "git_ai_appApp.swift"
            ],
            sources: [
                "ContentView.swift",
                "Models/CleanupModels.swift",
                "Support/AppDelegate.swift",
                "Support/AppConstants.swift",
                "Support/Cleanup/CleanupBroadScanPolicy.swift",
                "Support/Cleanup/CleanupFormatting.swift",
                "Support/Cleanup/CleanupSelectionLogic.swift",
                "Support/Cleanup/RustCleanupBridge.swift",
                "Views/Cleanup/CleanupBroadScanWarningView.swift",
                "Views/Cleanup/CleanupPreview.swift",
                "Views/Cleanup/CleanupResultsView.swift",
                "Views/Cleanup/CleanupView.swift",
                "Views/Settings/CleanupSettingsView.swift"
            ],
            linkerSettings: [
                .unsafeFlags(["-L", "../../../target/debug", "-L", "../../../target/release"]),
                .linkedLibrary("git_ai_core")
            ]
        ),
        .executableTarget(
            name: "GitAiMacApp",
            dependencies: ["GitAiMacLogic"],
            path: "git-ai-app",
            exclude: [
                "Assets.xcassets",
                "Models",
                "Support",
                "Views"
            ],
            sources: [
                "git_ai_appApp.swift"
            ]
        ),
        .testTarget(
            name: "LauncherLogicTests",
            dependencies: ["GitAiMacLogic"],
            path: "LauncherLogicTests"
        ),
    ]
)
