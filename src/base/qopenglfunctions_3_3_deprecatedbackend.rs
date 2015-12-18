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
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_3_3_DeprecatedBackend13versionStatusEv() ;
  // proto:  void QOpenGLFunctions_3_3_DeprecatedBackend::NewQOpenGLFunctions_3_3_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLFunctions_3_3_DeprecatedBackend)=1
pub struct QOpenGLFunctions_3_3_DeprecatedBackend {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFunctions_3_3_DeprecatedBackend {
  pub fn versionStatus<RetType, T: QOpenGLFunctions_3_3_DeprecatedBackend_versionStatus<RetType>>(&mut self, value: T) -> RetType {
    return value.versionStatus(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_3_DeprecatedBackend_versionStatus<RetType> {
  fn versionStatus(self, rsthis: &mut QOpenGLFunctions_3_3_DeprecatedBackend) -> RetType;
}

// proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_3_3_DeprecatedBackend_versionStatus<()> for () {
  fn versionStatus(self, rsthis: &mut QOpenGLFunctions_3_3_DeprecatedBackend) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_3_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_3_3_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_3_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_3_3_DeprecatedBackend<T: QOpenGLFunctions_3_3_DeprecatedBackend_NewQOpenGLFunctions_3_3_DeprecatedBackend>(value: T) -> QOpenGLFunctions_3_3_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_3_3_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_3_DeprecatedBackend_NewQOpenGLFunctions_3_3_DeprecatedBackend {
  fn NewQOpenGLFunctions_3_3_DeprecatedBackend(self) -> QOpenGLFunctions_3_3_DeprecatedBackend;
}

// proto: void QOpenGLFunctions_3_3_DeprecatedBackend::NewQOpenGLFunctions_3_3_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_3_DeprecatedBackend_NewQOpenGLFunctions_3_3_DeprecatedBackend for (&'a mut QOpenGLContext) {
  fn NewQOpenGLFunctions_3_3_DeprecatedBackend(self) -> QOpenGLFunctions_3_3_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_3_3_DeprecatedBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

