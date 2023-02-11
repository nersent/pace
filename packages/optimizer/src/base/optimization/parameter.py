from abc import ABC, abstractmethod
from typing import Optional


class Param(ABC):
    _id = 0

    def __init__(self):
        self._id = Param._id
        Param._id += 1

    @abstractmethod
    def __call__(self):
        pass

    @abstractmethod
    def _set(self, value):
        pass


class IntegerParam(Param):
    def __init__(self, default_value: int):
        super().__init__()
        self._default_value = default_value
        self._min: Optional[int] = None
        self._max: Optional[int] = None
        self._possible_values: list[int] = []
        self._current_value = default_value
        self._step: Optional[int] = None

    def min_max(self, min, max) -> 'IntegerParam':
        self._min = min
        self._max = max
        return self

    def list(self, values: list[int]) -> 'IntegerParam':
        self._possible_values = values
        if self._min is None:
            self._min = 0
        if self._max is None:
            self._max = len(values)
        if self._step is None:
            self._step = 1
        return self

    def step(self, step: int) -> 'IntegerParam':
        self._step = step
        return self

    def __call__(self):
        return self._current_value

    def _set(self, value: int):
        self._current_value = int(value)


class FloatParam(Param):
    def __init__(self, default_value: float):
        super().__init__()
        self._default_value = default_value
        self._min: Optional[float] = None
        self._max: Optional[float] = None
        self._possible_values: list[float] = []
        self._current_value = default_value
        self._step: Optional[float] = None

    def min_max(self, min, max) -> 'FloatParam':
        self._min = min
        self._max = max
        return self

    def list(self, values: list) -> 'FloatParam':
        self._possible_values = values
        return self

    def step(self, step: float) -> 'FloatParam':
        self._step = step
        return self

    def __call__(self):
        return self._current_value

    def _set(self, value: float):
        self._current_value = float(value)


def filter_param_attrs(obj) -> list[str]:
    return list(filter(lambda r: isinstance(
        getattr(obj, r), Param), dir(obj)))


def get_params(obj) -> dict[str, Param]:
    return {k: getattr(obj, k) for k in filter_param_attrs(obj)}
