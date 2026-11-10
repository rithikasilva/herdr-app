set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

default:
    @just --list

check: test

notices:
    bun scripts/notices.ts

build:
    cargo build --locked

test:
    bun test scripts/files.test.ts scripts/machine-add.test.ts scripts/picker.test.ts scripts/palette.test.ts scripts/agents.test.ts scripts/spaces.test.ts scripts/worktree.test.ts
    cargo test --locked

docs-generate: build
    bun scripts/docs.ts generate

docs-check: build
    bun test scripts/docs.test.ts
    bun scripts/docs.ts check

[linux]
install:
    bash scripts/install-linux

[macos]
install:
    bash scripts/install-macos

[windows]
install:
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/install-windows.ps1
