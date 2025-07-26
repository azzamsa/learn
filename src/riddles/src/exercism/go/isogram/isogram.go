package isogram

import (
	"strings"
)

func IsIsogram(word string) bool {
	current := ""
	word = strings.ToLower(word)
	for k := range word {
		char := string(word[k])
		// spaces and hyphens are okay
		if char == " " || char == "-" {
			continue
		}
		current = char
		for i, v := range word {
			// don't compare with itself.
			if i == k {
				continue
			}
			if current == string(v) {
				return false
			}
		}
	}
	return true
}
