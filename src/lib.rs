pub mod womscp {
    use std::net::TcpStream;
    use std::io::Read;


    const WOMSCP_LEN :usize = 9;

    #[derive(Debug)]
    pub struct WOMSCP {
        pub version: u8,
        pub m_id: u16,
        pub s_id: u8,
        pub t: u8,
        pub data: u32,
    }

    impl From<[u8; WOMSCP_LEN]> for WOMSCP {
        fn from(buf: [u8; WOMSCP_LEN]) -> Self {
            WOMSCP { 
                version: buf[0], 
                m_id: u16::from_be_bytes([buf[1], buf[2]]),
                s_id: buf[3], 
                t: buf[4], 
                data: u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]])
            }
        }
    }

    impl TryFrom<TcpStream> for WOMSCP {
        type Error = std::io::Error;

        fn try_from(mut stream: TcpStream) -> Result<Self, Self::Error> {
            let mut buf :[u8; WOMSCP_LEN] = [0; WOMSCP_LEN];

            if let Err(e) = stream.read(&mut buf) {
                return Err(e);
            }

            return Ok(Self::from(buf));
        }
    }
}
