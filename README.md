# File Explorer

A file system explorer built with Rust and the Iced GUI framework. Inspired by a video on creating file explorers in Rust, I decided to build one myself.

## Features

- Search by extension (e.g., ".txt").
- Search for files by their full name and extension, returning the file path.
- File navigation displaying directories and their paths.

## How it Works

The program scans your file system and adds every file to a hashmap (folder functionality is not yet implemented). The hashmap is cached, so the first run with `cargo run` may take some time (approximately 10 to 15 minutes on a slow laptop with around 300 GB of data). The resulting file size is around 200 MB. Subsequent searches are very fast, finding files in microseconds (compared to Windows' 15 minutes).

## TODO

There are many more features I want to add:
- Improved UI
- Ability to open files
- Ability to search for folders
- Make the entire process asynchronous (I'm new to Rust and haven't tackled this yet)
- Improve render time (searching by extension is very fast, but displaying results with Iced takes around 10 seconds, which might be due to Iced or my implementation)
// - Load multiple drives (currently hardcoded to load the C drive; to change this, update the `update_cache` function and modify the line `for entry in WalkDir::new(r"C:\").into_iter().filter_map(|e| e.ok())` to match your drive)


## Installation

```bash
git clone https://github.com/yourusername/file-explorer
cd file-explorer
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
