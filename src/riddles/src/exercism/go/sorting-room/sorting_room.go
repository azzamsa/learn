package sorting

import (
	"fmt"
	"strconv"
)

// DescribeNumber should return a string describing the number.
func DescribeNumber(f float64) string {
	s := strconv.FormatFloat(f, 'f', 1, 64)
	return fmt.Sprintf("This is the number %s", s)
}

type NumberBox interface {
	Number() int
}

// DescribeNumberBox should return a string describing the NumberBox.
func DescribeNumberBox(nb NumberBox) string {
	num := float64(nb.Number())
	s := strconv.FormatFloat(num, 'f', 1, 64)
	return fmt.Sprintf("This is a box containing the number %s", s)
}

type FancyNumber struct {
	n string
}

func (i FancyNumber) Value() string {
	return i.n
}

type FancyNumberBox interface {
	Value() string
}

// ExtractFancyNumber should return the integer value for a FancyNumber
// and 0 if any other FancyNumberBox is supplied.
func ExtractFancyNumber(fnb FancyNumberBox) int {
	switch v := fnb.(type) {
	case FancyNumber:
		i, _ := strconv.Atoi(v.Value())
		return i
	default:
		return 0
	}

}

// DescribeFancyNumberBox should return a string describing the FancyNumberBox.
func DescribeFancyNumberBox(fnb FancyNumberBox) string {
	// string -> int
	i := ExtractFancyNumber(fnb)
	// int -> float
	f := float64(i)
	// float -> string
	s := strconv.FormatFloat(f, 'f', 1, 64)
	return fmt.Sprintf("This is a fancy box containing the number %s", s)
}

// DescribeAnything should return a string describing whatever it contains.
func DescribeAnything(i interface{}) string {
	switch v := i.(type) {
	case float64:
		return DescribeNumber(v)
	case int:
		return DescribeNumber(float64(v))
	case NumberBox:
		return DescribeNumberBox(v)
	case FancyNumberBox:
		return DescribeFancyNumberBox(v)
	default:
		return "Return to sender"
	}
}
