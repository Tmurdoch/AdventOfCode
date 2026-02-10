
def all_same(lst):
    return len(set(lst)) == 1

def invalids_in_range(first: int, second: int) -> list[int]:
    invalids: list[int] = []
    for i in range(first, second + 1):
        current_subset_size = 1
        i_string = str(i)
        while current_subset_size <= int(len(i_string) / 2): #max substring is half + half
            subsets =[i_string[i: i+ current_subset_size] for i in range(0, len(i_string), current_subset_size)] 
            if all_same(subsets):
                if i not in invalids:
                    invalids.append(i)
            current_subset_size += 1
    return invalids



input_file = open("input.txt", "r+")

raw_input = input_file.read()

ranges = [x.strip() for x in raw_input.split(",")]

formatted_ranges = []

for r in ranges:
    both = r.split("-")
    formatted_ranges.append([int(x) for x in both])

invalids = []

for f in formatted_ranges:
    first, second = f
    current_invalids = invalids_in_range(int(first), int(second))
    for c in current_invalids:
        invalids.append(c)

assert invalids_in_range(11, 22) == [11, 22]
assert invalids_in_range(95, 115) == [99, 111]
assert invalids_in_range(998, 1012) == [999, 1010]
assert invalids_in_range(1188511880, 1188511890) == [1188511885]


print(invalids) 
print(sum(invalids))



