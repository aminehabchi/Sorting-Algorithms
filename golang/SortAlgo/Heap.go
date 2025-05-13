package Sorting

func Heap(arr []int) {
	l := len(arr)
	for i := l/2 - 1; i >= 0; i-- {
		Heapfy(arr, l, i)
	}

	for i := l - 1; i > 0; i-- {
		arr[0], arr[i] = arr[i], arr[0]
		Heapfy(arr, i, 0)
	}
}
func Heapfy(arr []int, l, i int) {
	left := 2*i + 1
	right := 2*i + 2
	index := i
	if left < l && arr[index] < arr[left] {
		index = left
	}
	if right < l && arr[index] < arr[right] {
		index = right
	}
	if index != i {
		arr[i], arr[index] = arr[index], arr[i]
		Heapfy(arr, l, index)
	}
}
