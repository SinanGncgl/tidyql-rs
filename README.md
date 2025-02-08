# tidyql-rs

tidyql-rs is a terminal-based application for browsing and formatting SQL files. It provides a user interface to navigate through files, view their content, and format SQL files with syntax highlighting.

![tidyql-rs in action](assets/tidyql.gif)

## Features

- Browse files in the current directory
- View file content with syntax highlighting
- Format SQL files with customizable options
- Save formatted SQL files
- Notifications for actions

## Installation

To install and run tidyql-rs, follow these steps:

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/tidyql-rs.git
    cd tidyql-rs
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

3. Run the application:

    ```sh
    cargo run --release
    ```

## Usage

### Key Bindings

- `q`: Quit the application
- `Down Arrow`: Move down in the file list
- `Up Arrow`: Move up in the file list
- `f`: Format the selected SQL file
- `s`: Save the formatted SQL file

### User Interface

The user interface is divided into three main sections:

1. **File Browser**: Located on the left side, this section displays the list of files in the current directory. Use the arrow keys to navigate through the files.
2. **File Content**: Located on the right side, this section displays the content of the selected file with syntax highlighting. If the file is an SQL file and has been formatted, the formatted content will be displayed.
3. **Commands**: Located at the bottom, this section displays the available commands.

### Notifications

Notifications are displayed at the top right corner of the screen to inform you about the status of actions such as formatting and saving files.

## Dependencies

- [ratatui](https://crates.io/crates/ratatui): A Rust library for building terminal user interfaces
- [syntect](https://crates.io/crates/syntect): A Rust library for syntax highlighting

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
