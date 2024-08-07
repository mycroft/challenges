const std = @import("std");

pub fn file_to_string(allocator: std.mem.Allocator, filepath: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(filepath, .{});
    defer file.close();

    const file_content = try file.readToEndAlloc(allocator, 1024 * 1024);

    return file_content;
}
