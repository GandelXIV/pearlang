import sys, os

class Statement:
    def __init__(self):
        self.command = None
        self.arguments = None
        self.block = []

def parse(raw, entrylevel=0):
    lines = []
    statement = []
    word = ""
    isstring = False
    level = 0
    while raw != "":
        char = raw[0]
        raw = raw[1:]
        if char == "\"" :
            if not isstring:
                isstring = True
            else:
                isstring = False
            word += "\""
        elif char == "\\" and isstring: # add string escaping here in the future
            pass
        elif isstring: # blocks all other rules if we are in a string, elifs and else are not executed
            word += char
        elif char == " ":
            if word != "":
                statement.append(word)
                word = ""
        elif char == "\t":
            level += 1
        elif char == "\n":
            if word != "":
                statement.append(word)
                word = ""
            if statement != []:
                if level > entrylevel:
                    print("entering down", level, entrylevel, statement)
                    block, rest = parse(raw, entrylevel=level)
                    unindent = block.pop()
                    lines[-1].append([statement, *block])
                    lines.append(unindent)
                    raw = rest
                elif level < entrylevel:
                    print("entering up", level, entrylevel, statement)
                    lines.append(statement) # last statement is unindented
                    return lines, raw
                else:
                    print("normal", level, entrylevel, statement)
                    lines.append(statement)
                statement = []
            else: pass # just shut the autoformat
            level = 0

        else:
            word += char
    return lines

def main():
    pass

# test strings
assert parse(
"""
echo 123
echo "hello"
echo "some space here" Arg
test
"""
) == [['echo', '123'], ['echo', '"hello"'], ['echo', '"some space here"', 'Arg'], ['test']]

# basic sysntax tests
assert parse(
"""
echo 123
test
something a b c

exit
""") == [['echo', '123'], ['test'], ['something', 'a', 'b', 'c'], ['exit']]

# blocks and indentation 1
assert parse(
"""
test
while true
    \techo 1 + 1

    \tsomething

echo abc
test
""") == [['test'], ['while', 'true', [['echo', '1', '+', '1'], ['something']]], ['echo', 'abc'], ['test']]

# blocks and indentation 2
print( parse(
"""
test
while true
    \tif condition
        \t\techo something
        \t\techo else next
    \telse
        \t\tdo something else
        \tonerror
            \t\t\tidk

echo abc
test
"""))

print("All ok!")

if __name__ == "__main__":
    main()
