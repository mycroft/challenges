const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

fn is_valid(password: []const u8) bool {
    var count_duplicate: usize = 0; // has at least 2 paris of same letters like aa, bb, cc
    var last_duplicate: ?usize = null;

    var has_triple = false; // has at least 3 letters following themselves as abc, bcd, ..
    for (password, 0..) |c, idx| {
        if (c == 'i' or c == 'o' or c == 'l') {
            return false;
        }

        if (idx < password.len - 1 and password[idx + 1] == c) {
            if (last_duplicate == null or last_duplicate.? < idx - 1) {
                count_duplicate += 1;
                last_duplicate = idx;
            }
        }

        if (idx < password.len - 2 and c == password[idx + 1] - 1 and c == password[idx + 2] - 2) {
            has_triple = true;
        }
    }
    return count_duplicate == 2 and has_triple;
}

fn next(password: []u8) []u8 {
    var idx: usize = 0;

    // check if there is a forbidden char in the password first
    for (password) |c| {
        if (c == 'i' or c == 'o' or c == 'l') {
            password[idx] += 1;
            idx += 1;
            while (idx < password.len) : (idx += 1) {
                password[idx] = 'a';
            }

            return password;
        }

        idx += 1;
    }

    // add 1 to last char
    idx = password.len - 1;

    while (true) {
        password[idx] += 1;

        if (password[idx] == 'i' or password[idx] == 'o' or password[idx] == 'l') {
            password[idx] += 1;
            return password;
        }

        if (password[idx] <= 'z') {
            return password;
        }

        password[idx] = 'a';

        if (idx == 0) {
            break;
        }

        idx -= 1;
    }

    return password;
}

fn next_valid(password: []u8) []u8 {
    while (true) {
        _ = next(password);

        // aoc.printf("{s}\n", .{password});

        if (is_valid(password)) {
            return password;
        }
    }
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    const password = try allocator.dupe(u8, input);
    defer allocator.free(password);

    // aoc.printf("{s}\n", .{password});

    _ = next_valid(password);

    aoc.printf("step1: {s}\n", .{password});

    _ = next_valid(password);

    aoc.printf("step2: {s}\n", .{password});

    return .{ .step1 = 0, .step2 = 0 };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    _ = challenge;

    const input = "hepxcrrq";

    _ = try solve(allocator, input);

    // aoc.printf("step1: {d}\n", .{result.step1});
    // aoc.printf("step2: {d}\n", .{result.step2});
}

test "valid" {
    try testing.expect(is_valid("abcdffaa"));
    try testing.expect(is_valid("ghjaabcc"));

    try testing.expect(!is_valid("hijklmmn"));
    try testing.expect(!is_valid("abbceffg"));
    try testing.expect(!is_valid("abbcegjk"));

    try testing.expect(is_valid("hepxxyzz"));
}

test "next" {
    var entry: []u8 = try testing.allocator.dupe(u8, "abclyyyy");
    try testing.expect(std.mem.eql(u8, next(entry), "abcmaaaa"));
    testing.allocator.free(entry);

    entry = try testing.allocator.dupe(u8, "a");
    try testing.expect(std.mem.eql(u8, next(entry), "b"));
    testing.allocator.free(entry);

    entry = try testing.allocator.dupe(u8, "z");
    try testing.expect(std.mem.eql(u8, next(entry), "a"));
    testing.allocator.free(entry);

    entry = try testing.allocator.dupe(u8, "zz");
    try testing.expect(std.mem.eql(u8, next(entry), "aa"));
    testing.allocator.free(entry);

    entry = try testing.allocator.dupe(u8, "bb");
    try testing.expect(std.mem.eql(u8, next(entry), "bc"));
    testing.allocator.free(entry);
}

test "next_valid" {
    var entry: []u8 = try testing.allocator.dupe(u8, "abcdefgh");
    try testing.expect(std.mem.eql(u8, next_valid(entry), "abcdffaa"));
    testing.allocator.free(entry);

    entry = try testing.allocator.dupe(u8, "ghijklmn");
    try testing.expect(std.mem.eql(u8, next_valid(entry), "ghjaabcc"));
    testing.allocator.free(entry);
}
