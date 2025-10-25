# cupnp: small tool for forwarding uPnP

`cupnp` is a tiny tool for managing port forwards on your home router. Suitable for short sessions where you just need a quick port forward, or for putting in systemd timers, cron, etc to keep ports regularly open without having to dive into your router's terrible user interface for this.

You do not need to be root to run this program; it requires no elevated permissions.

## Installation

Please install cargo and a rust compiler. [rustup makes this easy.](https://rustup.rs) Then run this:

```bash
cargo install cupnp
```

## Example

**NOTE:** a lot of (even nice ones) home routers don't support lease duration on uPnP port forwards; they will not expire, so you must remove them yourself.

```bash
# expose port 8000 over tcp with a 1 hour lease
$ cupnp expose 8000 tcp 3600

# tcp and 3600 are the default, so we can do this:
$ cupnp expose 8000

# to remove, the same, just delete instead of expose:
$ cupnp delete 8000

# for udp:
$ cupnp delete 8000 udp
```

## systemd Timer

You can create systemd files to keep this port forward up until you tell it to turn off. Should work just fine in `$HOME/.config/systemd/user` if that is desired. You will want to create a pair for each port forward.

`.timer` file:

```systemd
[Unit]
Description=cupnp port forward
After=network-online.target

[Timer]
OnStartupSec=30min
OnUnitInactiveSec=30min

[Install]
WantedBy=timers.target
```

`.service` file:

```systemd
[Unit]
Description=cupnp port forward
After=network-online.target

[Service]
Type=oneshot
ExecStart=cupnp expose 8000
ExecStop =cupnp delete 8000
```

## License

MIT

## Author

Erik Hollensbe <git@hollensbe.org>
