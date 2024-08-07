const std = @import("std");

pub const Challenge = struct {
    year: []const u8,
    day: []const u8,

    pub fn new(year: []const u8, day: []const u8) Challenge {
        return Challenge{
            .year = year,
            .day = day,
        };
    }
};

pub fn printf(comptime format: []const u8, args: anytype) void {
    const stdout = std.io.getStdOut().writer();

    _ = stdout.print(format, args) catch {};
}

pub fn eprintf(comptime format: []const u8, args: anytype) void {
    const stderr = std.io.getStdErr().writer();

    _ = stderr.print(format, args) catch {};
}
