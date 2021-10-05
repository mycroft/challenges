using System;

class BirdCount
{
    private int[] birdsPerDay;

    public BirdCount(int[] birdsPerDay)
    {
        this.birdsPerDay = birdsPerDay;
    }

    public static int[] LastWeek()
    {
        return new int[] { 0, 2, 5, 3, 7, 8, 4 };
    }

    public int Today()
    {
        return birdsPerDay[birdsPerDay.Length - 1];
    }

    public void IncrementTodaysCount()
    {
        birdsPerDay[birdsPerDay.Length - 1] += 1;
    }

    public bool HasDayWithoutBirds()
    {
        for(int i = 0; i < birdsPerDay.Length; i ++) {
            if (birdsPerDay[i] == 0) {
                return true;
            }
        }

        return false;
    }

    public int CountForFirstDays(int numberOfDays)
    {
        int count = 0;
        for (int i = 0; i < numberOfDays; i ++) {
            count += birdsPerDay[i];
        }

        return count;
    }

    public int BusyDays()
    {
        int count = 0;
        foreach (int c in birdsPerDay) {
            if (c >= 5) {
                count ++;
            }
        }

        return count;
    }
}
