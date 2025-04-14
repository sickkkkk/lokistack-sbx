FROM grafana/grafana:latest
ENV VM_LOGS_PLUGIN_VERSION=v0.16.3
RUN mkdir -p /var/lib/grafana/plugins/victoriametrics-logs-datasource && \
    curl -L "https://github.com/VictoriaMetrics/victorialogs-datasource/releases/download/${VM_LOGS_PLUGIN_VERSION}/victoriametrics-logs-datasource-${VM_LOGS_PLUGIN_VERSION}.zip" \
    -o /tmp/vm-logs-plugin.zip && \
    unzip /tmp/vm-logs-plugin.zip -d /var/lib/grafana/plugins/victoriametrics-logs-datasource && \
    rm /tmp/vm-logs-plugin.zip