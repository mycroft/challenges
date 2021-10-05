using System;

public class Player
{
    public int RollDie()
    {
        return new Random().Next(1, 18);
    }

    public double GenerateSpellStrength()
    {
        return new Random().NextDouble() * 100.0;
    }
}
