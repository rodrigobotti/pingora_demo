# pingora_demo

Very simple example -- based on the [offical
documentation](https://github.com/cloudflare/pingora/blob/main/docs/quick_start.md)
-- of a round robin load balancer writen using
[Pingora](https://github.com/cloudflare/pingora/).

The project is a single repository that builds two separate binaries:

- [server](./src/server.rs)
    - extremely simple http server using Axum
    - single endpoint `GET /` which returns the value of the environment
      variable `SERVER_NAME`
- [proxy](./src/proxy.rs)
    - round robin loadbalancer using Pingora
    - proxies and balances requests to the servers defined by the environment
      variable `SERVER_LIST`

## Running locally

```sh
docker compose up

# or, alternatively if you don't wish to lock your terminal:
# docker compose up -d
```

And go grab a water. It takes an eternity to build the docker images and run.

### Topology

```
http://127.0.0.1:8000/ --> [proxy] ----> http://server_1:8001/ [server]
                                    |
                                    |-->  http://server_2:8001/ [server]
                                    |
                                    |-->  http://server_3:8001/ [server]
```

## TODO

- [x] Add upstream health check
- [ ] Use smaller final base docker image -- while still correctly linking GLIBC
- [ ] Maybe build the images separately and reference them in the
  [docker-compose.yml](./docker-compose.yml) instead of building them through
  compose to decrease the time to run the complete topology locally
- [ ] Add some observability capabilities