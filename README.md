# What is `melt`?

`melt` is a command-line utility for working with [snowflake identifiers](https://en.wikipedia.org/wiki/Snowflake_ID), as used by Twitter and Discord.

# What are snowflakes?

A snowflake is a number used to uniquely identify a piece of data. Twitter uses it for tweets, e.g. the `1212702693736767490` in <https://twitter.com/qntm/status/1212702693736767490> is a snowflake. Discord uses it for pretty much everything: users, servers, channels, messages, roles… You can access these snowflakes by going into your Advanced settings, enabling Developer Mode, then right-clicking on something to select Copy ID. Originally specified in [this document](https://github.com/twitter-archive/snowflake/blob/b3f6a3c6ca8e1b6847baa6ff42bf72201e2c2231/README.mkd), they have been used by Twitter since 2010 and by Discord since the beginning. Instagram uses a format that's based on snowflakes but slightly different, and is not supported by `melt`.

A snowflake consists of 4 pieces of information: Timestamp, data center ID, worker ID, and sequence number. The interesting one is the timestamp: a representation of the date and time when the snowflake was generated, exact to the millisecond. This means that the time it was created (and thus the approximate time the tweet was sent, the Discord server was created, etc.) can be calculated from the snowflake itself. This is called melting the snowflake.

# Installation

Please see the install instructions for your operating system:

* [Windows](https://github.com/fenhl/melt/blob/main/assets/doc/install-windows.md)
* [NixOS](https://github.com/fenhl/melt/blob/main/assets/doc/install-nixos.md)

If your operating system is not listed here, or if you would like to manage updates of `melt` using [`cargo-update`](https://crates.io/crates/cargo-update), follow [the instructions for building from source](https://github.com/fenhl/melt/blob/main/assets/doc/build.md).

## Tab completion

**Bash**
```bash
echo "source <(COMPLETE=bash melt)" >> ~/.bashrc
```

**Elvish**
```elvish
echo "eval (E:COMPLETE=elvish melt | slurp)" >> ~/.elvish/rc.elv
```

**Fish**
```fish
echo "COMPLETE=fish melt | source" >> ~/.config/fish/config.fish
```

**Powershell**
```powershell
echo '$env:COMPLETE = "powershell"; melt | Out-String | Invoke-Expression; Remove-Item Env:\COMPLETE' >> $PROFILE
```
Note that to execute scripts in PowerShell on Windows, including [`$PROFILE`][$Profile],
the [execution policy][ExecutionPolicies] needs to be set to `RemoteSigned` at minimum.

[$Profile]: https://learn.microsoft.com/en-us/powershell/scripting/learn/shell/creating-profiles
[ExecutionPolicies]: https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies

**Zsh**
```zsh
echo "source <(COMPLETE=zsh melt)" >> ~/.zshrc
```

# Usage

By default, `melt` takes Twitter snowflakes from stdin, one per line, and converts them into UNIX timestamps with milliseconds:

```sh
$ echo 1212702693736767490 | melt
1577965827.770
```

Snowflakes can also be passed as command-line arguments:

```sh
$ melt 1212702693736767490
1577965827.770
```

The `--format` (or `-f`) flag can be used to modify the output format, as [defined by `chrono`](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html). Additionally, the formatting directives `%^d`, `%^w`, and `%^s` may be used to include the data center ID, worker ID, or sequence number, respectively.

```sh
$ echo 1212702693736767490 | melt -f '%d.%m.%Y %H:%M:%S'
02.01.2020 11:50:27
```

The flag `-H` is a shorthand and specifies a format of `%Y-%m-%d %H:%M:%S`:

```sh
$ echo 1212702693736767490 | melt -H
2020-01-02 11:50:27
```

Similarly, the flag `-T` is a shorthand and specifies a format of `%+` (the ISO 8601 / RFC 3339 date & time format):

```sh
$ echo 1212702693736767490 | melt -T
2020-01-02T11:50:27.770+00:00
```

The `--timezone` (or `-z`) flag can be used to change the timezone in which formatted times are displayed. The timezone must be given as a name from [the Olson timezone database](https://en.wikipedia.org/wiki/Tz_database). By default, UTC is used. This flag has no effect on UNIX timestamps (i.e. when none of `-f`, `--format`, `-H` are specified), which are always in UTC.

```sh
$ echo 1212702693736767490 | melt -f '%d.%m.%Y %H:%M:%S' -z Europe/Berlin
02.01.2020 12:50:27
```

The `--epoch` (or `-e`) flag can be used to change the epoch from Twitter's default to Discord's:

```sh
$ echo 86841168427495424 | melt -e discord
1309539522.641
```

A different epoch, given as a UNIX timestamp, may also be specified:

```sh
$ echo 0 | melt -e 0
0
```

The `--discord-format` (or `-d`) flag can be used to format the times using [Discord's timestamp syntax](https://discord.com/developers/docs/reference#message-formatting), which can be pasted into a Discord message to get human-readable timestamps. Possible values are `d` (or  `short-date`), `D` (or `long-date`), `t` (or `short-time`), `T` (or `long-time`), `f` (or `short-date-time`), `F` (or `long-date-time`), and `R` (or `relative`). Using this flag implies `--epoch=discord` and disables the `--timezone` parameter, since Discord will display the times in the system timezone of the viewer.

```sh
$ echo 86841168427495424 | melt -dF
<t:1440774947:F>
```

(When sent as a message, my Discord app displays the above as `Friday, 28 August 2015 15:15`. The exact format may vary depending on your locale.)
