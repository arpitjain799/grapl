variable "localstack_tag" {
  type        = string
  description = "The tagged version of localstack we should deploy."
}

variable "kafka_broker_port" {
  type        = number
  description = "Kafka Broker's port to listen on, for other Nomad clients"
  default     = 19092
}

variable "kafka_broker_port_for_host_os" {
  type        = number
  description = "Kafka Broker's port to listen on, for things on the host OS (like Pulumi)"
  default     = 29092
}

variable "kafka_jmx_port" {
  type        = number
  description = "Port for Kafka JMX"
  default     = 9101
}

variable "localstack_port" {
  type        = number
  description = "Port for Localstack"
  default     = 4566
}

variable "zookeeper_port" {
  type        = number
  description = "Port for Zookeeper"
  default     = 2181
}

variable "PLUGIN_REGISTRY_DB_USERNAME" {
  type        = string
  description = "The username for the plugin registry db"
  default     = "postgres"
}

variable "PLUGIN_REGISTRY_DB_PASSWORD" {
  type        = string
  description = "The password for the plugin registry db"
  default     = "postgres"
}


variable "PLUGIN_WORK_QUEUE_DB_USERNAME" {
  type        = string
  description = "The username for the plugin work queue db"
  default     = "postgres"
}

variable "PLUGIN_WORK_QUEUE_DB_PASSWORD" {
  type        = string
  description = "The password fort he plugin work queue db"
  default     = "postgres"
}

variable "ORGANIZATION_MANAGEMENT_DB_USERNAME" {
  type        = string
  description = "The username for the organization management db"
  default     = "postgres"
}

variable "ORGANIZATION_MANAGEMENT_DB_PASSWORD" {
  type        = string
  description = "The password for the organization management db"
  default     = "postgres"
}

# These database ports must match what's in
# `pulumi/grapl/__main__.py`; sorry for the duplication :(
variable "PLUGIN_REGISTRY_DB_PORT" {
  type        = number
  description = "The port for the plugin registry db"
  default     = 5432
}

variable "PLUGIN_WORK_QUEUE_DB_PORT" {
  type        = number
  description = "The port for the plugin work queue db"
  default     = 5532
}

variable "ORGANIZATION_MANAGEMENT_DB_PORT" {
  type        = number
  description = "The port for the organization management db"
  default     = 5632
}

locals {
  # This is the equivalent of `localhost` within a bridge network.
  # Useful for, for instance, talking to Zookeeper from Kafka without Consul Connect
  localhost_within_bridge = attr.unique.network.ip-address
  zookeeper_endpoint      = "${local.localhost_within_bridge}:${var.zookeeper_port}"
}


####################
# Jobspecs
####################
# NOTES:
# - Services in `grapl-core.nomad` should not try to service-discover
#   local-infra services via Consul Connect; use bridge+static.
#   This is because these services won't exist in prod.

