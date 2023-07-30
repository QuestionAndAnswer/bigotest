package otest

import (
	"fmt"
	"time"
)

type Measurement struct {
	N    int
	Time time.Duration
}

func RunN[T any](
	targetFn func(data T),
	dataBakeFn func(n int) T,
	maxN int, points int,
) []Measurement {
	ns := linspace(1, maxN, points)

	res := make([]Measurement, 0, len(ns))

	for _, n := range ns {
		data := dataBakeFn(n)

		start := time.Now()
		targetFn(data)
		elapsed := time.Since(start)

		res = append(res, Measurement{N: n, Time: elapsed})
	}

	return res
}

func RunOTest(name string, fn func() []Measurement, preheat int, repeates int) {
	fmt.Print("\033[s")
	for i := 0; i < preheat; i++ {
		fmt.Printf("\033[u\033[Kpreheating %v: %v/%v", name, i+1, preheat)
		fn()
	}

	RemoveReports(name)
	for i := 0; i < repeates; i++ {
		fmt.Printf("\033[u\033[K%v: %v/%v", name, i+1, repeates)
		m := fn()
		WriteReport(fmt.Sprintf("%v_%v", name, i), m)
	}
	fmt.Println()
}
