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

### Notes

- Unit tests should typically be placed alongside the code files they are testing (e.g., `transcript.rs` and `transcript_tests.rs` in the same directory).

## Tasks

- [x] 1.0 Initialize Rust project structure
  - [x] 1.1 Create `Cargo.toml` with necessary dependencies
  - [x] 1.2 Set up `src/main.rs` with async runtime entry point
- [ ] 2.0 Implement CLI argument parsing
  - [x] 2.1 Define flags `--new`, `--load`, and `--model` in `src/cli.rs`
  - [ ] 2.2 Hook CLI parsing into `main.rs`
- [ ] 3.0 Build chat backend abstraction
  - [ ] 3.1 Define `ChatBackend` trait in `chat_backend.rs`
  - [ ] 3.2 Implement `OllamaBackend` using `async-openai`
- [ ] 4.0 Create REPL loop and message handling
  - [ ] 4.1 Prompt user until `/exit` or EOF
  - [ ] 4.2 Maintain `Vec<Message>` with `role` and `content`
  - [ ] 4.3 Stream responses as tokens arrive
- [ ] 5.0 Support conversation persistence
  - [ ] 5.1 Autoload default transcript on startup
  - [ ] 5.2 Autosave conversation after each turn
  - [ ] 5.3 Implement `--new` and update `last` symlink
  - [ ] 5.4 Implement `--load` to continue from existing file
- [ ] 6.0 Graceful exit and error handling
  - [ ] 6.1 Flush unsaved state on SIGINT/SIGTERM or `/exit`
  - [ ] 6.2 Display HTTP/JSON errors without crashing
- [ ] 7.0 Configuration options
  - [ ] 7.1 Apply `--model` flag to choose the backend model
  - [ ] 7.2 Optionally enable coloured output using `colored` crate
- [ ] 8.0 Testing
  - [ ] 8.1 Unit tests for transcript load/save and message formatting
- [ ] 9.0 Continuous integration and documentation
  - [ ] 9.1 Configure GitHub Actions to run `cargo test` and `clippy`
  - [ ] 9.2 Provide usage instructions and architecture overview in README
