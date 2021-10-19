using System;
using System.Collections.Generic;
using System.Linq;

public class HighScores
{
    private List<int> scores = new List<int>();
    public HighScores(List<int> list)
    {
        scores = list;
    }

    public List<int> Scores()
        => scores;

    public int Latest()
        => scores[scores.Count - 1];

    public int PersonalBest()
        => scores.Max();

    public List<int> PersonalTopThree()
        => scores.OrderByDescending(x => x).Take(3).ToList();
}