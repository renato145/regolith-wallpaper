# regolith-wallpaper
Set wallpapers on regolith3

## Installation

### Linker (faster compilation)

#### On Windows
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

#### On Linux
```bash
# Ubuntu
sudo apt-get install lld clang
# Arch
sudo pacman -S lld clang
```

#### On MacOS
```bash
brew install michaeleisel/zld/zld
```
