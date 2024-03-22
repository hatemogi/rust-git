use git2::Repository;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let pwd = env::current_dir()?;
    let repo = Repository::init(pwd)?;
    let branches = repo.branches(None)?;
    for branch in branches {
        let b = branch?;
        println!("branch => {:?}, type => {:?}", b.0.name(), b.1);
    }
    println!("Hello, world!");
    Ok(())
}
