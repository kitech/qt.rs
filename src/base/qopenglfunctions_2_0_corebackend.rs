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
  // proto: QOpenGLVersionStatus QOpenGLFunctions_2_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_2_0_CoreBackend13versionStatusEv() -> i32;
  // proto: void QOpenGLFunctions_2_0_CoreBackend::NewQOpenGLFunctions_2_0_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QOpenGLFunctions_2_0_CoreBackend)=1
pub struct QOpenGLFunctions_2_0_CoreBackend {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFunctions_2_0_CoreBackend {
  pub fn versionStatus<T: QOpenGLFunctions_2_0_CoreBackend_versionStatus>(&mut self, value: T) -> i32 {
    value.versionStatus(self);
    return 1;
  }
}

pub trait QOpenGLFunctions_2_0_CoreBackend_versionStatus {
  fn versionStatus(self, this: &mut QOpenGLFunctions_2_0_CoreBackend) -> i32;
}

// proto: QOpenGLVersionStatus QOpenGLFunctions_2_0_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_2_0_CoreBackend_versionStatus for () {
  fn versionStatus(self, this: &mut QOpenGLFunctions_2_0_CoreBackend) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_2_0_CoreBackend13versionStatusEv()};
    unsafe {_ZN32QOpenGLFunctions_2_0_CoreBackend13versionStatusEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_2_0_CoreBackend {
  pub fn NewQOpenGLFunctions_2_0_CoreBackend<T: QOpenGLFunctions_2_0_CoreBackend_NewQOpenGLFunctions_2_0_CoreBackend>(value: T) -> QOpenGLFunctions_2_0_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_2_0_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_0_CoreBackend_NewQOpenGLFunctions_2_0_CoreBackend {
  fn NewQOpenGLFunctions_2_0_CoreBackend(self) -> QOpenGLFunctions_2_0_CoreBackend;
}

// proto: void QOpenGLFunctions_2_0_CoreBackend::NewQOpenGLFunctions_2_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_2_0_CoreBackend_NewQOpenGLFunctions_2_0_CoreBackend for (&'a mut QOpenGLContext) {
  fn NewQOpenGLFunctions_2_0_CoreBackend(self) -> QOpenGLFunctions_2_0_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_2_0_CoreBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

