
# This is not specified by default for nomad agent -dev
plugin_dir = "/opt/nomad/plugins"

####################
# Plugin configs
####################

plugin "docker" {
  # https://www.nomadproject.io/docs/drivers/docker#plugin-options
  config {
    allow_privileged = true

    volumes {
      # Required for the bind mount for docker.sock
      enabled = true
    }

    # We need net_admin for dnsmasq to work. Everything else should be default
    allow_caps = ["net_admin", "chown", "dac_override", "fsetid", "fowner", "mknod", "net_raw", "setgid", "setuid",
      "setfcap", " setpcap", "net_bind_service", "sys_chroot", "kill", "audit_write"
    ]
  }
}

plugin "firecracker-task-driver" {}

####################
# Client config
####################

client {
  meta = {
    # See constraint{} in plugin.nomad
    "is_grapl_plugin_host" = true

    # Turn on consul connect proxy debug logs. Consul connect sidecars now have access logs, etc.
    connect.log_level = "debug"
  }
}

####################
# Telemetry configs
####################

telemetry {
  # enable metrics for nomad
  # metrics path is /v1/metrics?format=prometheus
  collection_interval        = "1s"
  disable_hostname           = true
  prometheus_metrics         = true
  publish_allocation_metrics = true
  publish_node_metrics       = true
}
