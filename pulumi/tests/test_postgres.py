import unittest

import pytest
from infra.postgres import PostgresConfigValues

_INSTANCE_TYPE = "arbitrary"


class TestPostgres(unittest.TestCase):
    def test_version_assertions(self) -> None:
        # Must be 13.7 or greater

        with pytest.raises(AssertionError):
            PostgresConfigValues(instance_type=_INSTANCE_TYPE, postgres_version="13.3")
        with pytest.raises(AssertionError):
            PostgresConfigValues(instance_type=_INSTANCE_TYPE, postgres_version="13")

        PostgresConfigValues(instance_type=_INSTANCE_TYPE, postgres_version="13.7")

        PostgresConfigValues(instance_type=_INSTANCE_TYPE, postgres_version="13.9")

        PostgresConfigValues(instance_type=_INSTANCE_TYPE, postgres_version="14.0")
