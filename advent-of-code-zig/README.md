# Advent of code, but in Zig

## Building & Running

```sh
$ cd /path/to/challenges/advent-of-code-zig/
$ zig build
$ zig build test
$ zig build run
```

## Adding cases

For each year, it is required to declare a new module for each day. This will be done in `src/YEAR/main.zig`:

```zig
const day01 = @import("./day01.zig").main;
const day02 = @import("./day02.zig").main;
const day03 = @import("./day03.zig").main;

...

const DAYS = [_]Day{
    Day{ .name = "day01", .main = day01 },
    Day{ .name = "day02", .main = day02 },
    Day{ .name = "day03", .main = day03 },
    ...
};
```

Adding a new year is similar, and this happens in `src/main.zig`.