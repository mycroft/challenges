using System;
using System.Collections.Generic;

public class Authenticator
{
    public Identity Admin {
        get {
            return new Identity{
                Email="admin@ex.ism",
                FacialFeatures=new FacialFeatures{
                    EyeColor="green",
                    PhiltrumWidth=0.9M,
                },
                NameAndAddress=new String[]{"Chanakya"},
            };
        }
    }

    public IDictionary<string, Identity> Developers {
        get {
            return new Dictionary<string, Identity>{
                ["Bertrand"] = new Identity{
                    Email="bert@ex.ism",
                    FacialFeatures=new FacialFeatures{
                        EyeColor="blue",
                    },
                },
                ["Anders"] = new Identity{
                    FacialFeatures=new FacialFeatures{
                        PhiltrumWidth=0.85M,
                    },
                    NameAndAddress=new String[]{"Chanakya", "Redmond"},
                },
            };
        }
    }
}

//**** please do not modify the FacialFeatures class ****
public class FacialFeatures
{
    public string EyeColor { get; set; }
    public decimal PhiltrumWidth { get; set; }
}

//**** please do not modify the Identity class ****
public class Identity
{
    public string Email { get; set; }
    public FacialFeatures FacialFeatures { get; set; }
    public IList<string> NameAndAddress { get; set; }
}
