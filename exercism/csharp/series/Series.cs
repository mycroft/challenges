using System;
using System.Collections.Generic;

public static class Series
{
    public static string[] Slices(string numbers, int sliceLength)
    {
        List<string> results = new List<string>();

        if (sliceLength <= 0 || sliceLength > numbers.Length) {
            throw new ArgumentException();
        }

        for(int i = 0; i <= numbers.Length - sliceLength; i ++) {
            results.Add(numbers[i..(i + sliceLength)]);
        }

        return results.ToArray();
    }
}