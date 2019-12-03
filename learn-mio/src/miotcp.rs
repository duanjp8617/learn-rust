use std::net;
use std::thread;
use std::io;
use mio::{Poll, Ready, Token, PollOpt, Events};
use mio::net::TcpListener;

pub fn accept() {
    let listener = TcpListener::bind(&"127.0.0.1:0".parse().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    let t = thread::spawn(move || {
        net::TcpStream::connect(&addr).unwrap();
    });

    let poll = Poll::new().unwrap();
    poll.register(&listener, Token(1), Ready::readable(), PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(1024);

    let mut shutdown = false;
    while !shutdown {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            println!("Receive a new connection request");
            shutdown = true;

            if event.token() == Token(1) {
                println!("This is the correct token that we registered");
            }

            if event.readiness().is_readable() {
                println!("This is a read event");
            }

            if listener.accept().is_ok() {
                println!("the connection requets is accepted");
            }

        }
    }

    if listener.accept().unwrap_err().kind() == io::ErrorKind::WouldBlock {
        println!("the listener would block");
    }

    t.join().unwrap();
}

