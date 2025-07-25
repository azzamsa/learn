package raindrops

import (
	"strconv"
	"strings"
)

func Convert(number int) string {
	result := []string{}

	if number%3 == 0 {
		result = append(result, "Pling")
	}
	if number%5 == 0 {
		result = append(result, "Plang")
	}
	if number%7 == 0 {
		result = append(result, "Plong")
	}

	if len(result) == 0 {
		return strconv.Itoa(number)
	}
	return strings.Join(result, "")
}
