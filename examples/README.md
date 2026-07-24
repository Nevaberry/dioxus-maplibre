# Examples

## Web showcase — Tailscale

```bash
cd examples/showcase-web
dx serve --web --addr "$(tailscale ip -4)" --port 8080 --locked
```

Open `http://<tailscale-ip>:8080` from another tailnet device.

## Mobile showcase — localhost

```bash
cd examples/showcase-mobile
dx serve --web --addr 127.0.0.1 --port 8081 --locked
```

Open `http://localhost:8081`.

## Mobile showcase — Tailscale

View only:

```bash
cd examples/showcase-mobile && dx serve --web --addr "$(tailscale ip -4)" --port 8081 --locked
```

Open `http://<tailscale-ip>:8081`.

PWA/offline testing over HTTPS:

```bash
tailscale serve --bg http://127.0.0.1:8081 && cd examples/showcase-mobile && dx serve --web --addr 127.0.0.1 --port 8081 --locked
```

Cleanup: `tailscale serve reset`.
