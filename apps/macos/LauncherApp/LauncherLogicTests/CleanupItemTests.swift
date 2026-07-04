import XCTest
@testable import GitAiMacLogic

final class CleanupItemTests: XCTestCase {
    func testNodeModulesDisplayNameIncludesParentFolder() {
        let item = CleanupItem(
            path: "/tmp/sample-app/node_modules",
            target: "node_modules",
            sizeBytes: 1024
        )

        XCTAssertEqual(item.displayName, "sample-app / node_modules")
        XCTAssertEqual(item.parentFolderName, "sample-app")
    }

    func testWorkspacePathUsesWorkspaceGroup() {
        let item = CleanupItem(
            path: "/Users/test/Documents/Wordspace/project/target",
            target: "build",
            sizeBytes: 2048
        )

        XCTAssertEqual(item.groupName, "Workspace Projects")
    }
}
