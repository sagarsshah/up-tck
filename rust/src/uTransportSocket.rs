
use async_trait::async_trait;

use std::net::TcpStream;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::thread;

//use std::io::*;


use up_rust::{
    transport::{datamodel::UTransport, validator::Validators},
    uprotocol::{
        Data, UAttributes, UCode, UMessage, UMessageType, UPayload, UPayloadFormat, UStatus, UUri,
    },
    uri::validator::UriValidator,
};





use crate::constants::DISPATCHER_ADDR;
use crate::constants::BYTES_MSG_LENGTH;


//use uprotocol::utransport::{Utransport, UtransportError};
trait UtransportExt {
    //topic_to_listener: HashMap< Vec<u8>, Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>>;
  
    //fn new()->Self;
    fn __listen(& mut self);
    fn socket_init(& mut self);
    fn _handle_publish_message(& mut self, umsg: UMessage);
}
pub struct UtrasnsportSocket
{
    //test_v:i32,
    socket: TcpStream,
    topic_to_listener: HashMap< Vec<u8>, Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>>,
       
}

impl UtrasnsportSocket {
   
    fn new() -> Self { 
     //let test_v = 32;
     let mut socket:TcpStream = TcpStream::connect(DISPATCHER_ADDR).expect("Failed to connect to dispatcher"); 
     
     let mut topic_to_listener: HashMap< Vec<u8>, Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>> =  HashMap::new();
     UtrasnsportSocket{socket,topic_to_listener}}

    }
impl UtransportExt for UtrasnsportSocket{
   
   //fn new() -> Self { 
    //let test_v = 32;
    //let mut socket:TcpStream = TcpStream::connect(DISPATCHER_ADDR)?; 
    
    //let mut topic_to_listener: HashMap< Vec<u8>, Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>> =  HashMap::new();
    //UTrasnsport_Socket{socket,topic_to_listener}}



 fn socket_init(& mut self)
 {
      // Define the address of the dispatcher
     // let dispatcher_addr = DISPATCHER_ADDR.parse::<SocketAddr>()?;

      // Create a TCP socket and connect to the dispatcher
      //let mut
       
      //self.socket = TcpStream::connect(DISPATCHER_ADDR);

      // Create a HashMap to store reqid_to_future mapping.
   // let mut reqid_to_future: HashMap<Bytes, Future> = HashMap::new();

    // Create a HashMap to store topic_to_listener mapping.
    //let mut topic_to_listener: HashMap<Bytes, Vec<listner>> = HashMap::new();
   // let mut topic_to_listener: HashMap< Vec<u8>, Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>> =  HashMap::new();



    let thread_handle = thread::spawn(|| {
        // Call the __listen function
        self.__listen();
    });

    // Wait for the thread to finish
    let _ = thread_handle.join();
 }

 
 fn __listen(& mut self) {
    // Implementation of the __listen function goes here
    // This function should include the logic of listening for responses


    loop {
        // Receive data from the socket
        let mut buffer: [u8; BYTES_MSG_LENGTH] = [0; BYTES_MSG_LENGTH];
       
        let bytes_read = match self.socket.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                // Handle socket errors (e.g., connection closed)
                eprintln!("Socket error: {}", e);
                break;
            }
        };

        // Check if no data is received
        if bytes_read == 0 {
            continue;
        }

        let umessage:Result<UMessage,_> = Err(buffer);


     //let mut umessage = UMessage::new(); // Assuming UMessage is a protobuf-generated message type

     // Assuming umsg_serialized is the byte array obtained from SerializeToString()
     //if let Err(err) = umessage.merge_from_bytes(&buffer) {
       //  eprintln!("Error deserializing UMessage: {}", err);
     //} else {
       //  umessage.attributes;
        
     //}
    // let type_value:UMessageType = umessage.attributes.type_.;

            

       
        // Assuming you have UAttributes and UMessageType defined elsewhere
        
        
        match umessage.attributes.type_.enum_value()  {
            Ok(mt ) => match mt {
            UMessageType::UMESSAGE_TYPE_PUBLISH | UMessageType::UMESSAGE_TYPE_REQUEST => {
                self._handle_publish_message(umessage);
            }
            UMessageType::UMESSAGE_TYPE_UNSPECIFIED => todo!(),
            UMessageType::UMESSAGE_TYPE_RESPONSE => todo!(),
        }
            Err(_) => todo!(),
       // ERR(unkown_code:i32) => println!("unkown code"),
          
        }

        
    }
}
fn _handle_publish_message( & mut self,umsg: UMessage) {
    let topic_b = umsg.attributes.source;

    if let Some(listeners) = self.topic_to_listener.get(&topic_b) {
        //println!("{} Handle Topic", std::any::type_name::<Self>());

        for listener in listeners {
            listener.on_receive(umsg);
        }
    } else {
        //println!("{} Topic not found in Listener Map, discarding...", std::any::type_name::<Self>());
    }
}
}

