#!/bin/bash

if (( $EUID != 0 )); then
    echo "You need to run as root. Exiting."
    exit 1
fi

# Copy binary to /opt
cp -v binary/batmon /opt/

# Place systemd service & timer
cp -v systemd/batmon.{service,timer} /etc/systemd/system/

# Start and enable timer
systemctl start batmon.timer && systemctl enable batmon.timer
echo "Running \"systemctl daemon-reload\""
systemctl daemon-reload

exit 0
