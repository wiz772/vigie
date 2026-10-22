# Vigie Protocol

**Protocol version:** 1
**Status:** Initial MVP specification (development phase)
**Purpose:** Define the data exchanged between a Vigie agent and a Vigie server.

This document describes the protocol independently of any programming language or implementation. It defines the meaning, types, and validation rules of the data exchanged between the agent and the server.

The initial MVP focuses on system metrics. Filesystem events, operating-system logs, and Vigie agent logs are reserved for future implementation and MUST NOT be transmitted until their schemas are defined.

## 1. Normative language

The terms MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY are used as described in RFC 2119:

- **MUST / MUST NOT**: absolute requirement or prohibition.
- **SHOULD / SHOULD NOT**: recommended behavior unless there is a documented reason to do otherwise.
- **MAY**: optional behavior.

## 2. Protocol versioning

Every batch includes a `protocol_version` field.

- The initial protocol version is `1`.
- A version 1 sender MUST set `protocol_version` to `1`.
- The server MUST reject batches using an unsupported protocol version.
- During development, the protocol MAY change without preserving compatibility. The agent and server are expected to be updated together.
- The development phase ends with the first publicly tagged release of Vigie. From that release onward, changes that alter the meaning of existing fields, remove required fields, or change the interpretation of existing payloads MUST be assigned a new protocol version, and the change MUST be documented.

The protocol version is independent of the agent software version. `agent_version` identifies the agent software, while `protocol_version` identifies the data format and communication contract.

## 3. Data model

A batch contains metadata and a list of timestamped messages.

### 3.1 Batch

| Field | Type | Required | Description |
|---|---|---|---|
| `protocol_version` | Unsigned 32-bit integer | Yes | Protocol version. Version 1 uses `1`. |
| `agent_version` | String | Yes | Version of the agent software. |
| `agent_id` | String | Yes | Stable identifier for this agent installation. |
| `host_id` | String | Yes | Stable identifier for the monitored machine or virtual machine. |
| `batch_id` | String | Yes | Identifier of this batch, unique among the batches of the same agent. Used to detect duplicates. |
| `messages` | List of Message | Yes | Messages included in the batch. |

Requirements:

- `agent_version`, `agent_id`, `host_id`, and `batch_id` MUST NOT be empty.
- Identifiers MUST be treated as opaque strings by receivers. Receivers MUST NOT assume a particular format or operating system.
- The agent SHOULD persist its `agent_id` across restarts.
- The agent SHOULD keep the same `host_id` for the same machine.
- The agent MUST NOT generate a new `agent_id` or `host_id` for every batch.
- An empty `messages` list is structurally valid, although an agent SHOULD avoid sending empty batches unless necessary.

`batch_id` follows specific rules, described in section 5.3.

The exact generation, provisioning, and persistence mechanisms for `agent_id` and `host_id` are implementation details and are not defined by this specification.

### 3.2 Message

| Field | Type | Required | Description |
|---|---|---|---|
| `timestamp_ms` | Unsigned 64-bit integer | Yes | Collection time in milliseconds since the Unix epoch. |
| `payload` | Payload | Yes | The content of the message. |

`timestamp_ms` represents the collection time, expressed as milliseconds since 1970-01-01 00:00:00 UTC, according to the agent's clock. It does not represent the time at which the server received the message.

- The sender MUST provide a timestamp in Unix epoch milliseconds.
- The server SHOULD record its own receipt timestamp separately, so that clock drift between agent and server can be detected.
- The server SHOULD reject timestamps outside its accepted range. Clock-skew tolerance and the policy for future timestamps are implementation settings that the server MUST document.

### 3.3 Payload

A payload identifies the category of a message.

| Payload category | Version 1 status | Description |
|---|---|---|
| `Metrics` | Supported | Contains a MetricsPayload. |
| `FileSystem` | Reserved | Intended for future filesystem events. |
| `SystemLog` | Reserved | Intended for future operating-system logs. |
| `VigieAgentLog` | Reserved | Intended for future Vigie agent logs. |

Only `Metrics` is supported by the initial MVP. A version 1 sender MUST NOT transmit reserved categories. A receiver MUST reject them unless they are explicitly defined and supported by its implementation.

## 4. Metrics

