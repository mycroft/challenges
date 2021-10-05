using System;

static class SavingsAccount
{
    public static float InterestRate(decimal balance)
    {
        float rate = 0.0f;

        switch (balance) {
            case < 0:
                rate = -3.213f;
                break;
            case < 1000:
                rate = 0.5f;
                break;
            case < 5000:
                rate = 1.621f;
                break;
            default:
                rate = 2.475f;
                break;
        }

        return rate;
    }

    public static decimal Interest(decimal balance)
    {
        decimal coeff = (decimal)InterestRate(balance);
        if (coeff < 0) {
            coeff *= -1;
        }

        return balance * coeff / 100;
    }

    public static decimal AnnualBalanceUpdate(decimal balance)
    {
        return balance + Interest(balance);
    }

    public static int YearsBeforeDesiredBalance(decimal balance, decimal targetBalance)
    {
        int years = 0;

        do {
            years += 1;
            balance = AnnualBalanceUpdate(balance);

        } while(balance < targetBalance);

        return years;
    }
}
