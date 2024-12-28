#![allow(non_camel_case_types)]
#![cfg(not(windows))]

use libc::{
    SIGALRM, SIGHUP, SIGQUIT, SIGTERM, SIGUSR1, 
};
use std::mem;

const JMP_BUF_WIDTH: usize =
    mem::size_of::<usize>()*8;
type jmp_buf = [i8; JMP_BUF_WIDTH];

static mut SHUT_DOWN: bool = false;
static mut RETURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];
const MOCK_SIGNAL_AT: usize = 3;

extern "C"{
    //#[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn setjmp(_: *mut i8) -> i32;

    //#[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn longjmp(_: *mut i8);
}

#[inline]
fn ptr_to_jmp_buf() -> *mut i8{
    unsafe { &RETURN_HERE as *const i8 as *mut i8}
}

#[inline]
fn return_early(){
    let franken_project = ptr_to_jmp_buf();
    unsafe{longjmp(franken_project)};
}

fn register_signal_handlers(){
    unsafe{
        //libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_signals as usize);
    }
}

#[allow(deadcode)]
fn handle_signals(sig: i32){
    register_signal_handlers();

    let should_shut_down = match sig{
        SIGHUP => false,
        SIGALRM => false,
        SIGTERM => true,
        SIGQUIT => true,
        SIGUSR1 => true,
        _ => false,
    };

    unsafe{
        SHUT_DOWN = should_shut_down;
    }

    return_early();
}

fn print_depth(depth: usize){
    for _ in 0..depth{
        print!("#");
    }
    println!();
}

fn dive(depth: usize, max_depth: usize){
    unsafe {
        if SHUT_DOWN{
            println!("!");
            return;
        }
    }
    print_depth(depth);

    if depth >= max_depth{
        return;
    } else if depth == MOCK_SIGNAL_AT{
        unsafe{
            libc::raise(SIGUSR1);
        }
    } else {
        dive(depth+1, max_depth);
    }
    print_depth(depth);
}

pub fn sisj(){
    const JUMP_SET: i32 = 0;

    register_signal_handlers();

    let return_point = ptr_to_jmp_buf();
    let rc = unsafe{setjmp(return_point)};
    if rc == JUMP_SET{
        dive(0,10);
    } else {
        println!("early return");
    }

    println!("finishing!");
}

use std::time::{Duration};
use std::thread::{sleep};
use std::process;


pub fn main(){
    register_signal_handlers();

    let pid = process::id();
    println!("{}", pid);

    let delay = Duration::from_secs(1);

    for i in 1_usize..{
        println!("{}", i);
        unsafe{
            if SHUT_DOWN{
                println!("*bow*");
                return;
            }
        }

        sleep(delay);

        let signal = if i>10{
            SIGTERM
        } else {
            SIGUSR1
        };

        unsafe{
            libc::raise(signal);
        }
    }
    unreachable!();
}

#[allow(dead_code)]
fn handle_sigterm(_signal: i32){
    register_signal_handlers();

    println!("SIGTERM");
    unsafe{
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal: i32){
    register_signal_handlers();

    println!("SIGUSR1");
}

fn add(a:i32, b:i32)-> i32{
    a+b
}

pub fn sixty(){
    let delay = Duration::from_secs(1);

    let pid = process::id();
    println!("{}", pid);

    for i in 1..=60{
        sleep(delay);
        println!(". {}", i);
    }
}

use libc::{signal, raise};
use libc::{SIG_DFL, SIG_IGN};

pub fn change_sigfun(){
    unsafe{
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }

    println!("OK, here");

    unsafe{
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }

    println!("not OK");
}