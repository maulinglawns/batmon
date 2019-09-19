#!/bin/bash

logdir="/var/log/batmon/"
logfile="batmon.log"


if (( $EUID != 0 )); then
    echo "You need to run as root. Exiting."
    exit 1
fi

# Create log dir if needed
if [[ ! -d "$logdir" ]]; then
    echo "Creating log directory $logdir"
    mkdir "$logdir" && touch "$logdir$logfile"
fi

# Copy binary to /opt
cp -v binary/batmon /opt/

# Place systemd service & timer
cp -v systemd/batmon.{service,timer} /etc/systemd/system/

# Start and enable timer
systemctl start batmon.timer && systemctl enable batmon.timer

exit 0
