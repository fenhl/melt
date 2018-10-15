`melt` is a command-line utility for working with [Twitter snowflake](https://github.com/twitter/snowflake/blob/b3f6a3c6ca8e1b6847baa6ff42bf72201e2c2231/README.mkd) identifiers. It has built-in support for both Twitter's and Discord's epochs, and also allows specifying custom epochs to work with other implementations.

# Dependencies

* Python 3.6
* [snowflake.py](http://github.com/fenhl/python-snowflake)

# Usage

By default, `melt` takes snowflakes from stdin, one per line, and converts them into UNIX timestamps with milliseconds. Snowflakes can also be passed as command-line arguments.

```sh
$ echo 844882040566702080 | melt
1490270550.277
```

The `--format` (or `-f`) flag can be used to modify the output format, as defined in [Python's `datetime` module](https://docs.python.org/3/library/datetime.html#strftime-and-strptime-behavior). Additionally, the formatting directives `%^d`, `%^w`, and `%^s` may be used to include the data center ID, worker ID, or sequence number, respectively. The flag `-H` is a shorthand and specifies a format of `%Y-%m-%d %H:%M:%S`.

```sh
$ echo 844882040566702080 | melt -f '%d.%m.%Y %H:%M:%S'
23.03.2017 12:02:30
```

The `--epoch` (or `-e`) flag can be used to change the epoch from Twitter's default to Discord's. A different epoch, given as a UNIX timestamp, may also be specified.

```sh
$ echo 86841168427495424 | melt -e discord
1309539522.641
```
