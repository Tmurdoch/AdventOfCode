
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
    for i in range(first, second + 1):
        string_number = str(i)
        if len(string_number) % 2 != 0:
            continue
        string1 = string_number[0:int(len(string_number) / 2)]
        string2 = string_number[int(len(string_number) / 2) : ]
        if int(string1) == int(string2):
            #match
            invalids.append(int(string_number))

print(sum(invalids))



