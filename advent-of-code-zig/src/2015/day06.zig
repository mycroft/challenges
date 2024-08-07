const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

const Verb = enum {
    Toggle,
    TurnOn,
    TurnOff,
};

const Point = struct {
    x: usize,
    y: usize,
};

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    _ = allocator;

    var step1_num: u32 = 0;
    var step2_num: i32 = 0;

    var grid: [1000][1000]bool = undefined;
    @memset(&grid[0], false);
    @memset(&grid, grid[0]);

    var brightness: [1000][1000]i32 = undefined;
    @memset(&brightness[0], 0);
    @memset(&brightness, brightness[0]);

    var iter = std.mem.split(u8, input, "\n");

    while (iter.next()) |line| {
        if (line.len == 0) {
            break;
        }
        var current_verb: Verb = undefined;

        // aoc.printf("{s}\n", .{line});

        var sub_iter = std.mem.split(u8, line, " ");

        var verb = sub_iter.next().?;
        if (std.mem.eql(u8, "turn", verb)) {
            verb = sub_iter.next().?;
        }

        if (std.mem.eql(u8, "on", verb)) {
            current_verb = Verb.TurnOn;
        } else if (std.mem.eql(u8, "off", verb)) {
            current_verb = Verb.TurnOff;
        } else {
            current_verb = Verb.Toggle;
        }

        var coord = sub_iter.next().?;
        var idx = std.mem.indexOfScalar(u8, coord, ',').?;

        // aoc.printf("{s} {s}\n", .{ coord[0..idx], coord[idx + 1 ..] });

        const point_from = Point{
            .x = try std.fmt.parseInt(usize, coord[0..idx], 10),
            .y = try std.fmt.parseInt(usize, coord[idx + 1 ..], 10),
        };

        _ = sub_iter.next();

        coord = sub_iter.next().?;
        idx = std.mem.indexOfScalar(u8, coord, ',').?;

        const point_to = Point{
            .x = try std.fmt.parseInt(usize, coord[0..idx], 10),
            .y = try std.fmt.parseInt(usize, coord[idx + 1 ..], 10),
        };

        // aoc.printf("{any} // {any}\n", .{ point_from, point_to });

        for (point_from.x..1 + point_to.x) |x| {
            for (point_from.y..1 + point_to.y) |y| {
                switch (current_verb) {
                    Verb.Toggle => {
                        grid[x][y] = !grid[x][y];
                        brightness[x][y] += 2;
                    },
                    Verb.TurnOn => {
                        grid[x][y] = true;
                        brightness[x][y] += 1;
                    },
                    Verb.TurnOff => {
                        grid[x][y] = false;
                        brightness[x][y] = @max(brightness[x][y] - 1, 0);
                    },
                }
            }
        }
    }

    for (0..1000) |x| {
        for (0..1000) |y| {
            if (grid[x][y]) {
                step1_num += 1;
            }
            step2_num += brightness[x][y];
        }
    }

    return .{ .step1 = step1_num, .step2 = @intCast(step2_num) };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}
