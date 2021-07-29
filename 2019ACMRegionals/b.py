import sys

def diff(r,m):
    s = 2*max(r,m)
    pasta = [r]
    check = [False for i in range(s)]
    inc = 1 + (r == 1)
    a = r
    count = 1

    check[a] = True
    while not check[m]:
        #print("a: {}".format(a))
        #print("inc: {}".format(inc))
        check[inc] = True
        a = a + inc
        if a < s:
            check[a] = True
        count = count + 1
        for i in pasta:
            if a - i < s:
                check[a - i] = True
        pasta.append(a)
        while inc < s and check[inc]:
            inc = inc + 1
    return count

for s in sys.stdin:
    s = s.split()
    r = int(s[0])
    m = int(s[1])
    print(diff(r,m))
