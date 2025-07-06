package interest

// InterestRate returns the interest rate for the provided balance.
func InterestRate(balance float64) float32 {
	switch {
	case balance < 0.0:
		return 3.213
	case balance >= 0.0 && balance < 1_000:
		return 0.5
	case balance >= 1_000 && balance < 5_000:
		return 1.621
	case balance >= 5_000:
		return 2.475
	}
	return 0
}

// Interest calculates the interest for the provided balance.
func Interest(balance float64) float64 {
	rate := float64(InterestRate(balance))
	return rate * balance / 100
}

// AnnualBalanceUpdate calculates the annual balance update, taking into account the interest rate.
func AnnualBalanceUpdate(balance float64) float64 {
	return balance + Interest(balance)
}

// YearsBeforeDesiredBalance calculates the minimum number of years required to reach the desired balance.
func YearsBeforeDesiredBalance(balance, targetBalance float64) int {
	totalYears := 0
	currentBalance := balance
	for currentBalance < targetBalance {
		currentBalance = AnnualBalanceUpdate(currentBalance)
		totalYears++
	}
	return totalYears
}
