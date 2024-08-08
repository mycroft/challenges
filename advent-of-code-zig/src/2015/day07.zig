const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

const Operator = enum {
    Apply,
    And,
    Or,
    Not,
    RShift,
    LShift,
};

const Inst = struct {
    op: Operator,
    left_value: []const u8,
    right_value: ?[]const u8 = null,
    dest: []const u8,
    used: bool = false,
};

fn parse(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Inst) {
    var iter = std.mem.split(u8, input, "\n");

    var instructions = std.ArrayList(Inst).init(allocator);

    while (iter.next()) |line| {
        if (line.len == 0) {
            break;
        }

        var iter_line = std.mem.split(u8, line, " ");
        var inst: ?Inst = null;

        if (std.mem.indexOf(u8, line, "OR") != null) {
            const f0 = iter_line.next().?;
            _ = iter_line.next(); // OR
            const f1 = iter_line.next().?;
            _ = iter_line.next(); // ->
            const f2 = iter_line.next().?;

            inst = Inst{ .op = Operator.Or, .left_value = f0, .right_value = f1, .dest = f2 };
        } else if (std.mem.indexOf(u8, line, "AND") != null) {
            const f0 = iter_line.next().?;
            _ = iter_line.next(); // AND
            const f1 = iter_line.next().?;
            _ = iter_line.next(); // ->
            const f2 = iter_line.next().?;

            inst = Inst{ .op = Operator.And, .left_value = f0, .right_value = f1, .dest = f2 };
        } else if (std.mem.indexOf(u8, line, "LSHIFT") != null) {
            const f0 = iter_line.next().?;
            _ = iter_line.next(); // LSHIFT
            const f1 = iter_line.next().?;
            _ = iter_line.next(); // ->
            const f2 = iter_line.next().?;

            inst = Inst{ .op = Operator.LShift, .left_value = f0, .right_value = f1, .dest = f2 };
        } else if (std.mem.indexOf(u8, line, "RSHIFT") != null) {
            const f0 = iter_line.next().?;
            _ = iter_line.next(); // RSHIFT
            const f1 = iter_line.next().?;
            _ = iter_line.next(); // ->
            const f2 = iter_line.next().?;

            inst = Inst{ .op = Operator.RShift, .left_value = f0, .right_value = f1, .dest = f2 };
        } else if (std.mem.indexOf(u8, line, "NOT") != null) {
            _ = iter_line.next(); // NOT
            const f0 = iter_line.next().?;
            _ = iter_line.next(); // ->
            const f1 = iter_line.next().?;

            inst = Inst{ .op = Operator.Not, .left_value = f0, .dest = f1 };
        } else if (std.mem.indexOf(u8, line, "->") != null) {
            const f0 = iter_line.next().?;
            _ = iter_line.next(); // ->
            const f1 = iter_line.next().?;

            inst = Inst{ .op = Operator.Apply, .left_value = f0, .dest = f1 };
        } else {
            aoc.printf("unparsable line: s{s}\n", .{line});
        }

        if (inst != null) {
            try instructions.append(inst.?);
        }
    }

    return instructions;
}

fn solve(allocator: std.mem.Allocator, input: []const u8, override: ?u32) !u32 {
    const instructions = try parse(allocator, input);
    defer instructions.deinit();

    var values = std.StringHashMap(u32).init(allocator);
    defer values.deinit();

    if (override != null) {
        try values.put("b", override.?);
    }

    var rule_was_used = false;

    while (true) {
        rule_was_used = false;
        for (instructions.items, 0..) |instruction, idx| {
            if (instruction.used) {
                continue;
            }

            var res_value: u32 = undefined;

            if (instruction.op == Operator.Apply) {
                res_value = std.fmt.parseInt(u32, instruction.left_value, 10) catch values.get(instruction.left_value) orelse continue;
            } else if (instruction.op == Operator.Not) {
                const lvalue = std.fmt.parseInt(u32, instruction.left_value, 10) catch values.get(instruction.left_value) orelse continue;
                res_value = ~lvalue;
            } else if (instruction.op == Operator.And) {
                const lvalue = std.fmt.parseInt(u32, instruction.left_value, 10) catch values.get(instruction.left_value) orelse continue;
                const rvalue = std.fmt.parseInt(u32, instruction.right_value.?, 10) catch values.get(instruction.right_value.?) orelse continue;

                res_value = lvalue & rvalue;
            } else if (instruction.op == Operator.Or) {
                const lvalue = std.fmt.parseInt(u32, instruction.left_value, 10) catch values.get(instruction.left_value) orelse continue;
                const rvalue = std.fmt.parseInt(u32, instruction.right_value.?, 10) catch values.get(instruction.right_value.?) orelse continue;

                res_value = lvalue | rvalue;
            } else if (instruction.op == Operator.LShift) {
                const lvalue = std.fmt.parseInt(u32, instruction.left_value, 10) catch values.get(instruction.left_value) orelse continue;
                const rvalue = std.fmt.parseInt(u32, instruction.right_value.?, 10) catch values.get(instruction.right_value.?) orelse continue;

                res_value = lvalue << @intCast(rvalue);
            } else if (instruction.op == Operator.RShift) {
                const lvalue = std.fmt.parseInt(u32, instruction.left_value, 10) catch values.get(instruction.left_value) orelse continue;
                const rvalue = std.fmt.parseInt(u32, instruction.right_value.?, 10) catch values.get(instruction.right_value.?) orelse continue;

                res_value = lvalue >> @intCast(rvalue);
            }

            try values.put(instruction.dest, res_value);

            if (override != null and std.mem.eql(u8, "b", instruction.dest)) {
                try values.put("b", override.?);
            }

            instructions.items[idx].used = true;
            rule_was_used = true;
        }

        if (!rule_was_used) {
            break;
        }
    }

    return values.get("a").?;
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const step_1_result = try solve(allocator, file_content, null);
    const step_2_result = try solve(allocator, file_content, step_1_result);

    aoc.printf("step1: {d}\n", .{step_1_result});
    aoc.printf("step2: {d}\n", .{step_2_result});
}
