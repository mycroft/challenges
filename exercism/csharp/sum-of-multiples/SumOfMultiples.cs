using System;
using System.Collections.Generic;

public static class SumOfMultiples
{
    public static int Sum(IEnumerable<int> multiples, int max)
    {
        int sum = 0;

        for(int i = 1; i < max; i ++) {
            foreach(int n in multiples) {
                if (n != 0 && i % n == 0) {
                    sum += i;
                    break;
                }
            }
        }

        return sum;
    }
}