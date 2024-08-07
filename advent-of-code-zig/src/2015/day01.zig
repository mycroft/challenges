const std = @import("std");
const futils = @import("../futils.zig");
const testing = std.testing;

fn solve(input: []const u8) struct { step1: i32, step2: i32 } {
    var res1: i32 = 0;
    var res2: ?i32 = null;

    for (input, 1..) |c, i| {
        switch (c) {
            '(' => {
                res1 += 1;
            },
            ')' => {
                res1 -= 1;
            },
            else => {},
        }

        if (res2 == null and res1 == -1) {
            res2 = @intCast(i);
        }
    }

    return .{ .step1 = res1, .step2 = res2 orelse 0 };
}

pub fn main(allocator: std.mem.Allocator) anyerror!void {
    const stdout = std.io.getStdOut().writer();

    const file_content = try futils.file_to_string(allocator, "./input/2015/day01.txt");
    defer allocator.free(file_content);

    const result = solve(file_content);

    try stdout.print("step1: {d}\n", .{result.step1});
    try stdout.print("step2: {d}\n", .{result.step2});
}

test "step1" {
    try testing.expect(solve("(())").step1 == 0);
    try testing.expect(solve("()()").step1 == 0);

    try testing.expect(solve("(()(()(").step1 == 3);
    try testing.expect(solve("(((").step1 == 3);
    try testing.expect(solve("))(((((").step1 == 3);

    try testing.expect(solve("())").step1 == -1);
    try testing.expect(solve("))(").step1 == -1);

    try testing.expect(solve(")))").step1 == -3);
    try testing.expect(solve(")())())").step1 == -3);
}

test "step2" {
    try testing.expect(solve(")").step2 == 1);
    try testing.expect(solve("()())").step2 == 5);
}
