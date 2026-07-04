import XCTest
@testable import GitAiMacLogic

final class CleanupFormattingTests: XCTestCase {
    func testFormatsByteCount() {
        XCTAssertFalse(CleanupFormatting.formatSize(1024).isEmpty)
    }
}
