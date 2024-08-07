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

Then, just use the following template:

```zig
const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    _ = allocator;
    _ = input;

    return .{ .step1 = 0, .step2 = 0 };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}
```

Adding a new year is similar, and this happens in `src/main.zig`.