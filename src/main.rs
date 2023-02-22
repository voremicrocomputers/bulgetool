use libe621::database::BulgeDB;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("package name required! try `dependentlist <package>`");
        std::process::exit(1);
    }
    let package = &args[1];
    let db = BulgeDB::from_file("/etc/bulge/databases/bulge.db").expect("failed to open bulge database!");
    let deps = db.find_and_order_dependents_of_package(package);
    println!("{}", package);
    for dep in deps.iter() {
        println!("{}", dep.name);
    }
}
