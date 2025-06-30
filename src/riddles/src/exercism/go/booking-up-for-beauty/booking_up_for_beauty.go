package booking

import (
	"fmt"
	"time"
)

// Schedule returns a time.Time from a string containing a date.
func Schedule(date string) time.Time {
	layout := "1/2/2006 15:04:05"
	return parseTime(layout, date)
}

// HasPassed returns whether a date has passed.
func HasPassed(date string) bool {
	layout := "January 2, 2006 15:04:05"
	t := parseTime(layout, date)
	return time.Now().After(t)
}

// IsAfternoonAppointment returns whether a time is in the afternoon.
func IsAfternoonAppointment(date string) bool {
	layout := "Monday, January 2, 2006 15:04:05"
	t := parseTime(layout, date)
	return t.Hour() >= 12 && t.Hour() < 18
}

// Description returns a formatted string of the appointment time.
func Description(date string) string {
	layout := "1/2/2006 15:04:05"
	t := parseTime(layout, date)
	return fmt.Sprintf("You have an appointment on %s", t.Format("Monday, January 2, 2006, at 15:04."))
}

// AnniversaryDate returns a Time with this year's anniversary.
func AnniversaryDate() time.Time {
	return time.Date(2025, time.September, 15, 00, 0, 0, 0, time.UTC)
}

// https://pkg.go.dev/time#pkg-constants
func parseTime(layout, value string) time.Time {
	t, err := time.Parse(layout, value)
	// To avoid returning a zero-value, which is confusing.
	if err != nil {
		panic(fmt.Sprintf("invalid date format: %q, error: %v", value, err))
	}
	return t
}
