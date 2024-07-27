fn takes_ref(_: &str) {}

fn main() {
    takes_ref(&&"");
}
