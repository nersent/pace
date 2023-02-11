from typing import Any, Callable, Generic, TypeVar, Union
from packages.optimizer.src.base.optimization.parameter import FloatParam, IntegerParam, Param, get_params
import pygad
import numpy as np


class GeneticAlgorithmOptimizer():
    def __init__(
        self,
        params,
        criterion: Callable[[dict], float],
        generations: int = 1000,
        on_generation: Callable[[int, int], None] = None,
        print_genome=False,
        **args
    ):
        params = get_params(params)
        self.params = params
        self.genome = self._build_genome(params)

        self.criterion = criterion
        self.generations = generations
        self.on_generation = on_generation

        if print_genome:
            print(self.genome)

        def fitness_func(solution, sol_idx):
            return self._fitness(solution, sol_idx)

        def on_generation(instance: pygad.GA):
            self.on_generation(
                instance.generations_completed, self.generations)

        self.ga_instance = pygad.GA(
            num_generations=self.generations,
            num_genes=len(self.genome),
            num_parents_mating=2,
            sol_per_pop=2,
            parent_selection_type="sss",
            fitness_func=fitness_func,
            gene_space=list(self.genome.values()),
            on_generation=on_generation if self.on_generation else None,
            gene_type=np.float32,
            **args
        )

    def _build_genome(self, params: dict[str, Param]) -> dict[str, dict[str, Any]]:
        genome = {}
        for name, param in params.items():
            if isinstance(param, IntegerParam) or isinstance(param, FloatParam):
                genome[name] = {}

                if param._min is not None:
                    genome[name]["low"] = param._min
                if param._max is not None:
                    genome[name]["high"] = param._max
                if param._step is not None:
                    genome[name]["step"] = param._step

        return genome

    def _genome_to_params(self, genome: list) -> dict[str, Any]:
        params = dict(zip(self.genome, genome))
        normalized_params = {}
        for name, param in self.params.items():
            if isinstance(param, IntegerParam):
                normalized_params[name] = int(params[name])
            elif isinstance(param, FloatParam):
                normalized_params[name] = float(params[name])
            else:
                normalized_params[name] = params[name]
        return normalized_params

    def _fitness(self, genome: list, solution_id) -> float:
        params = self._genome_to_params(genome)
        return self.criterion(params)

    def run(self) -> dict[str, Any]:
        self.ga_instance.run()

        best_solution_genome = self.ga_instance.best_solution()[0]
        best_params = self._genome_to_params(best_solution_genome)

        return best_params
