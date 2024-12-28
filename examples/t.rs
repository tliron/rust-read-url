pub fn main() {
    let context = read_url::Context::new();

    let bases = vec![
        context.new_working_dir_url().unwrap().into(),
        context.new_url("file:///etc").unwrap(),
        context.new_url("https://emblemparade.com").unwrap(),
    ];

    let url = context.new_valid_any_or_file_url("target", &bases).unwrap();
    dump(&*url);

    let url = context.new_valid_any_or_file_url(".gitignore", &bases).unwrap();
    println!();
    dump(&*url);
    read(&*url);

    let url = context.new_valid_any_or_file_url("redhat-release", &bases).unwrap();
    println!();
    dump(&*url);
    read(&*url);

    let url = context.new_valid_any_or_file_url("robots.txt", &bases).unwrap();
    println!();
    dump(&*url);
    read(&*url);
}

fn dump(url: &dyn read_url::URL) {
    println!("url:  {}", url);
    println!("base: {}", url.base().unwrap());
}

fn read(url: &dyn read_url::URL) {
    let mut reader = url.open().unwrap();
    let mut s = std::string::String::new();
    reader.read_to_string(&mut s).unwrap();
    print!("{}", s);
}
