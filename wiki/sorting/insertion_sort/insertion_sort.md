## Introduction
This algorithm kinda sucks.

## Pseudocode[^wikipedia-pseudocode]
```
function insertion_sort(array A)
    i ← 1
    while i < length(A)
        x ← A[i]
        j ← i - 1
        while j >= 0 and A[j] > x
            A[j+1] ← A[j]
            j ← j - 1
        end while
        A[j+1] ← x[4]
        i ← i + 1
    end while
end function
```
[^wikipedia-pseudocode]: [Insertion Sort on Wikipedia](https://en.wikipedia.org/wiki/Insertion_sort#Algorithm)

## History

## Visualization
<div class="aspect-ratio" ><iframe src="https://tertsdiepraam.github.io/Sorters/"></iframe></div>

## Math
$$ \sin\theta $$
## Blah blah
