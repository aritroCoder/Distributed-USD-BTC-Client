use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    process::Command,
};

fn main() {
    let mut child_handlers = vec![];
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    let start_time = chrono::Utc::now().timestamp() + 1; // Start in 1 second from now

    for _ in 0..5 {
        let child = Command::new("./child")
            .arg(format!("--start={start_time}"))
            .arg("--mode=cache")
            .arg("--times=10")
            .spawn()
            .expect("failed to execute child");

        child_handlers.push(child);
    }

    let mut rates_list: Vec<f64> = vec![];
    // Accept connections from child processes
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream, &mut rates_list);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
        if rates_list.len() == 5 {
            break;
        }
    }

    for mut child in child_handlers {
        let ecode = child.wait().expect("failed to wait on child");
        assert!(ecode.success());
    }

    let avg_rate = rates_list.iter().sum::<f64>() / rates_list.len() as f64;
    println!("Average USD price of BTC is: {}", avg_rate);
}

fn handle_connection(mut stream: TcpStream, rates_list: &mut Vec<f64>) {
    let mut buffer = [0; 8];

    // Read data from the child process
    match stream.read(&mut buffer) {
        Ok(_size) => {
            let received = f64::from_be_bytes(buffer);
            rates_list.push(received);
        }
        Err(e) => {
            eprintln!("Error reading from child process: {}", e);
        }
    }
}
