use std::net::TcpStream;
use std::io::Read;
use serde_json::to_string;
use std::sync::Arc;

use std::collections::HashMap;

use std::thread;
use base64::Engine;
use serde::{Serialize, Deserialize};
use protobuf::{Message, MessageDyn};
//use up_rust::rpc::RpcMapper;
//use up_rust::{UCode, UMessage, UStatus};
use up_rust::{
  //  transport::validator::Validators,
    rpc::RpcMapper, transport::datamodel::UTransport, uprotocol::{
        umessage, UCode, UMessage, UStatus, UUri 
    }
   // uri::validator::UriValidator,
};


use crate::uTransportSocket::UtrasnsportSocket;
use crate::utils::{base64_to_protobuf_bytes, protobuf_to_base64, send_socket_data,convert_json_to_jsonstring};

#[derive(Serialize)]
pub struct JsonData {
    action: String,
    message: String,
}
// Define a listener type alias
//type Listener = Box<dyn Fn(Result<(), UStatus>) + Send + Sync + 'static>;

type Listener = Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>;
//#[derive(Clone)]
pub struct SocketTestAgent {
    utransport: UtrasnsportSocket,
   // possible_received_protobufs: Vec<UMessage>,
  // listener:Listener,
    clientsocket: TcpStream,
    listner_map: Vec<String>,
}

impl Clone for SocketTestAgent {
    fn clone(&self) -> Self {
        SocketTestAgent {
            utransport: self.utransport.clone(), // Assuming UtrasnsportSocket implements Clone
            clientsocket: self.clientsocket.try_clone().expect("Failed to clone TcpStream"), // Clone TcpStream
            listner_map: self.listner_map.clone(), // Clone Vec<String>
        }
    }
    
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl SocketTestAgent  {
   pub fn new(test_clientsocket: TcpStream, utransport: UtrasnsportSocket) -> Self {
        let clientsocket = test_clientsocket.try_clone().expect("Failed to clone socket");
     //   let possible_received_protobufs = vec![UMessage::default()]; // Modify with appropriate initialization
      //  let listener: Listener = None; 
       
            
        SocketTestAgent {
            utransport,
    
            clientsocket,
    
            listner_map: Vec::new(),
            
            
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
    
    async fn receive_from_tm(&mut self,mut clientsocket: TcpStream, utransport: UtrasnsportSocket) {
       
        //let testlistner:Listener = Box::new(self.testa);  
            // Clone Arc to capture it in the closure
            let arc_self = Arc::new(self.clone());  
            let cloned_Arc_self = Arc::clone(&arc_self);
       // let listener:Listener = Box::new(move |result: Result<UMessage, UStatus>| cloned_Arc_self.on_receive(result));

    //   let listener:Listener = Box::new( arc_self.on_receive(Result<UMessage, UStatus>));
       



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

   
                    let status = match action.as_str() {
                        "SEND_COMMAND" => {match self.utransport.send(umsg).await{Ok(_) =>{println!("message sent successfully");()}Err(status)=>{println!("failed to send message");()}}},
                        "REGISTER_LISTENER_COMMAND" => {
                            let cloned_listener = Arc::clone(&arc_self);
                          //  let cloned_listener_data: Listener = Box::new(move |result: Result<UMessage, UStatus>| cloned_listener.on_receive(result));

                            let cloned_listener_data: Listener = Box::new(move |result: Result<UMessage, UStatus>| cloned_listener.on_receive(result));
                            //self.utransport.register_listener(umsg.attributes.source.clone().unwrap(),cloned_listener_data);
                            self.utransport.register_listener(umsg.attributes.source.clone().unwrap(),cloned_listener_data);
                            ()}, // Assuming listener can be cloned
                       "UNREGISTER_LISTENER_COMMAND" => {self.utransport.unregister_listener(umsg.attributes.source.clone().unwrap(),&self.listner_map[0]);()}, // Assuming listener can be cloned
                        _ => {||UStatus { code: UCode::OK.into(), message: Some("Unknown action".to_string()), details: todo!(), special_fields: todo!() };}, // Modify with appropriate handling
                    };


                  let mut status_clone= status.clone();  
                 let base64_str  = serde_json::to_string(&status).unwrap();
                    let json_message = JsonData{
                        action:"uStatus".to_owned(),
                        message: base64_str, 
                        
                
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
        let json_message_str = convert_json_to_jsonstring(&json_message);
        //let json_message_str = serde_json:to_string(&json_message).expect("Failed to serialize JSON");

        let message = json_message_str.as_bytes();

        if let Err(err) = send_socket_data(self.clientsocket , &message) {
            eprintln!("Error sending message: {}", err);
        }

    }
    fn close_connection(&self) {
        self.clientsocket.shutdown(std::net::Shutdown::Both).expect("Failed to close socket");
    }
}
