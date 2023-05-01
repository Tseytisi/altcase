# AltCase

> Tseytisi, April 2023

Simple tool for converting text to alternating case. aLLoWiNg YoU tO EaSiLy wRiTe LiKe tHiS.

Works as both a command-line program, and as GUI application.

Has a mode that ensures all occurrences of 'i' are lowercase, and all occurrences of 'L' are uppercase, to avoid confusion.

### How to install
1. Clone the repository 
```sh
git clone https://github.com/Tseytisi/altcase
```
2. Ensure the Rustup package is installed, and that the proper toolchain is configured
```sh
rustup default stable
```
3. Inside the cloned repository, in the same folder where the `Cargo.toml` file is located, build the application
```sh
cargo build --release
```
4. The compiled binary should now be located inside the folder at `target/release/altcase`

### How to run
To get a help and usage screen from the application, type
```sh 
altcase --help
```

By default, the application runs in command-line mode, meaning no GUI will be shown or loaded.

To launch the application with GUI, run
```sh 
altcase --gui
```

### Planned features
- Allow input through pipe
- Split input on newlines so each line gets the best mapping (in normal conversion mode)
- Start with capital or non-capital letter in simple mode depending on the first character
- GUI: Copied-to-clipboard notifier