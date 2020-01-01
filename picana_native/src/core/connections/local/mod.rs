use mio::{event::Source, unix::SourceFd, Interest, Registry, Token};
use socketcan::{CANFrame, CANSocket};
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd};

//All this is because mio doesn't have an adapter for cansockets
// Wrapper type arround CANSocket for eventing!
pub struct MIOCANSocket {
    socket: CANSocket,
}

//Builder methods and Convenience methods
#[allow(dead_code)]
impl MIOCANSocket {
    pub fn from(socket: CANSocket) -> MIOCANSocket {
        MIOCANSocket { socket: socket }
    }

    pub fn read_frame(&self) -> io::Result<CANFrame> {
        self.socket.read_frame()
    }

    pub fn write_frame(&self, frame: CANFrame) -> io::Result<()> {
        self.socket.write_frame(&frame)
    }

    pub fn write_frame_insist(&self, frame: CANFrame) -> io::Result<()> {
        self.socket.write_frame_insist(&frame)
    }
}

// Allow MIOCANSocket to be polled by mio
// The EventLoop object provided by mio is our main point of contact.
// Interaction with the event loop is in the form of the register, register_opt, reregister and deregister functions.
// These functions allow our code to control how the event loop interacts with the incoming client connections.
impl Source for MIOCANSocket {
    fn register(
        &mut self,
        registry: &Registry, // Types of readiness
        token: Token, // A Token is used to identify the state related to a connected socket. We register with the event loop using a token.
        interest: Interest, //set of events we are interested in being notified of
    ) -> io::Result<()> {
        // Use raw file descriptor to trap events i guess?
        SourceFd(&self.socket.as_raw_fd()).register(registry, token, interest)
    }

    fn reregister(
        &mut self,
        registry: &Registry, // Types of readiness
        token: Token,
        interest: Interest,
        //opts: PollOpt,
    ) -> io::Result<()> {
        SourceFd(&self.socket.as_raw_fd()).reregister(registry, token, interest)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        SourceFd(&self.socket.as_raw_fd()).deregister(registry)
    }
}

//Same notes from https://github.com/oefd/tokio-socketcan/blob/master/src/lib.rs
impl Clone for MIOCANSocket {
    /// Clone the CANSocket by using the `dup` syscall to get another
    /// file descriptor. This method makes clones fairly cheap and
    /// avoids complexity around ownership
    fn clone(&self) -> Self {
        let fd = self.socket.as_raw_fd();
        unsafe {
            // essentially we're cheating and making it cheaper/easier
            // to manage multiple references to the socket by relying
            // on the posix behaviour of `dup()` which essentially lets
            // the kernel worry about keeping track of references;
            // as long as one of the duplicated file descriptors is open
            // the socket as a whole isn't going to be closed.
            let new_fd = libc::dup(fd);
            let new = CANSocket::from_raw_fd(new_fd);
            MIOCANSocket::from(new)
        }
    }
}
