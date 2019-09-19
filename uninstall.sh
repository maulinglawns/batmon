#!/bin/bash

if (( $EUID != 0 )); then
    echo "You need to run as root. Exiting."
    exit 1
fi

# Stop systemd
echo "Stopping systemd files"
systemctl stop batmon.timer && systemctl disable batmon.timer

# Remove systemd files
echo "Deleting systemd files"
rm -rf /etc/systemd/system/batmon.*

# Remove binary
echo "Removing binary in /opt/batmon"
rm -rf /opt/batmon

echo "Uninstall complete"

exit 0
