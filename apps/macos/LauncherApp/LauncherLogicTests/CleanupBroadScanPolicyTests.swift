import XCTest
@testable import GitAiMacLogic

final class CleanupBroadScanPolicyTests: XCTestCase {
    func testDetectsBroadRoots() {
        XCTAssertTrue(CleanupBroadScanPolicy.isBroadScanPath("/", homePath: "/Users/test"))
        XCTAssertTrue(CleanupBroadScanPolicy.isBroadScanPath("/Users", homePath: "/Users/test"))
        XCTAssertTrue(CleanupBroadScanPolicy.isBroadScanPath("/Users/test", homePath: "/Users/test"))
        XCTAssertTrue(CleanupBroadScanPolicy.isBroadScanPath("/Users/test/Documents", homePath: "/Users/test"))
    }

    func testAllowsFocusedProjectFolders() {
        XCTAssertFalse(CleanupBroadScanPolicy.isBroadScanPath("/Users/test/project", homePath: "/Users/test"))
        XCTAssertFalse(CleanupBroadScanPolicy.isBroadScanPath("/Users/test/Documents/app/node_modules", homePath: "/Users/test"))
    }
}
