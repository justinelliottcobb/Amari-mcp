-- Migration for precomputed Cayley tables storage and indexing
-- This enables zero-latency Cayley table lookups for geometric algebra operations

-- Table for storing precomputed Cayley tables
CREATE TABLE cayley_tables (
    id SERIAL PRIMARY KEY,
    signature_p INTEGER NOT NULL,
    signature_q INTEGER NOT NULL,
    signature_r INTEGER NOT NULL,
    dimensions INTEGER NOT NULL,
    basis_count INTEGER NOT NULL,
    table_data BYTEA NOT NULL, -- Compressed binary data for the Cayley table
    metadata JSONB DEFAULT '{}',
    computed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    computation_time_ms REAL,
    checksum VARCHAR(64), -- SHA256 checksum for data integrity

    -- Ensure unique signatures
    UNIQUE(signature_p, signature_q, signature_r)
);

-- Index for fast signature lookups
CREATE INDEX idx_cayley_signature ON cayley_tables(signature_p, signature_q, signature_r);
CREATE INDEX idx_cayley_dimensions ON cayley_tables(dimensions);
CREATE INDEX idx_cayley_basis_count ON cayley_tables(basis_count);

-- Table for tracking Cayley table usage statistics
CREATE TABLE cayley_usage_stats (
    id SERIAL PRIMARY KEY,
    signature_p INTEGER NOT NULL,
    signature_q INTEGER NOT NULL,
    signature_r INTEGER NOT NULL,
    access_count BIGINT DEFAULT 0,
    last_accessed TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    total_time_saved_ms BIGINT DEFAULT 0, -- Estimated time saved by caching

    UNIQUE(signature_p, signature_q, signature_r),
    FOREIGN KEY(signature_p, signature_q, signature_r)
        REFERENCES cayley_tables(signature_p, signature_q, signature_r)
        ON DELETE CASCADE
);

-- Index for usage tracking
CREATE INDEX idx_usage_signature ON cayley_usage_stats(signature_p, signature_q, signature_r);
CREATE INDEX idx_usage_access_count ON cayley_usage_stats(access_count DESC);
CREATE INDEX idx_usage_last_accessed ON cayley_usage_stats(last_accessed DESC);

-- Table for precomputed common geometric algebra signatures
CREATE TABLE precomputed_signatures (
    id SERIAL PRIMARY KEY,
    signature_p INTEGER NOT NULL,
    signature_q INTEGER NOT NULL,
    signature_r INTEGER NOT NULL,
    name VARCHAR(100), -- e.g., "3D Euclidean", "4D Spacetime", "Conformal GA"
    description TEXT,
    priority INTEGER DEFAULT 0, -- Higher priority = precompute first
    is_essential BOOLEAN DEFAULT FALSE,
    use_cases TEXT[], -- Array of common use cases

    UNIQUE(signature_p, signature_q, signature_r)
);

-- Essential geometric algebra signatures to precompute
INSERT INTO precomputed_signatures (signature_p, signature_q, signature_r, name, description, priority, is_essential, use_cases) VALUES
-- Most essential signatures (priority 100)
(3, 0, 0, '3D Euclidean', 'Standard 3D geometric algebra (Cl(3,0))', 100, true,
 ARRAY['3D rotations', 'robotics', 'computer graphics', 'physics simulations']),
(2, 0, 0, '2D Euclidean', 'Plane geometric algebra (Cl(2,0))', 95, true,
 ARRAY['2D graphics', 'complex numbers', 'planar rotations']),
(4, 1, 0, 'Conformal GA (3D)', 'Conformal geometric algebra for 3D space', 90, true,
 ARRAY['conformal transformations', 'circle/sphere operations', 'inversions']),

-- Important signatures (priority 80-90)
(3, 1, 0, '4D Spacetime', 'Spacetime geometric algebra (Cl(3,1))', 85, true,
 ARRAY['relativity', 'spacetime rotations', 'Lorentz transformations']),
(1, 1, 0, '2D Minkowski', 'Hyperbolic plane (Cl(1,1))', 80, false,
 ARRAY['hyperbolic geometry', '2D relativity']),
(2, 1, 0, '3D Minkowski-like', 'Mixed signature 3D space', 75, false,
 ARRAY['special geometries', 'mixed metric spaces']),

-- Extended coverage (priority 50-70)
(5, 0, 0, '5D Euclidean', 'Higher-dimensional Euclidean space', 70, false,
 ARRAY['higher-dimensional rotations', 'research applications']),
