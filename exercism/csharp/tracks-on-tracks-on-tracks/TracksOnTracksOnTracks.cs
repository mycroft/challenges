using System;
using System.Collections.Generic;

public static class Languages
{
    public static List<string> NewList()
    {
        return new List<string>();
    }

    public static List<string> GetExistingLanguages()
    {
        return new List<string>() { "C#", "Clojure", "Elm" };
    }

    public static List<string> AddLanguage(List<string> languages, string language)
    {
        languages.Add(language);
        return languages;
    }

    public static int CountLanguages(List<string> languages)
    {
        return languages.Count;
    }

    public static bool HasLanguage(List<string> languages, string language)
    {
        foreach(string lang in languages) {
            if (lang == language) {
                return true;
            }
        }

        return false;
    }

    public static List<string> ReverseList(List<string> languages)
    {
        languages.Reverse();
        return languages;
    }

    public static bool IsExciting(List<string> languages)
    {
        return (languages.Count >= 1 && languages[0] == "C#") || ((languages.Count == 2 || languages.Count == 3) && languages[1] == "C#");
    }

    public static List<string> RemoveLanguage(List<string> languages, string language)
    {
        var list = NewList();

        foreach(string l in languages) {
            if (l == language) {
                continue;
            }
            list.Add(l);
        }

        return list;
    }

    public static bool IsUnique(List<string> languages)
    {
        List<string> duplicates = NewList();

        foreach(string l in languages) {
            if (HasLanguage(duplicates, l)) {
                return false;
            } else {
                AddLanguage(duplicates, l);
            }
        }

        return true;
    }
}
