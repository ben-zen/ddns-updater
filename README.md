# ddns-updater #

It updates Hurricane Electric dynamic DNS records, ideally as a service.

## Configuration format ##

Configuration files should be TOML files with records stored in the following
format:
```toml
[comment]
host = "host-name"
key = "secret-key"
interface = "enp3s0"
record_type = "A"
```

All comments should be unique, and should not contain periods. Technically in
TOML, what we're using as a comment is actually a key in the overall key-value
tree, and this results in some unfortunate emergent behaviors.

## Deploying ##

Build then install the binary:
```sh
$ cargo build --release
$ sudo cp target/release/ddns-updater /usr/bin/local/ddns-updater
```

The included `.service` file requires an edit to define the user the script will
run under, and then both it and the `.timer` file should be installed in
`/etc/systemd/system/`. Then, with a few `systemctl` calls, you can set up the
updater to run automatically on the hour:

```sh
# systemctl enable ddns-updater.timer
# systemctl start ddns-updater.timer
$ systemctl status ddns-updater
```

Finally, build your configuration file like the files in the `data` directory,
and install them either in `/srv/ddns`, the default location for the script, or
in your location of choice.

## Future plans ##

- Support for more record types. The default will remain A records, with
  reasonable defaults for any additional fields so existing configuration files
  will continue to work.
- Support for more endpoints. I use HE for my DNS, so it's what I test the
  most, but other endpoints with the same API should work fine.
- Support for listening for IP changes? This seems to be a very open-ended
  question right now.
