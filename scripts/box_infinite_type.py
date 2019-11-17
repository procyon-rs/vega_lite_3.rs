import sys

in_struct = False

with open(sys.argv[1], 'r', encoding="utf-8", errors='replace') as f:
    with open(sys.argv[2], 'w+', encoding="utf-8", errors='replace') as t:
        for line in f:
            if "struct SpecClass" in line:
                in_struct = True
            if in_struct and "pub spec: "in line:
                t.write("pub spec: Option<Box<SpecClass>>,\n")
                in_struct = False
            else:
                t.write(line)

