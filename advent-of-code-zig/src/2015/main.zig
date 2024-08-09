const std = @import("std");
const aoc = @import("../aoc.zig");

const day01 = @import("./day01.zig").main;
const day02 = @import("./day02.zig").main;
const day03 = @import("./day03.zig").main;
const day04 = @import("./day04.zig").main;
const day05 = @import("./day05.zig").main;
const day06 = @import("./day06.zig").main;
const day07 = @import("./day07.zig").main;
const day08 = @import("./day08.zig").main;
const day09 = @import("./day09.zig").main;

const Day = struct {
    name: []const u8,
    main: fn (std.mem.Allocator, aoc.Challenge) anyerror!void,
    skipped: bool = false,
};

const DAYS = [_]Day{
    Day{ .name = "day01", .main = day01 },
    Day{ .name = "day02", .main = day02 },
    Day{ .name = "day03", .main = day03 },
    Day{ .name = "day04", .main = day04, .skipped = true },
    Day{ .name = "day05", .main = day05 },
    Day{ .name = "day06", .main = day06 },
    Day{ .name = "day07", .main = day07 },
    Day{ .name = "day08", .main = day08 },
    Day{ .name = "day09", .main = day09 },
};

pub fn main(allocator: std.mem.Allocator, year: []const u8) !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("you're in 2015' main!\n", .{});

    inline for (DAYS, 1..) |day, num| {
        try stdout.print("-- day {d} --\n", .{num});
        const challenge = aoc.Challenge.new(year, day.name);

        if (day.skipped) {
            try stdout.print("marked as skipped...\n", .{});
            continue;
        }

        try day.main(allocator, challenge);
    }
}
