
input_file = open("input.txt", "r+")

raw_input = input_file.read()

ranges = [x.strip() for x in raw_input.split(",")]

formatted_ranges = []

for r in ranges:
    both = r.split("-")
    formatted_ranges.append([int(x) for x in both])


print(formatted_ranges)
