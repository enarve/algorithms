# Selection sort

def sort(arr):
    unsorted = arr
    sorted = []
    while unsorted:
        index_of_min = 0
        min = arr[0]
        for i, x in enumerate(unsorted):
            if x < min:
                min = x
                index_of_min = i
        sorted.append(unsorted.pop(index_of_min))
    return sorted

# Test
arr = [3, 5, 13, 7, 0, -4, 2, 11, 27, 9]
print(arr)
print(sort(arr))
