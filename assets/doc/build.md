# Building from source

If [pre-built binaries](https://github.com/fenhl/melt#installation) are not available for your system, or if you would like to manage updates of `melt` using [`cargo-update`](https://crates.io/crates/cargo-update), follow these instructions:

1. Install Rust:
    * On Windows, download and run [rustup-init.exe](https://win.rustup.rs/) and follow its instructions. If asked to install Visual C++ prerequisites, use the “Quick install via the Visual Studio Community installer” option. You can uncheck the option to launch Visual Studio when done.
    * On other platforms, please see [the Rust website](https://www.rust-lang.org/tools/install) for instructions.
2. Open a command line:
    * On Windows, right-click the start button, then click “Terminal”, “Windows PowerShell”, or “Command Prompt”.
    * On other platforms, look for an app named “Terminal” or similar.
3. In the command line, run the following command. Depending on your computer, this may take a while.

    ```
    cargo install --git=https://github.com/fenhl/melt --branch=main
    ```