(4, 0, 0, '4D Euclidean', 'Four-dimensional Euclidean space', 65, false,
 ARRAY['4D rotations', 'quaternion extensions']),
(6, 0, 0, '6D Euclidean', 'Six-dimensional Euclidean space', 60, false,
 ARRAY['6D physics', 'extended geometric algebra']),
(1, 0, 0, '1D Euclidean', 'Trivial 1D case', 55, false,
 ARRAY['teaching', 'trivial cases']),
(0, 1, 0, '1D Minkowski', 'Degenerate Minkowski space', 50, false,
 ARRAY['degenerate cases', 'mathematical completeness']),

-- Research/advanced signatures (priority 30-40)
(7, 0, 0, '7D Euclidean', 'Seven-dimensional Euclidean space', 40, false,
 ARRAY['advanced research', 'high-dimensional GA']),
(5, 1, 0, '6D Spacetime-like', 'Extended spacetime signature', 35, false,
 ARRAY['higher-dimensional spacetime', 'theoretical physics']),
(8, 0, 0, '8D Euclidean', 'Eight-dimensional Euclidean space', 30, false,
 ARRAY['maximum practical dimension', 'research applications']);

-- Function to get the table identifier string
CREATE OR REPLACE FUNCTION cayley_table_id(p INTEGER, q INTEGER, r INTEGER)
RETURNS TEXT AS $$
BEGIN
    RETURN 'cayley_' || p || '_' || q || '_' || r;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Function to estimate Cayley table size in bytes
CREATE OR REPLACE FUNCTION cayley_table_size_bytes(dimensions INTEGER)
RETURNS BIGINT AS $$
BEGIN
    -- Size = (2^dimensions)^3 * 8 bytes per float64
    RETURN (1::BIGINT << (3 * dimensions)) * 8;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- View for Cayley table information with computed statistics
CREATE VIEW cayley_table_info AS
SELECT
    ct.signature_p,
    ct.signature_q,
    ct.signature_r,
    cayley_table_id(ct.signature_p, ct.signature_q, ct.signature_r) as table_id,
    ps.name,
    ps.description,
    ct.dimensions,
    ct.basis_count,
    ct.computed_at,
    ct.computation_time_ms,
    LENGTH(ct.table_data) as stored_size_bytes,
    cayley_table_size_bytes(ct.dimensions) as expected_size_bytes,
    COALESCE(us.access_count, 0) as access_count,
    us.last_accessed,
    COALESCE(us.total_time_saved_ms, 0) as total_time_saved_ms,
    ps.priority,
    ps.is_essential,
    ps.use_cases
FROM cayley_tables ct
LEFT JOIN precomputed_signatures ps USING (signature_p, signature_q, signature_r)
LEFT JOIN cayley_usage_stats us USING (signature_p, signature_q, signature_r)
ORDER BY ps.priority DESC NULLS LAST, ct.computed_at DESC;

-- Trigger to automatically create usage stats entry when Cayley table is inserted
CREATE OR REPLACE FUNCTION create_cayley_usage_stats()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO cayley_usage_stats (signature_p, signature_q, signature_r)
    VALUES (NEW.signature_p, NEW.signature_q, NEW.signature_r)
    ON CONFLICT DO NOTHING;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_create_cayley_usage_stats
    AFTER INSERT ON cayley_tables
    FOR EACH ROW
    EXECUTE FUNCTION create_cayley_usage_stats();

-- Function to update usage statistics
CREATE OR REPLACE FUNCTION update_cayley_usage(p INTEGER, q INTEGER, r INTEGER, time_saved_ms REAL DEFAULT 0)
RETURNS VOID AS $$
BEGIN
    INSERT INTO cayley_usage_stats (signature_p, signature_q, signature_r, access_count, total_time_saved_ms)
    VALUES (p, q, r, 1, time_saved_ms)
    ON CONFLICT (signature_p, signature_q, signature_r)
    DO UPDATE SET
        access_count = cayley_usage_stats.access_count + 1,
        last_accessed = NOW(),
        total_time_saved_ms = cayley_usage_stats.total_time_saved_ms + time_saved_ms;
END;
$$ LANGUAGE plpgsql;

-- Create indexes for performance
CREATE INDEX idx_precomputed_priority ON precomputed_signatures(priority DESC);
CREATE INDEX idx_precomputed_essential ON precomputed_signatures(is_essential);

-- Grant permissions (adjust as needed for your setup)
-- GRANT SELECT, INSERT, UPDATE ON ALL TABLES IN SCHEMA public TO amari_mcp_user;
-- GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO amari_mcp_user;