### 4.1 MetricsPayload

| Field | Type | Required | Description |
|---|---|---|---|
| `metrics` | List of MetricSample | Yes | Metric readings collected at the message timestamp. |

- A metrics payload MAY contain several readings for different components.
- The timestamp of the containing Message applies to all readings in the payload.
- Within one payload, the same combination of metric name and `component` MUST NOT appear more than once.
- An empty `metrics` list is structurally valid, but an agent SHOULD omit a metrics message when it has no readings to report.

### 4.2 MetricSample

| Field | Type | Required | Description |
|---|---|---|---|
| `metric` | Metric | Yes | Metric name and value. |
| `component` | String | Yes | Identifies the component being measured. |

- `component` MUST NOT be empty.
- The receiver MUST treat `component` as an opaque string. It MUST NOT assume that component names are consistent across different operating systems or agents.
- The agent MUST use the same name for the same component across successive measurements, so that the server can group readings correctly.

Examples of component names: `cpu`, `memory`, `disk0`, `C:`. These examples are illustrative; a universal naming convention is not defined in version 1.

### 4.3 Metric

Version 1 defines the following metrics:

| Metric name | Value type | Unit | Validation |
|---|---|---|---|
| `CpuPercent` | 64-bit floating-point number | Percent | MUST be finite and between 0 and 100, inclusive. |
| `MemoryTotalBytes` | Unsigned 64-bit integer | Bytes | |
| `MemoryUsedBytes` | Unsigned 64-bit integer | Bytes | MUST NOT exceed the matching total (see below). |
| `DiskTotalBytes` | Unsigned 64-bit integer | Bytes | |
| `DiskUsedBytes` | Unsigned 64-bit integer | Bytes | MUST NOT exceed the matching total (see below). |

Additional requirements:

- `CpuPercent` MUST NOT be NaN or positive or negative infinity.
- The used-versus-total rule applies within a single MetricsPayload, between readings that have the same `component`. If either reading is absent, the rule is not checked.
- If a collector cannot measure a value, it MUST omit the reading. It MUST NOT report zero as a substitute for "unknown".
- A receiver MUST NOT assume that a batch contains every metric for every component.
- Missing readings MUST NOT be interpreted as zero.
- The receiver MUST reject unknown metric names in this revision (see section 9, open decisions).
- The protocol does not define a sampling frequency. The agent implementation is responsible for its collection interval.

## 5. Validation and error handling

The server MUST treat all received data as untrusted, including data from legitimate agents.

Successful deserialization is not sufficient validation. For example, a payload containing a `CpuPercent` of 150 is invalid even if it can be decoded.

Before storing a batch, the server MUST:

1. Verify that the protocol version is supported.
2. Verify that required fields are present and have the specified types.
3. Reject empty required identifiers (including `batch_id`) and component names.
4. Validate metric names and numeric values.
5. Reject unsupported payload categories.
6. Enforce its limits for request size, message count, and metric count.

If a batch is invalid, the server MUST NOT store any part of it. Partial acceptance is not defined for version 1: the server accepts and stores a valid batch, or rejects it as a whole.

### 5.1 Minimum limits

To let agents know what they can safely send, every server MUST accept batches within the following limits. A server MAY accept more, and MAY reject batches that exceed them:

- request body: 1 MiB;
- messages per batch: 1000;
- samples per MetricsPayload: 1000;
- length of any string field (including identifiers): 256 characters.

An agent SHOULD stay within these limits.

### 5.2 Permanent and temporary failures

- A batch rejected because it is invalid is a **permanent** failure. The agent MUST NOT resend the same batch unchanged. It SHOULD log the failure and discard the batch.
- A failure caused by a server that is unreachable or overloaded is **temporary**. The agent SHOULD keep the batch and retry with an increasing delay between attempts. When it retries, it MUST resend the batch with the same `batch_id` (see section 5.3).
- A batch recognized as a duplicate (section 5.3) is **not a failure**.

The transport layer MUST allow an agent to tell these cases apart. Exact HTTP status codes and the error response format are not defined yet. Error responses SHOULD NOT expose secrets or internal implementation details.

### 5.3 Batch identifiers and duplicate handling

An agent may not know whether a batch was received, for example when a connection is lost before the response arrives. It then resends the batch. `batch_id` allows the server to recognize such a resend and avoid storing the data twice.

