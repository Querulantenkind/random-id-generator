# Random ID Generator

A robust, flexible, and interactive Command Line Interface (CLI) tool written in Rust for generating various types of unique identifiers.

Whether you need a standard UUID v4, a URL-friendly NanoID, or a custom random string for passwords or tokens, this tool has you covered. It supports both a quick CLI-argument mode for scripts and an interactive wizard for human users.

## Features

*   **Multiple ID Types**:
    *   **UUID v4**: Standard Universally Unique Identifiers.
    *   **NanoID**: Secure, URL-friendly, unique string IDs.
    *   **Custom**: Fully customizable random strings (Numeric, Alphabetic, Hexadecimal, or custom character sets).
*   **Interactive Mode**: A user-friendly wizard guides you through the generation process if no arguments are provided.
*   **Bulk Generation**: Generate thousands of IDs at once with the `--count` flag.
*   **Clipboard Support**: Automatically copy generated IDs to your system clipboard with `--copy`.
*   **High Performance**: Built with Rust for speed and safety.

## Installation

### From Source

Ensure you have Rust and Cargo installed.

1.  Clone the repository:
    ```bash
    git clone https://github.com/Querulantenkind/random-id-generator.git
    cd random-id-generator
    ```

2.  Install globally:
    ```bash
    cargo install --path .
    ```

Now you can run `random-id-generator` from anywhere in your terminal.

## Usage

### Interactive Mode (Wizard)

Simply run the tool without any arguments to start the interactive wizard:

```bash
random-id-generator
```

You will be prompted to select the ID type, length (for custom IDs), quantity, and whether to copy the result to the clipboard.

### CLI Mode

You can also use flags and subcommands for quick, scriptable generation.

#### Generate a UUID
```bash
random-id-generator uuid
# Output: add8bb7b-f226-41e4-815e-94e9f6c51e6f
```

#### Generate a NanoID
```bash
random-id-generator nanoid
# Output: Ws7gtBdi46XjkhgwWzH1F
```

#### Generate Custom IDs

**Numeric (PIN code style):**
```bash
random-id-generator custom --numeric --length 6
# Output: 849201
```

**Hexadecimal:**
```bash
random-id-generator custom --hex --length 32
```

**Custom Character Set:**
```bash
random-id-generator custom --chars "ABC123" --length 10
```

#### Bulk Generation & Clipboard

Generate 5 UUIDs and copy them to the clipboard:

```bash
random-id-generator --count 5 --copy uuid
```

## Options & Flags

| Flag | Description | Default |
|------|-------------|---------|
| `-c`, `--count <N>` | Generate `N` IDs. | `1` |
| `--copy` | Copy output to system clipboard. | `false` |
| `-h`, `--help` | Print help information. | - |

### Custom Command Options

| Flag | Description |
|------|-------------|
| `-l`, `--length <N>` | Length of the custom string. | `16` |
| `--numeric` | Use digits 0-9 only. |
| `--alpha` | Use letters a-z, A-Z only. |
| `--hex` | Use hex characters 0-9, a-f. |
| `--chars <STR>` | Use a custom set of characters. |

## Development

To build and run the project locally:

```bash
# Run in debug mode
cargo run -- uuid

# Build release binary
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
