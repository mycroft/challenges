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
const day10 = @import("./day10.zig").main;
const day11 = @import("./day11.zig").main;
const day12 = @import("./day12.zig").main;
const day13 = @import("./day13.zig").main;
const day14 = @import("./day14.zig").main;
const day15 = @import("./day15.zig").main;

const Day = struct {
    name: []const u8,
    main: fn (std.mem.Allocator, aoc.Challenge) anyerror!void,
    skipped: bool = false,
};

const DAYS = [_]Day{
    Day{ .name = "day01", .main = day01 },
    Day{ .name = "day02", .main = day02 },
    Day{ .name = "day03", .main = day03 },
    Day{ .name = "day04", .main = day04, .skipped = false },
    Day{ .name = "day05", .main = day05 },
    Day{ .name = "day06", .main = day06 },
    Day{ .name = "day07", .main = day07 },
    Day{ .name = "day08", .main = day08 },
    Day{ .name = "day09", .main = day09 },
    Day{ .name = "day10", .main = day10 },
    Day{ .name = "day11", .main = day11 },
    Day{ .name = "day12", .main = day12 },
    Day{ .name = "day13", .main = day13 },
    Day{ .name = "day14", .main = day14 },
    Day{ .name = "day15", .main = day15 },
};

pub fn main(allocator: std.mem.Allocator, year: []const u8) !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("you're in 2015' main!\n", .{});

    const filter: []const u8 = if (std.os.argv.len > 1) std.mem.span(std.os.argv[1]) else "";

    const all_now = std.time.milliTimestamp();

    inline for (DAYS, 1..) |day, num| {
        if (filter.len == 0 or std.mem.eql(u8, filter, day.name)) {
            try stdout.print("-- day {d} --\n", .{num});
            const challenge = aoc.Challenge.new(year, day.name);

            if (!day.skipped) {
                const now = std.time.milliTimestamp();
                try day.main(allocator, challenge);
                const now_end = std.time.milliTimestamp();

                try stdout.print("-- day {d} ended after {d} ms --\n\n", .{ num, now_end - now });
            } else {
                try stdout.print("marked as skipped...\n\n", .{});
            }
        }
    }

    const all_now_end = std.time.milliTimestamp();

    try stdout.print("completed all challenges after {d} ms.\n", .{all_now_end - all_now});
}
