## Relevant Files

- `Cargo.toml` - Project configuration and dependencies.
- `src/main.rs` - CLI entry point and REPL loop.
- `src/cli.rs` - Command line argument parser using `clap`.
- `src/chat_backend.rs` - `ChatBackend` trait abstraction.
- `src/ollama_backend.rs` - Ollama implementation of `ChatBackend` using `async-openai`.
- `src/transcript.rs` - Load and save conversation transcripts in JSONL.
- `src/error.rs` - Custom error types for HTTP and JSON issues.
- `tests/transcript_tests.rs` - Unit tests for transcript load/save.
- `tests/message_format_tests.rs` - Unit tests for message structure.
- `.github/workflows/ci.yml` - GitHub Actions config for `cargo test` and `clippy`.
- `tasks/tasks-prd-local-chat-cli.md` - Task list for implementing the PRD.
- `README.md` - Project overview and usage instructions.

### Notes

- Unit tests should typically be placed alongside the code files they are testing (e.g., `transcript.rs` and `transcript_tests.rs` in the same directory).

## Tasks

- [x] 1.0 Initialize Rust project structure
  - [x] 1.1 Create `Cargo.toml` with necessary dependencies
  - [x] 1.2 Set up `src/main.rs` with async runtime entry point
- [x] 2.0 Implement CLI argument parsing
  - [x] 2.1 Define flags `--new`, `--load`, and `--model` in `src/cli.rs`
  - [x] 2.2 Hook CLI parsing into `main.rs`
- [x] 3.0 Build chat backend abstraction
  - [x] 3.1 Define `ChatBackend` trait in `chat_backend.rs`
  - [x] 3.2 Implement `OllamaBackend` using `async-openai`
- [x] 4.0 Create REPL loop and message handling
  - [x] 4.1 Prompt user until `/exit` or EOF
  - [x] 4.2 Maintain `Vec<Message>` with `role` and `content`
  - [x] 4.3 Stream responses as tokens arrive
- [x] 5.0 Support conversation persistence
  - [x] 5.1 Autoload default transcript on startup
  - [x] 5.2 Autosave conversation after each turn
  - [x] 5.3 Implement `--new` and update `last` symlink
  - [x] 5.4 Implement `--load` to continue from existing file
- [x] 6.0 Graceful exit and error handling
  - [x] 6.1 Flush unsaved state on SIGINT/SIGTERM or `/exit`
  - [x] 6.2 Display HTTP/JSON errors without crashing
- [x] 7.0 Configuration options
  - [x] 7.1 Apply `--model` flag to choose the backend model
  - [x] 7.2 Optionally enable coloured output using `colored` crate
- [x] 8.0 Testing
  - [x] 8.1 Unit tests for transcript load/save and message formatting
- [x] 9.0 Continuous integration and documentation
  - [x] 9.1 Configure GitHub Actions to run `cargo test` and `clippy`
  - [x] 9.2 Provide usage instructions and architecture overview in README
