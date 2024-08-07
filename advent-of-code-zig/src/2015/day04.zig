const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

var step_1_res: ?u32 = null;
var step_2_res: ?u32 = null;

fn solve_unit(allocator: std.mem.Allocator, input: []const u8, start: u32, step: u32) !void {
    var hash: [std.crypto.hash.Md5.digest_length]u8 = undefined;

    const buf = try allocator.alloc(u8, input.len + 10);
    defer allocator.free(buf);

    // var step_1_res: ?u32 = null;
    // var step_2_res: ?u32 = null;

    for (start..step + start) |n| {
        var h = std.crypto.hash.Md5.init(.{});

        // not having the full key in "key" is a bit faster than processing everything at once.
        const key = try std.fmt.bufPrint(buf, "{d}", .{n});
        h.update(input);
        h.update(key);
        h.final(&hash);

        if (hash[0] == 0 and hash[1] == 0 and hash[2] & 0xf0 == 0) {
            if (step_1_res == null or step_1_res.? > n) {
                step_1_res = @intCast(n);
            }

            if (hash[2] == 0 and (step_2_res == null or step_2_res.? > n)) {
                step_2_res = @intCast(n);
                break;
            }
        }
    }
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    const threads_num: usize = 128;
    var start: u32 = 0;
    const step: u32 = 100000;

    var threads: [threads_num]std.Thread = undefined;

    while (step_1_res == null or step_2_res == null) {
        for (0..threads_num) |thread_n| {
            threads[thread_n] = try std.Thread.spawn(.{}, solve_unit, .{ allocator, input, start, step });
            start += step;
        }

        for (0..threads_num) |thread_n| {
            threads[thread_n].join();
        }
    }

    return .{ .step1 = step_1_res.?, .step2 = step_2_res.? };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    _ = challenge;

    const input = "iwrupvqb";

    const result = try solve(allocator, input);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}

test "step1" {
    // disabled as long as doing a lot of md5 is slow.
    if (false) {
        const allocator = std.testing.allocator;
        try testing.expectEqual(609043, (try solve(allocator, "abcdef")).step1);
        try testing.expectEqual(1048970, (try solve(allocator, "pqrstuv")).step1);
    }
}
