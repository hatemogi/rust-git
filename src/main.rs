use git2::Repository;

fn main() {
    let repo = match Repository::init("/Users/dante/work/rust-git") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    let branches = repo.branches(None).unwrap().enumerate();
    for _branch in branches {
        println!("a branch");
    }
    println!("Hello, world!");
}
