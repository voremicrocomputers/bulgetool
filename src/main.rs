use std::collections::{BTreeSet, HashMap};
use libe621::database::{BulgeDB, Package};

fn help() {
    println!("bulgetool v{} - a simple companion tool for bulge", env!("CARGO_PKG_VERSION"));
    println!("bulgetool help -> show this help message");
    println!("bulgetool deplist <package> -> list all packages that depend on <package>");
    println!("bulgetool blame <filename> -> list all packages that own <filename>");
}

fn deplist(args: Vec<String>) {
    let package = &args[0];
    let depth = if args.len() > 1 {
        Some(args[1].parse::<usize>().expect("invalid depth!"))
    } else {
        None
    };
    let db = BulgeDB::from_file("/etc/bulge/databases/bulge.db").expect("failed to open bulge database!");
    let deps = db.find_and_order_dependents_of_package(package, depth);
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

fn trace_dependency(args: Vec<String>) {
    let from = &args[0];
    let to = &args[1];
    let db = BulgeDB::from_file("/etc/bulge/databases/bulge.db").expect("failed to open bulge database!");

    let mut open_set: BTreeSet<String> = BTreeSet::new();
    let mut came_from: HashMap<String, String> = HashMap::new();
    let mut g_score: HashMap<String, i32> = HashMap::new();
    let mut f_score: HashMap<String, i32>= HashMap::new();

    open_set.insert(from.clone());

    g_score.insert(from.clone(), 0);
    f_score.insert(from.clone(), 0);

    loop {
        let current = open_set.iter().min_by_key(|n| f_score.get(&*(*n).clone()).unwrap()).cloned();
        if current.is_none() {
            println!("no path found!");
            return;
        }
        let current = current.unwrap();
        if &current == to {
            let mut path = vec![current.clone()];
            let mut current = current.clone();
            while came_from.contains_key(&current) {
                current = came_from.get(&current).cloned().unwrap();
                path.push(current.clone());
            }
            path.reverse();

            let mut path_str = String::new();
            for p in path.iter() {
                path_str.push_str(&format!("{} -> ", p));
            }
            path_str.pop();
            path_str.pop();
            path_str.pop();
            path_str.pop();
            println!("{}", path_str);

            return;
        }
        open_set.retain(|n| n != &current);
        let deps = db.installed_packages.iter().find(|p| p.name == current).unwrap().dependencies.clone();
        for (i, dep) in deps.iter().map(|d| db.installed_packages.iter().find(|p| &p.name == d)).enumerate() {
            if dep.is_none() {
                println!("{} is not installed!", deps[i]);
                continue;
            }
            let dep = dep.unwrap();
            let dep_name = dep.name.clone();
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if !g_score.contains_key(&dep_name) || tentative_g_score < *g_score.get(&dep.name).unwrap() {
                came_from.insert(dep_name.clone(), current.clone());
                g_score.insert(dep_name.clone(), tentative_g_score);
                f_score.insert(dep_name.clone(), tentative_g_score);
                if !open_set.contains(&dep_name) {
                    open_set.insert(dep.name.clone());
                }
            }
        }
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
        "tracedep" => trace_dependency(args),
        _ => {
            eprintln!("unknown command! see bulgetool help");
            std::process::exit(1);
        }
    }
}
