package cars

// SuccessRate is used to calculate the ratio of an item being created without
// error for a given speed
func SuccessRate(speed int) float64 {
	if speed <= 0 {
		return 0
	} else if speed <= 4 {
		return 100. / 100.
	} else if speed <= 8 {
		return 90. / 100.
	} else if speed <= 10 {
		return 77. / 100.
	}
	return 0
}

// CalculateProductionRatePerHour for the assembly line, taking into account
// its success rate
func CalculateProductionRatePerHour(speed int) float64 {
	return SuccessRate(speed) * 221 * float64(speed)
}

// CalculateProductionRatePerMinute describes how many working items are
// produced by the assembly line every minute
func CalculateProductionRatePerMinute(speed int) int {
	return int(CalculateProductionRatePerHour(speed) / 60)
}

// CalculateLimitedProductionRatePerHour describes how many working items are
// produced per hour with an upper limit on how many can be produced per hour
func CalculateLimitedProductionRatePerHour(speed int, limit float64) float64 {
	nolimit := CalculateProductionRatePerHour(speed)

	if limit > nolimit {
		return nolimit
	} else {
		return limit
	}
}
