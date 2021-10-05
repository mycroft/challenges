using System;

static class LogLine
{
    public static string Message(string logLine)
    {
        int index = logLine.IndexOf(' ');

        return logLine.Substring(index + 1).Trim();
    }

    public static string LogLevel(string logLine)
    {
        int ll_start = logLine.IndexOf('[') + 1;
        int ll_end = logLine.IndexOf(']') - 1;

        return logLine.Substring(ll_start, ll_end).Trim().ToLower();
    }

    public static string Reformat(string logLine)
    {
        string loglevel = LogLevel(logLine);
        string message = Message(logLine);

        return $"{message} ({loglevel})";
    }
}
