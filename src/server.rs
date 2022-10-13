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
        
        let request_line: &str = &http_request[0];

        // Sanity Check
        println!("{}{:?}\n",("Request Line").cyan(), request_line);
        println!("{}{:#?}\n",("Request:  ").cyan(), http_request);

        // Determines our response based on the request header
        match request_line{
            "GET / HTTP/1.1" =>{
                println!("{}{}",("Match case:  ").cyan(),("'Get / HTTP/1.1'").blue());  // Sanity Check
                determine_request("index.html", stream)
            },
            "GET /favicon.ico HTTP/1.1" =>{
                println!("{}",("\n\nDo Nothing\nClient Requesting Favicon.ico\n").yellow());
            }
            _ => {
                println!("{}{}",("Match request case case:  ").cyan(),("FAILED").red());  // Sanity Check
                determine_request("404.html", stream)
            },
        };
    }


    fn determine_request(targe_file: &str, mut stream: TcpStream){
        let contents = fs::read_to_string(targe_file).unwrap();
        let length = contents.len();
        let status = match targe_file{
            "index.html" => "HTTP/1.1 200 OK",
            "404.html" => "HTTP/1.1 404 NOT FOUND",
            _ => "Failed to determine request status based on target file.",
        };

        println!("{}{}",("Response Status:  ").cyan(),status.blue());

        let response =   // This should only be the response to "127.0.0.1:3000"
            format!("{status}\r\nContent-Length: {length}\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();                
    }
 }

