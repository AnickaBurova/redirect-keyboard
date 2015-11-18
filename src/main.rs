extern crate byteorder;
extern crate argparse;
#[cfg(target_os="macos")]
extern crate nix;

use std::net::{TcpListener,TcpStream};
use std::io::{Error, ErrorKind, Result  };
use std::thread;
use byteorder::{ReadBytesExt, WriteBytesExt,  LittleEndian};
use std::sync::mpsc::{Sender, channel};
use std::time::Duration;
use argparse::{ArgumentParser, Store,StoreTrue};
use std::io::prelude::*;
use std::io;

#[cfg(target_os="macos")]
use nix::sys::termios;


#[derive(Clone)]
struct Config {
    skip_client : bool,
    port : u16,
    localip : String,
    outsideip : String,
    master : bool
}

enum Message{
    Error,
    KeyPress(u8)
}

fn execute_message(msg : Message){
    match msg{
        Message::KeyPress(code) => println!("received key: {}", code),
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


#[cfg(target_os="macos")]
fn run_master(stream : &mut TcpStream) -> Result<()>{
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
    for byte in std::io::stdin().bytes() {
        let byte = byte.unwrap();
        if byte == 3 {
            break;
        } else {
            println!("You pressed byte {}", byte);
            send_message(stream,Message::KeyPress(byte));
        }
    }
    println!("Goodbye!");
    termios::tcsetattr(0, termios::TCSADRAIN, &saved_term).unwrap();
    Ok(())
}
#[cfg(not(target_os="macos"))]
fn run_master(stream : &mut TcpStream) -> Result<()>{
    println!("Not supported on windows");
    Ok(())
}

fn run_slave(stream : &mut TcpStream) -> Result<()>{
    loop {
        let code : u8 = try!(stream.read_u8());
        let msg = match code{
            1 => Message::KeyPress(try!(stream.read_u8())),
            _ => Message::Error
        };
        execute_message(msg);
    }
    Ok(())
}

fn run_sync(stream : &mut TcpStream, config : Config) -> Result<()>{

    if config.master {
        run_master(stream)
    }
    else
    {
        run_slave(stream)
    }
}

fn try_run_client(config : Config) -> Result<()>{
    println!("Trying to connect to {}:{}",config.outsideip, config.port );
    let mut stream = try!(TcpStream::connect((&config.outsideip as &str,config.port)));
    run_sync(&mut stream,config)
}


fn run_server(config : Config) -> Result<()> {
    println!("Creating server on {}:{}" , config.localip,config.port );
    let listener = try!(TcpListener::bind((&config.localip as &str,config.port)));
    println!("Waiting for new connections");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                let cfg = config.clone();
                thread::spawn(move||{
                    println!("connected");
                    run_sync(&mut stream.try_clone().unwrap(),cfg)
                });
            }
            Err(e) => {
                println!("Connection failed {}",e );
            }
        }
    }

    drop(listener);
    Ok(())
}


fn main() {

    let mut config = Config{
        skip_client : false,
        port: 24012,
        localip : "127.0.0.1".to_owned(),
        outsideip : "127.0.0.1".to_owned(),
        master : false
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Synchronise clipboard content between two computers.");
        ap.refer(&mut config.master)
            .add_option(&["-m","--master"],StoreTrue,"Act as a master, this machine will redirect keyboard to the other one.");
        ap.refer(&mut config.skip_client)
            .add_option(&["-s","--skip_client"],StoreTrue,"Skip connecting to client and create server right away.");
        ap.refer(&mut config.port)
            .add_option(&["-p","--port"],Store,"Port address");
        ap.refer(&mut config.localip)
            .add_option(&["-l","--local"],Store,"Local ip address");
        ap.refer(&mut config.outsideip)
            .add_option(&["-o","--outside"],Store,"Outside ip address");
        ap.parse_args_or_exit();
    }
    println!("local: {}, outside: {}, port: {}",config.localip, config.outsideip, config.port );
    let _ = if config.skip_client{
        println!("Skiping connecting to client");
        run_server(config)
    }else {match try_run_client(config.clone()){
        Err(_) => {
            println!("Could not connect to server, creating own.");
            run_server(config)},
        _ => Ok(())
    }};
}
