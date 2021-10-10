using System;
using System.Collections.Generic;

public class RobotNamesRegistry
{
    public static HashSet<string> knownNames = new HashSet<string>();
}

public class Robot
{
    private string name;
    private Random rand;  

    public Robot()
    {
        this.rand = new Random();
        this.Reset();
    }

    public string Name { 
        get { return this.name; }
    }

    public void Reset()
    {
        do {
            this.name = "";

            for(int i = 0; i < 2; i ++) {
                this.name += (char)this.rand.Next('A', 'Z' + 1);
            }

            for(int i = 0; i < 3; i ++) {
                this.name += (char)this.rand.Next('0', '9' + 1);
            }
            
        } while(false == RobotNamesRegistry.knownNames.Add(this.name));
    }
}