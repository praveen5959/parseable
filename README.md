<h2 align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/parseablehq/.github/main/images/logo-dark.png">
      <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/parseablehq/.github/main/images/logo.svg">
      <a href="https://www.parseable.io" target="_blank"><img src="https://raw.githubusercontent.com/parseablehq/.github/main/images/logo.svg" alt="Parseable" width="600" height="150" /></a>
    </picture>
    <br>
    Log Lake for the cloud-native world
</h2>

<div align="center">

[![Docker Pulls](https://img.shields.io/docker/pulls/parseable/parseable?logo=docker&label=Docker%20Pulls)](https://hub.docker.com/r/parseable/parseable)
[![Slack](https://img.shields.io/badge/slack-brightgreen.svg?logo=slack&label=Community&style=flat&color=%2373DC8C&)](https://logg.ing/community)
[![Docs](https://img.shields.io/badge/stable%20docs-parseable.io%2Fdocs-brightgreen?style=flat&color=%2373DC8C&label=Docs)](https://logg.ing/docs)
[![Build](https://img.shields.io/github/checks-status/parseablehq/parseable/main?style=flat&color=%2373DC8C&label=Checks)](https://github.com/parseablehq/parseable/actions)

[Key Concepts](https://www.parseable.io/docs/concepts) | [Features](https://github.com/parseablehq/parseable#rocket-highlights) | [Documentation](https://www.parseable.io/docs) | [Demo](https://demo.parseable.com/login?q=eyJ1c2VybmFtZSI6ImFkbWluIiwicGFzc3dvcmQiOiJhZG1pbiJ9) | [Integrations](https://www.parseable.io/docs/category/integrations) | [FAQ](https://www.parseable.io/docs/faq)

</div>

Parseable is a **cloud native, log analytics platform, with a focus on performance & resource efficiency**. Parseable is useful for use cases where **complete data ownership, security and privacy are paramount**.

To experience Parseable UI, checkout [demo.parseable.com ↗︎](https://demo.parseable.com/login?q=eyJ1c2VybmFtZSI6ImFkbWluIiwicGFzc3dvcmQiOiJhZG1pbiJ9). You can also view the [demo video ↗︎](https://www.parseable.com/video.mp4).

## QuickStart :zap:

<details>
<summary><a href="https://www.parseable.com/docs/docker-quick-start">Docker Image</a></summary>
<p>

You can <a href="https://www.parseable.com/docs/docker-quick-start">get started with Parseable Docker</a> with a simple Docker run and then send data via cURL to understand how you can ingest data to Parseable. Below is the command to run Parseable in local storage mode with Docker.

```bash
docker run -p 8000:8000 \
  parseable/parseable:latest \
  parseable local-store
```

Once this runs successfully, you'll see dashboard at [http://localhost:8000 ↗︎](http://localhost:8000). You can login to the dashboard default credentials `admin`, `admin`.

To ingest data, run the below command. This will send logs to the `demo` stream. You can see the logs in the dashboard.

```bash
curl --location --request POST 'http://localhost:8000/api/v1/ingest' \
--header 'X-P-Stream: demo' \
--header 'Authorization: Basic YWRtaW46YWRtaW4=' \
--header 'Content-Type: application/json' \
--data-raw '[
    {
        "id": "434a5f5e-2f5f-11ed-a261-0242ac120002",
        "datetime": "24/Jun/2022:14:12:15 +0000",
        "host": "153.10.110.81"
    }
]'
```

</p>
</details>

<details>
<summary><a href="https://www.parseable.com/docs/docker-quick-start">Executable Binary</a></summary>
<p>

You can download and run the Parseable binary on your laptop.

- Linux or MacOS

```bash
curl -fsSL https://logg.ing/install | bash
```

- Windows

```pwsh
powershell -c "irm https://logg.ing/install-windows | iex"
```

Once this runs successfully, you'll see dashboard at [http://localhost:8000 ↗︎](http://localhost:8000). You can login to the dashboard default credentials `admin`, `admin`.

To ingest data, run the below command. This will send logs to the `demo` stream. You can see the logs in the dashboard.

```bash
curl --location --request POST 'http://localhost:8000/api/v1/ingest' \
--header 'X-P-Stream: demo' \
--header 'Authorization: Basic YWRtaW46YWRtaW4=' \
--header 'Content-Type: application/json' \
--data-raw '[
    {
        "id": "434a5f5e-2f5f-11ed-a261-0242ac120002",
        "datetime": "24/Jun/2022:14:12:15 +0000",
        "host": "153.10.110.81"
    }
]'
```

</p>
</details>

## Why Parseable :question:

### Performance & resource efficiency

Parseable is written in Rust, with a clear focus on performance while ensuring a much lower CPU and memory footprint (compared to Java, Go based systems). When compared with Elastic, Parseable uses ~80% lesser memory and ~50% lesser CPU, while offering a better ingestion rate. This means you can run Parseable on smaller instances, saving costs.

### Easy of use

One of the key challenges users said they face today is the complexity of setting a logging system like Elastic. There are so many moving parts, and it's hard to get started. Parseable is designed to be simple to use, with a single binary that can be run on almost anywhere. The Console is built in the binary itself, so you can start using it without any additional setup.

### Take control of your data

With Apache Arrow and Apache Parquet as the underlying data formats, Parseable stores log data in an optimized, compressed manner as Parquet files. This means you get complete control and access to your data. You can use Parseable query and analysis, but also can plugin tools from wider Parquet ecosystem for further processing, analysis, and visualization.

### Enterprise ready

- High availability & Cluster mode
- Local cache & storage
- [OpenTelemetry support ↗︎](https://opentelemetry.io/)
- [Alerts ↗︎](https://www.parseable.io/docs/alerts)
- [Role based access control ↗︎](https://www.parseable.io/docs/rbac)
- [OAuth2 support ↗︎](https://www.parseable.io/docs/oidc)
- [Grafana based visualization ↗︎](https://github.com/parseablehq/parseable-datasource)
- [LLM ↗︎](https://www.parseable.io/docs/llm)
- [Stats ↗︎](https://www.postman.com/parseable/workspace/parseable/request/22353706-b32abe55-f0c4-4ed2-9add-110d265888c3)

## How do people use Parseable :bulb:

- **Audit & Compliance** - Organizations that need to store logs in a secure, compliant manner. Parseable's direct to S3 bucket storage mode ensures that logs are stored in a secure, cost effective manner, and can be accessed only by authorized users, while all the data is queryable in real-time.

- **Observability & Monitoring** - A very large chunk of observability data is logs. Organizations that need to monitor their systems, applications, and infrastructure in real-time use Parseable as the primary log storage system so they get timely alerts, and can analyze logs in real-time.

- **Log Analytics** - Not all logs are created equal. For example application logs are seldom useful after a few days pass, but if same application also logs all the user interactions, that data is very valuable for product managers, and can be stored for a longer period. Several businesses store such high value logs and slice / dice them as needed.

## Motivation :dart:

Traditionally, logging has been seen as a text search problem. Log volumes were not high, and data ingestion or storage were not really issues. This led us to today, where all the logging platforms are primarily text search engines.

But with log data growing exponentially, today's log data challenges involve whole lot more – Data ingestion, storage, and observation, all at scale. We are building Parseable to address these challenges.

## Contributing :trophy:

[Contribution guide ↗︎](https://www.parseable.io/docs/contributing).

<a href="https://github.com/parseablehq/parseable/graphs/contributors"><img src="https://contrib.rocks/image?repo=parseablehq/parseable" /></a>

### Supported by

<a href="https://fossunited.org/" target="_blank"><img src="http://fossunited.org/files/fossunited-badge.svg"></a>
<img referrerpolicy="no-referrer-when-downgrade" src="https://static.scarf.sh/a.png?x-pxid=cb5c7633-1c88-4792-be58-6228c476cef5" />
