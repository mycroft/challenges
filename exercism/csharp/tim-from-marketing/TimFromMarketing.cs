using System;

static class Badge
{
    public static string Print(int? id, string name, string? department)
    {
        string res = name;

        if (id != null) {
            res = $"[{id}] - " + res;
        }

        if (department == null) {
            department = "owner";
        }

        res += $" - {department.ToUpper()}";

        return res;
    }
}
