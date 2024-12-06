# Day 1 -  Historian Hysteria
## Part 1 - Inter List Distance Calculation
Date: 6 Dec 2024
Time to solve: 60 mins

This one was pretty straightforward, you're given two lists which I've managed to store in a raw text format (input.txt), the task is to find the inter list distance. Sum of differences between respective sorted list elemets.

The task was broken down into:
1. Input parsing (reading lines from text file).
2. String opertations (splitting a string by three spaces as provided from AoC and parsing each into an integer with error handling).
3. Sorting (easily acheived through std unstable_sort).
4. Zipped traversal and distance calculation.

## Part 2 - Inter List Similarity Score
Date: 6 Dec 2024
Time to sovle: 35 mins

We're tasked with calculating a `similarity_score`, a fancy means of saying the sum of each element in left list multiplied by its frequency of occurrence in the right list.

Since we already had the typical input parsing + string operations + sorting already handeld, this task was broken down into:
1. Advanced binary search (because the list was already sorted) (binary search + two pointer left right spread for multiple matches) of each left list element.
2. Caching of key-value pairs of left_element: frequency for reduced computation and improved efficiency.

Overall the tasks were pretty straightforward and gentle enough for Day-1 and it was fun to be able to handle these. Can improvements be made here? For sure, we can bring in multithreading for the extended portion of the binary search and a lot of other such improvements that would border on over-engineering.