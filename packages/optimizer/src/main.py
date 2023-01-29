import sys
import time
import dupa
from os import path
import numpy as np
import pandas as pd
import pygad
from tqdm import tqdm
import matplotlib.pyplot as plt

if (__name__ == "__main__"):
    df_path = path.abspath(
        path.join(path.dirname(__file__), "./fixtures/btc_1d.csv"))
    manager = dupa.AssetDataProviderManager()
    id = manager.load(df_path)

    genome_declaration = {
        "rsi_length": {
            "low": 2,
            "high": 365,
            "step": 1,
        }
    }

    def compute_fitness(inputs: list[float], solution_id: int):
        genome = dict(zip(genome_declaration, inputs))
        metrics = manager.example_strategy(id, {
            "rsi_length": int(genome["rsi_length"])
        }, False)
        return metrics.omega_ratio

    def view(genome):
        metrics = manager.example_strategy(id, {
            "rsi_length": int(genome["rsi_length"])
        }, True)

        time_history = np.array(metrics.time_history)
        time_history = pd.to_datetime(time_history, unit="s")
        equity_history = np.array(metrics.equity_history)
        returns_history = np.array(metrics.returns_history)
        fill_size_history = np.array(metrics.fill_size_history)

        print(f"Sharpe Ratio: {metrics.sharpe_ratio}")
        print(f"Omega Ratio: {metrics.omega_ratio}")
        print(f"Total closed trades: {metrics.total_closed_trades}")

        df = pd.DataFrame({
            "time": time_history,
            "equity": equity_history,
            "returns": returns_history,
            "fill_size": fill_size_history
        })

        df.to_csv("output.csv", index=False)

        plt.plot(time_history, equity_history)
        plt.xlabel("Time")
        plt.ylabel("Equity")
        plt.show()

        plt.plot(time_history, returns_history)
        plt.xlabel("Time")
        plt.ylabel("Returns")
        plt.show()

        plt.plot(time_history, fill_size_history)
        plt.xlabel("Time")
        plt.ylabel("Fill Size")
        plt.show()

    num_generations = 10000
    pbar = tqdm(total=num_generations)
    ga_instance = pygad.GA(
        num_generations=num_generations,
        num_genes=len(genome_declaration),
        num_parents_mating=2,
        sol_per_pop=2,
        parent_selection_type="sss",
        # crossover_type="single_point",
        # mutation_type="random",
        # keep_parents=-1,
        # mutation_percent_genes=30,
        fitness_func=compute_fitness,
        gene_space=list(genome_declaration.values()),
        on_generation=lambda _: pbar.update(1),
        # parallel_processing=["process", 8]
    )
    ga_instance.run()
    best_genome = ga_instance.best_solution()[0]
    best_genome_mapped = dict(zip(genome_declaration, best_genome))
    print(best_genome_mapped)
    ga_instance.plot_fitness()
    view(best_genome_mapped)

    # times: list[float] = []

    # for i in range(0, 5000):
    #     start = time.perf_counter()
    #     metrics = manager.example_strategy(id, {
    #         "rsi_length": 14
    #     })
    #     end = time.perf_counter()
    #     times.append(end - start)

    # print(metrics.)

    # @dupa.Counter
    # def say_hello():
    #     print("hello")

    # say_hello()
    # say_hello()
    # say_hello()
    # say_hello()
    # say_hello()
    # say_hello()

    # print(say_hello.count)
    # print(dupa.chuj("strategy/tests/fixtures/example.csv"))
