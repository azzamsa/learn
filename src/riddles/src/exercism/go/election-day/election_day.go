package electionday

import "fmt"

// type ElectionResult struct {
// 	// Name of the candidate
// 	Name string
// 	// Votes of votes the candidate had
// 	Votes int
// }

// NewVoteCounter returns a new vote counter with
// a given number of initial votes.
func NewVoteCounter(initialVotes int) *int {
	// This is pass by value
	// return the address
	// `&` The & operator generates a pointer to its operand.
	return &initialVotes
}

// VoteCount extracts the number of votes from a counter.
func VoteCount(counter *int) int {
	// This is pass by pointer
	// I need to return the value,
	// return the value
	if counter == nil {
		return 0
	}
	return *counter
}

// IncrementVoteCount increments the value in a vote counter.
func IncrementVoteCount(counter *int, increment int) {
	// total := *counter + increment
	// counter = &total
	*counter += increment
}

// NewElectionResult creates a new election result.
func NewElectionResult(candidateName string, votes int) *ElectionResult {
	return &ElectionResult{Name: candidateName, Votes: votes}
}

// DisplayResult creates a message with the result to be displayed.
func DisplayResult(result *ElectionResult) string {
	return fmt.Sprintf("%s (%d)", result.Name, result.Votes)
}

// DecrementVotesOfCandidate decrements by one the vote count of a candidate in a map.
func DecrementVotesOfCandidate(results map[string]int, candidate string) {
	// votes, _ := results[candidate]
	// results[candidate] = votes - 1
	results[candidate]--
}
