## System Info GraphQL Server

This application exposes a GraphQL server that allows users to query system information about the machine that the server is running on.

### Installation and Usage

First download and compile:
```
$ git clone https://github.com/mschmo/sys-info-graphql
$ cd sys-info-graphql
$ cargo build --release
```

Run the program:
```
$ ./target/release/sys-info-graphql
Server started on localhost:5000
```

By default the server runs on `localhost:5000`. You can modify this by setting the env var `SYS_GQL_HOST`.

### Available Types/Metrics

| Type       | Description |
|------------|-------------|
|osType      |Operating system type, such as "Linux", "Darwin", or "Windows"|
|osRelease   |Operating system release version number|
|hostname    |System hostname|
|cpuCount    |Number of processors. Equivalent to `get_nprocs` on Linux|
|cpuSpeed    |Clock speed of the processor in MHz or GHz|
|bootTime    |The uptime of the system (seconds), and the amount of time spent in idle process (seconds)|
|procCount   |Current total number of processes|
|diskInfo    |Overall and free disk space|
|loadAverage |Average CPU and IO utilization of the last one, five, and 10 minute periods|
|memoryInfo  |Information about the system's RAM usage. Note: On Mac OS X and Windows, the buffers and cached variables of the memoryInfo are zero.|

### Example

Query:
```
{
  osType
  osRelease

  hostname

  cpuCount
  cpuSpeed(unit: GHz)

  bootTime {
    upSeconds
    idleSeconds
  }
  
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

Result:
```json
{
  "data": {
    "osType": "Darwin",
    "osRelease": "16.7.0",
    "hostname": "Matts-MacBook-Pro.local",
    "cpuCount": "4",
    "cpuSpeed": "2.70",
    "bootTime": {
      "upSeconds": "1502954551",
      "idleSeconds": "859945"
    },
    "procCount": "313",
    "diskInfo": {
      "free": "88766.26",
      "total": "120881.01"
    },
    "loadAverage": {
      "one": "1.9091796875",
      "five": "2.4375",
      "fifteen": "2.7626953125"
    },
    "memoryInfo": {
      "total": "8.39",
      "free": "0.09",
      "available": "2.32",
      "buffers": "0.00",
      "cached": "0.00",
      "swapTotal": "0.00",
      "swapFree": "0.00"
    }
  }
}
```