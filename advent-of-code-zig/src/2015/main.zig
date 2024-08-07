const std = @import("std");
const aoc = @import("../aoc.zig");

const day01 = @import("./day01.zig").main;
const day02 = @import("./day02.zig").main;
const day03 = @import("./day03.zig").main;

const Day = struct {
    name: []const u8,
    main: fn (std.mem.Allocator, aoc.Challenge) anyerror!void,
};

const DAYS = [_]Day{
    Day{ .name = "day01", .main = day01 },
    Day{ .name = "day02", .main = day02 },
    Day{ .name = "day03", .main = day03 },
};

pub fn main(allocator: std.mem.Allocator, year: []const u8) !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("you're in 2015' main!\n", .{});

    inline for (DAYS, 1..) |day, num| {
        try stdout.print("-- day {d} --\n", .{num});
        try day.main(allocator, aoc.Challenge.new(year, day.name));
    }
}
