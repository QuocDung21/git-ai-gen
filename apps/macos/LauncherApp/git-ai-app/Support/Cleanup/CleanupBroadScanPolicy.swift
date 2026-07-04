import Foundation

public enum CleanupBroadScanPolicy {
    public static func isBroadScanPath(_ path: String, homePath: String = FileManager.default.homeDirectoryForCurrentUser.path) -> Bool {
        let standardizedPath = NSString(string: path).expandingTildeInPath
        return standardizedPath == "/"
            || standardizedPath == "/Users"
            || standardizedPath == homePath
            || standardizedPath == "\(homePath)/Documents"
    }
}
