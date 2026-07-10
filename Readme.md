# png2ascii

A small command-line tool written in Rust that renders images directly in your terminal using colored block characters, giving a quick "ASCII art" style preview of any PNG (or other image) without leaving the shell.

## How it works

`png2ascii` loads an image, downsamples it into blocks according to a given ratio, averages the color of each block, and prints a `██` character colored with that averaged RGB value using truecolor terminal escape codes. The result is a blocky, colorized rendition of the image in your terminal.

## Requirements

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (edition 2024 or later toolchain)
- A terminal that supports truecolor (24-bit) ANSI escape sequences for best results

## Installation

Clone the repository and build the release binary with Cargo:

```sh
git clone <repository-url>
cd png2ascii
cargo build --release
```

The compiled binary will be available at `target/release/png2ascii`.

You can also build and run in one step during development:

```sh
cargo run -- --path path/to/image.png
```

## Usage

```sh
png2ascii --path <PATH> [--ratio <RATIO>]
png2ascii --interactive
```

### Options

| Flag              | Short | Description                                                                 | Default |
|-------------------|-------|-------------------------------------------------------------------------------|---------|
| `--path <PATH>`   | `-p`  | Path to the image file to render                                              | —       |
| `--ratio <RATIO>` | `-r`  | Downsampling ratio; higher values shrink the rendered output (pixels per block) | `2`     |
| `--interactive`   | `-i`  | Prompt for the image path and ratio interactively instead of using flags      | `false` |

If `--path` is not provided and `--interactive` is not set, the program prints usage guidance and exits.

### Examples

Render an image with the default ratio:

```sh
png2ascii --path assets/logo.png
```

Render an image with a coarser ratio (smaller output, faster rendering):

```sh
png2ascii --path assets/logo.png --ratio 4
```

Run interactively and be prompted for the path and ratio:

```sh
png2ascii --interactive
```

## Supported image formats

Image decoding is powered by the [`image`](https://crates.io/crates/image) crate, so any format it supports (PNG, JPEG, BMP, GIF, and more) should work, not just PNG.

## Project structure

```
src/
├── main.rs                    # Entry point; wires everything together
├── args.rs                    # CLI argument definitions (clap)
├── image_utils.rs             # Image loading/decoding
├── rendering_utils.rs         # Block-averaging and terminal rendering
└── user_interaction_utils.rs  # CLI/interactive input handling
```

## Dependencies

- [`clap`](https://crates.io/crates/clap) — command-line argument parsing
- [`colored`](https://crates.io/crates/colored) — terminal truecolor output
- [`image`](https://crates.io/crates/image) — image decoding

## License

No license has been specified for this project yet.
