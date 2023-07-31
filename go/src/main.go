package main

import (
	"gobigotest/src/otest"
	"math/rand"
)

const POINTS = 200
const REPEATES = 3
const PREHEATES = 1

func bakeArray(n int) []int       { return make([]int, n) }
func bakeN(n int) int             { return n }
func bakeRandomArray(n int) []int { return rand.Perm(n) }

func main() {
	otest.RunOTest("seq_search", func() []otest.Measurement {
		search := func(data []int, target int) (int, bool) {
			for _, v := range data {
				if v == target {
					return v, true
				}
			}

			return -1, false
		}

		alg := func(data []int) {
			for i := 0; i < 10; i++ {
				search(data, 100)
			}
		}

		return otest.RunN(alg, bakeArray, 10000000, POINTS)
	}, PREHEATES, REPEATES)

	otest.RunOTest("fast_power", func() []otest.Measurement {
		power := func(N int, M int) int {
			power := N
			sum := 1

			for M > 0 {
				if (M & 1) == 1 {
					sum *= power
				}
				power = power * power
				M = M >> 1
			}
			return sum
		}

		alg := func(n int) {
			for i := 0; i < 10000000; i++ {
				power(1, n)
			}
		}

		return otest.RunN(alg, bakeN, 10000000, POINTS)
	}, PREHEATES, REPEATES)

	otest.RunOTest("qsort", func() []otest.Measurement {
		partition := func(arr []int, low, high int) int {
			pivot := arr[high]
			i := low
			for j := low; j < high; j++ {
				if arr[j] < pivot {
					arr[i], arr[j] = arr[j], arr[i]
					i++
				}
			}
			arr[i], arr[high] = arr[high], arr[i]
			return i
		}

		var sort func(arr []int, low, high int)
		sort = func(arr []int, low, high int) {
			if low < high {
				p := partition(arr, low, high)
				sort(arr, low, p-1)
				sort(arr, p+1, high)
			}
		}

		alg := func(data []int) {
			dataCopy := make([]int, len(data))
			for i := 0; i < 4; i++ {
				copy(dataCopy, data)

				sort(dataCopy, 0, len(data)-1)
			}
		}

		return otest.RunN(alg, bakeRandomArray, 1000000, POINTS)
	}, PREHEATES, REPEATES)

	otest.RunOTest("bubble_sort", func() []otest.Measurement {
		sort := func(data []int) {
			for i := 0; i < len(data)-1; i++ {
				for j := 0; j < len(data)-1; j++ {
					if data[j] > data[j+1] {
						data[j], data[j+1] = data[j+1], data[j]
					}
				}
			}
		}

		alg := func(data []int) {
			dataCopy := make([]int, len(data))
			for i := 0; i < 4; i++ {
				copy(dataCopy, data)

				sort(data)
			}
		}

		return otest.RunN(alg, bakeRandomArray, 10000, POINTS)
	}, PREHEATES, REPEATES)
}
