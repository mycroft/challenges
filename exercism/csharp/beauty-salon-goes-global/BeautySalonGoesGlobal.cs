using System;
using System.Globalization;

public enum Location
{
    NewYork,
    London,
    Paris
}

public enum AlertLevel
{
    Early,
    Standard,
    Late
}

public static class Appointment
{
    public static DateTime ShowLocalTime(DateTime dtUtc) => dtUtc + TimeZoneInfo.Local.GetUtcOffset(dtUtc);

    public static DateTime Schedule(string appointmentDateDescription, Location location)
    {
        DateTime dt = DateTime.Parse(appointmentDateDescription);

        var tzName = location switch {
            Location.Paris => "Europe/Paris",
            Location.London => "Europe/London",
            Location.NewYork => "America/New_York",
            _ => "UTC"
        };

        var tzi = TimeZoneInfo.FindSystemTimeZoneById(tzName);

        return dt - tzi.GetUtcOffset(dt);
    }

    public static DateTime GetAlertTime(DateTime appointment, AlertLevel alertLevel)
    {
        var delay = alertLevel switch {
            AlertLevel.Early => new TimeSpan(24, 0, 0),
            AlertLevel.Standard => new TimeSpan(1, 45, 0),
            AlertLevel.Late => new TimeSpan(0, 30, 0),
            _ => throw new ArgumentException(),
        };

        return appointment - delay;
    }

    public static bool HasDaylightSavingChanged(DateTime dt, Location location)
    {
        var tzName = location switch {
            Location.Paris => "Europe/Paris",
            Location.London => "Europe/London",
            Location.NewYork => "America/New_York",
            _ => "UTC"
        };

        var tzi = TimeZoneInfo.FindSystemTimeZoneById(tzName);
        return tzi.IsDaylightSavingTime(dt) != tzi.IsDaylightSavingTime(dt.AddDays(-7));
    }

    public static DateTime NormalizeDateTime(string dtStr, Location location)
    {
        var cultureInfoName = location switch {
            Location.Paris => "fr-FR",
            Location.London => "en-GB",
            Location.NewYork => "en-US",
            _ => "UTC"
        };

        try {
            return DateTime.Parse(dtStr, CultureInfo.CreateSpecificCulture(cultureInfoName));
        } catch {
            return new DateTime(1, 1, 1, 0, 0, 0);
        }
    }
}
