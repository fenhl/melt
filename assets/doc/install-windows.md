1. Press <kbd>Windows</kbd><kbd>I</kbd> open the Settings app
2. Navigate to System → About
3. In the “Device specifications” section, check what the second half of the “System type” says, and download the appropriate file:

    * [ARM-based processor](https://github.com/fenhl/melt/releases/latest/download/melt-arm.exe)
    * [x86-based processor](https://github.com/fenhl/melt/releases/latest/download/melt-x86.exe)
    * [x64-based processor](https://github.com/fenhl/melt/releases/latest/download/melt-x64.exe)

    If your system type is not listed here, or if you would like to manage updates of `melt` using [`cargo-update`](https://crates.io/crates/cargo-update), follow [the instructions for building from source](https://github.com/fenhl/melt/blob/main/assets/doc/build.md).
4. Place the downloaded file into a folder where you keep your command-line programs (or create such a folder at a location of your choice), then rename the downloaded file to `melt.exe`
5. Click on the empty part of the path bar near the top of the File Explorer window, and copy the path to your clipboard
6. Press <kbd>Windows</kbd><kbd>R</kbd>, enter `SystemPropertiesAdvanced`, and click OK
7. In the System Properties window that opens, click “Environment Variables…”
8. In the “User variables for (your username)” section, find the `Path` variable, and click Edit
9. Click “New” and paste the path copied in step 5
10. Save your changes by clicking OK in all 3 System Properties windows
11. You can now use `melt` inside a command line, which can be opened by right-clicking the start button, then clicking “Terminal”, “Windows PowerShell”, or “Command Prompt”. See [the readme](https://github.com/fenhl/melt#usage) for usage instructions.
