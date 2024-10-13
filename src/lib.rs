pub mod womscp {

    #[derive(Debug)]
    pub struct WOMSCP {
        pub version: u8,
        pub m_id: u16,
        pub s_id: u8,
        pub t: u8,
        pub data: u32,
    }

    impl From<[u8; 10]> for WOMSCP {
        fn from(buf: [u8; 10]) -> Self {
            WOMSCP { 
                version: buf[0], 
                m_id: u16::from_be_bytes([buf[1], buf[2]]),
                s_id: buf[3], 
                t: buf[4], 
                data: u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]])
            }
        }
    }
}
