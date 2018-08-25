`melt` is a command-line utility for working with [Twitter snowflake](https://github.com/twitter/snowflake/blob/b3f6a3c6ca8e1b6847baa6ff42bf72201e2c2231/README.mkd) identifiers. It has built-in support for both Twitter's and Discord's epochs, and also allows specifying custom epochs to work with other implementations.

# Usage

By default, `melt` takes snowflakes from stdin, one per line, and converts them into UNIX timestamps with milliseconds. Snowflakes can also be passed as command-line arguments.

```sh
$ echo 844882040566702080 | melt
1490270550.277
```

The `--epoch` (or `-e`) flag can be used to change the epoch from Twitter's default to Discord's. A different epoch, given as a UNIX timestamp, may also be specified.

```sh
$ echo 86841168427495424 | melt -e discord
1309539522.641
```
