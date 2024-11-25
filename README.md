# AutoUml

AutoUml is a CLI tool written in Rust that automatically replaces all known possible HTML umlauts in a provided HTML file or in all HTML files in a provided directory.

## Usage

To use AutoUml, you need to have Rust installed on your system. You can install Rust from [here](https://www.rust-lang.org/tools/install).

### Running the CLI Tool

To run the CLI tool, use the following command:

```sh
cargo run -- <file_or_directory>
```

Replace `<file_or_directory>` with the path to the HTML file or directory containing HTML files that you want to process.

### Example

To replace HTML umlauts in a single file:

```sh
cargo run -- path/to/file.html
```

To replace HTML umlauts in all HTML files within a directory:

```sh
cargo run -- path/to/directory
```

The tool will recursively process all HTML files in the provided directory and its subdirectories, replacing known HTML umlauts with their corresponding HTML entities.

## Known HTML Umlauts

The following HTML umlauts are replaced by the tool:

- `ä` -> `&auml;`
- `ö` -> `&ouml;`
- `ü` -> `&uuml;`
- `Ä` -> `&Auml;`
- `Ö` -> `&Ouml;`
- `Ü` -> `&Uuml;`
- `ß` -> `&szlig;`

## License

This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.
