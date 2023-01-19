
//This trait provides poll function for socketRead struct
trait Future{
    type Output ;
    fn poll(&mut self , wake : fn()) -> Poll<Self::Output>;
}

//the data status inside the socket 
enum Poll<T>{
    Ready(T),
    Pending,
}

//<'a> is a lifetime parameter 
//that specify the lifetime of the reference SocketRead 
pub struct SocketRead<'a>{
    socket: &'a Socket,
}

impl Future for SocketRead<'_> {
    
//type creates an associated type 
    type Output = Vec<u8> ;
   
//
    fn poll(&mut self , wake : fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            Poll::Ready(self.socket.read_available_data())
        } else {
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }

    
}