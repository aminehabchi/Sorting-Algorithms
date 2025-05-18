# Rust Sorting Algorithms

This repository contains implementations of various sorting algorithms in Rust. Each algorithm is implemented with a focus on clarity and efficiency.

## Table of Contents

- [Algorithms](#algorithms)
  - [Bubble Sort](#bubble-sort)
  - [Insertion Sort](#insertion-sort)
  - [Selection Sort](#selection-sort)
  - [Merge Sort](#merge-sort)
  - [Quick Sort](#quick-sort)
  - [Shell Sort](#shell-sort)
  - [Heap Sort](#heap-sort)
  - [Counting Sort](#counting-sort)
- [Comparison](#comparison)
  - [Time Complexity](#time-complexity)
  - [Space Complexity](#space-complexity)
  - [Stability](#stability)
  - [Performance Benchmarks](#performance-benchmarks)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Algorithms

### Bubble Sort

```rust
fn bubble_sort(arr: &mut Vec<i32>) {
    let l: usize = arr.len();

    for i in 0..l-1 {
        for j in i+1..l {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}
```

**Description**: Bubble sort repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted.

**Note**: This implementation is actually a variant of bubble sort sometimes called "cocktail sort" or "bidirectional bubble sort" due to the indexing pattern.

### Insertion Sort

```rust
fn insertion_sort(arr: &mut Vec<i32>) {
    let l: usize = arr.len();

    for i in 1..l {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1] {
            arr.swap(j, j-1);
            j -= 1;
        }
    }
}
```

**Description**: Insertion sort builds the final sorted array one item at a time. It is efficient for small data sets and is often used as part of more sophisticated algorithms.

### Selection Sort

```rust
fn selection_sort(arr: &mut Vec<i32>) {
    let l: usize = arr.len();
    
    for i in 0..l-1 {
        let mut smallest_index = i;
        for j in i+1..l {
            if arr[smallest_index] > arr[j] {
                smallest_index = j;
            }
        }
        if smallest_index != i {
            arr.swap(i, smallest_index);
        }
    }
}
```

**Description**: Selection sort divides the input list into two parts: a sorted sublist and an unsorted sublist. It repeatedly finds the minimum element from the unsorted sublist and moves it to the end of the sorted sublist.

### Merge Sort

```rust
fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
    let l = arr.len();
    if l <= 1 {
        return arr.to_vec();
    }

    let left = merge_sort(&mut arr[0..l/2]);
    let right = merge_sort(&mut arr[l/2..]);

    let mut ar: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize = 0;

    loop {
        if i < left.len() && j < right.len() {
            if left[i] < right[j] {
                ar.push(left[i]);
                i += 1;
            } else {
                ar.push(right[j]);
                j += 1;
            }
        } else if i < left.len() {
            ar.push(left[i]);
            i += 1;
        } else if j < right.len() {
            ar.push(right[j]);
            j += 1;
        } else {
            break
        }
    }

    ar
}
```

**Description**: Merge sort is a divide-and-conquer algorithm that divides the input array into two halves, recursively sorts them, and then merges the sorted halves.

### Quick Sort

```rust
fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let l: usize = arr.len();
    let pivot: i32 = arr[0];
    
    let mut j = 1;

    for i in 1..l {
        if arr[i] < pivot {
            arr.swap(i, j);
            j += 1;
        }
    }
    arr.swap(j-1, 0); 

    quick_sort(&mut arr[0..j-1]);
    quick_sort(&mut arr[j..l]);
}
```

**Description**: Quick sort is a divide-and-conquer algorithm that works by selecting a 'pivot' element and partitioning the array around the pivot. This implementation uses the first element as the pivot.

### Shell Sort

```rust
fn shell_sort(arr: &mut Vec<i32>) {
    let mut gap: usize = arr.len() / 2;
    let len: usize = arr.len();

    while gap > 0 {
        for i in gap..len {
            let mut j = i;
            while j >= gap && arr[j] < arr[j - gap] {
                arr.swap(j, j - gap);
                j -= gap;
            }
        }
        gap /= 2;
    }
}
```

**Description**: Shell sort is an optimization of insertion sort that allows the exchange of items that are far apart. The idea is to arrange the list of elements so that, starting anywhere, taking every hth element produces a sorted list.

### Heap Sort

```rust
fn heapify(arr: &mut Vec<i32>, len: usize, i: usize) {
    let mut max = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < len && arr[l] > arr[max] {
        max = l;
    }

    if r < len && arr[r] > arr[max] {
        max = r;
    }

    if max != i {
        arr.swap(i, max);
        heapify(arr, len, max);
    }
}

fn heap_sort(arr: &mut Vec<i32>) {
    let len = arr.len();

    // Build max heap
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    // Extract elements one by one
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}
```

**Description**: Heap sort is a comparison-based sorting algorithm that uses a binary heap data structure. It divides its input into a sorted and an unsorted region, and iteratively shrinks the unsorted region by extracting the largest element and moving it to the sorted region.

### Counting Sort

```rust
fn counting_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut max = arr[0];
    for val in arr.iter() {
        if *val > max {
            max = *val;
        }
    }

    let mut count = vec![0; (max + 1) as usize];

    for val in arr.iter() {
        count[*val as usize] += 1;
    }

    let mut result = Vec::with_capacity(arr.len());

    for i in 0..count.len() {
        for _ in 0..count[i] {
            result.push(i as i32);
        }
    }

    result
}
```

**Description**: Counting sort is a non-comparison based sorting algorithm that works well when there is a limited range of input values. It counts the number of objects having distinct key values and uses these counts to determine the positions of each key value in the output sequence.

## Comparison

### Time Complexity

| Algorithm      | Best Case    | Average Case | Worst Case   |
|----------------|--------------|--------------|--------------|
| Bubble Sort    | O(n)         | O(n²)        | O(n²)        |
| Insertion Sort | O(n)         | O(n²)        | O(n²)        |
| Selection Sort | O(n²)        | O(n²)        | O(n²)        |
| Merge Sort     | O(n log n)   | O(n log n)   | O(n log n)   |
| Quick Sort     | O(n log n)   | O(n log n)   | O(n²)        |
| Shell Sort     | O(n log n)   | O(n log² n)  | O(n²)        |
| Heap Sort      | O(n log n)   | O(n log n)   | O(n log n)   |
| Counting Sort  | O(n + k)     | O(n + k)     | O(n + k)     |

Note: For counting sort, k is the range of input values.

### Space Complexity

| Algorithm      | Space Complexity |
|----------------|------------------|
| Bubble Sort    | O(1)             |
| Insertion Sort | O(1)             |
| Selection Sort | O(1)             |
| Merge Sort     | O(n)             |
| Quick Sort     | O(log n)         |
| Shell Sort     | O(1)             |
| Heap Sort      | O(1)             |
| Counting Sort  | O(k)             |

Note: For counting sort, k is the range of input values.

### Stability

A sorting algorithm is stable if it preserves the relative order of equal elements.

| Algorithm      | Stable |
|----------------|--------|
| Bubble Sort    | Yes    |
| Insertion Sort | Yes    |
| Selection Sort | No     |
| Merge Sort     | Yes    |
| Quick Sort     | No     |
| Shell Sort     | No     |
| Heap Sort      | No     |
| Counting Sort  | Yes    |

### Performance Benchmarks

For a real-world comparison, you might want to run benchmarks on different input sizes and patterns. Here's a general guideline on when to use each algorithm:

- **Small arrays or nearly sorted arrays**: Insertion Sort, Bubble Sort
- **General purpose**: Quick Sort, Merge Sort, Heap Sort
- **Limited range of integers**: Counting Sort
- **When stability matters**: Merge Sort, Bubble Sort, Insertion Sort, Counting Sort
- **When memory usage is a concern**: Heap Sort, Quick Sort

## Usage

```rust
fn main() {
    // Example usage for each sorting algorithm
    let mut arr: Vec<i32> = vec![5, 7, 10, 2, 9, 8, 1, 6, 0, 4];
    
    println!("Original array: {:?}", arr);
    
    // Choose one of the sorting methods:
    // bubble_sort(&mut arr);
    // insertion_sort(&mut arr);
    // selection_sort(&mut arr);
    // arr = merge_sort(&mut arr);
    // quick_sort(&mut arr);
    // shell_sort(&mut arr);
    heap_sort(&mut arr);
    // arr = counting_sort(&mut arr);
    
    println!("Sorted array: {:?}", arr);
}
```

## Contributing

Contributions are welcome! Here are some ways you can contribute:
- Implement additional sorting algorithms
- Improve existing implementations
- Add benchmarks for performance comparison
- Fix bugs
