pub trait Display {
    fn set_focus(&self);
    fn do_display(&self);
    fn reset_focus(&self);
}

/// 为了维持不变部分, 使用View实现Display, 并使用View的子类实现Display
pub struct View {}

impl Display for View {
    fn set_focus(&self) {
        println!("Setting focus");
    }
    fn do_display(&self) {
        println!("Displaying content");
    }
    fn reset_focus(&self) {
        println!("Resetting focus");
    }
}
