const std = @import("std");
const aoc = @import("./aoc.zig");

pub fn file_to_string(allocator: std.mem.Allocator, filepath: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(filepath, .{});
    defer file.close();

    const file_content = try file.readToEndAlloc(allocator, 1024 * 1024);

    return file_content;
}

pub fn get_input_filepath(allocator: std.mem.Allocator, challenge: aoc.Challenge) ![]const u8 {
    return try std.fmt.allocPrint(allocator, "./input/{s}/{s}.txt", .{ challenge.year, challenge.day });
}

pub fn get_challenge_input(allocator: std.mem.Allocator, challenge: aoc.Challenge) ![]const u8 {
    const filepath = try get_input_filepath(allocator, challenge);
    defer allocator.free(filepath);

    return try file_to_string(allocator, filepath);
}
