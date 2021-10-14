using System;
using System.Collections.Generic;

public interface IRemoteControlCar
{
    void Drive();
    int DistanceTravelled { get; }
};

public class ProductionRemoteControlCar : IRemoteControlCar, IComparable
{
    public int DistanceTravelled { get; private set; }
    public int NumberOfVictories { get; set; }

    public void Drive()
    {
        DistanceTravelled += 10;
    }

    public int CompareTo(object obj) {
        return this.NumberOfVictories - ((ProductionRemoteControlCar)obj).NumberOfVictories;
    }
}

public class ExperimentalRemoteControlCar : IRemoteControlCar
{
    public int DistanceTravelled { get; private set; }

    public void Drive()
    {
        DistanceTravelled += 20;
    }
}

public static class TestTrack
{
    public static void Race(IRemoteControlCar car)
    {
        car.Drive();       
    }

    public static List<ProductionRemoteControlCar> GetRankedCars(ProductionRemoteControlCar prc1,
        ProductionRemoteControlCar prc2)
    {
        List<ProductionRemoteControlCar> results = new List<ProductionRemoteControlCar>();

        if (prc1.CompareTo(prc2) <= 0) {
            results.Add(prc1);
            results.Add(prc2);
        } else {
            results.Add(prc2);
            results.Add(prc1);
        }
        return results;
    }
}
