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
  // proto:  void QOpenGLFunctions_3_0_DeprecatedBackend::NewQOpenGLFunctions_3_0_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_3_0_DeprecatedBackend13versionStatusEv() ;
}

// body block begin
// class sizeof(QOpenGLFunctions_3_0_DeprecatedBackend)=1
pub struct QOpenGLFunctions_3_0_DeprecatedBackend {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFunctions_3_0_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_3_0_DeprecatedBackend<T: QOpenGLFunctions_3_0_DeprecatedBackend_NewQOpenGLFunctions_3_0_DeprecatedBackend>(value: T) -> QOpenGLFunctions_3_0_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_3_0_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_0_DeprecatedBackend_NewQOpenGLFunctions_3_0_DeprecatedBackend {
  fn NewQOpenGLFunctions_3_0_DeprecatedBackend(self) -> QOpenGLFunctions_3_0_DeprecatedBackend;
}

// proto: void QOpenGLFunctions_3_0_DeprecatedBackend::NewQOpenGLFunctions_3_0_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_0_DeprecatedBackend_NewQOpenGLFunctions_3_0_DeprecatedBackend for (&'a mut QOpenGLContext) {
  fn NewQOpenGLFunctions_3_0_DeprecatedBackend(self) -> QOpenGLFunctions_3_0_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_3_0_DeprecatedBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_0_DeprecatedBackend {
  pub fn versionStatus<T: QOpenGLFunctions_3_0_DeprecatedBackend_versionStatus>(&mut self, value: T)  {
     value.versionStatus(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_0_DeprecatedBackend_versionStatus {
  fn versionStatus(self, rsthis: &mut QOpenGLFunctions_3_0_DeprecatedBackend) ;
}

// proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_3_0_DeprecatedBackend_versionStatus for () {
  fn versionStatus(self, rsthis: &mut QOpenGLFunctions_3_0_DeprecatedBackend)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_0_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_3_0_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

