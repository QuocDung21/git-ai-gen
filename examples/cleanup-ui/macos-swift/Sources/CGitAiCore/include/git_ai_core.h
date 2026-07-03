#ifndef GIT_AI_CORE_H
#define GIT_AI_CORE_H

#include <stdbool.h>
#include <stdint.h>

typedef void (*git_ai_cleanup_scan_callback)(
    const char *path,
    const char *target,
    uint64_t size_bytes,
    void *user_data
);
typedef bool (*git_ai_cleanup_should_cancel_callback)(void *user_data);

char *git_ai_cleanup_scan_node_modules(const char *path);
char *git_ai_cleanup_scan_build_folders(const char *path);
char *git_ai_cleanup_scan_devcleaner(const char *path);
char *git_ai_cleanup_scan_node_modules_stream(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    void *user_data
);
char *git_ai_cleanup_scan_node_modules_stream_cancellable(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    git_ai_cleanup_should_cancel_callback should_cancel,
    void *user_data
);
char *git_ai_cleanup_scan_build_folders_stream(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    void *user_data
);
char *git_ai_cleanup_scan_build_folders_stream_cancellable(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    git_ai_cleanup_should_cancel_callback should_cancel,
    void *user_data
);
char *git_ai_cleanup_scan_devcleaner_stream(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    void *user_data
);
char *git_ai_cleanup_scan_devcleaner_stream_cancellable(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    git_ai_cleanup_should_cancel_callback should_cancel,
    void *user_data
);
char *git_ai_cleanup_delete_paths(const char *paths_json);
void git_ai_free_string(char *s);

#endif
