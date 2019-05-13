use std::env;
use std::process::Command;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let mut gcc = Command::new("aarch64-none-elf-gcc");

    let mut output = None;
    let mut elf = None;
    let mut iargs = args.iter();

    while let Some(arg) = iargs.next() {
        gcc.arg(&arg);

        if arg == "-o" {
            output = iargs.next();
            let mut no_nro = output.unwrap().replace(".nro", "");
            no_nro.push_str(".elf");
            gcc.arg(&no_nro);
            elf = Some(no_nro);
        }
    }

    eprintln!("{:?}", gcc);
    assert!(gcc.status().unwrap().success());

    let output = output.unwrap();
    let elf = elf.unwrap();

    let mut linkle = Command::new("linkle");
    linkle.arg("nro")
        .arg(elf)
        .arg(output);
    eprintln!("{:?}", linkle);
    assert!(linkle.status().unwrap().success());
}
