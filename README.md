# 🤖 git-ai — AI-Powered Git Commit Assistant

**Version 3.0.0** • Rust • TUI Dashboard • Bilingual (Tiếng Việt / English)

> Generate high-quality Git commit messages with AI — fast.  
> Full-featured terminal UI for staging, diff viewing, committing, branching, stashing, and more.

---

## ✨ Key Features

- **Interactive TUI Dashboard** (`git-ai`) — 3-column live view: changes • diff • commands
- **AI Commit Snapshot** (`git-copydiff`) — capture diff + ready-to-paste AI prompt
- **One-Command Commit & Push** (`git-go`) — use AI message to commit + push instantly
- **Full Git Workflow** inside TUI:
  - Stage / unstage / revert files
  - View live diffs (staged, unstaged, untracked)
  - Commit history browser + diff viewer
  - Branch switcher, create & merge branches
  - Stash manager (push / pop / apply / drop)
  - Amend last commit
  - Remote info (ahead/behind, tracking)
  - Conflict detection
- **Bilingual** — Vietnamese & English (auto-detect from system or `git config`)
- **Theme Support** — Dracula (dark) and Premium Light
- **Workspace History** — quickly jump between recent projects
- **Cross-platform** — macOS, Linux, Windows (PowerShell)

---

## 🚀 Quick Start

### 1. Install via Cargo (recommended)

```bash
cargo install git-ai
```

Or build from source:

```bash
git clone https://github.com/your-org/git-ai.git
cd git-ai
cargo build --release
./target/release/git-ai install
```

### 2. Install shell aliases

```bash
git-ai install
```

This adds the following convenient aliases to your shell:

| Alias              | Description                    |
| ------------------ | ------------------------------ |
| `git-copydiff`     | Capture diff snapshot for AI   |
| `git-go`           | Commit + push using AI message |
| `git-ai`           | Launch full TUI dashboard      |
| `git-ai-uninstall` | Remove git-ai from your system |

After install, **reload your shell**:

```bash
# zsh / bash
source ~/.zshrc   # or ~/.bashrc

# fish
source ~/.config/fish/config.fish
```

### 3. (Optional) Set language manually

```bash
git-ai lang vi     # Vietnamese
git-ai lang en     # English
git-ai lang auto   # follow system locale
```

---

## 📖 Usage

### Launch the Dashboard (recommended)

```bash
git-ai
```

Inside the TUI you get a live, keyboard-driven Git control center.

**Common keys** (shown in right panel):

- `Space` — stage / unstage file
- `Enter` — view full diff / confirm
- `g` — commit & push (Go)
- `b` — branch manager
- `s` — stash manager
- `l` — commit log
- `?` / `h` — help
- `q` / `Esc` — close modal or quit

### CLI Commands

```bash
git-ai diff        # alias: git-copydiff
git-ai go          # alias: git-go
git-ai lang <vi|en|auto>
git-ai install
git-ai uninstall   # alias: git-ai-uninstall
git-ai reset       # factory reset (removes all git-ai config)
```

---

## 🛠️ Configuration

All settings are stored in Git global config (`git config --global`):

| Key                        | Example          | Description            |
| -------------------------- | ---------------- | ---------------------- |
| `git-ai.lang`              | `vi` / `en`      | Interface language     |
| `git-ai.theme`             | `dark` / `light` | UI theme               |
| `git-ai.workspace-history` | (auto)           | Recent project folders |

Reset everything:

```bash
git-ai reset
```

---

## 🏗️ Architecture (for contributors)

The project follows a clean, maintainable structure:

```
apps/tui/
├── src/
│   ├── main.rs      # Clap CLI entrypoint
│   ├── app/         # TUI state + event loop
│   ├── ui/          # ratatui rendering
│   └── cli/         # Non-TUI commands + shell install

bridge/ffi/          # dedicated C ABI crate
core/git-ai-core/    # pure/shared Rust logic
packaging/           # release artifacts and package recipes
scripts/             # check/build helpers
```

**Important rules** (see `AGENTS.md` for full details):

- All user strings must be bilingual (`core/git-ai-core/locales/en.yml` + `vi.yml`)
- Git operations only via `std::process::Command`
- No comments in source code
- New features → add to `ActiveModal` enum + `App` state
- Always use `app.theme()` for colors

Run the full local verification suite with:

```bash
scripts/check.sh
```

See `docs/architecture.md`, `docs/ffi.md`, and `docs/build-and-release.md` for the workspace, ABI, and release flows.

---

## 🤝 Contributing

1. Fork the repo
2. Create a feature branch
3. Follow the rules in `AGENTS.md`
4. Test on both dark & light themes + both languages
5. Open a Pull Request

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

## 🇻🇳 Tiếng Việt

**git-ai** là công cụ dòng lệnh hỗ trợ viết commit message bằng AI nhanh chóng, kèm giao diện TUI đầy đủ chức năng.

Các lệnh phổ biến:

```bash
git-ai                  # Mở dashboard tương tác
git-copydiff            # Chụp diff để gửi AI
git-go                  # Commit + push bằng nội dung AI
git-ai install          # Cài alias vào shell
git-ai lang vi          # Dùng tiếng Việt
```

Mọi cài đặt được lưu trong `git config --global`.

---

<p align="center">
  Made with ❤️ for developers who hate writing commit messages.
</p>