# This job is to spin up infrastructure needed to run Grapl locally (e.g. Redis) that we don't necessarily want to deploy in production (because AWS will manage it)
job "grapl-local-infra" {
  datacenters = ["dc1"]

  type = "service"

  group "redis" {
    # Redis will be available to Nomad Jobs (sans Consul Connect)
    # and the Host OS at localhost:6379
    network {
      mode = "bridge"
      port "redis" {
        static = 6379
      }
    }

    task "redis" {
      driver = "docker"

      config {
        image = "redis:latest"
        ports = ["redis"]
      }

      service {
        name = "redis"

        check {
          type    = "script"
          name    = "check_redis"
          command = "/bin/bash"
          # Interpolated by bash, not nomad
          args = [
            "-o", "errexit", "-o", "nounset",
            "-c",
            "redis-cli ping || exit 1",
          ]
          interval = "20s"
          timeout  = "10s"

          check_restart {
            limit           = 2
            grace           = "30s"
            ignore_warnings = false
          }
        }
      }
    }
  }

  group "localstack" {
    # Localstack will be available to Nomad Jobs (sans Consul Connect)
    # and the Host OS at localhost:4566
    network {
      mode = "bridge"
      port "localstack" {
        static = var.localstack_port
      }
    }

    task "localstack" {
      driver = "docker"

      config {
        # Once we move to Kafka, we can go back to the non-fork.
        image = "localstack-grapl-fork:${var.localstack_tag}"
        # Was running into this: https://github.com/localstack/localstack/issues/1349
        memory_hard_limit = 2048
        ports             = ["localstack"]
        privileged        = true
        volumes = [
          "/var/run/docker.sock:/var/run/docker.sock"
        ]
      }

      env {
        DEBUG        = 1
        EDGE_PORT    = var.localstack_port
        SERVICES     = "dynamodb,ec2,iam,s3,secretsmanager,sns,sqs"
        SQS_PROVIDER = "elasticmq"

        # These are used by the health check below; "test" is the
        # default value for these credentials in Localstack.
        AWS_ACCESS_KEY_ID     = "test"
        AWS_SECRET_ACCESS_KEY = "test"
      }

      service {
        name = "localstack"
        check {
          type    = "script"
          name    = "check_s3_ls"
          command = "/bin/bash"
          args = [
            "-o", "errexit", "-o", "nounset",
            "-c",
            # This uses the stuff in env { } - not Nomad interpolation.
            "aws --endpoint-url=http://localhost:${EDGE_PORT} s3 ls",
          ]
          interval = "10s"
          timeout  = "10s"

          check_restart {
            limit           = 2
            grace           = "30s"
            ignore_warnings = false
          }
        }
      }
    }
  }

  group "ratel" {
    network {
      mode = "bridge"
      port "ratel" {
        static = 8000
      }
    }

    task "ratel" {
      driver = "docker"

      config {
        image = "dgraph/ratel:latest"
        ports = ["ratel"]
      }

      service {
        name = "ratel"
      }
    }
  }

  group "kafka" {
    network {
      mode = "bridge"
      port "kafka-for-other-nomad-tasks" {
        static = var.kafka_broker_port
      }
      port "kafka-for-host-os" {
        static = var.kafka_broker_port_for_host_os
      }
    }

    task "kafka" {
      driver = "docker"

      config {
        image = "confluentinc/cp-kafka:7.0.1"
        ports = ["kafka-for-other-nomad-tasks", "kafka-for-host-os"]
      }

      resources {
        memory = 500
      }

      env {
        kafka_broker_port       = 9092 # Only used by healthcheck
        KAFKA_BROKER_ID         = 1
        KAFKA_ZOOKEEPER_CONNECT = local.zookeeper_endpoint

        # Some clients (like Pulumi) will need `host.docker.internal`
        # Some clients (like grapl-core services) will need localhost_within_bridge
        # We differentiate between which client it is based on which port we receive on.
        # So a receive on 29092 means HOST_OS
        KAFKA_ADVERTISED_LISTENERS = join(",", [
          "WITHIN_TASK://localhost:9092",
          "HOST_OS://host.docker.internal:${var.kafka_broker_port_for_host_os}",
          "OTHER_NOMADS://${local.localhost_within_bridge}:${var.kafka_broker_port}"
        ])
        KAFKA_AUTO_CREATE_TOPICS_ENABLE      = "false"
        KAFKA_LISTENER_SECURITY_PROTOCOL_MAP = "WITHIN_TASK:PLAINTEXT,HOST_OS:PLAINTEXT,OTHER_NOMADS:PLAINTEXT"
        KAFKA_INTER_BROKER_LISTENER_NAME     = "WITHIN_TASK"

        KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR         = 1
        KAFKA_TRANSACTION_STATE_LOG_MIN_ISR            = 1
        KAFKA_TRANSACTION_STATE_LOG_REPLICATION_FACTOR = 1
        KAFKA_GROUP_INITIAL_REBALANCE_DELAY_MS         = 0
        KAFKA_JMX_PORT                                 = var.kafka_jmx_port
        KAFKA_JMX_HOSTNAME                             = "localhost"
        KAFKA_LOG4J_ROOT_LOGLEVEL                      = "INFO"
      }

      service {
        name = "kafka"
        check {
          type    = "script"
          name    = "check_kafka"
          command = "/bin/bash"
          # Interpolated by bash, not nomad
          args = [
            "-o", "errexit", "-o", "nounset",
            "-c",
            "nc -vz localhost ${kafka_broker_port}",
          ]
          interval = "20s"
          timeout  = "10s"

          check_restart {
            limit           = 2
            grace           = "30s"
            ignore_warnings = false
          }
        }
      }

    }
  }

  group "zookeeper" {
    network {
      mode = "bridge"
      port "zookeeper" {
        static = var.zookeeper_port
        to     = var.zookeeper_port
      }
    }

    task "zookeeper" {
      driver = "docker"

      config {
        image = "confluentinc/cp-zookeeper:7.0.1"
        ports = ["zookeeper"] # may not be necessary
      }

      env {
        ZOOKEEPER_CLIENT_PORT = var.zookeeper_port
        ZOOKEEPER_TICK_TIME   = 2000
        KAFKA_OPTS            = "-Dzookeeper.4lw.commands.whitelist=ruok,dump"
      }

      service {
        name = "zookeeper"
        check {
          type    = "script"
          name    = "check_zookeeper"
          command = "/bin/bash"
          # Interpolated by bash, not nomad
          args = [
            "-o", "errexit", "-o", "nounset",
            "-c",
            "echo ruok | nc -w 2  localhost ${ZOOKEEPER_CLIENT_PORT} | grep imok || exit 2",
          ]
          interval = "20s"
          timeout  = "10s"

          check_restart {
            limit           = 2
            grace           = "30s"
            ignore_warnings = false
          }
        }
      }

    }
  }

  group "plugin-registry-db" {
    network {
      mode = "bridge"
      port "postgres" {
        static = var.PLUGIN_REGISTRY_DB_PORT
        to     = 5432 # postgres default
      }
    }

    task "plugin-registry-db" {
      driver = "docker"

      config {
        image = "postgres-ext:${var.localstack_tag}"
        ports = ["postgres"]
      }

      env {
        POSTGRES_USER     = var.PLUGIN_REGISTRY_DB_USERNAME
        POSTGRES_PASSWORD = var.PLUGIN_REGISTRY_DB_PASSWORD
      }

      service {
        name = "plugin-registry-db"

        check {
          type     = "script"
          name     = "check_postgres"
          command  = "pg_isready"
          args     = ["-U", "postgres"]
          interval = "20s"
          timeout  = "10s"

          check_restart {
            limit           = 2
            grace           = "30s"
            ignore_warnings = false
          }
        }
      }
    }
  }

  group "plugin-work-queue-db" {
    network {
      mode = "bridge"
      port "postgres" {
        static = var.PLUGIN_WORK_QUEUE_DB_PORT
        to     = 5432
      }
    }

    task "plugin-work-queue-db" {
      driver = "docker"

      config {
        image = "postgres-ext:${var.localstack_tag}"
        ports = ["postgres"]
      }

      env {
        POSTGRES_USER     = var.PLUGIN_WORK_QUEUE_DB_USERNAME
        POSTGRES_PASSWORD = var.PLUGIN_WORK_QUEUE_DB_PASSWORD
      }

      service {
        name = "plugin-work-queue-db"

        check {
          type     = "script"
          name     = "check_postgres"
          command  = "pg_isready"
          args     = ["-U", "postgres"]
          interval = "20s"
          timeout  = "10s"

          check_restart {
            limit           = 2
            grace           = "30s"
            ignore_warnings = false
          }
        }
      }
    }
  }


  group "organization-management-db" {
    network {
      mode = "bridge"
      port "postgres" {
        static = var.ORGANIZATION_MANAGEMENT_DB_PORT
        to     = 5432
      }
    }

    task "organization-management-db" {
      driver = "docker"

      config {
        image = "postgres-ext:${var.localstack_tag}"
        ports = ["postgres"]
      }

      env {
        POSTGRES_USER     = var.ORGANIZATION_MANAGEMENT_DB_USERNAME
        POSTGRES_PASSWORD = var.ORGANIZATION_MANAGEMENT_DB_PASSWORD
      }

      service {
        name = "organization-management-db"

        check {
          type     = "script"
          name     = "check_postgres"
          command  = "pg_isready"
          args     = ["-U", "postgres"]
          interval = "20s"
          timeout  = "10s"

          check_restart {
            limit           = 2
            grace           = "30s"
            ignore_warnings = false
          }
        }
      }
    }
  }
}
