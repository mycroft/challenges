using System;

public static class LogAnalysis 
{
    public static string SubstringAfter(this string str, string lookAt)
    {
        return str.Substring(str.IndexOf(lookAt) + lookAt.Length);
    }

    public static string SubstringBetween(this string str, string lookBefore, string lookAfter)
    {
        return str.Substring(str.IndexOf(lookBefore) + lookBefore.Length, str.IndexOf(lookAfter) - 1);
    }

    public static string Message(this string str)
    {
        return str.SubstringAfter("]: ");
    }

    public static string LogLevel(this string str)
    {
        return str.SubstringBetween("[", "]");
    }
}