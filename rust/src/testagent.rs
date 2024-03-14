use std::net::TcpStream;
use std::io::Read;

use std::collections::HashMap;

use std::thread;
use serde::{Serialize, Deserialize};
use protobuf::{Message, MessageDyn};
//use up_rust::rpc::RpcMapper;
//use up_rust::{UCode, UMessage, UStatus};
use up_rust::{
  //  transport::validator::Validators,
    rpc::RpcMapper, transport::datamodel::UTransport, uprotocol::{
        umessage, UCode, UMessage, UStatus 
    }
   // uri::validator::UriValidator,
};


use crate::uTransportSocket::UtrasnsportSocket;
use crate::utils::{base64_to_protobuf_bytes, protobuf_to_base64, send_socket_data};

pub struct JsonData {
    action: String,
    message: String,
}
// Define a listener type alias
//type Listener = Box<dyn Fn(Result<(), UStatus>) + Send + Sync + 'static>;

type Listener = Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>;
pub struct SocketTestAgent {
    utransport: UtrasnsportSocket,
   // possible_received_protobufs: Vec<UMessage>,
  // listener:Listener,
    clientsocket: TcpStream,
    listnerString: &str,
}

impl SocketTestAgent {
   pub fn new(test_clientsocket: TcpStream, utransport: UtrasnsportSocket) -> Self {
        let clientsocket = test_clientsocket.try_clone().expect("Failed to clone socket");
     //   let possible_received_protobufs = vec![UMessage::default()]; // Modify with appropriate initialization
      //  let listener: Listener = None; 
            
        SocketTestAgent {
            utransport,
    //        listener,
       //     possible_received_protobufs,
            clientsocket,
            listnerString,
            
        }
    }

    pub fn TM_receive_thread (&mut self)
{
    //let utransport_clone = utransport.clone();
    thread::spawn(move || {
        Self::receive_from_tm (self,self.clientsocket, self.utransport);
    });
}    


fn on_receive(&self,result:Result<UMessage, UStatus>) {
    println!("Listener onreceived");


    let mut json_message = JsonData {
        action: "onReceive".to_owned(),
        message: "None".to_string(),
    };

    match result {
        Ok(message) => json_message.message = protobuf_to_base64(&message),
        Err(status)=> println!("Received error status: {}", status),
     }

     self.send_to_tm(json_message);
  
}
    
    fn receive_from_tm(&mut self,clientsocket: TcpStream, utransport: UtrasnsportSocket) {
        //let testlistner:Listener = Box::new(self.testa);    
        let listener:Listener = Box::new(move |result| self.on_receive(result));





        loop {
            let mut recv_data = [0; 1024];
            let recv_result = clientsocket.read(&mut recv_data);
            //let recv_result = clientsocket
            match recv_result {
                Ok(bytes_received) => {
                    if bytes_received == 0 {
                        println!("Closing TA Client Socket");
                        break;
                    }
                    let recv_data_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&recv_data[..bytes_received]);
                    let json_msg: HashMap<String, String> = serde_json::from_str(&recv_data_str).unwrap(); // Assuming serde_json is used for JSON serialization/deserialization
                    let action = json_msg["action"].clone();
                    let umsg_base64 = json_msg["message"].clone();
                    let protobuf_serialized_data = base64_to_protobuf_bytes(&umsg_base64).expect("received data from TM is corrupt"); // Implement this function according to your logic

                    
                let mut umsg = UMessage::new(); // Assuming UMessage is a protobuf-generated message type

                    // Assuming umsg_serialized is the byte array obtained from SerializeToString()
                    if let Err(err) = umsg.merge_from_bytes(&protobuf_serialized_data) {
                       eprintln!("Error deserializing UMessage: {}", err);
                    } else {
                       eprint!("data seems to be correct!");                       
                    } 

                  
                
                    
             //       let umsg = RpcMapper::unpack_payload(protobuf_serialized_data, UMessage::default()); // Implement RpcMapper and UMessage accordingly
                    let status = match action.as_str() {
                        //"SEND_COMMAND" => self.utransport.send(umsg),
                        "REGISTER_LISTENER_COMMAND" => self.utransport.register_listener(Some(umsg.attributes.source.clone()),listener), // Assuming listener can be cloned
                       // "UNREGISTER_LISTENER_COMMAND" => self.utransport.unregister_listener(Some(umsg.attributes.source.clone()),&(self.listnerString)), // Assuming listener can be cloned
                        _ => UStatus { code: UCode::OK.into(), message: Some("Unknown action".to_string()), details: todo!(), special_fields: todo!() }, // Modify with appropriate handling
                    };
                    let json_message = JsonData{
                        action:"uStatus".to_owned(),
                        message: protobuf_to_base64(&status) 
                    };
                    self.send_to_tm(json_message);
            

                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                    break;
                }
            }
        }
    }
    


 


    fn send_to_tm(&self, json_message:JsonData) {
        // Sends JSON data to Test Manager
        let json_message_str = serde_json::to_string(&json_message).expect("Failed to serialize JSON");

        let message = json_message_str.as_bytes();

        if let Err(err) = send_socket_data(self.clientsocket , &message) {
            eprintln!("Error sending message: {}", err);
        }

    }
    fn close_connection(&self) {
        self.clientsocket.shutdown(std::net::Shutdown::Both).expect("Failed to close socket");
    }
}
