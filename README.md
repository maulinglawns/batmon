# batmon
Power off server on low battery

## Why?
If you run your server on an old laptop (it's actually not a bad idea, takes almost no place at all with the lid closed!),
you will probably not have the laptop-tools installed. Since, well, it's a server now, right?

Now, if you get a power outage (or if one of your kids suddenly decides to pull out the power cord!), your laptop/server will run along just fine as long as the battery allows.

But then it will ground to a very ungraceful **STOP**, without unmounting disks and shutting down things nicely, once the battery reaches 0 percent.

This will prevent that, and instead run a normal, graceful, `shutdown`.

Finally, if you get the chance to write _something_ called `batmon`, you should go for it!

## Ok, why Rust?
Because it was funner than a shell script.

## Dependencies?
No. Tested on Debian 9 and Debian 10. Should most likely work on *buntu as well.

## Installing
You can either:
- Clone this repo and build the binary yourself with: `cargo build --release`, or
- Clone this repo and run the install script:
```
sudo ./install.sh 
Installing binary in /opt/batmon
'binary/batmon' -> '/opt/batmon'
Installing systemd files
'systemd/batmon.service' -> '/etc/systemd/system/batmon.service'
'systemd/batmon.timer' -> '/etc/systemd/system/batmon.timer'
Created symlink /etc/systemd/system/timers.target.wants/batmon.timer → /etc/systemd/system/batmon.timer.
Running "systemctl daemon-reload"

NOTE:
batmon logs to syslog. Run:
sudo journalctl -u batmon
to see log entries.
```

## Uninstalling
Either delete the files manually, or run the uninstall script:
```
sudo ./uninstall.sh 
Stopping systemd files
Removed /etc/systemd/system/timers.target.wants/batmon.timer.
Deleting systemd files
Removing binary in /opt/batmon
Uninstall complete
```
