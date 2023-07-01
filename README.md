# gRPC service built with Tonic and Instrumented with Autometrics

## Local Observability Development

The easiest way to get up and running with this application is to clone the repo and get a local Prometheus setup using the [Autometrics CLI](https://github.com/autometrics-dev/am).

Read more about Autometrics in Rust [here](https://github.com/autometrics-dev/autometrics-rs) and general docs [here](https://docs.autometrics.dev/). 

Join the Autometrics Discord:
[![Discord Shield](https://discordapp.com/api/guilds/950489382626951178/widget.png?style=shield)](https://discord.gg/kHtwcH8As9)

### Install the Autometrics CLI

The recommended installation for macOS is via [Homebrew](https://brew.sh/):

```
brew install autometrics-dev/tap/am
```

Alternatively, you can download the latest version from the [releases page](https://github.com/autometrics-dev/am/releases)

Spin up local Prometheus and start scraping your application that listens on port :8080.

```
am start :8080
```

Now you can test your endpoints and generate some traffic.

## Testing the GRPC endpoints

Easiest way to test the endpoints is with `grpcurl` (`brew install grpcurl`).

```bash
grpcurl -plaintext -import-path ./proto -proto job.proto -d '{"name": "Tonic"}' 'localhost:50051' job.JobRunner.SendJob
```

returns

```
{
  "message": "Hello Tonic!"
}
```

Getting the list of jobs (currently hardcoded to return one job)

```bash
grpcurl -plaintext -import-path ./proto -proto job.proto -d '{}' 'localhost:50051' job.JobRunner.ListJobs
```

returns:

```
{
  "job": [
    {
      "id": 1,
      "name": "test"
    }
  ]
}
```

If you now inspect the Autometrics explorer on `http://localhost:6789` you will see your metrics.
