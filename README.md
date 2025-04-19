# promtail_cleaner

Utility to clean up scraped logs by Promtail.

Ref.: https://community.grafana.com/t/a-way-of-deleting-log-files-after-promtail-has-scraped-and-forwarded-them-to-loki/60626

This image is intended to be used as a sidecar container in a pod that is running Promtail. It will delete the log files after they have been scraped by Promtail.
