def jumping_on_the_clouds_revisited(n, k, clouds):
    energy = 100
    position = 0
    original_position = int(0)

    while True:
        position += k
        if position >= n:
            position -= n

        energy -= 1

        if clouds[position] == '1':
            energy -= 2

        if position == original_position:
            return energy

        if energy <= 0:
            return 0

if __name__ == "__main__":
    import sys
    n,k = input().split(' ')
    clouds = input().split(' ')
    print(jumping_on_the_clouds_revisited(int(n), int(k), clouds))
