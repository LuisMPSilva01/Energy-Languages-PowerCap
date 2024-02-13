def generate_dot_file():
    with open("file.dot", "w") as f:
        f.write("graph LargeGraph {\n")
        
        # Generating nodes
        f.write("\tnode [shape=circle];\n")
        f.write("\t{\n")
        f.write("\t\tnode [label=\"Node\", color=red];\n")
        for i in range(1, 1001):
            f.write(f"\t\t{i}; ")
            if i % 10 == 0:
                f.write("\n")
        f.write("\n\t}\n")

        # Generating edges
        f.write("\tedge [color=blue];\n")
        f.write("\t{\n")
        for i in range(1, 50000):
            for j in range(i+1, min(i+11, 50000)):
                f.write(f"\t\t{i} -- {j}; ")
                if j % 10 == 0:
                    f.write("\n")
        f.write("\n\t}\n")

        f.write("}")

generate_dot_file()
