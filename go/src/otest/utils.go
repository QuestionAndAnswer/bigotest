package otest

func linspace(start int, end int, step int) []int {
	if step <= 1 {
		return []int{start}
	}

	result := make([]int, step)
	delta := (end - start) / (step - 1)

	for i := 0; i < step; i++ {
		result[i] = start + delta*i
	}

	return result
}
