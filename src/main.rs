use std::env;
use std::process::Command;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let mut gcc = Command::new("aarch64-none-elf-gcc");

    let mut output = None;
    let mut elf = None;
    let mut iargs = args.iter();

    let mut linkle_args: Vec<String> = vec![];

    while let Some(arg) = iargs.next() {
        if arg == "-o" {
            gcc.arg(&arg);
            output = iargs.next();
            let mut no_nro = output.unwrap().replace(".nro", "");
            no_nro.push_str(".elf");
            gcc.arg(&no_nro);
            elf = Some(no_nro);
        } else if arg == "--icon-path" || arg == "--romfs-path" || arg == "--nacp-path" {
            linkle_args.push(arg.to_string());
            linkle_args.push(iargs.next().unwrap().to_string());
        } else {
            gcc.arg(&arg);
        }
    }

    eprintln!("{:?}", gcc);
    assert!(gcc.status().unwrap().success());

    let output = output.unwrap();
    let elf = elf.unwrap();

    let mut linkle = Command::new("linkle");
    linkle.arg("nro")
        .arg(elf)
        .arg(output)
        .args(linkle_args);
    eprintln!("{:?}", linkle);
    assert!(linkle.status().unwrap().success());
}