#[async_trait]
impl UTransport for UtrasnsportSocket{


     /// Sends a message using this transport's message exchange mechanism.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send. The `type`, `source` and`sink` properties of the [`crate::UAttributes`] contained
    ///   in the message determine the addressing semantics:
    ///   * `source` - The origin of the message being sent. The address must be resolved. The semantics of the address
    ///     depends on the value of the given [attributes' type](crate::UAttributes::type_) property .
    ///     * For a [`PUBLISH`](crate::UMessageType::UMESSAGE_TYPE_PUBLISH) message, this is the topic that the message should be published to,
    ///     * for a [`REQUEST`](crate::UMessageType::UMESSAGE_TYPE_REQUEST) message, this is the *reply-to* address that the sender expects to receive the response at, and
    ///     * for a [`RESPONSE`](crate::UMessageType::UMESSAGE_TYPE_RESPONSE) message, this identifies the method that has been invoked.
    ///   * `sink` - For a `notification`, an RPC `request` or RPC `response` message, the (resolved) address that the message
    ///     should be sent to.
    ///
    /// # Errors
    ///
    /// Returns an error if the message could not be sent.
    async fn send(&self, message: UMessage) -> Result<(), UStatus>
    {

     
    
    
        let umsg_serialized = message.to_string();

        match self.socket.write_all(&umsg_serialized).await {
            Ok(_) => {
                //info!("{} uMessage Sent", std::any::type_name::<Self>());
                UStatus {code: up_rust::uprotocol::UCode::OK, message: "OK",details:todo!(),special_fields:todo!() }}
            
            Err(_) => UStatus {code:up_rust::uprotocol::UCode::INTERNAL,message:"INTERNAL ERROR: OSError sending UMessage", details: todo!(), special_fields: todo!() } }
        }
        
        
       
    

    /// Receives a message from the transport.
    ///
    /// # Arguments
    ///
    /// * `topic` - The topic to receive the message from.
    ///
    /// # Errors
    ///
    /// Returns an error if no message could be received. Possible reasons are that the topic does not exist
    /// or that no message is available from the topic.
    async fn receive(&self, topic: UUri) -> Result<UMessage, UStatus>
    {
        
        async {
            // Your implementation here
            println!("receiving message");

            // Simulate asynchronous operation (e.g., sending over network)
           // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            // Simulate successful result
            let mut message = UMessage::new();
            Ok(message)
        }.await
        
    }

    /// Registers a listener to be called for each message that is received on a given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The (resolved) address to register the listener for.
    /// * `listener` - The listener to invoke.
    ///
    /// # Returns
    ///
    /// An identifier that can be used for [unregistering the listener](Self::unregister_listener) again.
    ///
    /// # Errors
    ///
    /// Returns an error if the listener could not be registered.
    async fn register_listener(
        &self,
        topic: UUri,
        listener: Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>,
    ) -> Result<String, UStatus>
    {

        let topic_serialized = topic.to_string();

        let mut topic_to_listener = self.topic_to_listener.lock().await;
        
        if let Some(listeners) = topic_to_listener.get_mut(&topic_serialized) {
            listeners.push(listener);
        } else {
            topic_to_listener.insert(topic_serialized, vec![listener]);
        }
    
        UStatus { code: up_rust::uprotocol::UCode::OK, message: "OK",details:todo!(),special_fields:todo!() }
    }


     
    

    /// Unregisters a listener for a given topic.
    ///
    /// Messages arriving on this topic will no longer be processed by this listener.
    ///
    /// # Arguments
    ///
    /// * `topic` - Resolved topic uri where the listener was registered originally.
    /// * `listener` - Identifier of the listener that should be unregistered.
    ///
    /// # Errors
    ///
    /// Returns an error if the listener could not be unregistered, for example if the given listener does not exist.
    async fn unregister_listener(&self, topic: UUri, listener: &str) -> Result<(), UStatus>
    {
        let topic_serialized = topic.to_string(); // Assuming SerializeToString returns Result<Vec<u8>, _>

        if let Some(listeners) = self.topic_to_listener.get_mut(&topic_serialized) {
            if listeners.len() > 1 {
                if let Some(index) = listeners.iter().position(|x| *x == listener) {
                    listeners.remove(index);
                }
            } else {
                self.topic_to_listener.remove(&topic_serialized);
            }
        }
    
          UStatus{code: up_rust::uprotocol::UCode::OK, message: "OK",details:todo!(),special_fields:todo!()} 
        //UStatus { code: up_rust::UCode::OK, message: "OK", details: todo!(), special_fields: todo!() }
    }
}