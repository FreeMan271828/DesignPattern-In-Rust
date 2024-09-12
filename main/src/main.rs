use virtual_proxy::Graphic;

fn main(){
    let mut proxy = virtual_proxy::ImageProxy::new(
        Box::from("Hello".to_string())
    );
    proxy.draw();
}