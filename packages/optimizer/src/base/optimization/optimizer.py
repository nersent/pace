from abc import ABC
from typing import Union

from packages.optimizer.src.base.optimization.parameter_builder import IntegerParameter, ListParameter


class Optimizer(ABC):
    def __init__(self, params: list[Union[IntegerParameter, ListParameter]]):
        self.params = params

    def fit(self, ):
        pass
