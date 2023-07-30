package otest

import (
	"encoding/csv"
	"fmt"
	"os"
	"strings"
)

const reportsDir = "gen/data"

func RemoveReports(name string) {
	entries, err := os.ReadDir(reportsDir)
	if err != nil {
		panic(err)
	}

	for _, file := range entries {
		if strings.HasPrefix(file.Name(), name) {
			path := fmt.Sprintf("%s/%s", reportsDir, file.Name())
			if err := os.Remove(path); err != nil {
				panic(err)
			}
		}
	}
}

func WriteReport(name string, data []Measurement) {
	fileName := fmt.Sprintf("%s/%s.csv", reportsDir, name)
	f, err := os.OpenFile(fileName, os.O_TRUNC|os.O_CREATE|os.O_WRONLY, 0666)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	w := csv.NewWriter(f)
	defer w.Flush()

	w.Write([]string{"n", "time"})

	for _, m := range data {
		w.Write([]string{
			fmt.Sprint(m.N),
			fmt.Sprint(m.Time.Nanoseconds()),
		})
	}
}
