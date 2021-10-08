using System;

public static class SimpleCalculator
{
    public static string Calculate(int operand1, int operand2, string operation)
    {
        int result = 0;

        try {
            switch(operation) {
                case "+":
                    result = operand1 + operand2;
                    break;
                case "*":
                    result = operand1 * operand2;
                    break;
                case "/":
                    result = operand1 / operand2;
                    break;
                case "":
                    throw new ArgumentException();
                case null:
                    throw new ArgumentNullException();
                default:
                    throw new ArgumentOutOfRangeException();
            }
        } catch(DivideByZeroException) {
            return "Division by zero is not allowed.";
        }

        return $"{operand1} {operation} {operand2} = {result}";
    }
}
