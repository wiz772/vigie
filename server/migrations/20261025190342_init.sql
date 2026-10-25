
CREATE TABLE hosts (
    host_id TEXT PRIMARY KEY,
    hostname TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE agents (
    agent_id TEXT PRIMARY KEY,
    host_id TEXT NOT NULL REFERENCES hosts(host_id),
    agent_version TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE batches (
    batch_id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL REFERENCES agents(agent_id),
    protocol_version INTEGER NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE metric_types (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    unit TEXT NOT NULL
);

INSERT INTO metric_types (name, unit)
VALUES
    ('CpuPercent', '%'),
    ('MemoryTotalBytes', 'bytes'),
    ('MemoryUsedBytes', 'bytes'),
    ('DiskTotalBytes', 'bytes'),
    ('DiskUsedBytes', 'bytes');

CREATE TABLE metric_samples (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    batch_id TEXT NOT NULL REFERENCES batches(batch_id),
    timestamp_ms BIGINT NOT NULL,
    metric_type INTEGER NOT NULL REFERENCES metric_types(id),
    value_float DOUBLE PRECISION,
    value_uint NUMERIC(20, 0),
    component TEXT NOT NULL,

    CONSTRAINT metric_samples_one_value CHECK (
        (value_float IS NOT NULL AND value_uint IS NULL)
        OR
        (value_float IS NULL AND value_uint IS NOT NULL)
    )
);