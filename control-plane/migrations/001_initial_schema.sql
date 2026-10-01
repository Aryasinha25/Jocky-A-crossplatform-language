CREATE TABLE IF NOT EXISTS endpoints (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    hostname VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL,
    architecture VARCHAR(50) NOT NULL,
    jocky_version VARCHAR(50) NOT NULL,
    registered_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_seen TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE IF NOT EXISTS investigations (
    id VARCHAR(255) NOT NULL,
    endpoint_id UUID NOT NULL REFERENCES endpoints(id) ON DELETE CASCADE,
    schema_version VARCHAR(50) NOT NULL,
    target VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    risk_score INTEGER NOT NULL,
    priority VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (endpoint_id, id)
);

CREATE INDEX IF NOT EXISTS idx_investigations_endpoint_id ON investigations(endpoint_id);
CREATE INDEX IF NOT EXISTS idx_investigations_start_time ON investigations(start_time);
CREATE INDEX IF NOT EXISTS idx_investigations_priority ON investigations(priority);

CREATE TABLE IF NOT EXISTS evidence (
    id VARCHAR(255) NOT NULL,
    investigation_id VARCHAR(255) NOT NULL,
    endpoint_id UUID NOT NULL,
    host VARCHAR(255) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    evidence_type VARCHAR(50) NOT NULL,
    source VARCHAR(50) NOT NULL,
    attributes_json JSONB NOT NULL,
    PRIMARY KEY (id, investigation_id, endpoint_id),
    FOREIGN KEY (endpoint_id, investigation_id) REFERENCES investigations(endpoint_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_evidence_investigation ON evidence(endpoint_id, investigation_id);
CREATE INDEX IF NOT EXISTS idx_evidence_type ON evidence(evidence_type);

CREATE TABLE IF NOT EXISTS findings (
    id VARCHAR(255) NOT NULL,
    investigation_id VARCHAR(255) NOT NULL,
    endpoint_id UUID NOT NULL,
    rule_id VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    severity VARCHAR(50) NOT NULL,
    confidence FLOAT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id, investigation_id, endpoint_id),
    FOREIGN KEY (endpoint_id, investigation_id) REFERENCES investigations(endpoint_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_findings_investigation ON findings(endpoint_id, investigation_id);
CREATE INDEX IF NOT EXISTS idx_findings_severity ON findings(severity);

CREATE TABLE IF NOT EXISTS relationships (
    id SERIAL PRIMARY KEY,
    investigation_id VARCHAR(255) NOT NULL,
    endpoint_id UUID NOT NULL,
    source_id VARCHAR(255) NOT NULL,
    relationship_type VARCHAR(50) NOT NULL,
    target_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (endpoint_id, investigation_id) REFERENCES investigations(endpoint_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_relationships_investigation ON relationships(endpoint_id, investigation_id);

CREATE TABLE IF NOT EXISTS timeline_events (
    id SERIAL PRIMARY KEY,
    investigation_id VARCHAR(255) NOT NULL,
    endpoint_id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    evidence_id VARCHAR(255) NOT NULL,
    host VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (endpoint_id, investigation_id) REFERENCES investigations(endpoint_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_timeline_events_investigation ON timeline_events(endpoint_id, investigation_id);
CREATE INDEX IF NOT EXISTS idx_timeline_events_timestamp ON timeline_events(timestamp);

CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    event_type VARCHAR(100) NOT NULL,
    endpoint_id UUID,
    details JSONB NOT NULL
);
