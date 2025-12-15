# yt-dlp-api

yt-dlp-api is an API that handles requests to yt-dlp to download media from various media services, such as YouTube or TikTok.

The API contains only one `/fetch?url=…` route that streams the media from the provided URL into the response of the request. It attempts to use the highest quality MP4 videos available, but if none are found, it defaults to the best available video.

## Prerequisites

You have to install [yt-dlp](https://github.com/yt-dlp/yt-dlp) as a globally available program. You may also have to install additional dependencies like ffmpeg.

## Running

1. Create a config by checking the [example file](config.example.toml), and name it, for example, `config.toml`.
2. Run the app: `RUST_LOG=info CONFIG_PATH=config.toml ./yt-dlp-api` where `config.toml` is your config filename.
3. Now the web-api will be accessible at the provided address.

Note: you can also pass the config values with environment values with `YTDLP_API` prefix. For example, `YTDLP_API_SERVER_URL=localhost:5000`.
