# calandriaha
```
aijay@CT101:~/calandriaha# RUST_LOG=calandriaha=debug cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/calandriaha`
[2022-04-28T22:13:30Z INFO  calandriaha] Starting status server on [::]:6969
[2022-04-28T22:13:30Z INFO  calandriaha] interrogate local system to determine local state
[2022-04-28T22:13:35Z INFO  calandriaha] Request status update from http://[::1]:6969
[2022-04-28T22:13:35Z DEBUG calandriaha] Serve GetStateResponse: Aquire Lock
[2022-04-28T22:13:35Z DEBUG calandriaha] Serve GetStateResponse: Release Lock
[2022-04-28T22:13:35Z DEBUG calandriaha] Status Update RESPONSE=Response {
        metadata: MetadataMap {
            headers: {
                "content-type": "application/grpc",
                "date": "Thu, 28 Apr 2022 22:13:35 GMT",
                "grpc-status": "0",
            },
        },
        message: GetStateResponse {
            ready_to_serve: true,
            prefered_serving_shard: 1,
            direct_connect_healthy: false,
            gateway_v4_healthy: false,
            gateway_v6_healthy: false,
            serving_shards: [],
        },
        extensions: Extensions,
    }
[2022-04-28T22:13:35Z DEBUG calandriaha] Updating GetStateResponse: Aquire Lock
[2022-04-28T22:13:35Z DEBUG calandriaha] Updating GetStateResponse: Release Lock
[2022-04-28T22:13:35Z DEBUG calandriaha] Ping: Direct Connect Target Healthy: true
[2022-04-28T22:13:35Z DEBUG calandriaha] Ping: IPv4 Gateway Target Healthy: true
[2022-04-28T22:13:35Z DEBUG calandriaha] Ping: IPv6 Gateway Target Healthy: true
[2022-04-28T22:13:35Z DEBUG calandriaha] Updating GetStateResponse: Aquire Lock
[2022-04-28T22:13:35Z DEBUG calandriaha] Updating GetStateResponse: Release Lock


[2022-04-28T22:13:40Z INFO  calandriaha] Request status update from http://[::1]:6969
[2022-04-28T22:13:40Z DEBUG calandriaha] Serve GetStateResponse: Aquire Lock
[2022-04-28T22:13:40Z DEBUG calandriaha] Serve GetStateResponse: Release Lock
[2022-04-28T22:13:40Z DEBUG calandriaha] Status Update RESPONSE=Response {
        metadata: MetadataMap {
            headers: {
                "content-type": "application/grpc",
                "date": "Thu, 28 Apr 2022 22:13:40 GMT",
                "grpc-status": "0",
            },
        },
        message: GetStateResponse {
            ready_to_serve: true,
            prefered_serving_shard: 2,
            direct_connect_healthy: true,
            gateway_v4_healthy: true,
            gateway_v6_healthy: true,
            serving_shards: [],
        },
        extensions: Extensions,
    }
[2022-04-28T22:13:40Z DEBUG calandriaha] Updating GetStateResponse: Aquire Lock
[2022-04-28T22:13:40Z DEBUG calandriaha] Updating GetStateResponse: Release Lock
[2022-04-28T22:13:40Z DEBUG calandriaha] Ping: Direct Connect Target Healthy: true
[2022-04-28T22:13:40Z DEBUG calandriaha] Ping: IPv4 Gateway Target Healthy: true
[2022-04-28T22:13:40Z DEBUG calandriaha] Ping: IPv6 Gateway Target Healthy: true
[2022-04-28T22:13:40Z DEBUG calandriaha] Updating GetStateResponse: Aquire Lock
[2022-04-28T22:13:40Z DEBUG calandriaha] Updating GetStateResponse: Release Lock
```
