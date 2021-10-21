using System;

public class RemoteControlCar
{
    private int batteryPercentage = 100;
    private int distanceDrivenInMeters = 0;
    private string[] sponsors = new string[0];
    private int latestSerialNum = 0;

    public void Drive()
    {
        if (batteryPercentage > 0)
        {
            batteryPercentage -= 10;
            distanceDrivenInMeters += 2;
        }
    }

    public void SetSponsors(params string[] sponsors)
    {
        this.sponsors = sponsors;
    }

    public string DisplaySponsor(int sponsorNum)
    {
        return sponsors[sponsorNum];
    }

    public bool GetTelemetryData(ref int serialNum,
        out int batteryPercentage, out int distanceDrivenInMeters)
    {
        if (serialNum < latestSerialNum) {
            batteryPercentage = -1;
            distanceDrivenInMeters = -1;
            serialNum = latestSerialNum;

            return false;
        }

        batteryPercentage = this.batteryPercentage;
        distanceDrivenInMeters = this.distanceDrivenInMeters;
        this.latestSerialNum = serialNum;

        return true;
    }

    public static RemoteControlCar Buy()
    {
        return new RemoteControlCar();
    }
}

public class TelemetryClient
{
    private RemoteControlCar car;

    public TelemetryClient(RemoteControlCar car)
    {
        this.car = car;
    }

    public string GetBatteryUsagePerMeter(int serialNum)
    {
        int batteryPercentage, distanceDrivenInMeters;
        var ret = car.GetTelemetryData(ref serialNum, out batteryPercentage, out distanceDrivenInMeters);

        if (ret == false || distanceDrivenInMeters == 0) {
            return "no data";
        }
        else
        {
            return $"usage-per-meter={(100 - batteryPercentage) / distanceDrivenInMeters}";
        }
            
    }
}
