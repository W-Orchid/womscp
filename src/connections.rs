use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

use sqlx::SqlitePool;

use womscp_lib::womscp::{Request, ResponseError, RequestFlags};


pub async fn handle_connection(conn :&SqlitePool, mut stream :TcpStream) {

    // parse request from stream
    match Request::try_from_tcp(&mut stream).await {
        Ok(req) => { 
            dbg!(&req);

            // if request is just a ping, respond appropriately
            if req.flags & RequestFlags::SrvrRdy as u8 == RequestFlags::SrvrRdy as u8 {
                if let Err(tcp_err) = stream.write_all(&[0]).await {
                    eprintln!("TCP write error: {:?}", tcp_err);
                }
                return;
            }

            // check if sensor exists
            if let Err(e) = get_sensor(&conn, &req).await {
                if let Err(tcp_err) = stream.write_all(&[e as u8]).await {
                    eprintln!("TCP write error: {:?}", tcp_err);
                }
                return;
            }

            // insert data into database
            if let Err(e) = insert_data(&conn, &req).await {
                if let Err(tcp_err) = stream.write_all(&[e as u8]).await {
                    eprintln!("TCP write error: {:?}", tcp_err);
                }
                return;
            }

            if let Err(tcp_err) = stream.write_all(&[0]).await {
                eprintln!("TCP write error: {:?}", tcp_err);
            }
            return;
        },

        Err(res_err) => {
            eprintln!("WOMSCP parsing error: {:#?}", res_err);
            if let Err(tcp_err) = stream.write_all(&[res_err as u8]).await {
                eprintln!("TCP write error: {:?}", tcp_err);
            }
            return;
        },
    }; 
}


pub async fn get_sensor(conn :&SqlitePool, req :&Request) -> Result<(), ResponseError> {
    let db_check = sqlx::query(
        "SELECT s_id, m_id FROM Sensors
                      WHERE s_id = $1 AND m_id = $2"
    )
        .bind(req.s_id)
        .bind(req.m_id)
        .execute(conn)
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

    Ok(())
}


pub async fn insert_data(conn :&SqlitePool, req :&Request) -> Result<(), ResponseError> {
    let is_dummy = if req.flags & RequestFlags::Dummy as u8 == RequestFlags::Dummy as u8 {
        true
    } else {
        false
    };

    let db_res = sqlx::query(
        "INSERT INTO SensorData
                VALUES(NULL, datetime('now'), $1, $2, $3, $4, $5)")
        .bind(req.m_id)
        .bind(req.s_id)
        .bind(req.sensor_type)
        .bind(req.data)
        .bind(is_dummy)
        .execute(conn)
        .await;

    if let Err(e) = db_res {
        eprintln!("Database error: {:#?}", e);
        return Err(ResponseError::Database);
    }

    Ok(())
}
