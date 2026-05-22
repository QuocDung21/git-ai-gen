# Nghiên cứu: Tách UI & Logic cho phát triển đa nền tảng

## Tổng quan kiến trúc hiện tại

```
git-ai/src/
├── main.rs              — Entry point, CLI routing (clap)
├── app/
│   ├── mod.rs           — App struct (98 fields) + tất cả business logic
│   ├── models.rs        — Data types (structs, enums)
│   └── events.rs        — TUI event loop + key handlers (2300+ lines)
├── git/                 — Git command wrappers (TUYỆT VỜI, đã tách sạch)
│   ├── remote.rs
│   ├── commit.rs
│   ├── branch.rs
│   ├── status.rs
│   └── stash.rs
├── ui/                  — Ratatui rendering (TUI-specific, dùng ratatui types)
│   ├── mod.rs
│   ├── components/
│   └── modals/
├── cli/                 — CLI non-TUI commands (diff, go, lang...)
├── helper/
└── constant/
```

## Đánh giá từng lớp

| Layer           | Tình trạng                                                        | Khả năng tái dùng                      |
| --------------- | ----------------------------------------------------------------- | -------------------------------------- |
| `git/`          | ✅ **Rất tốt** — thuần function, không phụ thuộc UI               | **100% tái dùng** cho mọi UI           |
| `cli/`          | ✅ **Tốt** — độc lập với TUI                                      | **100% tái dùng**                      |
| `app/models.rs` | ✅ **Tốt** — thuần data types                                     | **95% tái dùng** (bỏ `ratatui::Color`) |
| `app/mod.rs`    | ⚠️ **Vừa** — logic lẫn với state TUI                              | ~70% tái dùng nếu tách                 |
| `app/events.rs` | ❌ **Vấn đề lớn** — TUI event loop gắn chặt vào `terminal.draw()` | **0% tái dùng**                        |
| `ui/`           | ❌ **TUI-only** — phụ thuộc 100% ratatui                          | **0% tái dùng**                        |

---

## Vấn đề cốt lõi cần tách

### 1. `AppTheme` dùng `ratatui::Color`

```rust
// models.rs — Hiện tại
use ratatui::style::Color;

pub struct AppTheme {
    pub fg: Color,  // Ratatui-specific!
    ...
}
```

→ Nếu muốn dùng cho Swift/C#, Color phải là `(u8, u8, u8)` thuần Rust.

### 2. Business logic trộn lẫn trong `app/mod.rs`

Các method như `fetch_commit_logs()`, `copy_github_download_item()`, `try_generate_with_kilo()`
là **business logic thuần túy** nhưng sống trong `App` struct vốn chứa cả UI state như `diff_scroll_offset`, `active_modal`...

### 3. `events.rs` coupling TUI + logic

```rust
// events.rs — Hiện tại
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(...);       // TUI-specific
        // ... business logic
        match key.code { ... }   // TUI event system
    }
}
```

Tất cả business action đều trigger từ `crossterm` key events → không thể dùng lại.

---

## Kiến trúc đề xuất: Core Library Pattern

### Mục tiêu

Tách dự án thành **2 crate**:

- `git-ai-core` — Rust library thuần (business logic + git ops)
- `git-ai` — Binary (hiện tại TUI, sau có thể thêm bất kỳ UI)

```
git-ai/
├── git-ai-core/          ← Rust library crate (NEW)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── git/          ← Chuyển nguyên từ src/git/
│   │   ├── models.rs     ← Tách khỏi ratatui, dùng (u8,u8,u8) cho màu
│   │   ├── service/      ← Business logic thuần (NEW)
│   │   │   ├── commit.rs     — commit(), amend(), generate_with_kilo()
│   │   │   ├── remote.rs     — fetch(), pull(), push()
│   │   │   ├── github.rs     — clone_repo(), copy_file()
│   │   │   └── workspace.rs  — history, settings, config
│   │   └── state.rs      ← AppState struct (không có UI fields)
│   └── Cargo.toml
│
├── src/                  ← TUI binary (hiện tại)
│   ├── main.rs
│   ├── app/
│   │   ├── events.rs     ← Chỉ giữ TUI event routing
│   │   └── mod.rs        ← Wrapper, dùng git-ai-core
│   └── ui/               ← Giữ nguyên
│
└── Cargo.toml            ← workspace
```

