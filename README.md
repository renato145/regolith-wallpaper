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

## Usage

### Nvidia hybrid graphics

When using nvidia hybrid graphics you may need to run the program with the env
variable `__NV_PRIME_RENDER_OFFLOAD=1` to be able to se images.
