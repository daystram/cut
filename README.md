# :clipboard: Cut

[![Gitlab Pipeline Status](https://img.shields.io/gitlab/pipeline/daystram/cut/main)](https://gitlab.com/daystram/cut/-/pipelines)
[![Docker Pulls](https://img.shields.io/docker/pulls/daystram/cut)](https://hub.docker.com/r/daystram/cut)
[![MIT License](https://img.shields.io/github/license/daystram/cut)](https://github.com/daystram/cut/blob/master/LICENSE)

URL, snippet, and file sharing with ease. All content shared via __Cut__ is ephemeral, only in-memory storage is used.

## Features
- URL shortener
- Snippet editor with code highlighter
- Ephemeral file sharing
- Set expiry to shared content (with _'Burn on Read'_)
- Raw content endpoint (302 redirect, text response, or Content-Disposition download, respectively)

## Services
The application comes in two parts:

|Name|Code Name|Stack|
|----|:-------:|-----|
|Back-end|`cut-be`|[Rust](https://www.rust-lang.org/), [Actix Web](https://github.com/actix/actix-web), [Redis](https://redis.io/)|
|Front-end|`cut-fe`|[TypeScript](https://www.typescriptlang.org/), [Vue.js](https://vuejs.org/)|

## Develop
### cut-be
To ease development, [cargo-watch](https://github.com/passcod/cargo-watch) is used to live-reload the application. Install the tool as documented.

To begin developing, simply enter the sub-directory and run the development server:
```shell
$ cd cut-be
$ cargo watch -x 'run'
```

### cut-fe
Populate `.env.development` with the required credentials. 

To begin developing, simply enter the sub-directory and run the development server:
```shell
$ cd cut-fe
$ yarn
$ yarn serve
```

## Deploy
`cut-be`, and `cut-fe` are containerized and pushed to [Docker Hub](https://hub.docker.com/r/daystram/cut). They are tagged based on their application name and version, e.g. `daystram/cut:be` or `daystram/cut:be-v2.0.1`.

To run `cut-be`, run the following:
```console
$ docker run --name cut-be --env-file ./.env -p 8080:8080 -d daystram/cut:be
```

And `cut-fe` as follows:
```console
$ docker run --name cut-fe -p 80:80 -d daystram/cut:fe
```

### Dependencies
The following are required for `cut-be` to function properly:
- [Redis](https://redis.io/)

Their credentials must be provided in their respective services' configuration file.

### Docker Compose
For ease of deployment, the following `docker-compose.yml` file can be used to orchestrate the stack deployment:
```yaml
version: "3"
services:
  cut-be:
    image: daystram/cut:be
    ports:
      - "8080:8080"
    env_file:
      - /path_to_env_file/.env
    restart: unless-stopped
  cut-fe:
    image: daystram/cut:fe
    ports:
      - "80:80"
    restart: unless-stopped
  redis:
    image: redis:6.0-alpine
    expose:
      - 6379
    volumes:
      - /path_to_redis_data:/data
    restart: unless-stopped
```

## License
This project is licensed under the [MIT License](https://github.com/daystram/cut/blob/master/LICENSE).