use mio::{unix::EventedFd, Evented, Poll, PollOpt, Ready, Token};
use socketcan::{CANFrame, CANSocket};
use std::io;
use std::os::unix::io::AsRawFd;

//All this is because mio doesn't have an adapter for cansockets
// Wrapper type arround CANSocket for eventing!
pub struct MIOCANSocket {
    socket: CANSocket,
}

//Builder methods and Convenience methods
impl MIOCANSocket {
    pub fn from(socket: CANSocket) -> MIOCANSocket {
        MIOCANSocket { socket: socket }
    }

    pub fn read_frame(&self) -> io::Result<CANFrame> {
        self.socket.read_frame()
    }
}

// Allow MIOCANSocket to be polled by mio
// The EventLoop object provided by mio is our main point of contact.
// Interaction with the event loop is in the form of the register, register_opt, reregister and deregister functions.
// These functions allow our code to control how the event loop interacts with the incoming client connections.
impl Evented for MIOCANSocket {
    fn register(
        &self,
        poll: &Poll,     // Types of readiness
        token: Token, // A Token is used to identify the state related to a connected socket. We register with the event loop using a token.
        interest: Ready, //set of events we are interested in being notified of
        opts: PollOpt, // level or edge trigerring
    ) -> io::Result<()> {
        // Use raw file descriptor to trap events i guess?
        EventedFd(&self.socket.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.socket.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.socket.as_raw_fd()).deregister(poll)
    }
}
