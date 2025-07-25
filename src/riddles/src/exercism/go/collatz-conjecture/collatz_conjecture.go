package collatzconjecture

import "fmt"

func CollatzConjecture(n int) (int, error) {
	return step(n, 0)
}

func step(n int, count int) (int, error) {
	if n < 1 {
		return 0, fmt.Errorf("%d must be positive", n)
	}
	if n == 1 {
		return count, nil
	}

	count++

	if n%2 == 0 {
		return step(n/2, count)
	} else {
		return step(3*n+1, count)
	}
}
