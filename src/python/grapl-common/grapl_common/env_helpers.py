from __future__ import annotations

import logging
import os
from typing import TYPE_CHECKING, Any, Callable, NamedTuple, TypeVar

from botocore.client import Config
from typing_extensions import Protocol

if TYPE_CHECKING:
    from mypy_boto3_cloudwatch import CloudWatchClient  # pants: no-infer-dep
    from mypy_boto3_dynamodb import (  # pants: no-infer-dep
        DynamoDBClient,
        DynamoDBServiceResource,
    )
    from mypy_boto3_ec2 import EC2ServiceResource  # pants: no-infer-dep
    from mypy_boto3_route53 import Route53Client  # pants: no-infer-dep
    from mypy_boto3_s3 import S3Client, S3ServiceResource  # pants: no-infer-dep
    from mypy_boto3_secretsmanager import SecretsManagerClient  # pants: no-infer-dep
    from mypy_boto3_sns import SNSClient  # pants: no-infer-dep
    from mypy_boto3_sqs import SQSClient  # pants: no-infer-dep
    from mypy_boto3_ssm import SSMClient  # pants: no-infer-dep


T = TypeVar("T", covariant=True)


class FromEnv(Protocol[T]):
    def from_env(self, config: Config | None = None) -> T:
        pass


class FromEnvException(Exception):
    pass


class ClientGetParams(NamedTuple):
    boto3_client_name: str


def _client_get(
    client_create_fn: Callable[..., Any],
    params: ClientGetParams,
    config: Config | None = None,
) -> Any:
    """
    :param client_create_fn: the `boto3.client` or `boto3.resource` function
    """
    which_service = params.boto3_client_name
    endpoint_url = os.getenv("GRAPL_AWS_ENDPOINT")
    access_key_id = os.getenv("GRAPL_AWS_ACCESS_KEY_ID")
    access_key_secret = os.getenv("GRAPL_AWS_ACCESS_KEY_SECRET")
    access_session_token = os.getenv("GRAPL_AWS_ACCESS_SESSION_TOKEN")

    # determine the aws region
    if config is not None and config.region_name is not None:
        # prefer config's region if set
        region = config.region_name
    else:
        region = os.getenv("AWS_DEFAULT_REGION") or os.getenv("AWS_REGION")

    if not region:
        raise FromEnvException(
            "Please set AWS_REGION, AWS_DEFAULT_REGION, or config.region_name"
        )

    if all((endpoint_url, access_key_id, access_key_secret)):
        # Local, all are passed in from docker-compose.yml
        logging.info(f"Creating a local client for {which_service}")
        return client_create_fn(
            params.boto3_client_name,
            endpoint_url=endpoint_url,
            aws_access_key_id=access_key_id,
            aws_secret_access_key=access_key_secret,
            aws_session_token=access_session_token,
            region_name=region,
            config=config,
        )
    elif endpoint_url and not any((access_key_id, access_key_secret)):
        # Local or AWS doing cross-region stuff
        return client_create_fn(
            params.boto3_client_name,
            endpoint_url=endpoint_url,
            region_name=region,
            config=config,
        )
    elif not any((endpoint_url, access_key_id, access_key_secret)):
        # AWS
        logging.info(f"Creating a prod client for {which_service}")
        return client_create_fn(
            params.boto3_client_name,
            region_name=region,
            config=config,
        )
    else:
        raise FromEnvException(
            f"You specified access key but not endpoint for {params.boto3_client_name}?"
        )


_SQSParams = ClientGetParams(
    boto3_client_name="sqs",
)


class SQSClientFactory(FromEnv["SQSClient"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.client

    def from_env(self, config: Config | None = None) -> SQSClient:
        client: SQSClient = _client_get(
            self.client_create_fn, _SQSParams, config=config
        )
        return client


_SNSParams = ClientGetParams(
    boto3_client_name="sns",
)


class SNSClientFactory(FromEnv["SNSClient"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.client

    def from_env(self, config: Config | None = None) -> SNSClient:
        client: SNSClient = _client_get(
            self.client_create_fn, _SNSParams, config=config
        )
        return client


_EC2Params = ClientGetParams(
    boto3_client_name="ec2",
)


class EC2ResourceFactory(FromEnv["EC2ServiceResource"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.resource

    def from_env(self, config: Config | None = None) -> EC2ServiceResource:
        client: EC2ServiceResource = _client_get(
            self.client_create_fn, _EC2Params, config=config
        )
        return client


_SSMParams = ClientGetParams(
    boto3_client_name="ssm",
)


class SSMClientFactory(FromEnv["SSMClient"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.client

    def from_env(self, config: Config | None = None) -> SSMClient:
        client: SSMClient = _client_get(
            self.client_create_fn, _SSMParams, config=config
        )
        return client


_CloudWatchParams = ClientGetParams(
    boto3_client_name="cloudwatch",
)


class CloudWatchClientFactory(FromEnv["CloudWatchClient"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.client

    def from_env(self, config: Config | None = None) -> CloudWatchClient:
        client: CloudWatchClient = _client_get(
            self.client_create_fn, _CloudWatchParams, config=config
        )
        return client


_Route53Params = ClientGetParams(
    boto3_client_name="route53",
)


class Route53ClientFactory(FromEnv["Route53Client"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.client

    def from_env(self, config: Config | None = None) -> Route53Client:
        client: Route53Client = _client_get(
            self.client_create_fn, _Route53Params, config=config
        )
        return client


_S3Params = ClientGetParams(
    boto3_client_name="s3",
)


class S3ClientFactory(FromEnv["S3Client"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.client

    def from_env(self, config: Config | None = None) -> S3Client:
        client: S3Client = _client_get(self.client_create_fn, _S3Params, config=config)
        return client


class S3ResourceFactory(FromEnv["S3ServiceResource"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.resource

    def from_env(self, config: Config | None = None) -> S3ServiceResource:
        client: S3ServiceResource = _client_get(
            self.client_create_fn, _S3Params, config=config
        )
        return client


_DynamoDBParams = ClientGetParams(
    boto3_client_name="dynamodb",
)


class DynamoDBResourceFactory(FromEnv["DynamoDBServiceResource"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.resource

    def from_env(self, config: Config | None = None) -> DynamoDBServiceResource:
        client: DynamoDBServiceResource = _client_get(
            self.client_create_fn, _DynamoDBParams, config=config
        )
        return client


class DynamoDBClientFactory(FromEnv["DynamoDBClient"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.client

    def from_env(self, config: Config | None = None) -> DynamoDBClient:
        client: DynamoDBClient = _client_get(
            self.client_create_fn, _DynamoDBParams, config=config
        )
        return client


_SecretsManagerParams = ClientGetParams(
    boto3_client_name="secretsmanager",
)


class SecretsManagerClientFactory(FromEnv["SecretsManagerClient"]):
    def __init__(self, boto3_module: Any):
        self.client_create_fn = boto3_module.client

    def from_env(self, config: Config | None = None) -> SecretsManagerClient:
        client: SecretsManagerClient = _client_get(
            self.client_create_fn, _SecretsManagerParams, config=config
        )
        return client
