use sqlx::PgPool;

pub async fn insert_batch(
    pool: &PgPool,
    batch_id: &str,
    agent_id: &str,
    protocol_version: u32,
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO batches (
            batch_id,
            agent_id,
            protocol_version
        )
        VALUES (
            $1,
            $2,
            $3
        )
        "#,
    )
    .bind(batch_id)
    .bind(agent_id)
    .bind(protocol_version as i32)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}
