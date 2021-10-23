using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

public class LogParser
{
    public bool IsValidLine(string text)
        => new Regex(@"^\[(.*)\] (.*)$").IsMatch(text);

    public string[] SplitLogLine(string text)
    {
        Regex rx = new Regex(@"(.*)<.*>(.*)<.*>(.*)$");
        MatchCollection matches = rx.Matches(text);

        var res = new List<string>();

        foreach (Match match in matches)
        {
            GroupCollection groups = match.Groups;
            res.Add(groups[1].Value);
            res.Add(groups[2].Value);
            res.Add(groups[3].Value);
        }

        if (res.Count == 0) {
            res.Add(string.Empty);
        }

        return res.ToArray();
    }

    public int CountQuotedPasswords(string lines)
    {
        var count = 0;
        Regex rx = new Regex("\".*password", RegexOptions.IgnoreCase);

        foreach(string line in lines.Split("\n")) {
            if (rx.IsMatch(line)) {
                count += 1;
            }
        }

        return count;
    }

    public string RemoveEndOfLineText(string line)
        => new Regex("end-of-line[^ ]*").Replace(line, "");

    public string[] ListLinesWithPasswords(string[] lines)
    {
        Regex rx = new Regex("(password[^ ]+)", RegexOptions.IgnoreCase);
        List<string> res = new List<string>();

        foreach(string line in lines) {
            if (rx.IsMatch(line)) {
                MatchCollection matches = rx.Matches(line);
                GroupCollection groups = matches[0].Groups;
                res.Add(groups[1].Value + ": " + line);
                
            } else {
                res.Add("--------: " + line);
            }
            
        }

        return res.ToArray();
    }
}
