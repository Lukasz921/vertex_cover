from pulp import *
import sys

def solve_lp(filename):

    with open(filename, 'r') as f:
        lines = f.readlines()
    
    first_line = lines[0].strip().split()
    N = int(first_line[0])
    
    edges = []
    for line in lines[1:]:
        parts = line.strip().split()
        if len(parts) >= 2:
            edges.append((int(parts[0]), int(parts[1])))

    model = LpProblem("Vertex_Cover", LpMinimize)
    vertices = [LpVariable(f"x_{i}", lowBound=0, upBound=1, cat="Continuous") for i in range(N)]

    model += lpSum(vertices)

    for u, v in edges:
        model += vertices[u] + vertices[v] >= 1

    model.solve(PULP_CBC_CMD(msg=False))

    for i in range(N):
        print(f"{i} {value(vertices[i])}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        solve_lp(sys.argv[1])
    else:
        print("Cannot open file!!")