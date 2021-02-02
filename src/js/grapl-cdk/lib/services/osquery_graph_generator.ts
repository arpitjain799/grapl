import * as path from 'path';
import * as cdk from '@aws-cdk/core';
import * as ec2 from '@aws-cdk/aws-ec2';
import * as s3 from '@aws-cdk/aws-s3';
import { EventEmitter } from '../event_emitters';
import { RedisCluster } from '../redis';
import { GraplServiceProps } from '../grapl-cdk-stack';
import { FargateService } from "../fargate_service";
import { ContainerImage } from "@aws-cdk/aws-ecs";

interface OSQueryGraphGeneratorProps extends GraplServiceProps {
    writesTo: s3.IBucket;
}

export class OSQueryGraphGenerator extends cdk.NestedStack {
    readonly service: FargateService;

    constructor(
        parent: cdk.Construct,
        id: string,
        props: OSQueryGraphGeneratorProps
    ) {
        super(parent, id);

        const bucket_prefix = props.prefix.toLowerCase();
        const osquery_log = new EventEmitter(
            this,
            bucket_prefix + '-osquery-log'
        );

        const event_cache = new RedisCluster(this, 'OSQueryEventCache', props);
        event_cache.connections.allowFromAnyIpv4(ec2.Port.allTcp());

        this.service = new FargateService(this, id, {
            prefix: props.prefix,
            environment: {
                RUST_LOG: props.osquerySubgraphGeneratorLogLevel,
                BUCKET_PREFIX: bucket_prefix,
                EVENT_CACHE_CLUSTER_ADDRESS: event_cache.address,
            },
            vpc: props.vpc,
            eventEmitter: osquery_log,
            writesTo: props.writesTo,
            version: props.version,
            watchful: props.watchful,
            serviceImage: ContainerImage.fromAsset(path.join(__dirname, '../../../../../src/rust/'), {
                target: "osquery-subgraph-generator-deploy",
                buildArgs: {
                    "CARGO_PROFILE": "debug"
                },
                file: "Dockerfile",
            }),
            command: ["/osquery-subgraph-generator"],
            //metric_forwarder: props.metricForwarder,
        });

        this.service.service.cluster.connections.allowToAnyIpv4(
            ec2.Port.tcp(parseInt(event_cache.cluster.attrRedisEndpointPort))
        );
    }
}
