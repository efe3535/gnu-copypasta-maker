use std::env;
use std::io;
use std::process::exit;

fn main() {
    let mut replace_with = String::new();
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("What would you like to interject?\t");
        io::stdin().read_line(&mut replace_with).expect("Failed to get input.");
    } else {
        if args[1] != "--help" && args[1] != "-h" {
            replace_with.push_str(&args[1].trim());
        } 
        if args[1].starts_with("--") && args[1] != "--help" && args[1] != "-h" {
            println!("bad usage, see {} --help", args[0]);
            exit(1);
        }
        else if args[1] == "--help" || args[1] == "-h" {
            println!("gnu copypasta maker!\nusage: {} < stirng to replace linux > or if no arguments you will be prompted for it.", args[0]);
            exit(0);
        }
    }
    
    let joke = String::from("I'd just like to interject for a moment.  What you're referring to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component
of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX.
Many computer users run a modified version of the GNU system every day, without realizing it.  Through a peculiar turn of events, the version of GNU
which is widely used today is often called Linux, and many of its users are not aware that it is basically the GNU system, developed by the GNU Project.
There really is a Linux, and these people are using it, but it is just a part of the system they use.  Linux is the kernel: the program in the system
that allocates the machine's resources to the other programs that you run. The kernel is an essential part of an operating system, but useless by itself;
it can only function in the context of a complete operating system.  Linux is normally used in combination with the GNU operating system: the whole system
is basically GNU with Linux added, or GNU/Linux.  All the so-called Linux distributions are really distributions of GNU/Linux."); 
    let res = str::replace(&joke, "Linux", &replace_with.trim());
    println!("{}", res);
}
