use clojurecli::windows;
//use clojurecli;

fn main() {
    let args = windows::get_args();
    clojurecli::yoh();
    println!("{:?}", args);
}
