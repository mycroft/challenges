using System;

public class PhoneNumber
{
    public static string Clean(string phoneNumber)
    {
        string result = phoneNumber
            .Replace(" ", "")
            .Replace("(", "")
            .Replace(")", "")
            .Replace("-", "")
            .Replace(".", "")
            .Replace("+", "");

        if(result.Length == 11 && result[0] == '1') {
            result = result.Remove(0, 1);
        }

        if(result.Length != 10) {
            throw new ArgumentException();
        }

        // Check all chars are 0-9.
        foreach(var item in result) {
            if (item < '0' || item > '9') {
                throw new ArgumentException();
            }
        }

        if (result[0] == '0' || result[0] == '1') {
            throw new ArgumentException();
        }

        if (result[3] == '0' || result[3] == '1') {
            throw new ArgumentException();
        }

        return result;
    }
}