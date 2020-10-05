`melt` is a command-line utility for working with [Twitter snowflake](https://github.com/twitter/snowflake/blob/b3f6a3c6ca8e1b6847baa6ff42bf72201e2c2231/README.mkd) identifiers. It has built-in support for both Twitter's and Discord's epochs, and also allows specifying custom epochs to work with other implementations.

# Installation

1. Install Rust:
    * On Windows, download and run [rustup-init.exe](https://win.rustup.rs/) and follow its instructions.
    * On other platforms, please see [the Rust website](https://www.rust-lang.org/learn/get-started) for instructions.
2. (Skip this step if you're not on Windows.) If you're on Windows, you'll also need to download and install [Visual Studio](https://visualstudio.microsoft.com/vs/) (the Community edition should work). On the “Workloads” screen of the installer, make sure “Desktop development with C++” is selected. (Note that [Visual Studio Code](https://code.visualstudio.com/) is not the same thing as Visual Studio. You need VS, not VS Code.)
3. Open a command line:
    * On Windows, right-click the start button, then click “Windows PowerShell” or “Command Prompt”.
    * On other platforms, look for an app named “Terminal” or similar.
4. In the command line, run the following command. Depending on your computer, this may take a while.

    ```
    cargo install --git=https://github.com/fenhl/melt
    ```

# Usage

By default, `melt` takes snowflakes from stdin, one per line, and converts them into UNIX timestamps with milliseconds. Snowflakes can also be passed as command-line arguments.

```sh
$ echo 844882040566702080 | melt
1490270550.277
```

The `--format` (or `-f`) flag can be used to modify the output format, as [defined by `chrono`](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html). Additionally, the formatting directives `%^d`, `%^w`, and `%^s` may be used to include the data center ID, worker ID, or sequence number, respectively. The flag `-H` is a shorthand and specifies a format of `%Y-%m-%d %H:%M:%S`.

```sh
$ echo 844882040566702080 | melt -f '%d.%m.%Y %H:%M:%S'
23.03.2017 12:02:30
```

The `--timezone` (or `-z`) flag can be used to change the timezone in which formatted times are displayed. The timezone must be given as a name from the Olson timezone database. By default, UTC is used. This flag has no effect on UNIX timestamps (i.e. when none of `-f`, `--format`, `-H` are specified), which are always in UTC.

```sh
$ echo 844882040566702080 | melt -f '%d.%m.%Y %H:%M:%S' -z Europe/Berlin
23.03.2017 13:02:30
```

The `--epoch` (or `-e`) flag can be used to change the epoch from Twitter's default to Discord's. A different epoch, given as a UNIX timestamp, may also be specified.

```sh
$ echo 86841168427495424 | melt -e discord
1309539522.641
```
