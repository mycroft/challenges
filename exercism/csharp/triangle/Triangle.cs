using System;

public static class Triangle
{
    public static bool IsInvalid(double side1, double side2, double side3)
    {
        if(side1 == 0 || side2 == 0 || side3 == 0) {
            return true;
        }
        if(side1 > side2 && side1 > side3) {
            return side1 > (side2+side3);
        }
        if(side2 > side1 && side2 > side3) {
            return side2 > (side1+side3);
        }
        if(side3 > side1 && side3 > side1) {
            return side3 > (side1+side2);
        }
        return false;
    }

    public static bool IsScalene(double side1, double side2, double side3)
        => !IsInvalid(side1, side2, side3) && side1 != side2 && side2 != side3;

    public static bool IsIsosceles(double side1, double side2, double side3)
        => !IsInvalid(side1, side2, side3) && (side1 == side2 || side1 == side3 || side2 == side3);

    public static bool IsEquilateral(double side1, double side2, double side3) 
        => !IsInvalid(side1, side2, side3) && side1 == side2 && side2 == side3;
}