using System;

enum AccountType {
    Guest,
    User,
    Moderator
};

[Flags]
enum Permission : byte {
    None = 0,
    Delete = 0x01,
    Read = 0x02,
    Write = 0x04,
    All = Permission.Delete | Permission.Read | Permission.Write,
};

static class Permissions
{
    public static Permission Default(AccountType accountType)
    {
        return accountType switch {
            AccountType.Guest => Permission.Read,
            AccountType.User => Permission.Read | Permission.Write,
            AccountType.Moderator => Permission.All,
            _ => Permission.None,
        };
    }

    public static Permission Grant(Permission current, Permission grant) => current | grant;

    public static Permission Revoke(Permission current, Permission revoke) => current & (current ^ revoke);

    public static bool Check(Permission current, Permission check) => (current & check) == check;
}
