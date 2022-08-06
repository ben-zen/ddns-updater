# ddns-updater #

A Hurricane Electric dynamic DNS record updater, designed to be run as a service on Linux, with example SystemD unit files provided.

This implementation has checks to not try to update a record if it's currently the same address; this reduces the traffic to the DDNS service. The default endpoint for this is [ifconfig.me](https://ifconfig.me/ip/), but any service which returns a plaintext body in an HTTP response that only contains the requesting client's IP address will work.

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
