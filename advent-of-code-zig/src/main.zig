const std = @import("std");

const year2015 = @import("./2015/main.zig").main;

const Year = struct {
    name: []const u8,
    main: fn (std.mem.Allocator) anyerror!void,
};

const YEARS = [_]Year{
    Year{ .name = "2015", .main = year2015 },
};

pub fn main() !void {
    // const stdout = std.io.getStdOut().writer();

    var gpa = std.heap.GeneralPurposeAllocator(.{ .safety = true }){};
    defer _ = std.debug.assert(gpa.deinit() == .ok);
    const allocator = gpa.allocator();

    inline for (YEARS) |year| {
        // try stdout.print("you're running {s}'s challenges!\n", .{year.name});
        try year.main(allocator);
    }
}

test {
    std.testing.refAllDecls(@This());
}
