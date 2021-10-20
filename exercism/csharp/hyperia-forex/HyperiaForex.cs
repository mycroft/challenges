using System;

public struct CurrencyAmount
{
    private decimal amount;
    private string currency;

    public CurrencyAmount(decimal amount, string currency)
    {
        this.amount = amount;
        this.currency = currency;
    }

    public override bool Equals(object obj)
    {
        if (obj == null || GetType() != obj.GetType())
        {
            return false;
        }

        CurrencyAmount other = (CurrencyAmount)obj;

        if (other.currency != this.currency) {
            throw new ArgumentException();
        }
        
        return base.Equals (obj);
    }
    
    public override int GetHashCode()
    {
        return base.GetHashCode();
    }

    public static bool operator ==(CurrencyAmount rhs, CurrencyAmount lhs)
    {
        return rhs.Equals(lhs);
    }

    public static bool operator !=(CurrencyAmount rhs, CurrencyAmount lhs)
    {
        return !rhs.Equals(lhs);
    }

    public static bool operator <(CurrencyAmount rhs, CurrencyAmount lhs)
    {
        if (rhs.currency != lhs.currency) {
            throw new ArgumentException();
        }

        return rhs.amount < lhs.amount;
    }

    public static bool operator >(CurrencyAmount rhs, CurrencyAmount lhs)
    {
        if (rhs.currency != lhs.currency) {
            throw new ArgumentException();
        }

        return rhs.amount > lhs.amount;
    }

    public static CurrencyAmount operator +(CurrencyAmount rhs, CurrencyAmount lhs)
    {
        if (rhs.currency != lhs.currency) {
            throw new ArgumentException();
        }

        return new CurrencyAmount(
            rhs.amount + lhs.amount,
            rhs.currency
        );
    }

    public static CurrencyAmount operator -(CurrencyAmount rhs, CurrencyAmount lhs)
    {
        if (rhs.currency != lhs.currency) {
            throw new ArgumentException();
        }

        return new CurrencyAmount(
            rhs.amount - lhs.amount,
            rhs.currency
        );
    }

    public static CurrencyAmount operator *(CurrencyAmount rhs, CurrencyAmount lhs)
    {
        if (rhs.currency != lhs.currency) {
            throw new ArgumentException();
        }

        return new CurrencyAmount(
            rhs.amount * lhs.amount,
            rhs.currency
        );
    }

    public static CurrencyAmount operator *(CurrencyAmount rhs, decimal lhs)
        => new CurrencyAmount(rhs.amount * lhs, rhs.currency);

    public static CurrencyAmount operator *(decimal rhs, CurrencyAmount lhs)
        => new CurrencyAmount(rhs * lhs.amount, lhs.currency);

    public static CurrencyAmount operator /(CurrencyAmount rhs, CurrencyAmount lhs)
    {
        if (rhs.currency != lhs.currency) {
            throw new ArgumentException();
        }

        return new CurrencyAmount(
            rhs.amount / lhs.amount,
            rhs.currency
        );
    }

    public static CurrencyAmount operator /(CurrencyAmount rhs, decimal lhs)
    {
        return new CurrencyAmount(
            rhs.amount / lhs,
            rhs.currency
        );
    }

    public static implicit operator decimal(CurrencyAmount lhs) => lhs.amount;
    public static implicit operator double(CurrencyAmount lhs) => (double)lhs.amount;

    // TODO: implement type conversion operators
}
