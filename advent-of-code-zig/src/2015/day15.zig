const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

const Ingredient = struct {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
};

fn score(ingredients: std.ArrayList(Ingredient), visited: *std.ArrayList(u8)) struct { step1: i64, step2: i64 } {
    var capacity: i64 = 0;
    var durability: i64 = 0;
    var flavor: i64 = 0;
    var texture: i64 = 0;
    var calories: i64 = 0;

    for (visited.items, 0..) |visited_num, n| {
        capacity += @as(i64, visited_num) * ingredients.items[n].capacity;
        durability += @as(i64, visited_num) * ingredients.items[n].durability;
        flavor += @as(i64, visited_num) * ingredients.items[n].flavor;
        texture += @as(i64, visited_num) * ingredients.items[n].texture;
        calories += @as(i64, visited_num) * ingredients.items[n].calories;
    }

    var step1_score: i64 = @max(0, capacity) * @max(0, durability) * @max(0, flavor) * @max(0, texture);
    if (step1_score < 0) {
        step1_score = 0;
    }

    const step2_score: i64 = if (calories == 500) step1_score else 0;

    return .{ .step1 = step1_score, .step2 = step2_score };
}

fn compute(ingredients: std.ArrayList(Ingredient), visited: *std.ArrayList(u8), remaining: u8) !struct { step1: i64, step2: i64 } {
    if (visited.items.len == ingredients.items.len - 1) {
        try visited.append(remaining);
        const res = score(ingredients, visited);
        _ = visited.pop();
        return .{ .step1 = res.step1, .step2 = res.step2 };
    }

    const max_for_this_ingredient: u8 = remaining - @as(u8, @truncate(ingredients.items.len)) - @as(u8, @truncate(visited.items.len));
    var max_recipe: i64 = 0;
    var max_recipe_calories: i64 = 0;

    for (0..max_for_this_ingredient) |number| {
        try visited.append(@intCast(number));
        const res = try compute(ingredients, visited, remaining - @as(u8, @truncate(number)));
        _ = visited.pop();

        if (res.step1 > max_recipe) {
            max_recipe = res.step1;
        }

        if (res.step2 > max_recipe_calories) {
            max_recipe_calories = res.step2;
        }
    }

    return .{ .step1 = max_recipe, .step2 = max_recipe_calories };
}

fn parse(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Ingredient) {
    var result = std.ArrayList(Ingredient).init(allocator);

    var lines = std.mem.split(u8, input, "\n");

    while (lines.next()) |line| {
        if (line.len == 0) {
            break;
        }

        var parts = std.mem.split(u8, line, " ");
        _ = parts.next();
        _ = parts.next();
        const capacity = try allocator.dupe(u8, parts.next().?);
        defer allocator.free(capacity);
        const allocator_trimmed = std.mem.trimRight(u8, capacity, ",");

        _ = parts.next();
        const durability = try allocator.dupe(u8, parts.next().?);
        defer allocator.free(durability);
        const durability_trimmed = std.mem.trimRight(u8, durability, ",");

        _ = parts.next();
        const flavor = try allocator.dupe(u8, parts.next().?);
        defer allocator.free(flavor);
        const flavor_trimmed = std.mem.trimRight(u8, flavor, ",");

        _ = parts.next();
        const texture = try allocator.dupe(u8, parts.next().?);
        defer allocator.free(texture);
        const texture_trimmed = std.mem.trimRight(u8, texture, ",");

        _ = parts.next();
        const calories = try allocator.dupe(u8, parts.next().?);
        defer allocator.free(calories);
        const calories_trimmed = std.mem.trimRight(u8, calories, ",");

        try result.append(Ingredient{
            .capacity = try std.fmt.parseInt(i32, allocator_trimmed, 10),
            .durability = try std.fmt.parseInt(i32, durability_trimmed, 10),
            .flavor = try std.fmt.parseInt(i32, flavor_trimmed, 10),
            .texture = try std.fmt.parseInt(i32, texture_trimmed, 10),
            .calories = try std.fmt.parseInt(i32, calories_trimmed, 10),
        });
    }

    return result;
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: i64, step2: i64 } {
    const ingredients = try parse(allocator, input);
    defer ingredients.deinit();

    var visited = std.ArrayList(u8).init(allocator);
    defer visited.deinit();
    const result = try compute(ingredients, &visited, 100);

    return .{ .step1 = result.step1, .step2 = result.step2 };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}
