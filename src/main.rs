use libe621::database::BulgeDB;

fn help() {
    println!("bulgetool v{} - a simple companion tool for bulge", env!("CARGO_PKG_VERSION"));
    println!("bulgetool help -> show this help message");
    println!("bulgetool deplist <package> -> list all packages that depend on <package>");
    println!("bulgetool blame <filename> -> list all packages that own <filename>");
}

fn deplist(args: Vec<String>) {
    let package = &args[0];
    let db = BulgeDB::from_file("/etc/bulge/databases/bulge.db").expect("failed to open bulge database!");
    let deps = db.find_and_order_dependents_of_package(package);
    println!("{}", package);
    for dep in deps.iter() {
        println!("{}", dep.name);
    }
}

fn blame(args: Vec<String>) {
    let filename = &args[0];
    let db = BulgeDB::from_file("/etc/bulge/databases/bulge.db").expect("failed to open bulge database!");
    let packages = db.find_packages_by_filename(filename);
    for package in packages.iter() {
        println!("{} -> {}", package.name, package.installed_files.iter().find(|f| f == &filename || f.ends_with(filename)).unwrap());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("command required! see bulgetool help");
        std::process::exit(1);
    }
    let command = &args[1];
    let args = args[2..].to_vec();
    match command.as_str() {
        "help" => help(),
        "deplist" => deplist(args),
        "blame" => blame(args),
        _ => {
            eprintln!("unknown command! see bulgetool help");
            std::process::exit(1);
        }
    }
}
