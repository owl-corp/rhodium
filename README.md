# Rhodium

Rhodium is a rare, noble metal. Best known for its role in [photographic film processing](https://en.wikipedia.org/wiki/Rhodium#Applications), where it's used to tone prints and make them last basically forever. We're doing something vaguely similar here: taking images and distilling them down to their essence.

A Cloudflare Worker that accepts an image URL and returns a [perceptual hash](https://en.wikipedia.org/wiki/Perceptual_hashing) of the image, useful for near-duplicate detection.

## CI/CD

This repo is configured to be built in Cloudflare's infrastructure on push to main.

The `package.json` and `package-lock.json` exist solely to trick Cloudflare into caching.

The npm cache directory (`~/.npm`) is used to persist the Rust toolchain (`RUSTUP_HOME`) and Cargo registry/binaries (`CARGO_HOME`) between builds, avoiding a full Rust reinstall on every deploy.

The following build variables are configured in the Cloudflare dashboard to allow for this:

| Variable | Value |
|---|---|
| `CARGO_HOME` | `/opt/buildhome/.npm/cargo` |
| `RUSTUP_HOME` | `/opt/buildhome/.npm/rustup` |
| `CARGO_TARGET_DIR` | `/opt/buildhome/.npm/cargo-target` |
| `PATH` | `/opt/buildhome/.npm/cargo/bin:/opt/buildhome/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin` |

The build and deploy commands configured in the Cloudflare dashboard:

**Build command:**
```sh
[ -f "$RUSTUP_HOME/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo" ] || (curl https://sh.rustup.rs -sSf | sh -s -- -y --target wasm32-unknown-unknown) && . $CARGO_HOME/env && cargo install -q worker-build@0.8.5 --locked
```

**Deploy command:**
```sh
. $CARGO_HOME/env && npx wrangler deploy
```

## API

**Request**

JSON request:

```
POST /
Authorization: Bearer <your-api-key>
Content-Type: application/json

{"url":"https://example.com/image.jpg"}
```

Non-JSON requests are also accepted. If `Content-Type` is not `application/json`, the raw request body is used as the `url` value:

```
POST /
Authorization: Bearer <your-api-key>
Content-Type: text/plain

https://example.com/image.jpg
```

For JSON payloads, unknown keys are ignored and only `url` is used.

**Response**

```json
{ "hex": "a1b2c3d4e5f60718", "i64": -6801736598826993896 }
```

- `hex`: the 64-bit perceptual hash as a hex string
- `i64`: the same hash as a signed 64-bit integer (handy for database storage)

**Error response**

```json
{ "error": "...", "extra": { "...": "..." } }
```

- `error`: human-readable error message
- `extra`: optional map for additional error context; always returned as a map (empty if no context provided)

**Python example**

```python
import requests

API_URL = "https://worker-host.dev/"
API_KEY = "your-api-key"


def hamming_distance(a: int, b: int) -> int:
    return bin(a ^ b).count("1")


def is_similar(hash_a: int, hash_b: int, max_distance: int = 3) -> bool:
    return hamming_distance(hash_a, hash_b) <= max_distance


def fetch_phash(image_url: str) -> int:
    response = requests.post(
        API_URL,
        headers={
            "Authorization": f"Bearer {API_KEY}",
            "Content-Type": "application/json",
        },
        json={"url": image_url},
    )
    response.raise_for_status()
    return response.json()["i64"]


hash_a = fetch_phash("https://example.com/image-a.jpg")
hash_b = fetch_phash("https://example.com/image-b.jpg")

print("A:", hash_a)
print("B:", hash_b)
print("Similar:", is_similar(hash_a, hash_b, max_distance=3))
```


## Development

```sh
# Install pre-commit hooks
uvx prek install -f

# Run pre-commit hooks manually
uvx prek run --all-files

# Build and deploy
npx wrangler deploy

# Local dev (with live reload)
npx wrangler dev
```
