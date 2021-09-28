# Grafana Dashboard Sync

A command line application to synchronize a Grafana dashboard between multiple instances.

```
grafana-dashboard-sync 0.1.0
Copy a Grafana dashboard from one instance to another.

USAGE:
    grafana-dashboard-sync --dashboard-uid <dashboard-uid> --destination-api-key <destination-api-key> --destination-url <destination-url> --source-api-key <source-api-key> --source-url <source-url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --dashboard-uid <dashboard-uid>
        --destination-api-key <destination-api-key>
        --destination-url <destination-url>
        --source-api-key <source-api-key>
        --source-url <source-url>
```

## Example

You have a dashboard accessible at url https://dashboard.staging/d/3Z-RJHuGz/dashboard that you would like to copy to a second Grafana instance at https://dashboard.production .

To find the [uid](https://grafana.com/docs/grafana/latest/http_api/dashboard/#identifier-id-vs-unique-identifier-uid) we look at the dashboard url. In this case it is `3Z-RJHuGz`.

In order to programmatically access the Grafana instances we need to set up api keys through `Configuration -> API Keys` for both instances.

Now we can run the application:

```
cargo run -- --dashboard-uid 3Z-RJHuGz --source-url https://dashboard.staging --source-api-key {api-key} --destination-url https://dashboard.prod --destination-api-key {api-key}
```
