def count_apples(house_left, house_right, apple_tree_pos, fallen_apple_distances):
    for distance in fallen_apple_distances:
        if (distance + apple_tree_pos) >= house_left and \
                (distance + apple_tree_pos) <= house_right:
            yield 1

def count_oranges(house_left, house_right, orange_tree_pos, fallen_orange_distances):
    for distance in fallen_orange_distances:
        if (distance + orange_tree_pos) >= house_left and \
                (distance + orange_tree_pos) <= house_right:
            yield 1


if __name__ == "__main__":
    (house_left, house_right) = (int(e) for e in input().split())
    (apple_tree_pos, orange_tree_pos)= (int(e) for e in input().split())
    (num_apples, num_oranges) = (int(e) for e in input().split())
    fallen_apple_distances = (int(e) for e in input().split())
    fallen_orange_distances = (int(e) for e in input().split())

    print(sum(count_apples(house_left, house_right, apple_tree_pos, fallen_apple_distances)))
    print(sum(count_oranges(house_left, house_right, orange_tree_pos, fallen_orange_distances)))


