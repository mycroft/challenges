const std = @import("std");
const aoc = @import("../aoc.zig");

const day01 = @import("./day01.zig").main;
const day02 = @import("./day02.zig").main;
const day03 = @import("./day03.zig").main;
const day04 = @import("./day04.zig").main;

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
