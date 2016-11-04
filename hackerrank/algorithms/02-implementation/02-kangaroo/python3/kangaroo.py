def kangaroo(x1, v1, x2, v2):
    # Case: Kangaroo begin at the same spot
    if x1 == x2:
        return True

    # Case: Kangaroo 1 is further and as fast or faster than Kangaroo 2
    if x1 > x2 and v1 >= v2:
        return False

    # Case: Kangaroo 2 is further and as fast or faster than Kangaroo 1
    if x2 > x1 and v2 >= v1:
        return False

    # Case: Kangaroo 1 is further but slower than Kangaroo 2
    if x1 > x2 and v1 < v2:
        while True:
            x1 = x1 + v1
            x2 = x2 + v2
            # Check if Kangaroo 2 collided with or surpassed Kangaroo 1
            if x2 == x1:
                return True
            if x2 > x1:
                return False

    # Case: Kangaroo 2 is further but slower than Kangaroo 1
    if x2 > x1 and v2 < v1:
        while True:
            x1 = x1 + v1
            x2 = x2 + v2
            # Check if Kangaroo 1 collided with or surpassed Kangaroo 2
            if x2 == x1:
                return True
            if x1 > x2:
                return False

if __name__ == "__main__":
    (x1, v1, x2, v2) = (int(e) for e in input().split())
    result = kangaroo(x1, v1, x2, v2)
    if result:
        print("YES")
    else:
        print("NO")


