const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

const State = enum {
    Running,
    Resting,
};

const Champion = struct {
    speed: u32,
    duration: u32,
    rest: u32,
    current_distance: u32,
    points: u32,
    remaining: u32,
    state: State,
};

fn parse(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Champion) {
    var result = std.ArrayList(Champion).init(allocator);
    var lines = std.mem.split(u8, input, "\n");

    while (lines.next()) |line| {
        if (line.len == 0) {
            break;
        }
        var parts = std.mem.split(u8, line, " ");
        _ = parts.next(); // name
        _ = parts.next(); // can
        _ = parts.next(); // fly
        const speed = try std.fmt.parseInt(u32, parts.next().?, 10);
        _ = parts.next(); // km/s
        _ = parts.next(); // for
        const duration = try std.fmt.parseInt(u32, parts.next().?, 10);
        _ = parts.next(); // seconds,
        _ = parts.next(); // but
        _ = parts.next(); // then
        _ = parts.next(); // must
        _ = parts.next(); // rest
        _ = parts.next(); // for
        const rest = try std.fmt.parseInt(u32, parts.next().?, 10);

        const champion = Champion{
            .speed = speed,
            .duration = duration,
            .rest = rest,
            .current_distance = 0,
            .remaining = duration,
            .state = State.Running,
            .points = 0,
        };

        try result.append(champion);
    }

    return result;
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    const champions = try parse(allocator, input);
    defer champions.deinit();

    var second: u32 = 0;

    while (second < 2503) : (second += 1) {
        for (champions.items, 0..) |champion, idx| {
            if (champions.items[idx].remaining != 0) {
                if (champions.items[idx].state == State.Running) {
                    champions.items[idx].current_distance += champion.speed;
                }
                champions.items[idx].remaining -= 1;
                continue;
            }

            if (champions.items[idx].state == State.Running) {
                champions.items[idx].state = State.Resting;
                champions.items[idx].remaining = champion.rest - 1;
            } else {
                champions.items[idx].state = State.Running;
                champions.items[idx].remaining = champion.duration - 1;
                champions.items[idx].current_distance += champion.speed;
            }
        }

        var max_distance: u32 = champions.items[0].current_distance;

        for (champions.items) |champion| {
            if (champion.current_distance > max_distance) {
                max_distance = champion.current_distance;
            }
        }
        for (champions.items, 0..) |champion, idx| {
            if (champion.current_distance == max_distance) {
                champions.items[idx].points += 1;
            }
        }
    }

    var score_step1: u32 = 0;
    var score_step2: u32 = 0;

    for (champions.items) |champion| {
        if (champion.current_distance > score_step1) {
            score_step1 = champion.current_distance;
        }
        if (champion.points > score_step2) {
            score_step2 = champion.points;
        }
    }

    return .{ .step1 = score_step1, .step2 = score_step2 };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}
