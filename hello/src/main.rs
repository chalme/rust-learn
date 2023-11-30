use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

/// https://course.rs/advance-practice1/web-server.html
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request : Vec<String> = buf_reader.lines().map(|it| it.unwrap())
      .take_while(|it| !it.is_empty())
      .collect();

    println!("Request {:#?}", http_request);
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_num() {
        let string = String::from("1");
        let x = &string;

        println!("{:p}", x);

        let i  = 12;
        let p = &i;
        println!("111  {:p}", p);

        let x1: Box<str> = "hello, world".into();
        println!("{}", x1);
        assert_eq!(0.1_f32 + 0.2_f32 , 0.3_f32);

        let a = [1, 2, 3];

        let mut iter = a.iter();

// A call to next() returns the next value...
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());

        let vec = vec![1, 2, 3];
        for i in vec.into_iter() {
            // vec 的所有权已经转移到迭代器，vec 不再可用
            println!("{}", i);
        }
        let vec = vec![1, 2, 3];
        for i in vec.iter() {
            // vec 的所有权没有转移，仍然可以在迭代中使用
            println!("{}", i);
        }

        let mut vec = vec![1, 2, 3];
        for i in vec.iter_mut() {
            // 可以修改 vec 中的元素
            *i += 1;
        }

    }
}
