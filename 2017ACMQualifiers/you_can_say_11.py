import fileinput
isMult = "is a multiple of 11."
isNotMult = "is not a multiple of 11."

for line in fileinput.input():
    nospace = line.strip(' \n\t')
    intRep = int(nospace)
    if intRep != 0:
        if intRep % 11 == 0:
            print(nospace, isMult)
        else:
            print(nospace, isNotMult)

