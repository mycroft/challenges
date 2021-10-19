using System;
using System.Collections.Generic;

public struct Coord
{
    public Coord(ushort x, ushort y)
    {
        X = x;
        Y = y;
    }

    public ushort X { get; }
    public ushort Y { get; }

    public override bool Equals(object obj)
    {        
        if (obj == null || GetType() != obj.GetType())
            return false;

        Coord other = (Coord)obj;

        return other.X == X && other.Y == Y;
    }
    
    public override int GetHashCode()
        => base.GetHashCode();
}

public struct Plot
{
    public Coord A { get; }
    public Coord B { get; }
    public Coord C { get; }
    public Coord D { get; }

    public Plot(Coord a, Coord b, Coord c, Coord d) {
        A = a;
        B = b;
        C = c;
        D = d;
    }

    public ushort GetLongestSide()
        => (ushort)Math.Max(Math.Abs(A.X - B.X), Math.Abs(B.Y - C.Y));

    public override bool Equals(object obj)
    {
        if (obj == null || GetType() != obj.GetType())
        {
            return false;
        }

        Plot other = (Plot)obj;

        return other.A.Equals(A)
            && other.B.Equals(B)
            && other.C.Equals(C)
            && other.D.Equals(D);
    }

    public override int GetHashCode()
        => base.GetHashCode();
}


public class ClaimsHandler
{
    private HashSet<Plot> claims = new HashSet<Plot>();
    private Plot lastClaim;
 
    public void StakeClaim(Plot plot)
    {
        claims.Add(plot);
        lastClaim = plot;
    }

    public bool IsClaimStaked(Plot plot)
        => claims.Contains(plot);

    public bool IsLastClaim(Plot plot)
        => plot.Equals(lastClaim);

    public Plot GetClaimWithLongestSide()
    {
        int longestPath = 0;
        Object claimWithLongestSide = null;

        foreach(Plot item in claims) {
            if (item.GetLongestSide() > longestPath) {
                longestPath = item.GetLongestSide();
                claimWithLongestSide = item;
            }
        }

        if (claimWithLongestSide == null) {
            throw new ArgumentException();
        }

        return (Plot)claimWithLongestSide;
    }
}
