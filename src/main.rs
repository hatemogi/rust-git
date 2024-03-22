use git2::Repository;
use std::env;

fn main() {
    let pwd = env::current_dir().unwrap();

    let repo = match Repository::init(pwd) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    let branches = repo.branches(None).unwrap();
    for branch in branches {
        let b = branch.unwrap();
        println!("branch => {:?}", b.0.name());
    }
    println!("Hello, world!");
}
