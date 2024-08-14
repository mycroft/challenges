const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

fn parse(allocator: std.mem.Allocator, input: []const u8, with_me: bool) !std.StringHashMap(std.StringHashMap(i32)) {
    var res = std.StringHashMap(std.StringHashMap(i32)).init(allocator);

    if (with_me) {
        try res.put("me", std.StringHashMap(i32).init(allocator));
    }

    var lines = std.mem.split(u8, input, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) {
            break;
        }

        // I gotta find a new way to do parsing. This one sucks.
        var parts = std.mem.split(u8, line, " ");
        const who0 = parts.next().?;
        _ = parts.next(); // skipping "would"
        const verb = parts.next().?; // gain or lose
        var score = try std.fmt.parseInt(i32, parts.next().?, 10); // int
        _ = parts.next(); // happiness
        _ = parts.next(); // units
        _ = parts.next(); // by
        _ = parts.next(); // sitting
        _ = parts.next(); // next
        _ = parts.next(); // to

        parts = std.mem.split(u8, parts.next().?, ".");
        const who1 = parts.next().?;

        if (!res.contains(who0)) {
            try res.put(who0, std.StringHashMap(i32).init(allocator));
        }

        if (std.mem.eql(u8, "lose", verb)) {
            score *= -1;
        }

        var sub_hash = res.getPtr(who0).?;
        try sub_hash.put(who1, score);

        if (with_me) {
            try sub_hash.put("me", 0);

            sub_hash = res.getPtr("me").?;
            try sub_hash.put(who0, 0);
        }

        // aoc.printf("{s} {s} {s} {s}\n", .{ who0, verb, score, who1 });
    }

    return res;
}

fn visit(allocator: std.mem.Allocator, data: std.StringHashMap(std.StringHashMap(i32)), to_visit: *std.ArrayList([]const u8), visited: *std.ArrayList([]const u8)) !i32 {
    if (to_visit.items.len == 0) {
        var score: i32 = 0;

        for (visited.items, 0..) |current_item, n| {
            const other_item = if (n == 0) visited.items[visited.items.len - 1] else visited.items[n - 1];

            score += data.get(current_item).?.get(other_item).?;
            score += data.get(other_item).?.get(current_item).?;
        }

        return score;
    }

    var idx: usize = 0;
    var score: ?i32 = null;

    while (idx < to_visit.items.len) : (idx += 1) {
        const current_visited = to_visit.pop();
        var is_visited_already = false;

        for (visited.items) |item| {
            if (std.mem.eql(u8, item, current_visited)) {
                is_visited_already = true;
            }
        }

        if (is_visited_already) {
            try to_visit.insert(0, current_visited);
            continue;
        }

        try visited.append(current_visited);

        const current_score = try visit(allocator, data, to_visit, visited);

        _ = visited.pop();

        if (score) |opt_score| {
            if (opt_score < current_score) {
                score = current_score;
            }
        } else {
            score = current_score;
        }

        // cleanup
        try to_visit.insert(0, current_visited);

        // optimization: do not compute all starting points possible. We are handling a circular linked list,
        // and we will recompute things already known.
        if (visited.items.len == 0) {
            break;
        }
    }

    return score.?;
}

fn solve(allocator: std.mem.Allocator, input: []const u8, with_me: bool) !i32 {
    var parsed_input = try parse(allocator, input, with_me);
    defer parsed_input.deinit();

    var visitors = std.ArrayList([]const u8).init(allocator);
    defer visitors.deinit();

    var visited = std.ArrayList([]const u8).init(allocator);
    defer visited.deinit();

    var it = parsed_input.keyIterator();
    while (it.next()) |key| {
        try visitors.append(key.*);
    }

    const score = try visit(allocator, parsed_input, &visitors, &visited);

    // cleanup
    it = parsed_input.keyIterator();

    while (it.next()) |key| {
        parsed_input.getPtr(key.*).?.deinit();
    }

    return score;
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result_step1 = try solve(allocator, file_content, false);
    const result_step2 = try solve(allocator, file_content, true);

    aoc.printf("step1: {d}\n", .{result_step1});
    aoc.printf("step2: {d}\n", .{result_step2});
}
