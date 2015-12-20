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
  // proto:  void QOpenGLFunctions_1_1_CoreBackend::QOpenGLFunctions_1_1_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_1_CoreBackend13versionStatusEv();
}

// body block begin
// class sizeof(QOpenGLFunctions_1_1_CoreBackend)=1
pub struct QOpenGLFunctions_1_1_CoreBackend {
  pub qclsinst: *mut c_void,
}

  // proto:  void QOpenGLFunctions_1_1_CoreBackend::QOpenGLFunctions_1_1_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_1_CoreBackend {
  pub fn NewQOpenGLFunctions_1_1_CoreBackend<T: QOpenGLFunctions_1_1_CoreBackend_NewQOpenGLFunctions_1_1_CoreBackend>(value: T) -> QOpenGLFunctions_1_1_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_1_1_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_1_CoreBackend_NewQOpenGLFunctions_1_1_CoreBackend {
  fn NewQOpenGLFunctions_1_1_CoreBackend(self) -> QOpenGLFunctions_1_1_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_1_CoreBackend::QOpenGLFunctions_1_1_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_1_CoreBackend_NewQOpenGLFunctions_1_1_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_1_CoreBackend(self) -> QOpenGLFunctions_1_1_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_1_CoreBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_1_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_1_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_1_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_1_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_1_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_1_1_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

