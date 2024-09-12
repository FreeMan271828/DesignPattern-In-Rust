/// 假设这是服务器端接口
pub trait RemoteService {
    fn request();
}
pub struct RemoteServiceProxy{
    url: String,
    port: String
}
impl RemoteService for RemoteServiceProxy{
    fn request() {
        Self::connect();
        Self::send_request("request".to_string());
        let response: String = Self::receive_response();
        Self::process_response(response);
        Self::close_connect();
    }
}
impl RemoteServiceProxy{
    // 连接代码
    fn connect(){}
    // 发送请求
    fn send_request(request: String){}
    // 接受请求
    fn receive_response() -> String{
        "Hello".to_string()
    }
    // 关闭连接
    fn close_connect(){}
    // 请求处理
    fn process_response(response: String){}
}