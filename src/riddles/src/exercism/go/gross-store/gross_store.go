package gross

// Units stores the Gross Store unit measurements.
func Units() map[string]int {
	units := map[string]int{
		"quarter_of_a_dozen": 3,
		"half_of_a_dozen":    6,
		"dozen":              12,
		"small_gross":        120,
		"gross":              144,
		"great_gross":        1728,
	}
	return units
}

// NewBill creates a new bill.
func NewBill() map[string]int {
	return map[string]int{}
}

// AddItem adds an item to customer bill.
func AddItem(bill, units map[string]int, item, unit string) bool {
	score, ok := units[unit]
	if !ok {
		return false
	}

	qty, ok := bill[item]
	if !ok {
		// Insert new item
		bill[item] = score
	} else {
		bill[item] = qty + score
	}
	return true
}

// RemoveItem removes an item from customer bill.
func RemoveItem(bill, units map[string]int, item, unit string) bool {
	qty, ok := bill[item]
	if !ok {
		return false
	}

	_, ok = units[unit]
	if !ok {
		return false
	}

	// use `<=` to make sure finalQty < 0
	if qty <= 0 {
		return false
	}

	// will be zero if `1 - 1`
	if qty == 1 {
		delete(bill, item)
	} else {
		bill[item] = qty - 1
	}
	return true
}

// GetItem returns the quantity of an item that the customer has in his/her bill.
func GetItem(bill map[string]int, item string) (int, bool) {
	qty, exists := bill[item]
	if !exists {
		return 0, false
	}

	return qty, true

}
