import matplotlib.pyplot as plt
import os
import sys

filename = sys.argv[1]

os.system(f"cargo run {filename}")

def plotpath(algo, dataset_file, savefile):
    plt.clf()
    path = []
    with open(f"{algo}.txt", "r") as f:
        path = [int(x) for x in f.readline().strip()[1:-1].split(",")]

    coords = []
    with open(dataset_file, "r") as f:
        for line in f:
            if line.strip() == "EOF":
                break
            if line[0].isalpha():
                continue

            line = line.strip().split()
            coords.append((float(line[1]), float(line[2])))

    n_cities = len(coords)

    # Plot points
    plt.scatter([x[0] for x in coords], [x[1] for x in coords], s=10)
    for i in range(n_cities):
        from_ = coords[path[i] - 1]
        to_ = coords[path[(i + 1) % n_cities] - 1]

        plt.plot([from_[0], to_[0]], [from_[1], to_[1]], 'r')


    plt.savefig(f"{savefile}.jpg")

dataset_name = filename.split("/")[1].split(".")[0]
plotpath("genetic", filename, f"pathplots/genetic_{dataset_name}")
plotpath("two_opt", filename, f"pathplots/two_opt_{dataset_name}")