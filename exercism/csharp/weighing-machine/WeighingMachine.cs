using System;

enum Unit
{
    Pounds,
    Kilograms
}

class WeighingMachine
{
    private decimal inputWeightValue;

    public WeighingMachine()
    {
        Unit = Unit.Kilograms;
    }

    public decimal InputWeight
    {
        get {
            return inputWeightValue;
        }
        set {
            if (value < 0) {
                throw new ArgumentOutOfRangeException();
            }
            inputWeightValue = value;
        }
    }

    public int DisplayWeight {
        get {
            return (int)(inputWeightValue - TareAdjustment);
        }
    }

    public USWeight USDisplayWeight {
        get {
            switch (Unit) {
                case Unit.Kilograms:
                    return new USWeight((inputWeightValue + TareAdjustment) * (decimal)2.20462);
                case Unit.Pounds:
                    return new USWeight(inputWeightValue + TareAdjustment);
                default:
                    throw new ArgumentException();
            };
        }
        set {
            switch (Unit) {
                case Unit.Kilograms:
                    inputWeightValue = value.Pounds / (decimal)2.20462;
                    break;
                case Unit.Pounds:
                    inputWeightValue = value.Pounds;
                    break;
            };
        }
    }
    public int TareAdjustment { get; set; }
    public Unit Unit { get; set; }
}

struct USWeight
{
    private decimal weightInPounds;

    public USWeight(decimal weightInPounds)
    {
        this.weightInPounds = weightInPounds;
    }

    public int Pounds
    {
        get {
            return (int)weightInPounds;
        }
        set {
            weightInPounds = value;
        }
    }

    public int Ounces
    {
        get {
            return (int)((weightInPounds * 16) % 16);
        }
        set {
            weightInPounds = value * 16;
        }
    }
}
