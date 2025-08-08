CREATE TYPE cloud_provider AS ENUM ('aws', 'gcp', 'azure');

CREATE TABLE instance_pools (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    provider cloud_provider NOT NULL,
    region TEXT NOT NULL,
    instance_type TEXT NOT NULL,
    min_instances INTEGER NOT NULL,
    max_instances INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
