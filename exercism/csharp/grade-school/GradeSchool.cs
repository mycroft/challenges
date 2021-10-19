using System;
using System.Collections.Generic;
using System.Linq;

public class GradeSchool
{
    private Dictionary<string, int> grades = new Dictionary<string, int>();

    public void Add(string student, int grade)
        => grades.Add(student, grade);

    public IEnumerable<string> Roster()
        => from entry in grades
            orderby entry.Key
            orderby entry.Value
            select entry.Key;

    public IEnumerable<string> Grade(int grade)
        => from entry in grades
            where entry.Value == grade
            orderby entry.Key
            select entry.Key;
}
