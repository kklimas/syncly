# 🌀 Syncly

Minimalistic one-way folder sync tool written in Rust.  
Copies only new or changed files from a source directory to a target directory — fast, safe, and optionally in dry-run mode.

## 🚀 MVP Scope (Most Viable Product)

### ✅ Goal

Sync files from `source` to `target`:
- Only copy files that **don’t exist** in target or **differ** (by size or hash).
- Preserve folder structure.
- Show what would happen (`--dry-run`), or perform the actual copy.

### ✨ Features (MVP)

- [x] CLI with `--source`, `--target`, `--dry-run`, `--verbose`
- [x] Recursively scan both directories
- [x] Compare files by:
    - Relative path
    - File size
    - File hash
- [x] Copy only what’s needed
- [x] Show actions taken or planned

### ❌ Not in MVP (yet)

- Bi-directional sync
- File deletion
- .syncignore support
- Multithreading / async
- Networking

---

## 🧪 Example usage

```bash
syncly --source ./photos --target ./backup --dry-run --verbose