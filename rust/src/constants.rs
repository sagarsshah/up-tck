// Define constants for command strings
pub const SEND_COMMAND: &str = "send";
pub const REGISTER_LISTENER_COMMAND: &str = "registerlistener";
pub const UNREGISTER_LISTENER_COMMAND: &str = "unregisterlistener";
pub const INVOKE_METHOD_COMMAND: &str = "invokemethod";

// Define constants for addresses
pub const DISPATCHER_ADDR: (&str, u16) = ("127.0.0.1", 44444);
pub const TEST_MANAGER_ADDR: (&str, u16) = ("127.0.0.5", 12345);

// Define constant for maximum message length
pub const BYTES_MSG_LENGTH: usize = 32767;

#[test]
pub fn test_constants() {
    println!("SEND_COMMAND: {}", SEND_COMMAND);
    println!("REGISTER_LISTENER_COMMAND: {}", REGISTER_LISTENER_COMMAND);
    println!("UNREGISTER_LISTENER_COMMAND: {}", UNREGISTER_LISTENER_COMMAND);
    println!("INVOKE_METHOD_COMMAND: {}", INVOKE_METHOD_COMMAND);
    println!("DISPATCHER_ADDR: {:?}", DISPATCHER_ADDR);
    println!("TEST_MANAGER_ADDR: {:?}", TEST_MANAGER_ADDR);
    println!("BYTES_MSG_LENGTH: {}", BYTES_MSG_LENGTH);
}