# Grafana Dashboard Sync

A command line application to synchronize a Grafana dashboard between multiple instances.

```
grafana-dashboard-sync 0.1.0
Copy a Grafana dashboard from one instance to another.

USAGE:
    grafana-dashboard-sync --config <config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --config <config>
```

The `config` argument is a path to a config file that like

```toml
source_url = "https://dashboard.staging"
source_api_key = "api-key"
source_dashboard_uid = "3Z-RJHuGz"
destination_url = "https://dashboard.prod"
destination_api_key = "api-key"
```

The uid refers to https://grafana.com/docs/grafana/latest/http_api/dashboard/#identifier-id-vs-unique-identifier-uid and can be seen in the browser url.

The api keys can be configured in the Grafana web interface through `Configuration -> API Keys`.

The destination dashboard will either be created from scratch if it does not yet exist or will have its version incremented so that the previous version remains accesssible in the Grafana history.
