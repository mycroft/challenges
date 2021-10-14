using System;
using System.Collections.Generic;

public static class Isogram
{
    public static bool IsIsogram(string word)
    {
        HashSet<char> d = new HashSet<char>();
        foreach(char c_orig in word) {
            char c = Char.ToLower(c_orig);
            if(c < 'a' || c > 'z') {
                continue;
            }

            if (d.Contains(c)) {
                return false;
            }

            d.Add(c);
        }

        return true;
    }
}
