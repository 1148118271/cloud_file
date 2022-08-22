use std::io::{BufRead, ErrorKind};
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;


#[derive(Debug)]
pub struct Request {
    header: Header,
    body: Body,
    content_length: usize,
}


impl Request {
     pub async fn process(mut stream: TcpStream) -> Self {
         let mut msg_buf: Vec<u8> = vec![];
         let mut buf = [0u8; 200];

         let mut flag: bool = false;

         let mut request = Self {
             header: Header {v: vec![]},
             body: Body {v: String::new()},
             content_length: 0
         };

         loop {
            match stream.read(&mut buf).await {
                Ok(size) => {
                    if size == 0 {
                        println!("client disconnects.");
                        break;
                    }

                    if !flag {
                        if !Self::contains_get_http(&buf) {

                            // stream.shutdown().await.unwrap();
                            break;
                        }
                        flag = true;
                    }

                    msg_buf.extend_from_slice(&buf[..size]);

                    if request.content_length > 0 {
                        if msg_buf.len() < request.content_length {
                            continue
                        } else if msg_buf.len() == request.content_length {
                            request.body = Body::new(&msg_buf);
                            break;
                        }
                    }

                    let header = Self::read_end(&mut msg_buf);
                    if header.is_none() {
                        continue;
                    }
                    let header = header.unwrap();
                    request.header = header;

                    let content_length = request.header.get_content_length();

                    if content_length == 0 {
                        break;
                    }
                    if content_length == -1 {
                        stream.shutdown().await.unwrap();
                        break;
                    }
                    request.content_length = content_length as usize;
                    if msg_buf.len() == content_length as usize {
                        break;
                    }
                }
                Err(e) => {
                    println!("e => {}", e);
                    if e.kind() == ErrorKind::WouldBlock {
                        println!("data read complete.");
                        break;
                    }
                    panic!("{}", e);
                }
            }
        }
        request
    }


    fn read_end(msg_buf: &mut Vec<u8>) -> Option<Header> {
        let i = Self::contains_end(&msg_buf);
        if i == -1 {
            return None;
        }
        let (h, b) = msg_buf.split_at((i + 4) as usize);
        let h = &h[..(h.len() - 4)];
        let header = Header::init(h);
        if b.is_empty() {
            msg_buf.clear();
            return Some(header)
        }
        let body = b.to_vec();
        msg_buf.clear();
        msg_buf.extend_from_slice(&body);
        Some(header)
    }

    fn contains_end(source: &[u8]) -> isize {
        let end = [b'\r', b'\n', b'\r', b'\n'];
        let mut mid = [0;4];
        let mut flag: isize = -1;

        for i in 0..source.len() {
            if (i + 3) >= source.len() {
                return flag;
            }
            mid[0] = source[i];
            mid[1] = source[i + 1];
            mid[2] = source[i + 2];
            mid[3] = source[i + 3];
            if end == mid {
                flag = i as isize;
            }
        }
        return flag;
    }

    fn contains_get_http(buf: &[u8]) -> bool {
        unsafe {
            let string = String::from_utf8_unchecked(buf.to_vec());
            string.contains("GET HTTP")
        }
    }
}




#[derive(Debug)]
pub struct Header {
    v: Vec<String>
}

impl Header {
    pub fn init(msg: &[u8]) -> Header {
        let mut lines = msg.lines();
        let mut vec = vec![];
        loop {
            let line = lines.next();
            if line.is_none() {
                break;
            }
            let line_str = match line.unwrap() {
                Ok(v) => v,
                Err(e) => panic!("{}",e)
            };
            vec.push(line_str);
        }
        Header {
            v: vec
        }
    }

    pub fn get_content_length(&self) -> i32 {
        for str in &self.v {
            if str.to_ascii_uppercase()
                .contains(&(String::from("Content-Length"))
                    .to_ascii_uppercase()) {
                let ss = str.split(":").collect::<Vec<&str>>();
                let len = ss[1];
                return i32::from_str(len.trim()).unwrap_or(-1)
            }
        }
        0
    }
}

#[derive(Debug)]
pub struct Body {
    v: String
}

impl Body {
    pub fn new(v: &[u8]) -> Self {
        Body {
            v: String::from_utf8(v.to_vec()).unwrap_or(String::new())
        }
    }
}