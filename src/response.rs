
//! HTTP/1.1 200 OK
//! ## Transfer-Encoding: chunked
//!     代表这个报文采用了分块编码。
//!     这时，报文中的实体需要改为用一系列分块来传输。
//!     每个分块包含十六进制的长度值和数据，长度值独占一行，
//!     长度不包括它结尾的 CRLF（\r\n），也不包括分块数据结尾的 CRLF。
//!     最后一个分块长度值必须为 0，对应的分块数据没有内容，表示实体结束。
//!```
//!     write('HTTP/1.1 200 OK\r\n');
//!     write('Transfer-Encoding: chunked\r\n');
//!     write('\r\n');
//!
//!     write('b\r\n');   -- 第一个块 长度为十进制的 11 十六进制 B
//!     write('01234567890\r\n');
//!
//!     write('5\r\n');  -- 第二个块 长度为十进制的 5 十六进制 5
//!     write('12345\r\n');
//!
//!     write('0\r\n');  -- 最后一个分块 长度为十进制的 0 十六进制 0 表示实体结束。
//!     write('\r\n');
//!```


//! Access-Control-Allow-Headers: Content-Type,Authorization
//! Access-Control-Allow-Methods: GET,POST,OPTIONS
//! Access-Control-Allow-Origin: *
//! Connection: keep-alive
//! Content-Type: text/html
//! Date: Sun, 21 Aug 2022 15:04:27 GMT
//! Keep-Alive: timeout=4
//! Proxy-Connection: keep-alive
//! Server: nginx/1.22.0

pub struct Response {

}



pub async fn reponse_403() {

}