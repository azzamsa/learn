package lasagna

// TODO: define the 'PreparationTime()' function
// func PreparationTime(args) {
func PreparationTime(layers []string, avgPreparationTime int) int {
	if avgPreparationTime == 0 {
		avgPreparationTime = 2
	}
	return len(layers) * avgPreparationTime
}

// TODO: define the 'Quantities()' function
//
//	...
func Quantities(layers []string) (noodles int, sauce float64) {
	totalNoodles := 0
	totalSauce := 0

	for _, layer := range layers {
		switch layer {
		case "noodles":
			totalNoodles++
		case "sauce":
			totalSauce++
		}
	}

	noodles = 50 * totalNoodles
	sauce = 0.2 * float64(totalSauce)
	return
}

// TODO: define the 'AddSecretIngredient()' function
//
//	...
func AddSecretIngredient(friendList []string, myList []string) {
	myList[len(myList)-1] = friendList[len(friendList)-1]
}

// TODO: define the 'ScaleRecipe()' function
func ScaleRecipe(quantities []float64, portion int) []float64 {
	var result []float64
	for _, quantity := range quantities {
		scaled := quantity / 2 * float64(portion)
		result = append(result, scaled)
	}
	return result
}

// Your first steps could be to read through the tasks, and create
// these functions with their correct parameter lists and return types.
// The function body only needs to contain `panic("")`.
//
// This will make the tests compile, but they will fail.
// You can then implement the function logic one by one and see
// an increasing number of tests passing as you implement more
// functionality.
