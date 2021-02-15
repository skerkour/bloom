# Self hosting

Please first visit the [Architecture page](./architecture.md) in order to understand how pieces fit together.

You will need [Docker](https://docs.docker.com/get-docker/) or [Podman](https://podman.io/getting-started/installation) installed.


## Edit `.env`

Copy `bloom/env.example` to `.env` and edit it with good values

## Run migrations

```
$ docker run --ti --rm -v `pwd`/.env:/bloom/.env ghcr.io/skerkour/bloom:release
```

## Run Bloom

```
$ docker run --d -v `pwd`/.env:/bloom/.env -p 8080:8080 ghcr.io/skerkour/bloom:latest
```
