using System;

public static class PlayAnalyzer
{
    public static string AnalyzeOnField(int shirtNum)
    {
        return shirtNum switch {
            1 => "goalie",
            2 => "left back",
            3 => "left right",
            4 => "left right",
            5 => "right back",
            6 => "midfielder",
            7 => "midfielder",
            8 => "midfielder",
            9 => "left wing",
            10 => "striker",
            11 => "right wing",
            _ => throw new ArgumentOutOfRangeException("Invalid shirt num")
        };
    }

    public static string AnalyzeOffField(object report)
    {
        switch (report.GetType().ToString()) {
            case "System.Int32":
                return $"There are {report} supporters at the match.";
            case "System.String":
                return (string)report;
            case "Incident":
                return ((Incident)report).GetDescription();
            case "Foul":
                return ((Foul)report).GetDescription();
            case "Injury":
                var description = ((Injury)report).GetDescription();
                return $"Oh no! {description} Medics are on the field.";
            case "Manager":
                var manager = (Manager)report;
                var output = $"{manager.Name}";
                if (manager.Club != null) {
                    output += $" ({manager.Club})";
                }
                return output;
            default:
                throw new ArgumentException();
        };
    }
}
