#!/bin/bash

if (( $EUID != 0 )); then
    echo "You need to run as root. Exiting."
    exit 1
fi

# Copy binary to /opt
echo "Installing binary in /opt/batmon"
cp -v binary/batmon /opt/

# Place systemd service & timer
echo "Installing systemd files"
cp -v systemd/batmon.{service,timer} /etc/systemd/system/

# Start and enable timer
systemctl start batmon.timer && systemctl enable batmon.timer
echo -e "Running \"systemctl daemon-reload\"\n"
systemctl daemon-reload

echo "NOTE:"
echo "batmon logs to syslog. Run:"
echo "sudo journalctl -u batmon"
echo "to see log entries."

exit 0
