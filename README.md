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

The `config` argument is a path to a config file like the [example](config.toml).

The destination dashboard will either be created from scratch if it does not yet exist or will have its version incremented so that the previous version remains accessible in the Grafana history.