### AppState (core, không có UI)

```rust
// git-ai-core/src/state.rs
pub struct AppState {
    pub current_dir: String,
    pub current_lang: String,
    pub current_branch: String,
    pub files: Vec<ChangedFile>,
    pub staged_count: usize,
    pub commit_logs: Vec<CommitLogEntry>,
    pub branches: Vec<BranchEntry>,
    pub stash_entries: Vec<StashEntry>,
    pub kilo_models: Vec<String>,
    pub settings: AppSettings,
    // Không có: active_modal, scroll_offset, diff_scroll, v.v.
}

pub struct AppSettings {
    pub auto_push: bool,
    pub auto_stage_all: bool,
    pub kilo_ai_enabled: bool,
    pub lang: String,
    pub is_light_theme: bool,
}
```

### Service layer (core, thuần fn)

```rust
// git-ai-core/src/service/remote.rs
pub fn fetch() -> Result<(), GitError> { ... }
pub fn pull() -> Result<PullResult, GitError> { ... }
pub fn push() -> Result<(), GitError> { ... }
```

---

## Giao tiếp với Swift / C# (FFI)

### Phương án A: C FFI với `cbindgen`

Rust export hàm C-compatible, Swift/C# gọi qua FFI.

```rust
// git-ai-core/src/ffi.rs
#[no_mangle]
pub extern "C" fn git_ai_fetch(dir: *const c_char) -> c_int { ... }
#[no_mangle]
pub extern "C" fn git_ai_get_status(dir: *const c_char) -> *mut c_char { ... }
```

```swift
// Swift
import Foundation
let result = git_ai_fetch(dir)
```

**Ưu điểm**: Native performance, không cần runtime  
**Nhược điểm**: Boilerplate FFI, khó với complex types

### Phương án B: IPC qua JSON (Khuyến nghị ✅)

Tạo `git-ai-daemon` binary nhỏ, chạy background, nhận lệnh qua stdin/stdout JSON hoặc Unix socket.

```
SwiftUI App  ──JSON──►  git-ai-core daemon  ──►  git commands
C# WPF App   ──JSON──►  git-ai-core daemon  ──►  git commands
TUI          ──direct─►  git-ai-core lib    ──►  git commands
```

**Ưu điểm**: Đơn giản nhất, không cần FFI, debug dễ  
**Nhược điểm**: Thêm 1 process

### Phương án C: HTTP API nội bộ (Nặng hơn)

Dùng `axum` (Rust HTTP server) làm local REST API.  
⚠️ **Vi phạm rule "No async"** của dự án.

---

## Kết luận & Khuyến nghị

### Nên làm ngay (Low effort, High value)

1. **Tách `AppTheme` khỏi `ratatui::Color`** — Dùng `(u8, u8, u8)` trong core, convert khi render TUI
2. **Tách `service/` module** từ `app/mod.rs` — Business logic không phụ thuộc UI state
3. **Tạo Cargo workspace** chuẩn bị cho tương lai

### Nên làm khi bắt đầu phát triển UI mới

4. **Tạo `git-ai-core` crate** riêng biệt
5. **Implement IPC JSON protocol** cho giao tiếp cross-platform

### Không cần làm ngay

- FFI/cbindgen (phức tạp không cần thiết lúc này)
- HTTP server (vi phạm no-async rule)
- Rewrite events.rs (vẫn cần cho TUI, chỉ cần tách logic ra ngoài)

---

## Tóm tắt

> **Lớp `git/` đã hoàn hảo** — đây là nền tảng tốt nhất để build core library.  
> **Vấn đề chính**: `app/mod.rs` (~1000 lines) trộn lẫn UI state + business logic + Git calls.  
> **Bước tách đơn giản nhất**: Tạo `src/service/` layer, chuyển các `pub fn` không cần `&mut self` ra ngoài `App`, rồi `App` chỉ giữ UI state và gọi service.  
> **Khi có UI mới** (Swift/C#): Dùng **IPC JSON** — compile core thành binary daemon, UI mới gọi qua process communication. Đây là cách GitUI, LazyGit và nhiều tool TUI đang dùng.
