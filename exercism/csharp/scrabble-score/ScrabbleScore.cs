using System;
using System.Collections.Generic;

public static class ScrabbleScore
{
    public static int Score(string input)
    {
        int score = 0;

        Dictionary<string, int> letters = new Dictionary<string, int>();
        letters.Add("AEIOULNRST", 1);
        letters.Add("DG", 2);
        letters.Add("BCMP", 3);
        letters.Add("FHVWY", 4);
        letters.Add("K", 5);
        letters.Add("JX", 8);
        letters.Add("QZ", 10);

        foreach(var item in input) {
            foreach(var (str, z) in letters) {
                if (Array.Exists(str.ToCharArray(), el => el == Char.ToUpper(item))) {
                    score += z;
                }
            }
        }

        return score;
    }
}