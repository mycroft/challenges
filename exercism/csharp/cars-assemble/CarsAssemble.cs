using System;

static class AssemblyLine
{
    public static double ProductionRatePerHour(int speed)
    {
        double production = 221.0 * speed;

        switch (speed)
        {
            case <= 4:
                break;

            case <= 8:
                production *= 0.9;
                break;

            case 9:
                production *= 0.8;
                break;
            
            case 10:
                production *= 0.77;
                break;

        };

        return production;
    }

    public static int WorkingItemsPerMinute(int speed)
    {
        return (int)(Math.Floor(ProductionRatePerHour(speed)) / 60);
    }
}
