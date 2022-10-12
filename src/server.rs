/*
 * Launches the local server for user input and results viewing
 * 
 * Created by Philip Otter
 * October 2022
 */


 pub mod server {

    use std::{
        fs,
        net::{TcpListener,TcpStream},
        io::{prelude::*,BufReader},
    };
    use webbrowser;  // Used to open the web browser automatically
    use colored::Colorize;  // Used for debugging


    pub fn launch_server(){
        let addr = "127.0.0.1:3000";
        let protocol = "http://";
        let url_raw = format!("{}{}",protocol,addr);
        let url = url_raw.as_str();
        
        // Sanity Check
        println!("{}{}",("\nAttempted address:  ").cyan(), format!("{}{}",protocol,addr).as_str().blue());
        println!("{}{}",("url_raw: ").cyan(),url_raw.blue());
        println!("{}{}",("url:  ").cyan(),url.blue());

        let listener = TcpListener::bind(addr).unwrap();

        println!("{}{}",("Can open web browser:  ").cyan(), webbrowser::open(url).is_ok());  // Sanity check in console bool value

        for stream in listener.incoming(){
            let stream = stream.unwrap();
            
            println!("{}",("\n\nconnection established!").green());  // Sanity Check
            read_connection(stream);
        }
     }
    
    fn read_connection(mut stream: TcpStream){
        println!("{}",("fn read_connection Started\n\n").green());

        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result|result.unwrap())
            .take_while(|line| !line.is_empty())  // I think this is why we can't currently POST
            .collect();
        
        println!("{}{:#?}",("Request:  ").cyan(), http_request);
        
        let contents = fs::read_to_string("index.html").unwrap();
        let length = contents.len();
        let status = "HTTP/1.1 200 ok";

        let response =   // This should only be the response to "127.0.0.1:3000"
            format!("{status}\r\nContent-Length: {length}\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

 }

