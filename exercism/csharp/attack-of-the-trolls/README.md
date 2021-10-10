# Attack of the Trolls

Welcome to Attack of the Trolls on Exercism's C# Track.
If you need help running the tests or submitting your code, check out `HELP.md`.
If you get stuck on the exercise, check out `HINTS.md`, but try and solve it without using those first :)

## Introduction

## Attributes

A [C# `Attribute`](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/concepts/attributes/) provides a way to decorate a declaration to associate metadata to: a class, a method, an enum, a field, a property or any [other supported](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/concepts/attributes/#attribute-targets) declarations.

You can apply an attribute by adding it on the line before the declaration using a `ClassAttribute` and a `FieldAttribute`:

```csharp
[Class]
class MyClass
{
    [Field]
    int myField;
}
```

This declarative metadata only associates additional structured information to the code and does not modify its behavior, but that metadata is used by other part of the code to change how its target would behave or add, change or remove, restrict some its functionalities.

There is many [predefined and reserved attributes](https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/attributes/general#conditional-attribute), for example: `Flags`, `Obsolete`, `Conditional`, each has a specific that can be looked up on the C# documentation. Note that the full name of an attribute like [`Flags`](https://docs.microsoft.com/en-us/dotnet/api/system.flagsattribute?view=net-5.0) is `FlagsAttribute` by convention, but the suffix _Attribute_ can be omitted when applied on a declaration.

## Flag Enums

The C# [`enum` type](https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/enum) represents a fixed set of named constants (an enumeration).

Normally, one `enum` member can only refer to exactly one of those named constants. However, sometimes it is useful to refer to more than one constant. To do so, one can annotate the `enum` with the [`Flags` attribute](https://docs.microsoft.com/en-us/dotnet/api/system.flagsattribute?view=net-5.0). A _flags_ enum's constants are interpreted as bitwise _flags_ and therefor indicates the enum supports the bitwise operators and additional features like the method `Enum.HasFlag()`.

A flags enum can be defined as follows (using binary integer notation `0b`):

```csharp
[Flags]
enum PhoneFeatures
{
    Call = 0b00000001,
    Text = 0b00000010
}
```

A `PhoneFeatures` instance which value is `0b00000011` has both its `Call` _and_ `Text` flags set.

By default, the `int` type is used for enum member values. One can use a different integer type by specifying the type in the enum declaration:

```csharp
[Flags]
enum PhoneFeatures : byte
{
    Call = 0b00000001,
    Text = 0b00000010
}
```

## Instructions

In this exercise you'll be checking permissions of user accounts on an internet forum. The forum supports three different permissions:

- Read
- Write
- Delete

There are three types of accounts, each with different default permissions:

- Guests: can read posts.
- Users: can read and write posts.
- Moderators: can read, write and delete posts, they have all the permissions.

Sometimes individual permissions can be modified, it is possible for example to give a guest account the permission to also write posts or revoking all permissions from an account would result in having none of the permissions.

## 1. Get default permissions for an account type

First, define an `AccountType` enum to represent the three account types: `Guest`, `User` and `Moderator`.

Next, define a `Permission` enum to represent the three permission types: `Read`, `Write`, `Delete`, and two extra ones: `All` for having all permissions and `None` for having none of the permissions.

Then implement the (_static_) `Permissions.Default()` method to return the default permissions for a specific account type:

```csharp
Permissions.Default(AccountType.Guest)
// => Permission.Read
```

## 2. Grant a permission

Implement the (_static_) `Permissions.Grant()` method that grants (adds) a permission:

```csharp
Permissions.Grant(current: Permission.None, grant: Permission.Read)
// => Permission.Read
```

## 3. Revoke a permission

Implement the (_static_) `Permissions.Revoke()` method that revokes (removes) a permission:

```csharp
Permissions.Revoke(current: Permission.Read, grant: Permission.Read)
// => Permission.None
```

## 4. Check for a permission

Implement the (_static_) `Permissions.Check()` method that takes the current account's permissions and checks if the account is authorized for a given permission:

```csharp
Permissions.Check(current: Permission.Write, check: Permission.Read)
// => false
```

## Source

### Created by

- @ErikSchierboom

### Contributed to by

- @valentin-p
- @yzAlvin