package chessboard

// Declare a type named File which stores if a square is occupied by a piece - this will be a slice of bools
type File []bool

// Declare a type named Chessboard which contains a map of eight Files, accessed with keys from "A" to "H"
type Chessboard map[string]File

// CountInFile returns how many squares are occupied in the chessboard,
// within the given file.
func CountInFile(cb Chessboard, file string) int {
	total := 0

	for _, x := range cb[file] {
		if x == true {
			total++
		}
	}
	return total
}

// CountInRank returns how many squares are occupied in the chessboard,
// within the given rank.
func CountInRank(cb Chessboard, rank int) int {
	total := 0

	if rank < 1 && rank > 8 {
		return 0
	}

	// loop over each file, only take the specified line
	// loop the labels
	for label := range cb {
		// loop the files
		for index, val := range cb[label] {
			if index == rank-1 {
				if val == true {
					total++
				}
				break
			}
		}
	}

	return total
}

// CountAll should count how many squares are present in the chessboard.
func CountAll(cb Chessboard) int {
	total := 0
	for label := range cb {
		// loop the files
		for range cb[label] {
			total++
		}
	}
	return total
}

// CountOccupied returns how many squares are occupied in the chessboard.
func CountOccupied(cb Chessboard) int {
	total := 0
	for label := range cb {
		// loop the files
		for _, val := range cb[label] {
			if val == true {
				total++
			}
		}
	}
	return total
}
