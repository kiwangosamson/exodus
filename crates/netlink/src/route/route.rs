use crate::sys;

use std::io;


#[derive(Debug, Clone, Copy)]
pub struct Route {
    
}

pub struct Routes<'a, 'b> {
    pub(crate) socket: &'a mut sys::NetlinkSocket,
    pub(crate) buffer: &'b mut [u8],
}

impl<'a, 'b> Routes<'a, 'b> {

}

impl<'a, 'b> Iterator for Routes<'a, 'b> {
    type Item = Result<Route, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}