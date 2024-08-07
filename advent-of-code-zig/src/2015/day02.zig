const std = @import("std");
const futils = @import("../futils.zig");
const testing = std.testing;

fn solve(contents: []const u8) !struct { step1: u32, step2: u32 } {
    var lines = std.mem.split(u8, contents, "\n");

    var res1: u32 = 0;
    var res2: u32 = 0;

    while (lines.next()) |line| {
        if (line.len == 0) {
            break;
        }

        var dims = std.mem.split(u8, line, "x");
        const x = try std.fmt.parseInt(u32, dims.next().?, 10);
        const y = try std.fmt.parseInt(u32, dims.next().?, 10);
        const z = try std.fmt.parseInt(u32, dims.next().?, 10);

        // std.debug.print("{s} {d} {d} {d}\n", .{ line, x, y, z });

        res1 += (2 * x * y) + (2 * y * z) + (2 * x * z);
        res1 += @min(x * y, @min(x * z, y * z));

        res2 += x * y * z;
        const max_size = @max(x, @max(y, z));

        if (max_size == x) {
            res2 += 2 * y + 2 * z;
        } else if (max_size == y) {
            res2 += 2 * x + 2 * z;
        } else {
            res2 += 2 * x + 2 * y;
        }
    }

    return .{ .step1 = res1, .step2 = res2 };
}

pub fn main(allocator: std.mem.Allocator) anyerror!void {
    const stdout = std.io.getStdOut().writer();

    const file_content = try futils.file_to_string(allocator, "./input/2015/day02.txt");
    defer allocator.free(file_content);

    const result = try solve(file_content);

    try stdout.print("step1: {d}\n", .{result.step1});
    try stdout.print("step2: {d}\n", .{result.step2});
}

test "step1" {
    var result = try solve("2x3x4\n");
    try testing.expect(result.step1 == 58);

    result = try solve("1x1x10\n");
    try testing.expect(result.step1 == 43);

    result = try solve("1x1x10\n2x3x4\n");
    try testing.expect(result.step1 == 101);
}

test "step2" {
    var result = try solve("2x3x4\n");
    try testing.expect(result.step2 == 34);

    result = try solve("1x1x10\n");
    try testing.expect(result.step2 == 14);
}
