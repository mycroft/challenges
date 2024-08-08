const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

fn count_chars(input: []const u8) u32 {
    var res: u32 = 0;

    var idx: u32 = 1;

    while (idx < input.len - 1) : (idx += 1) {
        if (input[idx] == '\\') {
            idx += 1;

            if (input[idx] == 'x') {
                idx += 2;
            }
        }

        res += 1;
    }

    return res;
}

fn encode_chars_count(input: []const u8) u32 {
    var res: u32 = 0;

    // need to count number of \ & ",

    for (input) |c| {
        if (c == '"' or c == '\\') {
            res += 1;
        }

        res += 1;
    }

    return 2 + res;
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    _ = allocator;

    var iter = std.mem.split(u8, input, "\n");
    var step1: u32 = 0;
    var step2: u32 = 0;

    while (iter.next()) |line| {
        if (line.len == 0) {
            break;
        }
        // aoc.printf("{s} {d} {d}\n", .{ line, count_chars(line), encode_chars_count(line) });

        step1 += @as(u32, @intCast(line.len)) - count_chars(line);
        step2 += encode_chars_count(line) - @as(u32, @intCast(line.len));
    }

    return .{ .step1 = step1, .step2 = step2 };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}
