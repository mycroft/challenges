const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    var hash: [std.crypto.hash.Md5.digest_length]u8 = undefined;
    var n: u32 = 0;

    const buf = try allocator.alloc(u8, input.len + 10);
    defer allocator.free(buf);

    var step_1_res: ?u32 = null;
    var step_2_res: ?u32 = null;

    while (true) {
        var h = std.crypto.hash.Md5.init(.{});

        // not having the full key in "key" is a bit faster than processing everything at once.
        const key = try std.fmt.bufPrint(buf, "{d}", .{n});
        h.update(input);
        h.update(key);
        h.final(&hash);

        if (hash[0] == 0 and hash[1] == 0 and hash[2] & 0xf0 == 0) {
            if (step_1_res == null) {
                step_1_res = n;
            }

            if (hash[2] == 0) {
                step_2_res = n;
                break;
            }
        }

        n += 1;
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
