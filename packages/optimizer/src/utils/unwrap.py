from typing import Any, Optional


def unwrap_or(value: Optional[Any], default: Any) -> Any:
    if value is None:
        return default
    return value
