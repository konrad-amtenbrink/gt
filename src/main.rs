use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use git2::Repository;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let path = "";
    let repo = Repository::open(path).unwrap();
    let head = repo.head().unwrap();

    let reflog = repo.reflog(&head.name().unwrap()).unwrap();
    let last_commit = reflog.get(0).unwrap();
    let last_commit_msg = &last_commit.message().unwrap();

    ctx.set_contents(last_commit_msg.to_string().to_owned())
        .unwrap();
}
