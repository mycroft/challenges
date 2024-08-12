const std = @import("std");
const aoc = @import("../aoc.zig");
const futils = @import("../futils.zig");
const testing = std.testing;

const ParsingError = error{
    ExpectedInvalid,
};

// we need to parse the string here, with a lexer!
const LexicalKind = enum {
    LeftBrace, // {
    LeftBraket, // [
    RightBrace, // }
    RightBraket, // ]
    String, // "abc"
    Integer, // -?123
    Comma, // ,
    Colon, // :
};

const ExprKind = enum {
    Integer,
    String,
    Array,
    Hash,
};

const Expression = struct {
    kind: ExprKind,
    int_value: ?i32 = null,
    str_value: ?[]const u8 = null,

    // note: in case of a hash, we're dropping keys, as they do not matter;
    // we'll only store a list of sub values.
    content: ?std.ArrayList(Expression) = null,

    fn clean(self: Expression) void {
        if (self.content == null) {
            return;
        }

        for (self.content.?.items) |e| {
            e.clean();
        }

        self.content.?.deinit();
    }

    fn compute(self: Expression, with_filter: bool) i32 {
        var res: i32 = 0;

        switch (self.kind) {
            ExprKind.String => {},
            ExprKind.Integer => {
                res += self.int_value.?;
            },
            ExprKind.Array => {
                for (self.content.?.items) |item| {
                    res += item.compute(with_filter);
                }
            },
            ExprKind.Hash => {
                var has_red = false;

                for (self.content.?.items) |item| {
                    if (with_filter and item.kind == ExprKind.String and std.mem.eql(u8, item.str_value.?, "red")) {
                        has_red = true;
                    }
                }

                if (!has_red) {
                    for (self.content.?.items) |item| {
                        res += item.compute(with_filter);
                    }
                }
            },
        }

        return res;
    }
};

const Lexical = struct { kind: LexicalKind, value: []const u8 };

fn get_lexicals(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Lexical) {
    var lexicals = std.ArrayList(Lexical).init(allocator);
    var idx: usize = 0;

    while (idx < input.len) : (idx += 1) {
        switch (input[idx]) {
            '[' => try lexicals.append(.{ .kind = LexicalKind.LeftBraket, .value = "" }),
            ']' => try lexicals.append(.{ .kind = LexicalKind.RightBraket, .value = "" }),
            '{' => try lexicals.append(.{ .kind = LexicalKind.LeftBrace, .value = "" }),
            '}' => try lexicals.append(.{ .kind = LexicalKind.RightBrace, .value = "" }),
            ',' => try lexicals.append(.{ .kind = LexicalKind.Comma, .value = "" }),
            ':' => try lexicals.append(.{ .kind = LexicalKind.Colon, .value = "" }),
            '"' => {
                const start_str_idx = idx;
                idx += 1;
                while (input[idx] != '"') {
                    idx += 1;
                }
                const end_str_idx = idx;

                try lexicals.append(.{ .kind = LexicalKind.String, .value = input[start_str_idx + 1 .. end_str_idx] });
            },
            '0'...'9', '-' => {
                const start_str_idx = idx;
                idx += 1;
                while (input[idx] >= '0' and input[idx] <= '9') {
                    idx += 1;
                }
                const end_str_idx = idx;
                idx -= 1; // we're gone too far, let's back off by one
                try lexicals.append(.{ .kind = LexicalKind.Integer, .value = input[start_str_idx..end_str_idx] });
            },
            else => {},
        }
    }

    return lexicals;
}

fn parse(allocator: std.mem.Allocator, idx: *usize, lexicals: std.ArrayList(Lexical)) !Expression {
    if (lexicals.items[idx.*].kind == LexicalKind.Integer) {
        idx.* += 1;
        return Expression{ .kind = ExprKind.Integer, .int_value = try std.fmt.parseInt(i32, lexicals.items[idx.* - 1].value, 10) };
    }

    if (lexicals.items[idx.*].kind == LexicalKind.String) {
        idx.* += 1;
        return Expression{ .kind = ExprKind.String, .str_value = lexicals.items[idx.* - 1].value };
    }

    // array
    if (lexicals.items[idx.*].kind == LexicalKind.LeftBraket) {
        var list = std.ArrayList(Expression).init(allocator);
        idx.* += 1; // left braket

        while (idx.* < lexicals.items.len and lexicals.items[idx.*].kind != LexicalKind.RightBraket) {
            const expr = try parse(allocator, idx, lexicals);
            try list.append(expr);

            if (lexicals.items[idx.*].kind == LexicalKind.Comma) {
                idx.* += 1;
            } else if (lexicals.items[idx.*].kind != LexicalKind.RightBraket) {
                return ParsingError.ExpectedInvalid;
            }
        }

        idx.* += 1; // right braket

        return Expression{ .kind = ExprKind.Array, .content = list };
    }

    // hash
    if (lexicals.items[idx.*].kind == LexicalKind.LeftBrace) {
        var list = std.ArrayList(Expression).init(allocator);
        idx.* += 1; // left brace

        while (idx.* < lexicals.items.len and lexicals.items[idx.*].kind != LexicalKind.RightBrace) {
            if (lexicals.items[idx.*].kind != LexicalKind.String) {
                return ParsingError.ExpectedInvalid;
            }
            idx.* += 1;

            if (lexicals.items[idx.*].kind != LexicalKind.Colon) {
                return ParsingError.ExpectedInvalid;
            }
            idx.* += 1;

            const expr = try parse(allocator, idx, lexicals);
            try list.append(expr);

            if (lexicals.items[idx.*].kind == LexicalKind.Comma) {
                idx.* += 1;
            } else if (lexicals.items[idx.*].kind != LexicalKind.RightBrace) {
                return ParsingError.ExpectedInvalid;
            }
        }

        idx.* += 1; // right brace

        return Expression{ .kind = ExprKind.Hash, .content = list };
    }

    return ParsingError.ExpectedInvalid;
}

fn solve(allocator: std.mem.Allocator, input: []const u8) !struct { step1: u32, step2: u32 } {
    const lexicals = try get_lexicals(allocator, input);
    defer lexicals.deinit();

    var idx: usize = 0;
    const root_expr = try parse(allocator, &idx, lexicals);

    const step1 = root_expr.compute(false);
    const step2 = root_expr.compute(true);

    root_expr.clean();

    return .{ .step1 = @intCast(step1), .step2 = @intCast(step2) };
}

pub fn main(allocator: std.mem.Allocator, challenge: aoc.Challenge) anyerror!void {
    const file_content = try futils.get_challenge_input(allocator, challenge);
    defer allocator.free(file_content);

    const result = try solve(allocator, file_content);

    aoc.printf("step1: {d}\n", .{result.step1});
    aoc.printf("step2: {d}\n", .{result.step2});
}
