using System;

class RemoteControlCar
{
    private int speed = 0;
    private int battery = 100;
    private int distance = 0;
    private int batteryDrain;

    public RemoteControlCar(int speed, int batteryDrain)
    {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
    }

    public bool BatteryDrained()
    {
        return battery < batteryDrain;
    }

    public int DistanceDriven()
    {
        return distance;
    }

    public void Drive()
    {
        if (batteryDrain > battery) {
            return;
        }
        battery -= batteryDrain;
        distance += speed;
    }

    public static RemoteControlCar Nitro()
    {
        return new RemoteControlCar(50, 4);
    }
}

class RaceTrack
{
    private int distance = 0;

    public RaceTrack(int distance)
    {
        this.distance = distance;
    }

    public bool CarCanFinish(RemoteControlCar car)
    {
        do {
            car.Drive();
        } while (!car.BatteryDrained());

        return car.DistanceDriven() >= distance;
    }
}
