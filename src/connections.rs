use tokio::net::TcpStream;
use sqlx::SqlitePool;

use womscp_lib::womscp::{Request, ResponseError, RequestFlags};

pub async fn handle_connection(conn :&SqlitePool, stream :&TcpStream) -> Result<Request, ResponseError> {
    match Request::try_from(stream) {
        Ok(req) => { 
            dbg!(&req);

            let db_check = sqlx::query(
                "SELECT id FROM Sensors
                      WHERE sID = $1 AND mID = $2"
            )
                .bind(req.s_id)
                .bind(req.m_id)
                .fetch_one(conn)
                .await;

            if let Err(e) = db_check {
                match e {
                    sqlx::Error::RowNotFound => {
                        eprintln!("Error! Unrecognized node mID={}, sID={}", req.m_id, req.s_id);
                        return Err(ResponseError::Unrecognised);
                    },

                    _ => {
                        eprintln!("Database error: {:?}", e);
                        return Err(ResponseError::Database);
                    }
                }
            }

            let db_res = sqlx::query(
                "INSERT INTO Data(timestamp, mId, sID, data, dummy)
                VALUES(datetime('now'), $1, $2, $3, $4)")
                .bind(req.m_id)
                .bind(req.s_id)
                .bind(req.data)
                .bind(req.flags & RequestFlags::Dummy as u8)
                .fetch_all(conn)
                .await;

            if let Err(e) = db_res {
                eprintln!("Database error: {:?}", e);
                return Err(ResponseError::Database);
            }

            Ok(req)
        },
        Err(res_err) => {
            eprintln!("WOMSCP parsing error: {:?}", res_err);
            return Err(res_err);
        },
    }
}
