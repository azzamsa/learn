package collatzconjecture

import "fmt"

func CollatzConjecture(n int) (int, error) {
	return run(n, 0)
}

func run(n int, count int) (int, error) {
	if n < 1 {
		return 0, fmt.Errorf("%d must be positive", n)
	}
	if n == 1 {
		return count, nil
	}

	count++

	if n%2 == 0 {
		return run(n/2, count)
	} else {
		return run(3*n+1, count)
	}
}
