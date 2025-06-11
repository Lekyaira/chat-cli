# Chat CLI

Chat CLI is a Rust command-line tool for interacting with a local large language model served by [Ollama](https://github.com/jmorganca/ollama). The project aims to offer a small, offline REPL experience similar to ChatGPT.

## Status

The project is in early development. The CLI accepts a few arguments and now defines a simple backend abstraction via a `ChatBackend` trait (see `src/chat_backend.rs`):

- `--new <FILE>` start a new conversation log.
- `--load <FILE>` load an existing log.
- `--model <NAME>` choose the model to use (default `mistral`).

Future tasks will add the actual chat backend, streaming responses and transcript persistence.

## Building

Install Rust 1.87+ and run:

```bash
cargo build
```

## Usage

Run the CLI with cargo:

```bash
cargo run -- [OPTIONS]
```

Example:

```bash
cargo run -- --new mylog.jsonl --model mistral
```

Use `cargo run -- --help` to see all available options.

## Development

Ongoing work and planned features are tracked in [tasks/tasks-prd-local-chat-cli.md](tasks/tasks-prd-local-chat-cli.md). This README will evolve as the project grows.
