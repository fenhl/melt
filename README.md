# What is `melt`?

`melt` is a command-line utility for working with [snowflake identifiers](https://en.wikipedia.org/wiki/Snowflake_ID), as used by Twitter and Discord.

# What are snowflakes?

A snowflake is a number used to uniquely identify a piece of data. Twitter uses it for tweets, e.g. the `1212702693736767490` in <https://twitter.com/qntm/status/1212702693736767490> is a snowflake. Discord uses it for pretty much everything: users, servers, channels, messages, roles… You can access these snowflakes by going into your Advanced settings, enabling Developer Mode, then right-clicking on something to select Copy ID. Originally specified in [this document](https://github.com/twitter-archive/snowflake/blob/b3f6a3c6ca8e1b6847baa6ff42bf72201e2c2231/README.mkd), they have been used by Twitter since 2010 and by Discord since the beginning. Instagram uses a format that's based on snowflakes but slightly different, and is not supported by `melt`.

A snowflake consists of 4 pieces of information: Timestamp, data center ID, worker ID, and sequence number. The interesting one is the timestamp: a representation of the date and time when the snowflake was generated, exact to the millisecond. This means that the time it was created (and thus the approximate time the tweet was sent, the Discord server was created, etc.) can be calculated from the snowflake itself. This is called melting the snowflake.

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
    cargo install --git=https://github.com/fenhl/melt --branch=main
    ```

# Usage

By default, `melt` takes Twitter snowflakes from stdin, one per line, and converts them into UNIX timestamps with milliseconds. Snowflakes can also be passed as command-line arguments.

```sh
$ echo 1212702693736767490 | melt
1577965827.770
```

The `--format` (or `-f`) flag can be used to modify the output format, as [defined by `chrono`](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html). Additionally, the formatting directives `%^d`, `%^w`, and `%^s` may be used to include the data center ID, worker ID, or sequence number, respectively. The flag `-H` is a shorthand and specifies a format of `%Y-%m-%d %H:%M:%S`.

```sh
$ echo 1212702693736767490 | melt -f '%d.%m.%Y %H:%M:%S'
02.01.2020 11:50:27
```

The `--timezone` (or `-z`) flag can be used to change the timezone in which formatted times are displayed. The timezone must be given as a name from the Olson timezone database. By default, UTC is used. This flag has no effect on UNIX timestamps (i.e. when none of `-f`, `--format`, `-H` are specified), which are always in UTC.

```sh
$ echo 1212702693736767490 | melt -f '%d.%m.%Y %H:%M:%S' -z Europe/Berlin
02.01.2020 12:50:27
```

The `--epoch` (or `-e`) flag can be used to change the epoch from Twitter's default to Discord's. A different epoch, given as a UNIX timestamp, may also be specified.

```sh
$ echo 86841168427495424 | melt -e discord
1309539522.641
```
