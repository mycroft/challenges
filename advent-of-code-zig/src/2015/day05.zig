const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

fn is_nice(input: []const u8) bool {
    var voyels: u32 = 0;
    var has_double = false;

    const forbidden = [_][]const u8{
        "ab",
        "cd",
        "pq",
        "xy",
    };

    for (input, 0..) |c, n| {
        if (c == 'a' or c == 'e' or c == 'i' or c == 'o' or c == 'u') {
            voyels += 1;
        }

        if (n == input.len - 1) {
            continue;
        }

        if (c == input[n + 1]) {
            has_double = true;
        }

        for (forbidden) |x| {
            if (c == x[0] and input[n + 1] == x[1]) {
                return false;
            }
        }
    }

    return voyels >= 3 and has_double;
}

fn is_nice2(input: []const u8) bool {
    var has_middle_letter = false;
    var has_double = false;

    for (input, 0..) |_, n| {
        if (n > 0 and n < input.len - 1) {
            if (input[n - 1] == input[n + 1]) {
                has_middle_letter = true;
            }
        }

        var p = n + 2;

        while (p < input.len - 1) : (p += 1) {
            if (input[n] == input[p] and input[n + 1] == input[p + 1]) {
                has_double = true;
            }
        }
    }

    return has_middle_letter and has_double;
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    _ = allocator;

    var step1_num_nice: u32 = 0;
    var step2_num_nice: u32 = 0;

    var iter = std.mem.split(u8, input, "\n");

    while (iter.next()) |line| {
        if (line.len == 0) {
            break;
        }

        if (is_nice(line)) {
            step1_num_nice += 1;
        }

        if (is_nice2(line)) {
            step2_num_nice += 1;
        }
    }

    return .{ .step1 = step1_num_nice, .step2 = step2_num_nice };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    aoc.printf("step1: {d}\n", .{result.step1}); // 236
    aoc.printf("step2: {d}\n", .{result.step2}); // 51
}

test "step1" {
    try testing.expect(is_nice("ugknbfddgicrmopn"));
    try testing.expect(is_nice("aaa"));
    try testing.expect(!is_nice("jchzalrnumimnmhp"));
    try testing.expect(!is_nice("haegwjzuvuyypxyu"));
    try testing.expect(!is_nice("dvszwmarrgswjxmb"));
}

test "step2" {
    try testing.expect(is_nice2("qjhvhtzxzqqjkmpb"));
    try testing.expect(is_nice2("xxyxx"));
    try testing.expect(!is_nice2("uurcxstgmygtbstg"));
    try testing.expect(!is_nice2("ieodomkazucvgmuy"));
}
