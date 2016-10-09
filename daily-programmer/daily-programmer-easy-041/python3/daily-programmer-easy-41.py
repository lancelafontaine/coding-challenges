def printTopOrBottom(width):
    corners = '###'
    print(corners + width*'#' + corners)

def printRow(s, width):
    sides = '#  '
    print(sides + s + (width-len(s))*' ' + sides[::-1])

def surroundWithASCII(s):
    if len(s) > 80:
        width = 80
    else:
        width = len(s)

    printTopOrBottom(width)
    while len(s) > width:
        printRow(s[:width], width)
        s = s[width:]
    printRow(s, width)
    printTopOrBottom(width)

surroundWithASCII('lkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkancekjhkjahgsdjfhsgjdafadsgkdjfhadsgdsgkjfadsgkjfadsgkjfadsgkfjagfadsgfkjgf')
