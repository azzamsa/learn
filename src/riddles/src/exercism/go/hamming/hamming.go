package hamming

import "errors"

func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("Not equal length")
	}

	distance := 0
	for i, v := range a {
		if byte(v) != b[i] {
			distance++
		}
	}
	return distance, nil
}
