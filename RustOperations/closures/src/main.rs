/*----------------------------------------------------
  Demonstrate type name and operation of Rust closure
  - closure can be passed to, or returned from, 
    another function
----------------------------------------------------*/

/* display typename of T */
fn show_type<T>(_t: &T, nm: &str) {
  let typename = std::any::type_name::<T>();
  println!("\n  {nm}: {typename}");
}

/* function returning closure */
fn cl_demo() -> impl Fn(&str) {
  let prefix = "\n  ";
  /* define closure using passed argument and prefix capture */
  let cl = move |nm:&str| { println!("{}name: {nm:?}", prefix)};
  cl
}

fn main() {
  let demo = cl_demo();
  show_type(&demo, "cl");
  demo("Arthur Dent");
  println!("\n  That's all Folks!\n");
}
