def append_and_delete(s, t, k):
    # Determine number of identical prefix characters
    num_prefix_chars = 0
    for i in range(len(s)):
        if s[i] == t[i]:
            num_prefix_chars += 1
        else:
            break

    # delete suffix characters from s
    nums_chars_to_delete = len(s) - num_prefix_chars
    k -= nums_chars_to_delete
    # ...

    # check if can do any extra deletes if the string is empty

    # add the appends

    # check if it is possible that k == 0




if __name__ == "__main__":
    import sys
    s = input().strip()
    t = input().strip()
    k = int(input().strip())
    print(append_and_delete(s, t, k))
