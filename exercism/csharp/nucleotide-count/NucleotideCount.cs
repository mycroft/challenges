using System;
using System.Collections.Generic;

public static class NucleotideCount
{
    public static IDictionary<char, int> Count(string sequence)
    {
        Dictionary<char, int> dict = new Dictionary<char, int> {
            ['A'] = 0,
            ['C'] = 0,
            ['G'] = 0,
            ['T'] = 0,
        };

        foreach(char c in sequence) {
            if (!dict.ContainsKey(c)) {
                throw new ArgumentException();
            }
            dict[c] += 1;
        }
        
        return dict;
    }
}