-- Initial database schema for Amari MCP server
-- This migration creates tables for storing computation results and metadata

-- Table for storing computation results
CREATE TABLE computations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    computation_type VARCHAR(100) NOT NULL,
    result JSONB NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for fast lookups by name
CREATE INDEX idx_computations_name ON computations(name);

-- Index for filtering by computation type
CREATE INDEX idx_computations_type ON computations(computation_type);

-- Index for time-based queries
CREATE INDEX idx_computations_created_at ON computations(created_at);

-- Table for storing computation sessions
CREATE TABLE computation_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_name VARCHAR(255) NOT NULL,
    user_id VARCHAR(255),
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ended_at TIMESTAMP WITH TIME ZONE,
    metadata JSONB DEFAULT '{}'
);

-- Table for linking computations to sessions
CREATE TABLE session_computations (
    session_id UUID REFERENCES computation_sessions(id) ON DELETE CASCADE,
    computation_id UUID REFERENCES computations(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    PRIMARY KEY (session_id, computation_id)
);

-- Table for storing performance metrics
CREATE TABLE performance_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    computation_id UUID REFERENCES computations(id) ON DELETE CASCADE,
    operation_type VARCHAR(100) NOT NULL,
    execution_time_ms REAL NOT NULL,
    memory_usage_mb REAL,
    gpu_used BOOLEAN DEFAULT FALSE,
    batch_size INTEGER,
    input_size INTEGER,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for performance analysis
CREATE INDEX idx_performance_operation_type ON performance_metrics(operation_type);
CREATE INDEX idx_performance_timestamp ON performance_metrics(timestamp);
CREATE INDEX idx_performance_gpu_used ON performance_metrics(gpu_used);

-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to automatically update updated_at
CREATE TRIGGER update_computations_updated_at
    BEFORE UPDATE ON computations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE computations IS 'Stores mathematical computation results from Amari MCP server';
COMMENT ON TABLE computation_sessions IS 'Groups related computations into sessions';
COMMENT ON TABLE session_computations IS 'Links computations to their sessions with ordering';
COMMENT ON TABLE performance_metrics IS 'Tracks performance data for operations';

COMMENT ON COLUMN computations.name IS 'Unique identifier for the computation';
COMMENT ON COLUMN computations.computation_type IS 'Type of computation (geometric, tropical, autodiff, etc.)';
COMMENT ON COLUMN computations.result IS 'JSON-encoded computation result';
COMMENT ON COLUMN computations.metadata IS 'Additional metadata about the computation';

COMMENT ON COLUMN performance_metrics.execution_time_ms IS 'Time taken to execute the operation in milliseconds';
COMMENT ON COLUMN performance_metrics.memory_usage_mb IS 'Memory usage in megabytes';
COMMENT ON COLUMN performance_metrics.gpu_used IS 'Whether GPU acceleration was used';
COMMENT ON COLUMN performance_metrics.batch_size IS 'Size of batch for batch operations';
COMMENT ON COLUMN performance_metrics.input_size IS 'Size of input data';