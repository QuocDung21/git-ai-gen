import CGitAiCore
import Foundation

enum RustCleanupBridge {
    static func scan(path: String, kind: CleanupKind) throws -> [CleanupItem] {
        let json = try callString(
            { cPath in
                switch kind {
                case .nodeModules:
                    return git_ai_cleanup_scan_node_modules(cPath)
                case .buildFolders:
                    return git_ai_cleanup_scan_build_folders(cPath)
                case .devCleaner:
                    return git_ai_cleanup_scan_devcleaner(cPath)
                }
            }, input: path)

        try throwIfError(json)
        return try decode(CleanupScanResponse.self, from: json).items
    }

    static func delete(paths: [String]) throws -> [CleanupDeleteReport] {
        let data = try JSONEncoder().encode(paths)
        let payload = String(decoding: data, as: UTF8.self)

        let json = try callString(
            { cJson in
                git_ai_cleanup_delete_paths(cJson)
            }, input: payload)

        try throwIfError(json)
        return try decode(CleanupDeleteResponse.self, from: json).reports
    }

    static func scanStreaming(
        path: String,
        kind: CleanupKind,
        cancellationToken: ScanCancellationToken,
        onItem: @escaping (CleanupItem) -> Void
    ) throws {
        let context = StreamingScanContext(cancellationToken: cancellationToken, onItem: onItem)
        let retainedContext = Unmanaged.passRetained(context)
        defer { retainedContext.release() }

        let json = try path.withCString { cPath in
            let result: UnsafeMutablePointer<CChar>?
            switch kind {
            case .nodeModules:
                result = git_ai_cleanup_scan_node_modules_stream_cancellable(
                    cPath,
                    streamingScanCallback,
                    streamingShouldCancelCallback,
                    retainedContext.toOpaque()
                )
            case .buildFolders:
                result = git_ai_cleanup_scan_build_folders_stream_cancellable(
                    cPath,
                    streamingScanCallback,
                    streamingShouldCancelCallback,
                    retainedContext.toOpaque()
                )
            case .devCleaner:
                result = git_ai_cleanup_scan_devcleaner_stream_cancellable(
                    cPath,
                    streamingScanCallback,
                    streamingShouldCancelCallback,
                    retainedContext.toOpaque()
                )
            }

            guard let result else {
                throw CleanupBridgeError.nullResult
            }
            defer { git_ai_free_string(result) }
            return String(cString: result)
        }

        try throwIfError(json)
    }

    private static func callString(
        _ body: (UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?,
        input: String
    ) throws -> String {
        try input.withCString { cInput in
            guard let result = body(cInput) else {
                throw CleanupBridgeError.nullResult
            }
            defer { git_ai_free_string(result) }
            return String(cString: result)
        }
    }

    private static func decode<T: Decodable>(_ type: T.Type, from json: String) throws -> T {
        guard let data = json.data(using: .utf8) else {
            throw CleanupBridgeError.invalidUtf8
        }
        return try JSONDecoder().decode(T.self, from: data)
    }

    private static func throwIfError(_ json: String) throws {
        guard let data = json.data(using: .utf8) else { return }
        if let response = try? JSONDecoder().decode(CleanupErrorResponse.self, from: data) {
            throw CleanupBridgeError.rust(response.error)
        }
    }
}

final class ScanCancellationToken: @unchecked Sendable {
    private let lock = NSLock()
    private var cancelled = false

    func cancel() {
        lock.lock()
        cancelled = true
        lock.unlock()
    }

    var isCancelled: Bool {
        lock.lock()
        let value = cancelled
        lock.unlock()
        return value
    }
}

private final class StreamingScanContext {
    let cancellationToken: ScanCancellationToken
    let onItem: (CleanupItem) -> Void

    init(cancellationToken: ScanCancellationToken, onItem: @escaping (CleanupItem) -> Void) {
        self.cancellationToken = cancellationToken
        self.onItem = onItem
    }
}

private let streamingScanCallback: git_ai_cleanup_scan_callback = {
    path, target, sizeBytes, userData in
    guard let path, let target, let userData else { return }
    let context = Unmanaged<StreamingScanContext>.fromOpaque(userData).takeUnretainedValue()
    let item = CleanupItem(
        path: String(cString: path),
        target: String(cString: target),
        sizeBytes: sizeBytes
    )
    context.onItem(item)
}

private let streamingShouldCancelCallback: git_ai_cleanup_should_cancel_callback = { userData in
    guard let userData else { return false }
    let context = Unmanaged<StreamingScanContext>.fromOpaque(userData).takeUnretainedValue()
    return context.cancellationToken.isCancelled
}

enum CleanupBridgeError: LocalizedError {
    case nullResult
    case invalidUtf8
    case rust(String)

    var errorDescription: String? {
        switch self {
        case .nullResult:
            return "Rust returned a null pointer."
        case .invalidUtf8:
            return "Rust returned invalid UTF-8."
        case .rust(let message):
            return message
        }
    }
}
