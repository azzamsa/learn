package isogram

import (
	"strings"
	"unicode"
)

func IsIsogram(word string) bool {
	word = strings.ToLower(word)

	for i, v := range word {
		// `isLetter` skips checking non-letters
		// i+1 to avoid checking itself
		if unicode.IsLetter(v) && strings.ContainsAny(word[i+1:], string(v)) {
			return false
		}
	}
	return true
}
