`melt` is a command-line utility for working with [Twitter snowflake](https://github.com/twitter/snowflake) identifiers. It has built-in support for both Twitter's and Discord's epochs, and also allows specifying custom epochs to work with other implementations.

# Usage

```sh
$ echo 86841168427495424 | melt -e discord
1309539522.641
```
