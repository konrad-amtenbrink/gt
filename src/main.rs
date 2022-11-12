use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use git2::Repository;
use std::env::current_dir;

fn main() {
    let mut ctx: ClipboardContext =
        ClipboardProvider::new().expect("can not create clipboard context");
    let path = current_dir().expect("could not find current directory");
    let repo = Repository::open(path).expect("could not open current repository");
    let head = repo.head().expect("could not find head");

    let reflog = repo
        .reflog(&head.name().expect("could not find head"))
        .unwrap();

    let commit = reflog.get(0).expect("could not get repository history");
    let raw_commit_msg = commit
        .message()
        .expect("could not retrieve commit messsage");

    let split_index = raw_commit_msg
        .find(":")
        .expect("could not retrieve commit messsage")
        + 2;

    let (_, commit_msg) = &raw_commit_msg.split_at(split_index);

    ctx.set_contents(commit_msg.to_string())
        .expect("could not set content");
}
