extern crate byteorder;
#[cfg(target_os="macos")]
extern crate nix;
#[cfg(target_os="windows")]
mod win;
#[cfg(target_os="windows")]
use win::*;

use std::net::{TcpStream};
use std::io::{Error, ErrorKind, Result , stdin };
use self::byteorder::{ReadBytesExt};
use std::io::prelude::*;

#[cfg(target_os="macos")]
use self::nix::sys::termios;


pub fn run_slave(stream : &mut TcpStream) -> Result<()>{
    loop {
        let code : u8 = try!(stream.read_u8());
        let msg = match code{
            1 => Message::KeyPress(try!(stream.read_u8())),
            _ => Message::Error
        };
        execute_message(msg);
    }
}

#[cfg(target_os="macos")]
pub fn run_master(stream : &mut TcpStream) -> Result<()>{
    let saved_term = match termios::tcgetattr(0){
        Ok(t) => t,
        Err(_) => return Err(Error::new(ErrorKind::Other,"Failed to create termios!"))
    };
    let mut term = saved_term;
    // Unset canonical mode, so we get characters immediately
    term.c_lflag.remove(termios::ICANON);
    // Don't generate signals on Ctrl-C and friends
    term.c_lflag.remove(termios::ISIG);
    // Disable local echo
    term.c_lflag.remove(termios::ECHO);
    termios::tcsetattr(0, termios::TCSADRAIN, &term).unwrap();
    println!("Press Ctrl-C to quit");
    for byte in stdin().bytes() {
        let byte = byte.unwrap();
        if byte == 3 {
            break;
        } else {
            println!("You pressed byte {}", byte);
            send_message(stream,Message::KeyPress(byte)).unwrap();
        }
    }
    println!("Goodbye!");
    termios::tcsetattr(0, termios::TCSADRAIN, &saved_term).unwrap();
    Ok(())
}


#[cfg(not(target_os="macos"))]
pub fn run_master(stream : &mut TcpStream) -> Result<()>{
    println!("Not supported on non mac os");
    Ok(())
}

enum Message{
    Error,
    KeyPress(u8)
}

#[cfg(not(target_os="windows"))]
fn press_character(_ : char) -> Result<()>{
    Ok(())
}

fn execute_message(msg : Message){
    match msg{
        Message::KeyPress(code) => {
            println!("received key: {}", code as char);
            press_character(code as char).unwrap();
        },
        _ => println!("there is an error message!")
    };
}

fn send_message(stream : &mut TcpStream, msg : Message) -> Result<()>{
    match msg{
        Message::KeyPress(code) => {
            let _ = try!(stream.write(&[1u8,code]));
        }
        _ => {}
    }
    Ok(())
}
