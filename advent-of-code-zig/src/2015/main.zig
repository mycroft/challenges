const std = @import("std");

const day01 = @import("./day01.zig").main;
const day02 = @import("./day02.zig").main;

const Day = struct {
    name: []const u8,
    main: fn (std.mem.Allocator) anyerror!void,
};

const DAYS = [_]Day{
    Day{ .name = "day01", .main = day01 },
    Day{ .name = "day02", .main = day02 },
};

pub fn main(allocator: std.mem.Allocator) !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("you're in 2015' main!\n", .{});

    inline for (DAYS, 1..) |day, num| {
        try stdout.print("-- day {d} --\n", .{num});
        try day.main(allocator);
    }
}
