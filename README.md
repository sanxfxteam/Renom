# Renom

Renom is a simple program that allows you to rename your Unreal Engine projects.
It handles both Blueprint-only and C++ projects, in accordance with the
guidelines set forth
[here](https://unrealistic.dev/posts/rename-your-project-including-code).

```shell
> renom wizard

[ Welcome to Renom ]
> Choose a workflow: Rename a project
> Project root directory path: LyraStarterGame
> Provide a new name for the project: SpyroStarterGame
( apply ) set [URL] GameName = SpyroStarterGame in config file LyraStarterGame\Config/DefaultEngine.ini
( apply ) set [/Script/EngineSettings.GeneralProjectSettings] ProjectName = SpyroStarterGame in config file LyraStarterGame\Config/DefaultGame.ini
( apply ) rename file LyraStarterGame\LyraStarterGame.uproject to LyraStarterGame\SpyroStarterGame.uproject
( apply ) rename file LyraStarterGame to SpyroStarterGame

        [ Success ]
        Successfully renamed project LyraStarterGame to SpyroStarterGame.
```

Among other things, Renom:

- Provides workflows to rename projects, plugins, targets, and modules
- Detects project name, targets, modules, and other metadata
- Updates target, build, config, and source files
- Creates backups of all affected files to prevent data loss
- Supports consecutive renames

## Installation

You can install Renom either by downloading the binary release or by using
the Cargo package manager.

### Binary

Simply download the latest release
[here](https://github.com/UnrealisticDev/Renom/releases) and put the executable
(_.exe_) on your system PATH.

### Cargo

Renom is written in Rust, and Cargo is the package manager for Rust. Install the
Rust toolchain, which includes Cargo, by following the instructions
[here](https://www.rust-lang.org/tools/install). Once Cargo is installed, run
the following command to install Renom:

```shell
cargo install renom
```

This will pull and build Renom directly from
[crates.io](https://crates.io/crates/renom). If the build is successful, you
should be able to find the installed executable at
_C:/Users/{User}/.cargo/bin/renom.exe_.

### Building from Source

If you want to build Renom from source (for example, to try the latest features
or contribute to development), follow these steps:

#### 1. Install the Rust Toolchain

First, install Rust using `rustup`, the official Rust installer and version manager:

**Windows:**
- Download and run [rustup-init.exe](https://rustup.rs/)
- Follow the on-screen instructions (the default options work for most users)

**macOS and Linux:**
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal or run:
```shell
source $HOME/.cargo/env
```

Verify the installation:
```shell
rustc --version
cargo --version
```

#### 2. Clone the Repository

```shell
git clone https://github.com/UnrealisticDev/Renom.git
cd Renom
```

#### 3. Build the Project

Build Renom in release mode (optimized):
```shell
cargo build --release
```

The compilation may take a few minutes on the first run as it downloads and
compiles dependencies.

#### 4. Locate the Binary

After a successful build, the executable will be located at:
- **Windows:** `target/release/renom.exe`
- **macOS/Linux:** `target/release/renom`

#### 5. Run or Install

You can run Renom directly:
```shell
./target/release/renom
```

Or copy it to a directory in your PATH for system-wide access:
```shell
# Linux/macOS example
sudo cp target/release/renom /usr/local/bin/

# Windows example (run as Administrator in PowerShell)
# Copy-Item target\release\renom.exe C:\Windows\System32\
```

Alternatively, install it with Cargo (builds and installs from the current directory):
```shell
cargo install --path .
```

## Usage

Run the following command to see available options:

```shell
renom
```

To start an interactive session, run the following command instead:

```shell
renom wizard
```
