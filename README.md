# Rhodium

Rhodium is a rare, noble metal. Best known for its role in [photographic film processing](https://en.wikipedia.org/wiki/Rhodium#Applications), where it's used to tone prints and make them last basically forever. We're doing something vaguely similar here: taking images and distilling them down to their essence.

A Cloudflare Worker that accepts an image URL and returns a [perceptual hash](https://en.wikipedia.org/wiki/Perceptual_hashing) of the image, useful for near-duplicate detection.

## API

**Request**

```
POST /
Authorization: Bearer <your-api-key>
Content-Type: text/plain

https://example.com/image.jpg
```

**Response**

```json
{ "hex": "a1b2c3d4e5f60718", "i64": -6801736598826993896 }
```

- `hex`: the 64-bit perceptual hash as a hex string
- `i64`: the same hash as a signed 64-bit integer (handy for database storage)


## Development

```sh
# Build and deploy
npx wrangler deploy

# Local dev (with live reload)
npx wrangler dev
```
