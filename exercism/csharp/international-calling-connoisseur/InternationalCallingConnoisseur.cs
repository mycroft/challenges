using System;
using System.Collections.Generic;

public static class DialingCodes
{
    public static Dictionary<int, string> GetEmptyDictionary()
    {
        return new Dictionary<int,string>();
    }

    public static Dictionary<int, string> GetExistingDictionary()
    {
        var codes = GetEmptyDictionary();

        codes.Add(1, "United States of America");
        codes.Add(55, "Brazil");
        codes.Add(91, "India");

        return codes;
    }

    public static Dictionary<int, string> AddCountryToEmptyDictionary(int CountryCode, string CountryName)
    {
        var codes = GetEmptyDictionary();

        codes.Add(CountryCode, CountryName);

        return codes;
    }

    public static Dictionary<int, string> AddCountryToExistingDictionary(
        Dictionary<int, string> existingDictionary, int countryCode, string CountryName)
    {
        existingDictionary.Add(countryCode, CountryName);
        return existingDictionary;
    }

    public static string GetCountryNameFromDictionary(
        Dictionary<int, string> existingDictionary, int countryCode)
    {
        if (!existingDictionary.ContainsKey(countryCode)) {
            return "";
        }
        return existingDictionary[countryCode];
    }

    public static Dictionary<int, string> UpdateDictionary(
        Dictionary<int, string> existingDictionary, int countryCode, string countryName)
    {
        if (countryCode == 999) {
            return existingDictionary;
        }
        existingDictionary[countryCode] = countryName;
        return existingDictionary;   
    }

    public static Dictionary<int, string> RemoveCountryFromDictionary(
        Dictionary<int, string> existingDictionary, int countryCode)
    {
        existingDictionary.Remove(countryCode);

        return existingDictionary;
    }

    public static bool CheckCodeExists(Dictionary<int, string> existingDictionary, int countryCode)
    {
        return existingDictionary.ContainsKey(countryCode);
    }

    public static string FindLongestCountryName(Dictionary<int, string> existingDictionary)
    {
        string longest = "";

        foreach(var item in existingDictionary) {
            if (item.Value.Length > longest.Length) {
                longest = item.Value;
            }
        }

        return longest;
    }
}
