# Game of Life Screensaver

This repository contains a screensaver based on the popular cellular automaton, Conway's Game of Life, implemented in Rust using the Raylib game development library.

## Description

The screensaver displays an implementation of Game of Life, which is a cellular automaton devised by the British mathematician John Horton Conway in 1970. 

The game is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. The visualizations can result in interesting patterns and dynamics within the grid.

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) and the [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) package manager if you haven't already.

2. Clone this repository:
    ```
    git clone https://github.com/GKaszewski/screensaver.git
    ```
3. Build the screensaver:
    ```
    cd screensaver
    cargo build --release
    ```

4. Run the screensaver:
    ```
    cargo run
    ```

5. Replace the extension of the executable with `.scr` and copy it to the `C:\Windows\System32` directory.

## Usage

Simply run the screensaver with `cargo run`. It will create a window and start displaying the Game of Life.

Any user activity (mouse) will exit the screensaver.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under [MIT License](LICENSE).