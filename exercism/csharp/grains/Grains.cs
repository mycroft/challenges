using System;

public static class Grains
{
    public static ulong Square(int n)
        => n == 1 ? 1 :
            (n > 1 && n <= 64) ? 2 * Square(n - 1) :
            throw new ArgumentOutOfRangeException();

    public static ulong Total()
    {
        ulong total = 0;
        for(int i = 1; i <= 64; i ++) {
            total += Square(i);
        }
        return total;
    }
}