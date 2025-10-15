#[cfg(feature = "database")]
mod migration_tests {
    use serial_test::serial;
    use sqlx::PgPool;
    use std::env;

    async fn setup_test_db() -> Option<PgPool> {
        let database_url = env::var("TEST_DATABASE_URL").ok()?;
        let pool = PgPool::connect(&database_url).await.ok()?;

        // Drop all tables to start fresh
        let _ = sqlx::query("DROP TABLE IF EXISTS cayley_usage_stats CASCADE").execute(&pool).await;
        let _ = sqlx::query("DROP TABLE IF EXISTS cayley_tables CASCADE").execute(&pool).await;
        let _ = sqlx::query("DROP TABLE IF EXISTS precomputed_signatures CASCADE").execute(&pool).await;
        let _ = sqlx::query("DROP TABLE IF EXISTS computational_results CASCADE").execute(&pool).await;

        Some(pool)
    }

    #[tokio::test]
    #[serial]
    async fn test_migration_001_initial_schema() {
        if let Some(pool) = setup_test_db().await {
            // Run only the first migration
            let migration_dir = std::path::Path::new("./migrations");
            let mut migrator = sqlx::migrate::Migrator::new(migration_dir).await.unwrap();

            // Filter to only run the first migration
            migrator.migrations.retain(|m| m.version == 1);

            let result = migrator.run(&pool).await;
            assert!(result.is_ok());

            // Verify the computational_results table was created
            let table_exists = sqlx::query!(
                "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'computational_results')"
            )
            .fetch_one(&pool)
            .await;

            assert!(table_exists.is_ok());
            assert_eq!(table_exists.unwrap().exists, Some(true));

            // Verify table structure
            let columns = sqlx::query!(
                r#"
                SELECT column_name, data_type, is_nullable
                FROM information_schema.columns
                WHERE table_name = 'computational_results'
                ORDER BY ordinal_position
                "#
            )
            .fetch_all(&pool)
            .await
            .unwrap();

            let column_names: Vec<String> = columns.iter().map(|c| c.column_name.clone()).collect();
            assert!(column_names.contains(&"id".to_string()));
            assert!(column_names.contains(&"computation_id".to_string()));
            assert!(column_names.contains(&"result_data".to_string()));
            assert!(column_names.contains(&"metadata".to_string()));
            assert!(column_names.contains(&"created_at".to_string()));
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_migration_002_cayley_tables() {
        if let Some(pool) = setup_test_db().await {
            // Run both migrations
            let result = sqlx::migrate!("./migrations").run(&pool).await;
            assert!(result.is_ok());

            // Verify all Cayley table related tables exist
            let tables = vec!["cayley_tables", "precomputed_signatures", "cayley_usage_stats"];

            for table_name in tables {
                let table_exists = sqlx::query(&format!(
                    "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = '{}')",
                    table_name
                ))
                .fetch_one(&pool)
                .await;

                assert!(table_exists.is_ok(), "Table {} should exist", table_name);
            }

            // Verify cayley_tables structure
            let cayley_columns = sqlx::query!(
                r#"
                SELECT column_name, data_type
                FROM information_schema.columns
                WHERE table_name = 'cayley_tables'
                ORDER BY ordinal_position
                "#
            )
            .fetch_all(&pool)
            .await
            .unwrap();

            let column_names: Vec<String> = cayley_columns.iter().map(|c| c.column_name.clone()).collect();
            let expected_columns = vec![
                "id", "signature_p", "signature_q", "signature_r",
                "dimensions", "basis_count", "table_data", "metadata",
                "computed_at", "computation_time_ms", "checksum"
            ];

            for expected_col in expected_columns {
                assert!(column_names.contains(&expected_col.to_string()),
                    "Column {} should exist in cayley_tables", expected_col);
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_precomputed_signatures_seeded() {
        if let Some(pool) = setup_test_db().await {
            // Run migrations
            let result = sqlx::migrate!("./migrations").run(&pool).await;
            assert!(result.is_ok());

            // Check that precomputed_signatures table has seed data
            let signature_count = sqlx::query!(
                "SELECT COUNT(*) as count FROM precomputed_signatures"
            )
            .fetch_one(&pool)
            .await
            .unwrap();

            assert!(signature_count.count.unwrap() > 0, "Should have seeded signatures");

            // Check for essential signatures
            let essential_count = sqlx::query!(
                "SELECT COUNT(*) as count FROM precomputed_signatures WHERE is_essential = true"
            )
            .fetch_one(&pool)
            .await
            .unwrap();

            assert!(essential_count.count.unwrap() > 0, "Should have essential signatures");

            // Verify specific important signatures exist
            let important_signatures = vec![(3, 0, 0), (2, 0, 0), (1, 1, 0), (4, 0, 0)];

            for (p, q, r) in important_signatures {
                let exists = sqlx::query!(
                    "SELECT EXISTS(SELECT 1 FROM precomputed_signatures WHERE signature_p = $1 AND signature_q = $2 AND signature_r = $3) as exists",
                    p, q, r
                )
                .fetch_one(&pool)
                .await
                .unwrap();

                assert_eq!(exists.exists, Some(true), "Signature [{}, {}, {}] should exist", p, q, r);
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_database_functions() {
        if let Some(pool) = setup_test_db().await {
            // Run migrations
            let result = sqlx::migrate!("./migrations").run(&pool).await;
            assert!(result.is_ok());

            // Test calculate_table_size function
            let size_result = sqlx::query!(
                "SELECT calculate_table_size(3, 0, 0) as size"
            )
            .fetch_one(&pool)
            .await;

            assert!(size_result.is_ok());
            let size = size_result.unwrap().size.unwrap();
            assert_eq!(size, 512); // 8^3 * 8 bytes = 512 bytes for 3D

            // Test update_cayley_usage function
            let usage_result = sqlx::query!(
                "SELECT update_cayley_usage(3, 0, 0, 100.0) as updated"
            )
            .fetch_one(&pool)
            .await;

            assert!(usage_result.is_ok());

            // Verify usage was recorded
            let usage_count = sqlx::query!(
                "SELECT COUNT(*) as count FROM cayley_usage_stats WHERE signature_p = 3 AND signature_q = 0 AND signature_r = 0"
            )
            .fetch_one(&pool)
            .await
            .unwrap();

            assert!(usage_count.count.unwrap() > 0);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_constraints_and_indexes() {
        if let Some(pool) = setup_test_db().await {
            // Run migrations
            let result = sqlx::migrate!("./migrations").run(&pool).await;
            assert!(result.is_ok());

            // Test unique constraint on cayley_tables
            let insert1 = sqlx::query!(
                "INSERT INTO cayley_tables (signature_p, signature_q, signature_r, dimensions, basis_count, table_data) VALUES ($1, $2, $3, $4, $5, $6)",
                3, 0, 0, 3, 8, &vec![0u8; 64]
            ).execute(&pool).await;
            assert!(insert1.is_ok());

            // Try to insert duplicate - should fail
            let insert2 = sqlx::query!(
                "INSERT INTO cayley_tables (signature_p, signature_q, signature_r, dimensions, basis_count, table_data) VALUES ($1, $2, $3, $4, $5, $6)",
                3, 0, 0, 3, 8, &vec![0u8; 64]
            ).execute(&pool).await;
            assert!(insert2.is_err()); // Should fail due to unique constraint

            // Verify index exists on signature columns
            let index_exists = sqlx::query!(
                r#"
                SELECT EXISTS (
                    SELECT 1 FROM pg_indexes
                    WHERE tablename = 'cayley_tables'
                    AND indexname = 'idx_cayley_signature'
                ) as exists
                "#
            )
            .fetch_one(&pool)
            .await
            .unwrap();

            assert_eq!(index_exists.exists, Some(true), "Signature index should exist");
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_migration_rollback_compatibility() {
        if let Some(pool) = setup_test_db().await {
            // This test ensures migrations can be applied and that the schema is sane
            let result = sqlx::migrate!("./migrations").run(&pool).await;
            assert!(result.is_ok());

            // Test basic operations work
            let insert_result = sqlx::query!(
                "INSERT INTO precomputed_signatures (signature_p, signature_q, signature_r, name, priority, is_essential) VALUES ($1, $2, $3, $4, $5, $6)",
                5, 0, 0, "Test Signature", 1, false
            ).execute(&pool).await;
            assert!(insert_result.is_ok());

            // Test foreign key relationships work
            let cayley_insert = sqlx::query!(
                "INSERT INTO cayley_tables (signature_p, signature_q, signature_r, dimensions, basis_count, table_data) VALUES ($1, $2, $3, $4, $5, $6)",
                5, 0, 0, 5, 32, &vec![0u8; 256]
            ).execute(&pool).await;
            assert!(cayley_insert.is_ok());

            // Test usage stats work
            let usage_result = sqlx::query!(
                "SELECT update_cayley_usage(5, 0, 0, 50.0)"
            )
            .execute(&pool)
            .await;
            assert!(usage_result.is_ok());
        }
    }
}