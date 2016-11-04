def circular_array_rotation(array_size, num_rotations, array, queries):
    for query in queries:
        # eliminate need for more than one spin
        yield array[query-num_rotations % array_size]

def main():
    # obtain values
    var_line = (int(elem) for elem in input().split())
    (array_size, num_rotations, num_queries) = var_line
    array = [int(elem) for elem in input().split()]
    queries = []
    for _ in range(num_queries):
        queries.append(int(input()))

    results = list(circular_array_rotation(array_size, num_rotations, array, queries))
    for elem in results:
        print(elem)

if __name__ == "__main__":
    main()

