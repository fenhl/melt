#!/usr/bin/env python3

import sys

import datetime
import itertools
import pytz
import snowflake

EPOCHS = {
    'discord': 1420070400,
    'twitter': 1288834974.657
}

class CommandLineArgs:
    def __init__(self, args=sys.argv[1:]):
        self._epoch = EPOCHS['twitter']
        self.flakes = []
        self.format = None
        self.help = False
        self.timezone = pytz.utc
        self.parse_args(args)

    def parse_args(self, args):
        mode = None
        for arg in args:
            if mode == 'epoch':
                self.epoch = arg
                mode = None
            elif mode == 'format':
                self.format = arg
                mode = None
            elif mode == 'timezone':
                self.timezone = pytz.timezone(arg)
                mode = None
            elif arg.startswith('-'):
                if arg.startswith('--'):
                    if arg == '--epoch':
                        mode = 'epoch'
                    elif arg.startswith('--epoch='):
                        self.epoch = arg[len('--epoch='):]
                    elif arg == '--format':
                        mode = 'format'
                    elif arg.startswith('--format='):
                        self.format = arg[len('--format='):]
                    elif arg == '--help':
                        self.help = True
                    elif arg == '--timezone':
                        mode = 'timezone'
                    elif arg.startswith('--timezone='):
                        self.timezone = pytz.timezone(arg[len('--timezone='):])
                    else:
                        raise ValueError(f'Unrecognized flag: {arg}')
                else:
                    for i, short_flag in enumerate(arg):
                        if i == 0:
                            continue
                        if short_flag == 'H':
                            self.format = '%Y-%m-%d %H:%M:%S'
                        elif short_flag == 'e':
                            if len(arg) > i + 1:
                                self.epoch = arg[i + 1:]
                            else:
                                mode = 'epoch'
                            break
                        elif short_flag == 'f':
                            if len(arg) > i + 1:
                                self.format = arg[i + 1:]
                            else:
                                mode = 'format'
                            break
                        elif short_flag == 'h':
                            self.help = True
                        elif short_flag == 'z':
                            if len(arg) > i + 1:
                                self.timezone = pytz.timezone(arg[i + 1:])
                            else:
                                mode = 'timezone'
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

def format_melt(flake, epoch=EPOCHS['twitter'], format_str=None, timezone=pytz.utc):
    timestamp, data_center, worker, sequence = snowflake.melt(flake, twepoch=epoch * 1000)
    timestamp = pytz.utc.localize(datetime.datetime.fromtimestamp(timestamp / 1000)).astimezone(timezone)
    if format_str is None:
        return str(timestamp.timestamp())
    else:
        format_str = '%%'.join(
            part.replace('%^d', str(data_center)).replace('%^w', str(worker)).replace('%^s', str(sequence))
            for part in format_str.split('%%')
        )
        return f'{timestamp:{format_str}}'

if __name__ == '__main__':
    args = CommandLineArgs()
    if args.help:
        print('[ ** ] please see https://github.com/fenhl/melt/blob/master/README.md for usage instructions')
        sys.exit()
    flakes = args.flakes
    if not sys.stdin.isatty():
        flakes = itertools.chain(flakes, map(lambda line: int(line.strip()), sys.stdin))
    for flake in flakes:
        print(format_melt(flake, args.epoch, args.format, args.timezone))
