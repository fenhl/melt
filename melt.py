#!/usr/bin/env python3

import sys

import datetime
import itertools
import snowflake

EPOCHS = {
    'discord': 1420070400,
    'twitter': 1288834974.657
}

class CommandLineArgs:
    def __init__(self, args=sys.argv[1:]):
        self._epoch = EPOCHS['twitter']
        self.flakes = []
        self.parse_args(args)

    def parse_args(self, args):
        mode = None
        for arg in args:
            if mode == 'epoch':
                self.epoch = arg
                mode = None
            elif arg.startswith('-'):
                if arg.startswith('--'):
                    if arg == '--epoch':
                        mode = 'epoch'
                    elif arg.startswith('--epoch='):
                        self.epoch = arg[len('--epoch='):]
                    else:
                        raise ValueError(f'Unrecognized flag: {arg}')
                else:
                    for i, short_flag in enumerate(arg):
                        if i == 0:
                            continue
                        if short_flag == 'e':
                            if len(arg) > i + 1:
                                self.epoch = arg[i + 1:]
                            else:
                                mode = 'epoch'
                            break
                        else:
                            raise ValueError(f'Unrecognized flag: -{short_flag}')
            else:
                self.flakes.append(int(arg))

    @property
    def epoch(self):
        return self._epoch

    @epoch.setter
    def epoch(self, value):
        if isinstance(value, str) and value.lower() in EPOCHS:
            self._epoch = EPOCHS[value.lower()]
        else:
            self._epoch = float(value)

if __name__ == '__main__':
    args = CommandLineArgs()
    flakes = args.flakes
    if not sys.stdin.isatty():
        flakes = itertools.chain(flakes, map(lambda line: int(line.strip()), sys.stdin))
    for flake in flakes:
        timestamp, data_center, worker, sequence = snowflake.melt(flake, twepoch=args.epoch * 1000)
        timestamp = datetime.datetime.fromtimestamp(timestamp / 1000)
        print(timestamp.timestamp())
