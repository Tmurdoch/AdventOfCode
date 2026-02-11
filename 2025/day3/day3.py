
def maximum_size_subarray(bank: list[int], k: int) -> int:
    n = len(bank)
    result = []
    current_pos = 0  # Start searching from here
    
    for count in range(k):
        needed = k - count  # How many more digits do we need?
        remaining = n - current_pos  # How many positions are left?
        window_size = remaining - needed + 1  # How far ahead can we look?
        
        # Find the best digit in the valid window
        best = bank[current_pos]
        best_idx = current_pos
        
        for i in range(current_pos, current_pos + window_size):
            if bank[i] > best:
                best = bank[i]
                best_idx = i
        
        result.append(best)
        current_pos = best_idx + 1  # Next search starts after this position
    
    return int(''.join(map(str, result)))


    


f = open("input.txt", "r+")


lines = [x.strip() for x in f.readlines()]


max_voltage = 0
for line in lines:
    split_line = [int(x) for x in list(line)]
    ret = maximum_size_subarray(split_line, 12)
    print("line: ", line, "calc: ", ret)

    max_voltage += ret
print(maximum_size_subarray([1, 2, 3, 4], 2))

print(maximum_size_subarray([int(x) for x in list("234234234234278")], 12))

print(max_voltage)

    


