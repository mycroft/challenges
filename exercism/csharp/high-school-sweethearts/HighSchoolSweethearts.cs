using System;
using System.Globalization;
using System.Threading;


public static class HighSchoolSweethearts
{
    public static string DisplaySingleLine(string studentA, string studentB) => $"{studentA,29} ♡ {studentB,-29}";

    public static string DisplayBanner(string studentA, string studentB)
    {
        return @$"
     ******       ******
   **      **   **      **
 **         ** **         **
**            *            **
**                         **
**     {studentA} +  {studentB}    **
 **                       **
   **                   **
     **               **
       **           **
         **       **
           **   **
             ***
              *
";
    }

    public static string DisplayGermanExchangeStudents(string studentA
        , string studentB, DateTime start, float hours)
        => $"{studentA} and {studentB} have been dating since {start:dd.MM.yyyy} - that's {hours.ToString("n2", CultureInfo.CreateSpecificCulture("de-DE"))} hours";
}
