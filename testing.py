SOURCE = "The moonlight cast a gentle glow on the tranquil lake, illuminating the ripples in the water."
TARGET = "The sunlights cast a beamin glow on the radiants lake, illuminating the pattern in the water."


source_ints = [ord(c) for c in SOURCE]
target_ints = [ord(c) for c in TARGET]


def is_more_then_50_percent_different(lst: list[int], percent = 50) -> bool:
    if percent == 0:
        return not all(num == 0 for num in lst)
    non_zero_count = sum(1 for num in lst if num != 0)
    threshold = len(lst) * percent / 100
    return non_zero_count > threshold
 
MIN_CHUNK_LENGTH = 0
MAX_DIFFERENCE = 50

def copy_diff(source: list[int], target: list[int]) -> list[int]:
    chunk = []
    for i in range(len(source)):
        chunk.append(source[i] - target[i])
        if i == len(source):
            break
        elif is_more_then_50_percent_different(chunk, MAX_DIFFERENCE) and len(chunk) > MIN_CHUNK_LENGTH:
            chunk.pop()
            break
    return chunk
        
def edit_diff(source: list[int], target: list[int]) -> (list[int], list[int]):
    remove = []
    add = []
    for i in range(len(source)):
        remove.append(source[i])
        add.append(target[i])
        if i == len(source) or source[i] == target[i]:
            break
    return (remove, add)
        
def manage_diff(source: list[int], target: list[int]):
    diff = []
    i = 0
    while i < len(source):
        if source[i] == target[i]:
            copy_chunk = copy_diff(source[i:], target[i:])
            diff.append(["C"] + copy_chunk)
            i += len(copy_chunk)
        else:
            remove_chunk, add_chunk = edit_diff(source[i:], target[i:])
            diff.append(["R"]+remove_chunk)
            diff.append(["A"]+add_chunk)
            i += len(add_chunk)
    return diff

def flatten(lst):
    flat_list = []
    for row in lst:     
        flat_list.extend(row)
    return flat_list

def print_stats(diff_bit_length: int):
    diff_byte_length = diff_bit_length // 8
    total_text_length = len(SOURCE) + len(TARGET)
    print(diff_byte_length < total_text_length)
    print(diff_byte_length)
    print(total_text_length)
    
if __name__ == "__main__":
    print(source_ints, target_ints, sep="\n")
    diffs = manage_diff(source_ints, target_ints)
    diff = "".join(str(c) for c in flatten(diffs))
    diff =  "".join([c for c in list(diff) if c != "-"])
    print(diff)
    print_stats(315)


    