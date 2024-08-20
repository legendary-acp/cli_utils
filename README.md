# CLI Utils

CLI utils is a collection of command-line utilities written in Rust, providing essential tools similar to traditional Unix commands. This project includes implementations of the following commands:
```
    echo: Repeats the input provided to it.
    cat: Concatenates and displays the contents of files.
    ls: Lists the contents of directories.
    find: Locates files or directories within a specified path.
    grep: Searches for and matches text patterns within files.
```

## Installation

To install cli_utils, ensure you have Rust installed, then clone this repository and build the project:


```bash
git clone https://github.com/yourusername/cli_utils.git
cd cli_utils
cargo build --release
```

The compiled binaries will be located in the target/release/ directory. You can move them to a directory in your PATH for easier use.
## Usage
### echo

Repeats the input string:

```bash
cli_utils echo "Hello, World!"
```

### cat

Concatenates and displays the content of files:

```bash
cli_utils cat file1.txt file2.txt
```
### ls

Lists the contents of a directory:

```bash
cli_utils ls /path/to/directory
```

### find

Locates files or directories within a specified path:

```bash
cli_utils find /path/to/search -name "filename"
```

### grep

Searches for a pattern within files:

```bash
cli_utils grep "search_term" file.txt
```

## Contributing

Contributions are welcome! Please fork this repository and submit a pull request for any features or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for details.