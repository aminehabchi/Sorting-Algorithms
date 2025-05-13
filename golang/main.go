package main

import (
	Sorting "Sorting/SortAlgo"
	"fmt"
	"math/rand"
)

func main() {
	length := 5
	arr := generete(length)
	fmt.Println(arr)
	Sorting.Heap(arr)
	fmt.Println(arr)
	fmt.Println("is Sorted", IsSorted(arr))
}

func generete(n int) []int {
	arr := make([]int, n)

	for i := 0; i < len(arr); i++ {
		arr[i] = rand.Intn(100)
	}

	return arr
}

func IsSorted(arr []int) bool {
	if len(arr) < 2 {
		return true
	}

	ascending := true
	descending := true

	for i := 1; i < len(arr); i++ {
		if arr[i] < arr[i-1] {
			ascending = false
		}
		if arr[i] > arr[i-1] {
			descending = false
		}
	}

	return ascending || descending
}
