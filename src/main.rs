mod behavioral_patterns;


fn main() {
    println!("Hello, world!");
    let i = behavioral_patterns::strategy::TeXCompositor{};
    println!("{}", behavioral_patterns::strategy::do_compose(&i));
}
