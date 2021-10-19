using System;
using System.Collections.Generic;

public class FacialFeatures
{
    public string EyeColor { get; }
    public decimal PhiltrumWidth { get; }

    public FacialFeatures(string eyeColor, decimal philtrumWidth)
    {
        EyeColor = eyeColor;
        PhiltrumWidth = philtrumWidth;
    }

    public override bool Equals(Object obj)
    {
        if (obj == null || !this.GetType().Equals(obj.GetType())) {
            return false;
        }

        FacialFeatures of = (FacialFeatures)obj;
        return of.EyeColor == EyeColor && of.PhiltrumWidth == PhiltrumWidth;
    }

    public override int GetHashCode()
    {
        string s = $"{this.EyeColor}:{this.PhiltrumWidth}";
        return s.GetHashCode();
    }
}

public class Identity
{
    public string Email { get; }
    public FacialFeatures FacialFeatures { get; }

    public Identity(string email, FacialFeatures facialFeatures)
    {
        Email = email;
        FacialFeatures = facialFeatures;
    }

    public override bool Equals(Object o)
    {
        if ((o == null) || !o.GetType().Equals(o.GetType())) {
            return false;
        }

        Identity oi = (Identity)o;
        return oi.Email == this.Email && oi.FacialFeatures.Equals(this.FacialFeatures);
    }

    public override int GetHashCode()
        => $"{Email}:{FacialFeatures.EyeColor}:{FacialFeatures.PhiltrumWidth}".GetHashCode();
}

public class Authenticator
{
    private HashSet<Identity> registered_ids = new HashSet<Identity>();

    public static bool AreSameFace(FacialFeatures faceA, FacialFeatures faceB)
        => faceA.Equals(faceB);


    public bool IsAdmin(Identity identity)
        => identity.Equals(new Identity("admin@exerc.ism", new FacialFeatures("green", 0.9m)));

    public bool Register(Identity identity)
    {
        if (this.IsRegistered(identity)) {
            return false;
        }
        return registered_ids.Add(identity);
    }

    public bool IsRegistered(Identity identity)
        => registered_ids.Contains(identity);

    public static bool AreSameObject(Identity identityA, Identity identityB)
        => identityA == identityB;
}
