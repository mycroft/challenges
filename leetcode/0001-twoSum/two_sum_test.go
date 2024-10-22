package main

import "testing"

func twoSum(nums []int, target int) []int {
	indexMap := make(map[int]int)

	for i, num := range nums {
		complement := target - num

		if index, found := indexMap[complement]; found {
			return []int{index, i}
		}

		indexMap[num] = i
	}

	return []int{}
}

func TestBasic(t *testing.T) {
	testCases := []struct {
		nums   []int
		target int
		want   []int
	}{
		{[]int{2, 7, 11, 15}, 9, []int{0, 1}},
		{[]int{3, 2, 4}, 6, []int{1, 2}},
		{[]int{3, 3}, 6, []int{0, 1}},
		{[]int{-1, -2, -3, -4, -5}, -8, []int{2, 4}},
		{[]int{1, 5, 9, 15}, 10, []int{0, 2}},
	}

	for _, tc := range testCases {
		got := twoSum(tc.nums, tc.target)
		if len(got) != len(tc.want) || got[0] != tc.want[0] || got[1] != tc.want[1] {
			t.Errorf("twoSum(%v, %d) = %v; want %v", tc.nums, tc.target, got, tc.want)
		}
	}
}
