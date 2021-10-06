using System;

public static class ReverseString
{
    public static string Reverse(string input)
    {
        string res = "";

        // or foreach(char c in input.ToCharArray())
        for(int i = 0; i < input.Length; i ++) {
            res = input[i] + res;
        }

        return res;

        // or
        // using System.Linq;
        // return new String(input.Reverse().ToArray());
    }
}