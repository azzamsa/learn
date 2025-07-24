package parsinglogfiles

import (
	"fmt"
	"regexp"
)

func IsValidLine(text string) bool {
	//	left most must be one of ...
	re := regexp.MustCompile(`^\[(TRC|DBG|INF|WRN|ERR|FTL)\].*`)
	return re.MatchString(text)
}

func SplitLogLine(text string) []string {
	// `*` zero or more, `+` one or more
	re := regexp.MustCompile(`<[~*=-]*>`)
	return re.Split(text, -1)
}

func CountQuotedPasswords(lines []string) int {
	count := 0
	re := regexp.MustCompile(`(?i)".*password.*"`)
	for _, line := range lines {
		if re.MatchString(line) {
			count++
		}
	}
	return count
}

func RemoveEndOfLineText(text string) string {
	re := regexp.MustCompile(`end-of-line(\S+)`)
	return re.ReplaceAllString(text, "")
}

func TagWithUserName(lines []string) []string {
	logs := []string{}
	re := regexp.MustCompile(`User\s+(\w+)`)
	for _, line := range lines {
		matches := re.FindStringSubmatch(line)
		if len(matches) > 0 {
			line = fmt.Sprintf("[USR] %s %s", matches[1], line)
		}
		logs = append(logs, line)
	}

	return logs
}
