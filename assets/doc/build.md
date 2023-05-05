# Building from source

If [pre-built binaries](https://github.com/fenhl/melt#installation) are not available for your system, or if you would like to manage updates of `melt` using [`cargo-update`](https://crates.io/crates/cargo-update), follow these instructions:

1. (Skip this step if you're not on Windows.) If you're on Windows, you'll first need to download and install [Visual Studio](https://visualstudio.microsoft.com/vs/) (the Community edition should work). On the “Workloads” screen of the installer, make sure “Desktop development with C++” is selected. (Note that [Visual Studio Code](https://code.visualstudio.com/) is not the same thing as Visual Studio. You need VS, not VS Code.)
2. Install Rust:
    * On Windows, download and run [rustup-init.exe](https://win.rustup.rs/) and follow its instructions.
    * On other platforms, please see [the Rust website](https://www.rust-lang.org/tools/install) for instructions.
3. Open a command line:
    * On Windows, right-click the start button, then click “Terminal”, “Windows PowerShell”, or “Command Prompt”.
    * On other platforms, look for an app named “Terminal” or similar.
4. In the command line, run the following command. Depending on your computer, this may take a while.

    ```
    cargo install --git=https://github.com/fenhl/melt --branch=main
    ```
