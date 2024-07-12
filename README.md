# Better-Battlebit-API

**A Better Battlebit API written in Rust**

## Running

```bash
git clone "https://github.com/superyu1337/better-battlebit-api.git"
cd better-battlebit-api
cargo run --release
```

## But why?
Because the current state of the offical API is not the best. And that's putting it lightly.

Current issues I've encountered:

- Literally no documentation or API specification.
- Illegal BOM in JSON response
    - I have mentioned this to OkiDoki back in August, no change.
- The leaderboard endpoint is borderline unusable due to it's output structure.
    - It returns an array of unique objects, it can be parsed, but a better way is to split the endpoint, like in this API.
- Inconsistent encodings
    - Numbers on the official leaderboard endpoint are encoded as strings, while on the serverlist endpoint they are properly encoded as Numbers.
- Naming inconsistency
    - This one is not a huge dealbreaker like the other ones. The experience field on a clan data is `XP` instead of `Xp`.

## How does it work?
This API server requests data from the official API every minute, this data is parsed into structs using [my Rust battlebit API library](https://github.com/superyu1337/battlebit-api).  
The OpenAPI specification is built at compile time, and thanks to [utoipa](https://github.com/juhaku/utoipa) is also served through Swagger, Redoc and Rapidoc. This ensures that the Specification is always in line with what the server responds.

## Limitations
As said earlier, the limitation with this API is that the data is at a maximum, a minute old. (Unless the official API goes down)
