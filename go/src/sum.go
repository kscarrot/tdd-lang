package main

func Sum(array []int) int {
	sum := 0
	//可以用len做边界
	// for i := 0; i < len(array); i++ {
	// 	sum += array[i]
	// }

	//也可以 forr展开
	for _, num := range array {
		sum += num
	}
	return sum
}

func SumAllTails(numbersToSum ...[]int) []int {
	var sums []int
	for _, numbers := range numbersToSum {
		if len(numbers) == 0 {
			sums = append(sums, 0)
		} else {
			tail := numbers[1:]
			sums = append(sums, Sum(tail))
		}
	}

	return sums
}
