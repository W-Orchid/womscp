use tokio::net::TcpStream;
use sqlx::SqlitePool;

use womscp_lib::womscp::{Request, ResponseError, RequestFlags};

pub async fn handle_connection(conn :&SqlitePool, stream :&TcpStream) -> Result<(), ResponseError> {
    match Request::try_from(stream) {
        Ok(req) => { 
            dbg!(&req);

            if req.flags & RequestFlags::SrvrRdy as u8 == 1 {
                return Ok(());
            }

            let db_check = sqlx::query(
                "SELECT id FROM Sensors
                      WHERE s_id = $1 AND m_id = $2"
            )
                .bind(req.s_id)
                .bind(req.m_id)
                .fetch_one(conn)
                .await;

            if let Err(e) = db_check {
                match e {
                    sqlx::Error::RowNotFound => {
                        eprintln!("Error! Unrecognized node m_id={}, s_id={}", 
                            req.m_id, req.s_id);

                        return Err(ResponseError::Unrecognised);
                    },

                    _ => {
                        eprintln!("Database error: {:#?}", e);
                        return Err(ResponseError::Database);
                    }
                }
            }

            let is_dummy = if req.flags & RequestFlags::Dummy as u8 == 1 {
                true
            } else {
                false
            };

            let db_res = sqlx::query(
                "INSERT INTO SensorData
                VALUES(NULL, datetime('now'), $1, $2, $3, $4, $5)")
                .bind(req.m_id)
                .bind(req.s_id)
                .bind(req.data)
                .bind(req.sensor_type)
                .bind(is_dummy)
                .fetch_all(conn)
                .await;

            if let Err(e) = db_res {
                eprintln!("Database error: {:#?}", e);
                return Err(ResponseError::Database);
            }

            Ok(())
        },
        Err(res_err) => {
            eprintln!("WOMSCP parsing error: {:#?}", res_err);
            return Err(res_err);
        },
    }
}
