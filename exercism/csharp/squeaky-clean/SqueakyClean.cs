using System;
using System.Text;

public static class Identifier
{
    public static string Clean(string identifier)
    {
        var nextIsCamel = false;
        string repl = "";
        StringBuilder builder = new StringBuilder();

        foreach(char c in identifier) {
            if (c == '-') {
                nextIsCamel = true;
                continue;
            }

            switch (c) {
                case ' ':
                    repl = "_";
                    break;
                case '\0':
                    repl = "CTRL";
                    break;
                default:
                    if(Char.IsLetter(c) && !(c >= 'α' && c <= 'ω')) {
                        repl = "" + c;
                    } else {
                        repl = "";
                    }
                    break;
            };

            if (nextIsCamel) {
                repl = repl.ToUpper();
                nextIsCamel = false;
            }

            builder.Append(repl);
        }

        return builder.ToString();
    }
}
