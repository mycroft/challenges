using System;

class RemoteControlCar
{
    private int distance = 0;
    private int battery = 100;

    public static RemoteControlCar Buy()
    {
        return new RemoteControlCar();
    }

    public string DistanceDisplay()
    {
        return $"Driven {distance} meters";
    }

    public string BatteryDisplay()
    {
        string res = $"Battery at {battery}%";
        if (battery <= 0) { 
            res = "Battery empty";
        }

        return res;
    }

    public void Drive()
    {
        if (battery <= 0) {
            return;
        }

        distance += 20;
        battery -= 1;
    }
}
