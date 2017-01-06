use std::io;
error_chain! {
    // Bindings to types implementing std::error::Error.
    foreign_links {
        Io(io::Error);
    }

    errors {
        // EchoRequestTimeout(request_id: u32, timeout: u64) {
        //     description("Echo Request timed out")
        //         display("Echo Request {} timed out ({} s)", request_id, timeout)
        // }
        ConnectionFailed {
            description("Connection failed")
                display("Connection failed")
        }
        // ConnectionClosed {
        //     description("Remote end closed the connection")
        //         display("Remote end closed the connection")
        // }
        InvalidInvocation(reason: &'static str) {
            description("Invalid command")
                display("Invalid command: {}", reason)
        }
    }

}
