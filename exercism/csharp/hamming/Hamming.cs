using System;

public static class Hamming
{
    public static int Distance(string firstStrand, string secondStrand)
    {
        int count = 0;

        if (firstStrand.Length != secondStrand.Length) {
            throw new ArgumentException();
        }

        for (int i = 0; i < firstStrand.Length; i ++) {
            if (firstStrand[i] != secondStrand[i]) {
                count += 1;
            }
        }

        return count;
    }
}