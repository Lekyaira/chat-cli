# Product Requirements Document (PRD) – Local Chat CLI

## 1. Introduction / Overview

This project delivers a command‑line chat interface that lets a single developer converse with a local large‑language model served by Ollama, providing a “mini ChatGPT” experience entirely offline. The tool will help the author deepen their understanding of LLM plumbing in Rust and serve as a foundation for future agentic work.

## 2. Goals

1. **End‑to‑end local chat** – Send prompts and receive streaming responses from an Ollama‑hosted model via OpenAI‑compatible endpoints.
2. **Conversation persistence** – Automatically resume the most recent transcript or start a new one with explicit CLI flags.
3. **Developer learning** – Provide clear code and docs that showcase async Rust, streaming, and JSON message handling.

## 3. User Stories

_As an LLM tinkerer, I want to open a REPL and type a prompt so that the model answers in real time._
_As the same user, I want each session to auto‑save so that I can close the CLI and pick up the conversation later without extra commands._
_As a tester, I want to pass `--new mylog.jsonl` to start a fresh conversation file._

## 4. Functional Requirements

1. **REPL Loop** – The program MUST present a prompt (`> `) and read user input until EOF or `/exit`.
2. **Streaming Output** – The system MUST request streamed tokens and render them as they arrive.
3. **Message Format** – The system MUST build and maintain a `Vec<Message>` where each message contains `role` and `content`.
4. **Autoload Transcript** – On startup, the CLI MUST attempt to load `~/.local/share/chat_cli/last.jsonl` if no `--new` or `--load` flag is given.
5. **Autosave Transcript** – On each user turn, the current conversation MUST be appended to the active transcript file.
6. **New Session Flag** – `--new <file>` MUST clear the in‑memory history and begin logging to `<file>`, updating the “last” symlink.
7. **Explicit Load Flag** – `--load <file>` MUST load an existing transcript into memory and continue appending.
8. **Configurable Model** – `--model <name>` SHOULD override the default model (default: `mistral`).
9. **Graceful Exit** – On SIGINT/SIGTERM or `/exit`, the program MUST flush unsaved state and close cleanly.
10. **Error Reporting** – The program MUST detect and display HTTP or JSON errors without crashing.

## 5. Non‑Goals (Out of Scope)

- Tool/function calling
- Vector database memory
- Multi‑model routing
- GUI/TUI front‑ends
- Windows support
- Any form of encryption for transcripts

## 6. Design Considerations

- **UI** – Simple REPL; optional coloured output using `colored` crate.
- **File format** – One JSONL message per line for easy diffing and append‑only writes.
- **XDG Paths** – Respect `$XDG_DATA_HOME`, defaulting to `~/.local/share/chat_cli/`.

## 7. Technical Considerations

- **Language & Tooling** – Rust 1.87+, `tokio`, `async-openai`, `clap`, `serde_json`, `tracing`.
- **Ollama Endpoint** – Assumes Ollama is listening on `http://localhost:11434` with OpenAI compatibility enabled.
- **Streaming** – Use `ChatCompletionStream` from `async-openai`.
- **CI** – GitHub Actions with `cargo test` and `clippy`.
- **Future‑proofing** – Core chat logic abstracted behind a `trait ChatBackend` so `ollama‑rs` can be swapped in later.

## 8. Success Metrics

- Can handle **100 consecutive turns** without exceeding the 8 GB VRAM limit.
- **<2 s** time‑to‑first‑token on a fresh run with default model.
- **100% pass** on unit tests for transcript load/save and message formatting.

## 9. Open Questions

1. Should we support multiline user input blocks?
2. Do we want optional markdown rendering in‑terminal?
3. Should the binary name and project crate be `chat_cli` or something more descriptive?

---
