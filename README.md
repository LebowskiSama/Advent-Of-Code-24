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

# Day 2 -  Red-Nosed Reports
## Part 1 - N safe reports
Date: 6 Nov 2024
Time to solve: 50 mins

This one doesn't disappoint either. Fun puzzle here. Essentially, you're to judge reports (arrays) and how they progress. A report is considered a safe report if:
1. Constraint 1: The elements in the list are strictly ascending or strcitly descending.
2. Constraint 2: If the absolute difference between any two successive elements in a report is at least 1 and utmost 3.

The problem was broken down into:
1. Input parsing: reading lines from a text file.
2. String operations: splitting and casting strings into integers.
3. Array traversal and conditional statements: 
    - Calculate ascending or descending from the first two elements and store it as an integer (-1 for descending, 0 for 0, 1 for ascending). Use this to keep constraint 1 satisfied for ever difference calculated.
    - Calculate the successive differences during traversal and check for constraint 2. Break if not satisfied, increment safe reports if satisfied.

## Part 2 - Dampened accounted safety
Date: 7 Nov 2024
Time to solve: 130 mins

Alright, we now have a problem that gets you thinking.
As an extension of part 1, you can now remove one at most to deem a report safe.
- 1 ~~3~~ 2 4 5: becomes safe by removing 3 at level 2 - because 1 -> 3 is ascending but 3 -> 2 is descending, we're removing here to maintain ordering.
- 8 6 ~~4~~ 4 1: again, removing 4 here maintains ordering because 4 -> 4 is neither ascending nor descending.

The gist here is, you either remove a level to  
a. maintain ordering or   
b. ensure differences are within safety levels (at least 1, at most 3).

Bottom line: a brute force approach was taken, where the constraints are checked after popping each element of the array, increasing memory complexity by duplicating on a report level and compounding time complexity by iterating at least n<sup>2</sup> per report. Sure enough, a more optimal solution must exist.