const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

fn contains_or_add(haystack: *std.ArrayList([]const u8), needle: []const u8) !bool {
    for (haystack.items) |item| {
        if (std.mem.eql(u8, item, needle)) {
            return true;
        }
    }

    try haystack.append(needle);

    return false;
}

fn travel(weights: *std.StringHashMap(std.StringHashMap(u32)), visited: *std.ArrayList([]const u8), to_visit: *std.ArrayList([]const u8)) !struct { min: u32, max: u32 } {
    var min_value: u32 = 9999;
    var max_value: u32 = 0;

    if (to_visit.*.items.len == 0) {
        var weight: u32 = 0;
        var old_item: []const u8 = "";

        for (visited.items) |item| {
            if (old_item.len == 0) {
                old_item = item;
                continue;
            }

            weight += weights.get(old_item).?.get(item).?;

            old_item = item;
        }

        return .{ .min = weight, .max = weight };
    }

    for (0..to_visit.*.items.len) |_| {
        const city = to_visit.pop();
        try visited.append(city);

        const res = try travel(weights, visited, to_visit);

        if (res.min < min_value) {
            min_value = res.min;
        }
        if (res.max > max_value) {
            max_value = res.max;
        }

        _ = visited.pop();
        try to_visit.insert(0, city);
    }

    return .{ .min = min_value, .max = max_value };
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    var cities = std.ArrayList([]const u8).init(allocator);
    defer cities.deinit();

    var weights = std.StringHashMap(std.StringHashMap(u32)).init(allocator);
    defer weights.deinit();

    var lines_iter = std.mem.split(u8, input, "\n");
    while (lines_iter.next()) |line| {
        if (line.len == 0) {
            break;
        }
        var items_iter = std.mem.split(u8, line, " ");

        const from = items_iter.next().?;
        _ = items_iter.next(); // to
        const to = items_iter.next().?;
        _ = items_iter.next(); // =
        const weight = try std.fmt.parseInt(u32, items_iter.next().?, 10);

        // aoc.printf("{s} {s} {d}\n", .{ from, to, weight });

        _ = try contains_or_add(&cities, from);
        _ = try contains_or_add(&cities, to);

        if (!weights.contains(from)) {
            try weights.put(from, std.StringHashMap(u32).init(allocator));
        }

        if (!weights.contains(to)) {
            try weights.put(to, std.StringHashMap(u32).init(allocator));
        }

        try weights.getPtr(from).?.*.put(to, weight);
        try weights.getPtr(to).?.*.put(from, weight);
    }

    // here, solve the challenge (well, when you're ok with hash in hash memory leaks...)
    // this part is to be done

    var visited = std.ArrayList([]const u8).init(allocator);
    defer visited.deinit();
    const res = try travel(&weights, &visited, &cities);

    // memory clean-up
    var key_iter = weights.keyIterator();
    while (key_iter.next()) |key| {
        weights.getPtr(key.*).?.*.deinit();
    }

    return .{ .step1 = res.min, .step2 = res.max };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}

test "sample" {
    const input =
        \\London to Dublin = 464
        \\London to Belfast = 518
        \\Dublin to Belfast = 141
    ;

    const result = try solve(testing.allocator, input);

    try testing.expectEqual(605, result.step1);
    try testing.expectEqual(982, result.step2);
}