Agent requirements:

- The agent MUST generate `batch_id` once, when it creates the batch.
- The agent MUST resend the batch with the same `batch_id` on every retry. It MUST therefore persist the `batch_id` together with the batch for as long as the batch can be resent.
- The agent MUST NOT reuse a `batch_id` for a batch with different content.
- The `batch_id` MUST be unique among the batches of the same agent. The agent SHOULD use a UUID (version 4 or 7), which satisfies this requirement. The identifier is an opaque string of at most 256 characters (see section 5.1).
- The agent MUST NOT resend a batch that is older than the minimum retention window below. It SHOULD discard it instead.

Server requirements:

- The server identifies a batch by the pair (`agent_id`, `batch_id`).
- If the server receives a batch whose pair it has already stored, it MUST NOT store the data a second time.
- The server MUST treat such a duplicate as a **success** for the agent, so that the agent stops resending it. It MAY indicate in the response that the batch was a duplicate. The exact response format is not defined yet.
- The server MUST remember the identifiers of stored batches for at least 7 days (the minimum retention window). It MAY remember them longer.
- The server SHOULD store a batch and its contents atomically (all or nothing), so that a failure in the middle of storage cannot leave a batch recorded without its contents, which would cause a legitimate resend to be wrongly ignored.

## 6. Serialization and transport

JSON is the intended initial wire format.

The exact JSON representation of Payload and Metric variants MUST be finalized against the actual serializer configuration before the protocol is considered implementation-complete. Implementations MUST agree on how variant names and associated values are represented in JSON, and MUST NOT assume that the default enum serialization of a particular programming language matches this protocol.

A concrete JSON example will be added to this section once it has been generated by the implementation and verified against the actual serialization configuration.

The MVP is expected to use an HTTP-based API. The following details are not yet defined:

- endpoint paths and HTTP methods;
- authentication headers;
- request and response formats;
- retry behavior (beyond sections 5.2 and 5.3);
- compression;
- delivery guarantees.

These details MUST be documented before interoperability between independent implementations can be claimed.

## 7. Security and privacy

- The server MUST validate all incoming data independently of the agent.
- The agent MUST NOT include passwords, authentication tokens, private keys, or unrelated personal data in payloads.
- The server SHOULD authenticate agents before accepting their data.
- Communication SHOULD use TLS when data leaves a trusted local environment.
- The server SHOULD enforce request-size and rate limits.
- Logs and error messages SHOULD avoid exposing credentials or sensitive information.
- `agent_id`, `host_id`, and `batch_id` are identifiers, not authentication credentials. Knowing an identifier alone MUST NOT grant access.
- The security of the protocol MUST NOT depend on the secrecy of this specification.

The authentication scheme, agent enrollment process, credential rotation, and authorization model remain to be specified.

## 8. Storage guidance

This protocol defines the data exchanged with the server, not its database schema.

- The server SHOULD preserve the original collection timestamp, and SHOULD also record a separate receipt timestamp.
- The server SHOULD store metric names, values, component identifiers, and timestamps in a way that supports querying historical readings.
- Duplicate batches are handled as described in section 5.3. A uniqueness constraint on (`agent_id`, `batch_id`) is one way to implement this.

Table names, indexes, retention periods, and aggregation are implementation details not defined by this protocol.

## 9. Open decisions and not yet specified

Decisions to take before the first public release:

- **Unknown data.** This revision is strict: unknown metric names and unsupported payload categories cause the whole batch to be rejected. The alternative is to ignore unknown metrics, payload categories, and fields (and count them), so that a newer agent keeps working with an older server. The behavior for unknown fields inside known objects is not specified yet.
- **Component naming convention** (for example total CPU versus individual cores, partitions versus physical disks), and whether `CpuPercent` is machine-wide or per core.

Not yet specified:

- transport protocol and endpoint definitions;
- exact JSON encoding of enum variants;
- agent authentication and authorization;
- transport encryption requirements;
- format of acknowledgements and of duplicate indications in responses;
- schemas for filesystem events, system logs, and agent logs;
- identifier generation and persistence rules for `agent_id` and `host_id`.

These details should be defined before the protocol is considered complete for production use.