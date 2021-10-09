using System;

public static class CentralBank
{
    public static string DisplayDenomination(long @base, long multiplier)
    {
        long total = @base * multiplier;

        if (total < 0) {
            return "*** Too Big ***";
        }

        return $"{total}";
    }

    public static string DisplayGDP(float @base, float multiplier)
    {
        float total = @base * multiplier;

        if (Double.IsInfinity(total)) {
            return "*** Too Big ***";
        }

        return $"{total}";
    }

    public static string DisplayChiefEconomistSalary(decimal salaryBase, decimal multiplier)
    {
        decimal total;

        try {
            total = salaryBase * multiplier;
        } catch (OverflowException) {
            return "*** Much Too Big ***";
        }

        return $"{total}";
    }
}
