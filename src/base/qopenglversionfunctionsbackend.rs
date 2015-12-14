// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qopenglcontext::QOpenGLContext;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLVersionFunctionsBackend::NewQOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
  fn _ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLVersionFunctionsBackend)=1
pub struct QOpenGLVersionFunctionsBackend {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLVersionFunctionsBackend {
  pub fn NewQOpenGLVersionFunctionsBackend<T: QOpenGLVersionFunctionsBackend_NewQOpenGLVersionFunctionsBackend>(value: T) -> QOpenGLVersionFunctionsBackend {
    let rsthis = value.NewQOpenGLVersionFunctionsBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionFunctionsBackend_NewQOpenGLVersionFunctionsBackend {
  fn NewQOpenGLVersionFunctionsBackend(self) -> QOpenGLVersionFunctionsBackend;
}

// proto: void QOpenGLVersionFunctionsBackend::NewQOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
impl<'a> /*trait*/ QOpenGLVersionFunctionsBackend_NewQOpenGLVersionFunctionsBackend for (&'a mut QOpenGLContext) {
  fn NewQOpenGLVersionFunctionsBackend(self) -> QOpenGLVersionFunctionsBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLVersionFunctionsBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

