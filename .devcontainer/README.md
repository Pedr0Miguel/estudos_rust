# Devcontainer Setup for Rust Development

This devcontainer provides a complete Rust development environment with all necessary tools and VS Code extensions pre-configured.

## Recent Updates ✨

**Version Compatibility Fix**: Updated from Rust 1.75 to Rust 1.82 to resolve compatibility issues with modern cargo tools that require newer Rust versions or edition2024 features.

**Improved Tool Installation**: Added a robust installation script (`install-tools.sh`) with fallback versions for tools that may fail with the latest versions.

## What's Included

### Rust Toolchain
- Rust 1.82 (updated from 1.75 for better compatibility)
- Clippy (linting)
- Rustfmt (formatting)
- Rust-analyzer (language server)
- Rust source code for stdlib browsing

### Development Tools
- `cargo-watch` - automatic rebuilding on file changes
- `cargo-edit` - easy dependency management (with fallback to v0.12.3)
- `cargo-tree` - dependency tree visualization
- `cargo-expand` - macro expansion (with fallback to v1.0.95)
- `cargo-audit` - security vulnerability scanning (with fallback to v0.21.1)
- `cargo-outdated` - outdated dependency checking (with fallback to v0.15.0)
- `bacon` - background Rust code checker (with fallback to v2.18.2)

### VS Code Extensions
- **rust-analyzer** - Advanced Rust language support
- **Even Better TOML** - Enhanced TOML file support
- **crates** - Cargo.toml dependency management
- **CodeLLDB** - Debugging support
- **Test Explorer** - Test management and running
- **Hex Editor** - Binary file viewing
- **GitHub Copilot** - AI code assistance (if you have access)

### System Tools
- Git and GitHub CLI
- Build essentials (gcc, pkg-config, etc.)
- Debugging tools (lldb, gdb, valgrind, strace)
- Utilities (htop, tree, jq, vim, nano)

## Getting Started

### Prerequisites
- [Docker](https://docs.docker.com/get-docker/) installed on your machine
- [VS Code](https://code.visualstudio.com/) with the [Remote-Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Opening the Project

1. Open VS Code
2. Open the project folder (`estudos_rust`)
3. When prompted, click "Reopen in Container" or use the command palette:
   - Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
   - Type "Remote-Containers: Reopen in Container"
   - Press Enter

The first time you open the container, it will take a few minutes to build and install all components.

### Running Rust Code

Once the container is running, you can use the integrated terminal to run Rust commands:

```bash
# Navigate to any of the example projects
cd learn-to-code-with-rust/variables-and-mutability

# Run the project
cargo run

# Run with automatic rebuilding on changes
cargo watch -x run

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Debugging

The devcontainer includes LLDB debugger support. To debug your Rust code:

1. Set breakpoints in your code by clicking in the gutter
2. Press `F5` or go to Run → Start Debugging
3. Choose "LLDB" when prompted for debugger type

### Useful Cargo Commands

The devcontainer includes several useful cargo extensions:

```bash
# Add a new dependency
cargo add serde

# Remove a dependency
cargo rm serde

# Show dependency tree
cargo tree

# Check for security vulnerabilities
cargo audit

# Check for outdated dependencies
cargo outdated

# Find unused dependencies
cargo udeps

# Expand macros (useful for learning)
cargo expand
```

### Port Forwarding

The devcontainer automatically forwards these ports:
- **3000** - Common development server port
- **8000** - HTTP server port
- **8080** - Web server port

If your Rust application runs on these ports, they'll be accessible from your host machine.

### Customization

You can customize the devcontainer by editing:
- `.devcontainer/devcontainer.json` - Container configuration and VS Code settings
- `.devcontainer/Dockerfile` - Container image and installed packages

After making changes, rebuild the container:
1. Command palette (`Ctrl+Shift+P`)
2. "Remote-Containers: Rebuild Container"

### Configuration Files

The devcontainer includes several configuration options:

- **`devcontainer.json`** - Full configuration with all tools
- **`devcontainer.minimal.json`** - Minimal setup with only essential tools
- **`install-tools.sh`** - Smart installer script with fallback versions

To use the minimal configuration, rename `devcontainer.minimal.json` to `devcontainer.json`.

### Troubleshooting

**Container build fails with version errors:**
- This has been fixed in the latest version by upgrading to Rust 1.82
- If you still encounter issues, try using the minimal configuration

**Tool installation fails:**
- The installation script now includes fallback versions for problematic tools
- Check the container logs to see which tools succeeded/failed
- You can manually install failed tools later with specific versions

**Container takes too long to build:**
- The first build downloads and compiles many tools. Subsequent builds use Docker cache and are much faster.

**Permission issues:**
- The container runs as the `vscode` user with sudo access. If you encounter permission issues, try prefixing commands with `sudo`.

**Rust-analyzer not working:**
- Wait for the language server to initialize (you'll see progress in the status bar)
- Try reloading VS Code window: `Ctrl+Shift+P` → "Developer: Reload Window"

**Memory issues:**
- Rust compilation can be memory-intensive. Ensure Docker has sufficient memory allocated (8GB+ recommended for larger projects)

**Previous DevContainer Errors:**
- If you were getting errors about "edition2024" or version requirements, these should now be resolved
- Rebuild your container: `Ctrl+Shift+P` → "Remote-Containers: Rebuild Container"

### Learning Resources

This devcontainer is perfect for working through Rust tutorials and examples. Some recommended starting points:

1. Work through the projects in `learn-to-code-with-rust/`
2. Check out the [Rust Book](https://doc.rust-lang.org/book/)
3. Try [Rustlings exercises](https://github.com/rust-lang/rustlings)
4. Explore [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

Happy coding! 🦀