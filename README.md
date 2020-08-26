# ddns-updater #

It updates Hurricane Electric dynamic DNS records, ideally as a service.

## Configuration format ##

Configuration files should be TOML files with records stored in the following
format:
```toml
[comment]
host = "host-name"
key = "secret-key"
```

All comments should be unique, and should not contain periods. Technically in
TOML, what we're using as a comment is actually a key in the overall key-value
tree, and this results in some unfortunate emergent behaviors.

## Future plans ##

- Support for more record types. The default will remain A records, with
  reasonable defaults for any additional fields so existing configuration files
  will continue to work.
- Support for more endpoints. I use HE for my DNS, so it's what I test the
  most, but other endpoints with the same API should work fine.
- Support for listening for IP changes? This seems to be a very open-ended
  question right now.
