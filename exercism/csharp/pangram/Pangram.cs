using System;
using System.Collections.Generic;

public static class Pangram
{
    public static bool IsPangram(string input)
    {
        var letters = new Dictionary<char, bool>();

        foreach(char letter in input) {
            var l = char.ToLower(letter);
            if(l < 'a' || l > 'z') {
                continue;
            }

            if(!letters.ContainsKey(l)) {
                letters.Add(l, true);
            }
        }

        if (letters.Count != 26) {
            return false;
        }

        return true;
    }
}
