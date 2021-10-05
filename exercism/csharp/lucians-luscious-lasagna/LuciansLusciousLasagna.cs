class Lasagna
{
    public int ExpectedMinutesInOven() => 40;

    public int RemainingMinutesInOven(int time_in_oven) => ExpectedMinutesInOven() - time_in_oven;

    public int PreparationTimeInMinutes(int layers) => 2 * layers;

    public int ElapsedTimeInMinutes(int layers, int time_in_oven) => PreparationTimeInMinutes(layers) + time_in_oven;
}
