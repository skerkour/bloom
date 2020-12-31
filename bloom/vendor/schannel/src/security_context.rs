use winapi::shared::{sspi, winerror};
use winapi::shared::minwindef::ULONG;
use winapi::um::{minschannel, schannel};
use std::mem;
use std::ptr;
use std::io;

use crate::{INIT_REQUESTS, Inner, secbuf, secbuf_desc};
use crate::alpn_list::AlpnList;
use crate::cert_context::CertContext;
use crate::context_buffer::ContextBuffer;

use crate::schannel_cred::SchannelCred;

pub struct SecurityContext(sspi::CtxtHandle);

impl Drop for SecurityContext {
    fn drop(&mut self) {
        unsafe {
            sspi::DeleteSecurityContext(&mut self.0);
        }
    }
}

impl Inner<sspi::CtxtHandle> for SecurityContext {
    unsafe fn from_inner(inner: sspi::CtxtHandle) -> SecurityContext {
        SecurityContext(inner)
    }

    fn as_inner(&self) -> sspi::CtxtHandle {
        self.0
    }

    fn get_mut(&mut self) -> &mut sspi::CtxtHandle {
        &mut self.0
    }
}

impl SecurityContext {
    pub fn initialize(cred: &mut SchannelCred,
                      accept: bool,
                      domain: Option<&[u16]>,
                      requested_application_protocols: &Option<Vec<Vec<u8>>>)
                      -> io::Result<(SecurityContext, Option<ContextBuffer>)> {
        unsafe {
            let mut ctxt = mem::zeroed();

            if accept {
                // If we're performing an accept then we need to wait to call
                // `AcceptSecurityContext` until we've actually read some data.
                return Ok((SecurityContext(ctxt), None))
            }

            let domain = domain.map(|b| b.as_ptr() as *mut u16).unwrap_or(ptr::null_mut());

            let mut inbufs = vec![];

            // Make sure `AlpnList` is kept alive for the duration of this function.
            let mut alpns = requested_application_protocols.as_ref().map(|alpn| AlpnList::new(&alpn));
            if let Some(ref mut alpns) = alpns {
                inbufs.push(secbuf(sspi::SECBUFFER_APPLICATION_PROTOCOLS,
                                   Some(&mut alpns[..])));
            };

            let mut inbuf_desc = secbuf_desc(&mut inbufs[..]);

            let mut outbuf = [secbuf(sspi::SECBUFFER_EMPTY, None)];
            let mut outbuf_desc = secbuf_desc(&mut outbuf);

            let mut attributes = 0;

            match sspi::InitializeSecurityContextW(&mut cred.as_inner(),
                                                   ptr::null_mut(),
                                                   domain,
                                                   INIT_REQUESTS,
                                                   0,
                                                   0,
                                                   &mut inbuf_desc,
                                                   0,
                                                   &mut ctxt,
                                                   &mut outbuf_desc,
                                                   &mut attributes,
                                                   ptr::null_mut()) {
                winerror::SEC_I_CONTINUE_NEEDED => {
                    Ok((SecurityContext(ctxt), Some(ContextBuffer(outbuf[0]))))
                }
                err => {
                    Err(io::Error::from_raw_os_error(err as i32))
                }
            }
        }
    }

    unsafe fn attribute<T>(&self, attr: ULONG) -> io::Result<T> {
        let mut value = std::mem::zeroed();
        let status = sspi::QueryContextAttributesW(&self.0 as *const _ as *mut _,
                                                   attr,
                                                   &mut value as *mut _ as *mut _);
        if status == winerror::SEC_E_OK {
            Ok(value)
        } else {
            Err(io::Error::from_raw_os_error(status as i32))
        }
    }

    pub fn application_protocol(&self) -> io::Result<sspi::SecPkgContext_ApplicationProtocol> {
        unsafe {
            self.attribute(sspi::SECPKG_ATTR_APPLICATION_PROTOCOL)
        }
    }

    pub fn session_info(&self) -> io::Result<schannel::SecPkgContext_SessionInfo> {
        unsafe {
            self.attribute(minschannel::SECPKG_ATTR_SESSION_INFO)
        }
    }

    pub fn stream_sizes(&self) -> io::Result<sspi::SecPkgContext_StreamSizes> {
        unsafe {
            self.attribute(sspi::SECPKG_ATTR_STREAM_SIZES)
        }
    }

    pub fn remote_cert(&self) -> io::Result<CertContext> {
        unsafe {
            self.attribute(minschannel::SECPKG_ATTR_REMOTE_CERT_CONTEXT)
                .map(|p| CertContext::from_inner(p))
        }
    }

    pub fn local_cert(&self) -> io::Result<CertContext> {
        unsafe {
            self.attribute(minschannel::SECPKG_ATTR_LOCAL_CERT_CONTEXT)
                .map(|p| CertContext::from_inner(p))
        }
    }
}
