# 🌀 Syncly

Minimalistic one-way folder sync tool written in Rust.  
Copies only new or changed files from a source directory to a target directory and cleans up empty target directories
— fast, safe, and optionally in verbose mode.

## 🚀 MVP Scope (Most Viable Product)

### ✅ Goal

Sync files from `source` to `target`:
- Only copy files that **don’t exist** in target or **differ** (by size or hash).
- Preserve folder structure.
- Perform the actual copy.

### ✨ Features (MVP)

- [x] Run with env variables `SOURCE_DIR (required / string)`, `TARGET_DIR (required / string)`, `VERBOSE (optional / bool)`
- [x] Recursively scan both directories
- [x] Compare files by:
    - Relative path
    - File hash
- [x] Copy only what’s needed
- [x] Show actions taken

### ❌ Not in MVP (yet)

- Bi-directional sync
- File deletion
- .syncignore support
- Multithreading / async
- Networking

---
