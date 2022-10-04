package main

import (
	"fmt"
	"github.com/icza/gox/timex"
	"time"
)

func main() {
	diff := timex.Diff
	birth_date := time.Date(2000, 11, 28, 0, 0, 0, 0, time.UTC)
	year, month, day, _, _, _ := diff(birth_date, time.Now())

	fmt.Printf("You were born at: %s \n", birth_date.Format("Monday, 02 November 2006"))
	fmt.Printf("You are %d years, %d months, %d days \n", year, month, day)
}
