/* 
 * Created by Philip Otter
 * October 2022
 */

 mod server;

 pub use crate::server::server::launch_server;

fn main() {
    println!("Launching server\n");
    launch_server();
}
