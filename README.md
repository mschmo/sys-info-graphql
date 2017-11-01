## System Info GraphQL Server

This application exposes a GraphQL server that allows users to query system information about the machine that the server is running on.

### Installation and Usage

First download and compile

```bash
$ git clone https://github.com/mschmo/sys-info-graphql
$ cd sys-info-graphql
$ cargo build --release
```

Run the compiled program.
```bash
$ ./target/release/sys-info-graphql
```

By default the server runs on `localhost:5000`. You can modify this by setting the env var `SYS_GQL_HOST`.

### Available Metrics

| Metric     | Description |
|------------|-------------|
|osType      | Operating system type, such as "Linux", "Darwin", or "Windows"||
|osRelease   | Operating system release version number|
|hostname    | System hostname||
|cpuCount    |Number of CPUs|
|cpuSpeed    |Clock speed of the processor in MHz or GHz|
|bootTime    |On linux this is equivelant to `/proc/uptime`|
|procCount   |Current number of processes|
|diskInfo    |Disk information|
|loadAverage |Load Average|
|memoryInfo  | Memory Information|

### Example

Query:
```
{
  osType
  osRelease

  hostname

  cpuCount
  cpuSpeed(unit: GHz)

  bootTime
  procCount

  diskInfo(unit: MB) {
    free
    total
  }

  loadAverage {
    one
    five
    fifteen
  }

  memoryInfo(unit: GB) {
    total
    free
    available
    buffers
    cached
    swapTotal
    swapFree
  }
}
```
