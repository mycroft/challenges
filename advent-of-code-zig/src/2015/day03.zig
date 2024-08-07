const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

const Coord = struct {
    x: i32,
    y: i32,
};

fn solve(allocator: std.mem.Allocator, contents: []const u8) !struct { step1: u32, step2: u32 } {
    var current = Coord{
        .x = 0,
        .y = 0,
    };

    var step2_santa = current;
    var step2_robot_santa = current;

    var visited = std.AutoArrayHashMap(Coord, usize).init(allocator);
    defer visited.deinit();

    var step2_visited = std.AutoArrayHashMap(Coord, usize).init(allocator);
    defer step2_visited.deinit();

    // initial house
    try visited.put(current, 1);
    try step2_visited.put(current, 1);

    for (contents, 0..) |c, robot| {
        var dir = Coord{ .x = 0, .y = 0 };
        switch (c) {
            '^' => dir.y -= 1,
            'v' => dir.y += 1,
            '<' => dir.x -= 1,
            '>' => dir.x += 1,
            else => {
                aoc.printf("unhandled case: {c}\n", .{c});
            },
        }

        // step1 robot
        current.x += dir.x;
        current.y += dir.y;

        try visited.put(current, 1);

        // step2 robots
        if (robot % 2 == 0) {
            step2_santa.x += dir.x;
            step2_santa.y += dir.y;

            try step2_visited.put(step2_santa, 1);
        } else {
            step2_robot_santa.x += dir.x;
            step2_robot_santa.y += dir.y;

            try step2_visited.put(step2_robot_santa, 1);
        }
    }

    // aoc.printf("{d}\n", .{visited.keys().len});
    return .{ .step1 = @intCast(visited.keys().len), .step2 = @intCast(step2_visited.keys().len) };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const stdout = std.io.getStdOut().writer();

    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    try stdout.print("step1: {d}\n", .{result.step1});
    try stdout.print("step2: {d}\n", .{result.step2});
}

test "step1" {
    const allocator = std.testing.allocator;

    try testing.expectEqual(2, (try solve(allocator, ">")).step1);
    try testing.expectEqual(4, (try solve(allocator, "^>v<")).step1);
    try testing.expectEqual(2, (try solve(allocator, "^v^v^v^v^v")).step1);
}

test "step2" {
    const allocator = std.testing.allocator;

    try testing.expectEqual(3, (try solve(allocator, "^v")).step2);
    try testing.expectEqual(3, (try solve(allocator, "^>v<")).step2);
    try testing.expectEqual(11, (try solve(allocator, "^v^v^v^v^v")).step2);
}
