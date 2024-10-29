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
                .fetch_all(conn)
                .await;

            match db_check {
                Ok(rows) => {
                    if rows.len() == 0 {
                        eprintln!("Unrecognized sensor identifier! sID = {}, mID = {}", req.s_id, req.m_id);
                        return Err(ResponseError::Unrecognised);
                    }

                    if rows.len() > 1 {
                        panic!("More than one sensors identified with the following: sID = {}, mID = {}", req.s_id, req.m_id);
                    }
                },

                Err(e) => {
                    eprintln!("Database error: {:?}", e);
                    return Err(ResponseError::Database);
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
