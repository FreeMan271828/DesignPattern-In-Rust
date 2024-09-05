//! 策略模式,可以将一系列的算法封装起来,并可以相互替换,使得算法独立于客户而变化
//! 本例中有三个结构体SimpleCompositor, TeXCompositor, ArrayCompositor,
//! 对应了三个具体的算法, 它们的共同特征是要实现compose, 所以在do_compose这个
//! 算法使用类中, 它并不需要关心使用哪个具体的算法, 而是保证传过来的参数要实现ComposeTrait
//! 特征, 并使用特征的compose方法即可
pub trait ComposeTrait{
    fn compose(&self) -> String;
}

pub struct SimpleCompositor{}
impl ComposeTrait for SimpleCompositor{
    fn compose(&self) -> String{
        "SimpleCompositor".to_string()
    }
}

pub struct TeXCompositor{}
impl ComposeTrait for TeXCompositor{
    fn compose(&self) -> String{
        "TeXCompositor".to_string()
    }
}

pub struct ArrayCompositor{}
impl ComposeTrait for ArrayCompositor{
    fn compose(&self) -> String{
        "ArrayCompositor".to_string()
    }
}

pub fn do_compose<T: ComposeTrait>(input: &T) -> String{
    input.compose()
}