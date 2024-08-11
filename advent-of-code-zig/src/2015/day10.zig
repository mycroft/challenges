const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

fn iterate(allocator: std.mem.Allocator, input: std.ArrayList(u8)) !std.ArrayList(u8) {
    var result = try std.ArrayList(u8).initCapacity(allocator, 5 * 1024 * 1024);

    var current_c: u8 = input.items[0];
    var current_count: usize = 1;

    for (input.items[1..]) |item| {
        if (item == current_c) {
            current_count += 1;
            continue;
        }

        const current_idx = result.items.len;
        while (current_count > 0) {
            const c = current_count % 10;

            try result.insert(current_idx, @intCast(c));
            current_count = (current_count - c) / 10;
        }
        try result.insert(result.items.len, current_c);

        current_c = item;
        current_count = 1;
    }

    // finish!

    const current_idx = result.items.len;
    while (current_count > 0) {
        const c = current_count % 10;

        try result.insert(current_idx, @intCast(c));
        current_count = (current_count - c) / 10;
    }
    try result.insert(result.items.len, current_c);

    return result;
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    var idx: usize = 0;

    var step1_size: u32 = 0;
    var step2_size: u32 = 0;

    var res = try input_to_arraylist(allocator, input);
    var nres = res;
    defer res.deinit();

    while (idx < 50) {
        nres = try iterate(allocator, res);
        res.deinit();

        res = nres;
        idx += 1;

        if (idx == 40) {
            step1_size = @intCast(res.items.len);
        }
    }

    step2_size = @intCast(res.items.len);

    return .{ .step1 = step1_size, .step2 = step2_size };
}

fn input_to_arraylist(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(u8) {
    var result = std.ArrayList(u8).init(allocator);

    for (input) |c| {
        try result.append(c - '0');
    }

    return result;
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const input = "1113222113";
    _ = challenge;

    const result = try solve(allocator, input);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}

test "sample" {
    var res = try iterate(testing.allocator, "1");
    try testing.expect(std.mem.eql(u8, "11", res));
    testing.allocator.free(res);

    res = try iterate(testing.allocator, "21");
    try testing.expect(std.mem.eql(u8, "1211", res));
    testing.allocator.free(res);

    res = try iterate(testing.allocator, "1211");
    try testing.expect(std.mem.eql(u8, "111221", res));
    testing.allocator.free(res);
}
