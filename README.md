# batmon
Power off server on low battery

### Install script
```
sudo ./install.sh 
Installing binary in /opt/batmon
'binary/batmon' -> '/opt/batmon'
Installing systemd files
'systemd/batmon.service' -> '/etc/systemd/system/batmon.service'
'systemd/batmon.timer' -> '/etc/systemd/system/batmon.timer'
Warning: The unit file, source configuration file or drop-ins of batmon.timer changed on disk. Run 'systemctl daemon-reload' to reload units.
Running "systemctl daemon-reload"

NOTE:
batmon logs to syslog. Run:
sudo journalctl -u batmon
to see log entries.
```

