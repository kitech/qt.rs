// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qopenglversionfunctions.h
// dst-file: /src/gui/qopenglversionfunctions.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::qopenglcontext::QOpenGLContext; // 773
// use super::qopenglversionfunctions::QOpenGLVersionStatus; // 773
// use super::qopenglversionfunctions::QOpenGLVersionFunctionsBackend; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto: static void QAbstractOpenGLFunctionsPrivate::insertFunctionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v, QOpenGLVersionFunctionsBackend * backend);
  fn _ZN31QAbstractOpenGLFunctionsPrivate22insertFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatusP30QOpenGLVersionFunctionsBackend(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto: static QOpenGLVersionFunctionsBackend * QAbstractOpenGLFunctionsPrivate::functionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v);
  fn _ZN31QAbstractOpenGLFunctionsPrivate16functionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QAbstractOpenGLFunctionsPrivate::removeFunctionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v);
  fn _ZN31QAbstractOpenGLFunctionsPrivate22removeFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAbstractOpenGLFunctionsPrivate::QAbstractOpenGLFunctionsPrivate();
  fn _ZN31QAbstractOpenGLFunctionsPrivateC1Ev(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions_4_5_DeprecatedBackend::QOpenGLFunctions_4_5_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_4_5_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_4_5_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_2_DeprecatedBackend::QOpenGLFunctions_1_2_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_2_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_1_CoreBackend::QOpenGLFunctions_4_1_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_4_1_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_1_CoreBackend13versionStatusEv();
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_3_3_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_3_3_CoreBackend::QOpenGLFunctions_3_3_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_3_3_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_5_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_5_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_5_CoreBackend::QOpenGLFunctions_1_5_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_1_5_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_5_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_5_CoreBackend::QOpenGLFunctions_4_5_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLFunctions_4_4_CoreBackend::QOpenGLFunctions_4_4_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_4_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_4_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_3_CoreBackend::QOpenGLFunctions_4_3_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_4_3_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_3_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_3_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_3_0_DeprecatedBackend::QOpenGLFunctions_3_0_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_3_0_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_2_1_CoreBackend::QOpenGLFunctions_2_1_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_2_1_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_2_1_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_0_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_3_0_CoreBackend::QOpenGLFunctions_3_0_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_3_0_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_3_0_CoreBackend13versionStatusEv();
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_2_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_2_CoreBackend::QOpenGLFunctions_1_2_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_1_2_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_1_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_1_DeprecatedBackend::QOpenGLFunctions_1_1_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_1_1_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_2_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_2_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_2_CoreBackend::QOpenGLFunctions_4_2_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_4_2_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_2_0_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_2_0_CoreBackend::QOpenGLFunctions_2_0_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLFunctions_3_2_CoreBackend::QOpenGLFunctions_3_2_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_3_2_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_2_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_3_2_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
  fn _ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
  fn _ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractOpenGLFunctions::QAbstractOpenGLFunctions();
  fn _ZN24QAbstractOpenGLFunctionsC1Ev(qthis: *mut c_void);
  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
  fn _ZN24QAbstractOpenGLFunctions6d_funcEv(qthis: *mut c_void);
  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
  fn _ZN24QAbstractOpenGLFunctionsD0Ev(qthis: *mut c_void);
  // proto:  void QOpenGLFunctions_2_0_DeprecatedBackend::QOpenGLFunctions_2_0_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_2_0_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_0_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_2_0_DeprecatedBackend13versionStatusEv();
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_3_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_3_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_3_DeprecatedBackend::QOpenGLFunctions_1_3_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_1_3_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLFunctions_1_4_DeprecatedBackend::QOpenGLFunctions_1_4_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_1_4_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_4_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_4_DeprecatedBackend13versionStatusEv();
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_3_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_3_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_3_CoreBackend::QOpenGLFunctions_1_3_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_1_3_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
  fn _ZN20QOpenGLVersionStatusC1Ev(qthis: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_0_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_0_CoreBackend::QOpenGLFunctions_1_0_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_1_0_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_3_1_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_3_1_CoreBackend::QOpenGLFunctions_3_1_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_3_1_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLFunctions_1_1_CoreBackend::QOpenGLFunctions_1_1_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_1_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_4_CoreBackend::QOpenGLFunctions_1_4_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_1_4_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_4_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_4_CoreBackend13versionStatusEv();
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_0_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_0_CoreBackend::QOpenGLFunctions_4_0_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_4_0_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_3_3_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_3_3_DeprecatedBackend::QOpenGLFunctions_3_3_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractOpenGLFunctionsPrivate)=16
pub struct QAbstractOpenGLFunctionsPrivate {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_4_5_DeprecatedBackend)=1
pub struct QOpenGLFunctions_4_5_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_2_DeprecatedBackend)=1
pub struct QOpenGLFunctions_1_2_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_4_1_CoreBackend)=1
pub struct QOpenGLFunctions_4_1_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_3_3_CoreBackend)=1
pub struct QOpenGLFunctions_3_3_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_5_CoreBackend)=1
pub struct QOpenGLFunctions_1_5_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_4_5_CoreBackend)=1
pub struct QOpenGLFunctions_4_5_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_4_4_CoreBackend)=1
pub struct QOpenGLFunctions_4_4_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_4_3_CoreBackend)=1
pub struct QOpenGLFunctions_4_3_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_3_0_DeprecatedBackend)=1
pub struct QOpenGLFunctions_3_0_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_2_1_CoreBackend)=1
pub struct QOpenGLFunctions_2_1_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_0_DeprecatedBackend)=1
pub struct QOpenGLFunctions_1_0_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_3_0_CoreBackend)=1
pub struct QOpenGLFunctions_3_0_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_2_CoreBackend)=1
pub struct QOpenGLFunctions_1_2_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_1_DeprecatedBackend)=1
pub struct QOpenGLFunctions_1_1_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_4_2_CoreBackend)=1
pub struct QOpenGLFunctions_4_2_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_2_0_CoreBackend)=1
pub struct QOpenGLFunctions_2_0_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_3_2_CoreBackend)=1
pub struct QOpenGLFunctions_3_2_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLVersionFunctionsBackend)=1
pub struct QOpenGLVersionFunctionsBackend {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAbstractOpenGLFunctions)=16
pub struct QAbstractOpenGLFunctions {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_2_0_DeprecatedBackend)=1
pub struct QOpenGLFunctions_2_0_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_3_DeprecatedBackend)=1
pub struct QOpenGLFunctions_1_3_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_4_DeprecatedBackend)=1
pub struct QOpenGLFunctions_1_4_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_3_CoreBackend)=1
pub struct QOpenGLFunctions_1_3_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLVersionStatus)=1
pub struct QOpenGLVersionStatus {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_0_CoreBackend)=1
pub struct QOpenGLFunctions_1_0_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_3_1_CoreBackend)=1
pub struct QOpenGLFunctions_3_1_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_1_CoreBackend)=1
pub struct QOpenGLFunctions_1_1_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_1_4_CoreBackend)=1
pub struct QOpenGLFunctions_1_4_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_4_0_CoreBackend)=1
pub struct QOpenGLFunctions_4_0_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLFunctions_3_3_DeprecatedBackend)=1
pub struct QOpenGLFunctions_3_3_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractOpenGLFunctionsPrivate {
    return QAbstractOpenGLFunctionsPrivate{qclsinst: qthis};
  }
}
  // proto: static void QAbstractOpenGLFunctionsPrivate::insertFunctionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v, QOpenGLVersionFunctionsBackend * backend);
impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn insertFunctionsBackend_s<RetType, T: QAbstractOpenGLFunctionsPrivate_insertFunctionsBackend_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.insertFunctionsBackend_s();
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctionsPrivate_insertFunctionsBackend_s<RetType> {
  fn insertFunctionsBackend_s(self ) -> RetType;
}

  // proto: static void QAbstractOpenGLFunctionsPrivate::insertFunctionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v, QOpenGLVersionFunctionsBackend * backend);
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_insertFunctionsBackend_s<()> for (QOpenGLContext, QOpenGLVersionStatus, QOpenGLVersionFunctionsBackend) {
  fn insertFunctionsBackend_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivate22insertFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatusP30QOpenGLVersionFunctionsBackend()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN31QAbstractOpenGLFunctionsPrivate22insertFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatusP30QOpenGLVersionFunctionsBackend(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto: static QOpenGLVersionFunctionsBackend * QAbstractOpenGLFunctionsPrivate::functionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v);
impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn functionsBackend_s<RetType, T: QAbstractOpenGLFunctionsPrivate_functionsBackend_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.functionsBackend_s();
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctionsPrivate_functionsBackend_s<RetType> {
  fn functionsBackend_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionFunctionsBackend * QAbstractOpenGLFunctionsPrivate::functionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v);
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_functionsBackend_s<()> for (QOpenGLContext, QOpenGLVersionStatus) {
  fn functionsBackend_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivate16functionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN31QAbstractOpenGLFunctionsPrivate16functionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QAbstractOpenGLFunctionsPrivate::removeFunctionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v);
impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn removeFunctionsBackend_s<RetType, T: QAbstractOpenGLFunctionsPrivate_removeFunctionsBackend_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeFunctionsBackend_s();
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctionsPrivate_removeFunctionsBackend_s<RetType> {
  fn removeFunctionsBackend_s(self ) -> RetType;
}

  // proto: static void QAbstractOpenGLFunctionsPrivate::removeFunctionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v);
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_removeFunctionsBackend_s<()> for (QOpenGLContext, QOpenGLVersionStatus) {
  fn removeFunctionsBackend_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivate22removeFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN31QAbstractOpenGLFunctionsPrivate22removeFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus(arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractOpenGLFunctionsPrivate::QAbstractOpenGLFunctionsPrivate();
impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn NewQAbstractOpenGLFunctionsPrivate<T: QAbstractOpenGLFunctionsPrivate_NewQAbstractOpenGLFunctionsPrivate>(value: T) -> QAbstractOpenGLFunctionsPrivate {
    let rsthis = value.NewQAbstractOpenGLFunctionsPrivate();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctionsPrivate_NewQAbstractOpenGLFunctionsPrivate {
  fn NewQAbstractOpenGLFunctionsPrivate(self) -> QAbstractOpenGLFunctionsPrivate;
}

  // proto:  void QAbstractOpenGLFunctionsPrivate::QAbstractOpenGLFunctionsPrivate();
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_NewQAbstractOpenGLFunctionsPrivate for () {
  fn NewQAbstractOpenGLFunctionsPrivate(self) -> QAbstractOpenGLFunctionsPrivate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivateC1Ev()};
    unsafe {_ZN31QAbstractOpenGLFunctionsPrivateC1Ev(qthis)};
    let rsthis = QAbstractOpenGLFunctionsPrivate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_5_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_4_5_DeprecatedBackend {
    return QOpenGLFunctions_4_5_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_4_5_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_5_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_4_5_DeprecatedBackend::QOpenGLFunctions_4_5_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_5_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_4_5_DeprecatedBackend<T: QOpenGLFunctions_4_5_DeprecatedBackend_NewQOpenGLFunctions_4_5_DeprecatedBackend>(value: T) -> QOpenGLFunctions_4_5_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_4_5_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_5_DeprecatedBackend_NewQOpenGLFunctions_4_5_DeprecatedBackend {
  fn NewQOpenGLFunctions_4_5_DeprecatedBackend(self) -> QOpenGLFunctions_4_5_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_4_5_DeprecatedBackend::QOpenGLFunctions_4_5_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_5_DeprecatedBackend_NewQOpenGLFunctions_4_5_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_4_5_DeprecatedBackend(self) -> QOpenGLFunctions_4_5_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_4_5_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_4_5_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_5_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_4_5_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_4_5_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_5_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_5_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_4_5_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_4_5_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_2_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_2_DeprecatedBackend {
    return QOpenGLFunctions_1_2_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_2_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_2_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_2_DeprecatedBackend::QOpenGLFunctions_1_2_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_2_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_1_2_DeprecatedBackend<T: QOpenGLFunctions_1_2_DeprecatedBackend_NewQOpenGLFunctions_1_2_DeprecatedBackend>(value: T) -> QOpenGLFunctions_1_2_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_1_2_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_2_DeprecatedBackend_NewQOpenGLFunctions_1_2_DeprecatedBackend {
  fn NewQOpenGLFunctions_1_2_DeprecatedBackend(self) -> QOpenGLFunctions_1_2_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_2_DeprecatedBackend::QOpenGLFunctions_1_2_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_2_DeprecatedBackend_NewQOpenGLFunctions_1_2_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_2_DeprecatedBackend(self) -> QOpenGLFunctions_1_2_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_2_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_2_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_2_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_2_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_2_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_2_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_1_2_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_1_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_4_1_CoreBackend {
    return QOpenGLFunctions_4_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_4_1_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_1_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_4_1_CoreBackend::QOpenGLFunctions_4_1_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_1_CoreBackend {
  pub fn NewQOpenGLFunctions_4_1_CoreBackend<T: QOpenGLFunctions_4_1_CoreBackend_NewQOpenGLFunctions_4_1_CoreBackend>(value: T) -> QOpenGLFunctions_4_1_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_4_1_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_1_CoreBackend_NewQOpenGLFunctions_4_1_CoreBackend {
  fn NewQOpenGLFunctions_4_1_CoreBackend(self) -> QOpenGLFunctions_4_1_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_1_CoreBackend::QOpenGLFunctions_4_1_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_1_CoreBackend_NewQOpenGLFunctions_4_1_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_4_1_CoreBackend(self) -> QOpenGLFunctions_4_1_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_1_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_4_1_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_1_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_1_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_4_1_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_4_1_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_1_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_1_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_1_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_1_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_4_1_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_3_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_3_3_CoreBackend {
    return QOpenGLFunctions_3_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_3_3_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_3_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_3_3_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_3_3_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_3_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_3_3_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_3_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_3_3_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_3_3_CoreBackend::QOpenGLFunctions_3_3_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_3_3_CoreBackend {
  pub fn NewQOpenGLFunctions_3_3_CoreBackend<T: QOpenGLFunctions_3_3_CoreBackend_NewQOpenGLFunctions_3_3_CoreBackend>(value: T) -> QOpenGLFunctions_3_3_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_3_3_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_3_CoreBackend_NewQOpenGLFunctions_3_3_CoreBackend {
  fn NewQOpenGLFunctions_3_3_CoreBackend(self) -> QOpenGLFunctions_3_3_CoreBackend;
}

  // proto:  void QOpenGLFunctions_3_3_CoreBackend::QOpenGLFunctions_3_3_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_3_CoreBackend_NewQOpenGLFunctions_3_3_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_3_3_CoreBackend(self) -> QOpenGLFunctions_3_3_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_3_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_3_3_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_3_3_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_5_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_5_CoreBackend {
    return QOpenGLFunctions_1_5_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_5_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_5_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_5_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_5_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_5_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_5_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_5_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_5_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_5_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_1_5_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_1_5_CoreBackend::QOpenGLFunctions_1_5_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_5_CoreBackend {
  pub fn NewQOpenGLFunctions_1_5_CoreBackend<T: QOpenGLFunctions_1_5_CoreBackend_NewQOpenGLFunctions_1_5_CoreBackend>(value: T) -> QOpenGLFunctions_1_5_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_1_5_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_5_CoreBackend_NewQOpenGLFunctions_1_5_CoreBackend {
  fn NewQOpenGLFunctions_1_5_CoreBackend(self) -> QOpenGLFunctions_1_5_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_5_CoreBackend::QOpenGLFunctions_1_5_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_5_CoreBackend_NewQOpenGLFunctions_1_5_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_5_CoreBackend(self) -> QOpenGLFunctions_1_5_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_5_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_1_5_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_5_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_5_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_4_5_CoreBackend {
    return QOpenGLFunctions_4_5_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_4_5_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_5_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_4_5_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_4_5_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_5_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_5_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_5_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_4_5_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_4_5_CoreBackend::QOpenGLFunctions_4_5_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_5_CoreBackend {
  pub fn NewQOpenGLFunctions_4_5_CoreBackend<T: QOpenGLFunctions_4_5_CoreBackend_NewQOpenGLFunctions_4_5_CoreBackend>(value: T) -> QOpenGLFunctions_4_5_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_4_5_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_5_CoreBackend_NewQOpenGLFunctions_4_5_CoreBackend {
  fn NewQOpenGLFunctions_4_5_CoreBackend(self) -> QOpenGLFunctions_4_5_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_5_CoreBackend::QOpenGLFunctions_4_5_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_5_CoreBackend_NewQOpenGLFunctions_4_5_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_4_5_CoreBackend(self) -> QOpenGLFunctions_4_5_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_5_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_4_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_4_4_CoreBackend {
    return QOpenGLFunctions_4_4_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_4_4_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_4_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_4_4_CoreBackend::QOpenGLFunctions_4_4_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_4_CoreBackend {
  pub fn NewQOpenGLFunctions_4_4_CoreBackend<T: QOpenGLFunctions_4_4_CoreBackend_NewQOpenGLFunctions_4_4_CoreBackend>(value: T) -> QOpenGLFunctions_4_4_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_4_4_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_4_CoreBackend_NewQOpenGLFunctions_4_4_CoreBackend {
  fn NewQOpenGLFunctions_4_4_CoreBackend(self) -> QOpenGLFunctions_4_4_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_4_CoreBackend::QOpenGLFunctions_4_4_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_4_CoreBackend_NewQOpenGLFunctions_4_4_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_4_4_CoreBackend(self) -> QOpenGLFunctions_4_4_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_4_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_4_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_4_4_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_4_4_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_4_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_4_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_4_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_4_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_4_4_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_3_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_4_3_CoreBackend {
    return QOpenGLFunctions_4_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_4_3_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_3_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_4_3_CoreBackend::QOpenGLFunctions_4_3_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_3_CoreBackend {
  pub fn NewQOpenGLFunctions_4_3_CoreBackend<T: QOpenGLFunctions_4_3_CoreBackend_NewQOpenGLFunctions_4_3_CoreBackend>(value: T) -> QOpenGLFunctions_4_3_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_4_3_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_3_CoreBackend_NewQOpenGLFunctions_4_3_CoreBackend {
  fn NewQOpenGLFunctions_4_3_CoreBackend(self) -> QOpenGLFunctions_4_3_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_3_CoreBackend::QOpenGLFunctions_4_3_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_3_CoreBackend_NewQOpenGLFunctions_4_3_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_4_3_CoreBackend(self) -> QOpenGLFunctions_4_3_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_3_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_4_3_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_3_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_3_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_4_3_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_4_3_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_3_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_3_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_3_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_3_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_4_3_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_0_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_3_0_DeprecatedBackend {
    return QOpenGLFunctions_3_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_3_0_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_0_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_3_0_DeprecatedBackend::QOpenGLFunctions_3_0_DeprecatedBackend(QOpenGLContext * context);
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

  // proto:  void QOpenGLFunctions_3_0_DeprecatedBackend::QOpenGLFunctions_3_0_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_0_DeprecatedBackend_NewQOpenGLFunctions_3_0_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_3_0_DeprecatedBackend(self) -> QOpenGLFunctions_3_0_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_3_0_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_3_0_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_3_0_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_0_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_3_0_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_0_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_3_0_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_2_1_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_2_1_CoreBackend {
    return QOpenGLFunctions_2_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_2_1_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_2_1_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_2_1_CoreBackend::QOpenGLFunctions_2_1_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_2_1_CoreBackend {
  pub fn NewQOpenGLFunctions_2_1_CoreBackend<T: QOpenGLFunctions_2_1_CoreBackend_NewQOpenGLFunctions_2_1_CoreBackend>(value: T) -> QOpenGLFunctions_2_1_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_2_1_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_1_CoreBackend_NewQOpenGLFunctions_2_1_CoreBackend {
  fn NewQOpenGLFunctions_2_1_CoreBackend(self) -> QOpenGLFunctions_2_1_CoreBackend;
}

  // proto:  void QOpenGLFunctions_2_1_CoreBackend::QOpenGLFunctions_2_1_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_2_1_CoreBackend_NewQOpenGLFunctions_2_1_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_2_1_CoreBackend(self) -> QOpenGLFunctions_2_1_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_2_1_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_2_1_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_2_1_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_1_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_2_1_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_2_1_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_1_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_1_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_2_1_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_2_1_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_2_1_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_0_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_0_DeprecatedBackend {
    return QOpenGLFunctions_1_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_0_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_0_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_0_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_1_0_DeprecatedBackend<T: QOpenGLFunctions_1_0_DeprecatedBackend_NewQOpenGLFunctions_1_0_DeprecatedBackend>(value: T) -> QOpenGLFunctions_1_0_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_1_0_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_0_DeprecatedBackend_NewQOpenGLFunctions_1_0_DeprecatedBackend {
  fn NewQOpenGLFunctions_1_0_DeprecatedBackend(self) -> QOpenGLFunctions_1_0_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_0_DeprecatedBackend_NewQOpenGLFunctions_1_0_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_0_DeprecatedBackend(self) -> QOpenGLFunctions_1_0_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_0_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_0_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_0_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_0_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_0_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_0_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_1_0_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_0_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_3_0_CoreBackend {
    return QOpenGLFunctions_3_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_3_0_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_0_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_3_0_CoreBackend::QOpenGLFunctions_3_0_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_3_0_CoreBackend {
  pub fn NewQOpenGLFunctions_3_0_CoreBackend<T: QOpenGLFunctions_3_0_CoreBackend_NewQOpenGLFunctions_3_0_CoreBackend>(value: T) -> QOpenGLFunctions_3_0_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_3_0_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_0_CoreBackend_NewQOpenGLFunctions_3_0_CoreBackend {
  fn NewQOpenGLFunctions_3_0_CoreBackend(self) -> QOpenGLFunctions_3_0_CoreBackend;
}

  // proto:  void QOpenGLFunctions_3_0_CoreBackend::QOpenGLFunctions_3_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_0_CoreBackend_NewQOpenGLFunctions_3_0_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_3_0_CoreBackend(self) -> QOpenGLFunctions_3_0_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_0_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_3_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_3_0_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_3_0_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_3_0_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_0_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_3_0_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_0_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_3_0_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_2_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_2_CoreBackend {
    return QOpenGLFunctions_1_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_2_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_2_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_2_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_2_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_2_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_2_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_2_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_1_2_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_1_2_CoreBackend::QOpenGLFunctions_1_2_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_2_CoreBackend {
  pub fn NewQOpenGLFunctions_1_2_CoreBackend<T: QOpenGLFunctions_1_2_CoreBackend_NewQOpenGLFunctions_1_2_CoreBackend>(value: T) -> QOpenGLFunctions_1_2_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_1_2_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_2_CoreBackend_NewQOpenGLFunctions_1_2_CoreBackend {
  fn NewQOpenGLFunctions_1_2_CoreBackend(self) -> QOpenGLFunctions_1_2_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_2_CoreBackend::QOpenGLFunctions_1_2_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_2_CoreBackend_NewQOpenGLFunctions_1_2_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_2_CoreBackend(self) -> QOpenGLFunctions_1_2_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_2_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_1_2_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_2_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_1_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_1_DeprecatedBackend {
    return QOpenGLFunctions_1_1_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_1_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_1_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_1_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_1_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_1_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_1_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_1_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_1_1_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_1_1_DeprecatedBackend::QOpenGLFunctions_1_1_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_1_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_1_1_DeprecatedBackend<T: QOpenGLFunctions_1_1_DeprecatedBackend_NewQOpenGLFunctions_1_1_DeprecatedBackend>(value: T) -> QOpenGLFunctions_1_1_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_1_1_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_1_DeprecatedBackend_NewQOpenGLFunctions_1_1_DeprecatedBackend {
  fn NewQOpenGLFunctions_1_1_DeprecatedBackend(self) -> QOpenGLFunctions_1_1_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_1_DeprecatedBackend::QOpenGLFunctions_1_1_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_1_DeprecatedBackend_NewQOpenGLFunctions_1_1_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_1_DeprecatedBackend(self) -> QOpenGLFunctions_1_1_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_1_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_1_1_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_1_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_2_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_4_2_CoreBackend {
    return QOpenGLFunctions_4_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_4_2_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_2_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_2_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_4_2_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_4_2_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_2_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_2_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_2_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_2_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_4_2_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_4_2_CoreBackend::QOpenGLFunctions_4_2_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_2_CoreBackend {
  pub fn NewQOpenGLFunctions_4_2_CoreBackend<T: QOpenGLFunctions_4_2_CoreBackend_NewQOpenGLFunctions_4_2_CoreBackend>(value: T) -> QOpenGLFunctions_4_2_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_4_2_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_2_CoreBackend_NewQOpenGLFunctions_4_2_CoreBackend {
  fn NewQOpenGLFunctions_4_2_CoreBackend(self) -> QOpenGLFunctions_4_2_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_2_CoreBackend::QOpenGLFunctions_4_2_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_2_CoreBackend_NewQOpenGLFunctions_4_2_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_4_2_CoreBackend(self) -> QOpenGLFunctions_4_2_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_2_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_4_2_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_2_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_2_0_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_2_0_CoreBackend {
    return QOpenGLFunctions_2_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_2_0_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_2_0_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_0_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_2_0_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_2_0_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_0_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_0_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_2_0_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_2_0_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_2_0_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_2_0_CoreBackend::QOpenGLFunctions_2_0_CoreBackend(QOpenGLContext * context);
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

  // proto:  void QOpenGLFunctions_2_0_CoreBackend::QOpenGLFunctions_2_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_2_0_CoreBackend_NewQOpenGLFunctions_2_0_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_2_0_CoreBackend(self) -> QOpenGLFunctions_2_0_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_2_0_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_2_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_3_2_CoreBackend {
    return QOpenGLFunctions_3_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_3_2_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_2_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_3_2_CoreBackend::QOpenGLFunctions_3_2_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_3_2_CoreBackend {
  pub fn NewQOpenGLFunctions_3_2_CoreBackend<T: QOpenGLFunctions_3_2_CoreBackend_NewQOpenGLFunctions_3_2_CoreBackend>(value: T) -> QOpenGLFunctions_3_2_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_3_2_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_2_CoreBackend_NewQOpenGLFunctions_3_2_CoreBackend {
  fn NewQOpenGLFunctions_3_2_CoreBackend(self) -> QOpenGLFunctions_3_2_CoreBackend;
}

  // proto:  void QOpenGLFunctions_3_2_CoreBackend::QOpenGLFunctions_3_2_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_2_CoreBackend_NewQOpenGLFunctions_3_2_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_3_2_CoreBackend(self) -> QOpenGLFunctions_3_2_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_2_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_3_2_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_3_2_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_2_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_3_2_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_3_2_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_2_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_2_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_3_2_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_2_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_3_2_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionFunctionsBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLVersionFunctionsBackend {
    return QOpenGLVersionFunctionsBackend{qclsinst: qthis};
  }
}
  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
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

  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
impl<'a> /*trait*/ QOpenGLVersionFunctionsBackend_NewQOpenGLVersionFunctionsBackend for (QOpenGLContext) {
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

impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractOpenGLFunctions {
    return QAbstractOpenGLFunctions{qclsinst: qthis};
  }
}
  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn initializeOpenGLFunctions<RetType, T: QAbstractOpenGLFunctions_initializeOpenGLFunctions<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.initializeOpenGLFunctions(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_initializeOpenGLFunctions<RetType> {
  fn initializeOpenGLFunctions(self , rsthis: &mut QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_initializeOpenGLFunctions<i8> for () {
  fn initializeOpenGLFunctions(self , rsthis: &mut QAbstractOpenGLFunctions) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv()};
    let mut ret = unsafe {_ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractOpenGLFunctions::QAbstractOpenGLFunctions();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn NewQAbstractOpenGLFunctions<T: QAbstractOpenGLFunctions_NewQAbstractOpenGLFunctions>(value: T) -> QAbstractOpenGLFunctions {
    let rsthis = value.NewQAbstractOpenGLFunctions();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_NewQAbstractOpenGLFunctions {
  fn NewQAbstractOpenGLFunctions(self) -> QAbstractOpenGLFunctions;
}

  // proto:  void QAbstractOpenGLFunctions::QAbstractOpenGLFunctions();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_NewQAbstractOpenGLFunctions for () {
  fn NewQAbstractOpenGLFunctions(self) -> QAbstractOpenGLFunctions {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctionsC1Ev()};
    unsafe {_ZN24QAbstractOpenGLFunctionsC1Ev(qthis)};
    let rsthis = QAbstractOpenGLFunctions{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn d_func<RetType, T: QAbstractOpenGLFunctions_d_func<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.d_func(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_d_func<RetType> {
  fn d_func(self , rsthis: &mut QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_d_func<()> for () {
  fn d_func(self , rsthis: &mut QAbstractOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctions6d_funcEv()};
     unsafe {_ZN24QAbstractOpenGLFunctions6d_funcEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn FreeQAbstractOpenGLFunctions<RetType, T: QAbstractOpenGLFunctions_FreeQAbstractOpenGLFunctions<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractOpenGLFunctions(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_FreeQAbstractOpenGLFunctions<RetType> {
  fn FreeQAbstractOpenGLFunctions(self , rsthis: &mut QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_FreeQAbstractOpenGLFunctions<()> for () {
  fn FreeQAbstractOpenGLFunctions(self , rsthis: &mut QAbstractOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctionsD0Ev()};
     unsafe {_ZN24QAbstractOpenGLFunctionsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_2_0_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_2_0_DeprecatedBackend {
    return QOpenGLFunctions_2_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_2_0_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_2_0_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_2_0_DeprecatedBackend::QOpenGLFunctions_2_0_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_2_0_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_2_0_DeprecatedBackend<T: QOpenGLFunctions_2_0_DeprecatedBackend_NewQOpenGLFunctions_2_0_DeprecatedBackend>(value: T) -> QOpenGLFunctions_2_0_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_2_0_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_0_DeprecatedBackend_NewQOpenGLFunctions_2_0_DeprecatedBackend {
  fn NewQOpenGLFunctions_2_0_DeprecatedBackend(self) -> QOpenGLFunctions_2_0_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_2_0_DeprecatedBackend::QOpenGLFunctions_2_0_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_2_0_DeprecatedBackend_NewQOpenGLFunctions_2_0_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_2_0_DeprecatedBackend(self) -> QOpenGLFunctions_2_0_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_2_0_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_2_0_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_2_0_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_0_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_2_0_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_2_0_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_0_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_0_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_2_0_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_2_0_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_2_0_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_3_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_3_DeprecatedBackend {
    return QOpenGLFunctions_1_3_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_3_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_3_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_3_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_3_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_3_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_3_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_3_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_3_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_3_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_1_3_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_1_3_DeprecatedBackend::QOpenGLFunctions_1_3_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_3_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_1_3_DeprecatedBackend<T: QOpenGLFunctions_1_3_DeprecatedBackend_NewQOpenGLFunctions_1_3_DeprecatedBackend>(value: T) -> QOpenGLFunctions_1_3_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_1_3_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_3_DeprecatedBackend_NewQOpenGLFunctions_1_3_DeprecatedBackend {
  fn NewQOpenGLFunctions_1_3_DeprecatedBackend(self) -> QOpenGLFunctions_1_3_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_3_DeprecatedBackend::QOpenGLFunctions_1_3_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_3_DeprecatedBackend_NewQOpenGLFunctions_1_3_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_3_DeprecatedBackend(self) -> QOpenGLFunctions_1_3_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_3_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_1_3_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_3_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_4_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_4_DeprecatedBackend {
    return QOpenGLFunctions_1_4_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_4_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_4_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_4_DeprecatedBackend::QOpenGLFunctions_1_4_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_4_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_1_4_DeprecatedBackend<T: QOpenGLFunctions_1_4_DeprecatedBackend_NewQOpenGLFunctions_1_4_DeprecatedBackend>(value: T) -> QOpenGLFunctions_1_4_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_1_4_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_4_DeprecatedBackend_NewQOpenGLFunctions_1_4_DeprecatedBackend {
  fn NewQOpenGLFunctions_1_4_DeprecatedBackend(self) -> QOpenGLFunctions_1_4_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_4_DeprecatedBackend::QOpenGLFunctions_1_4_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_4_DeprecatedBackend_NewQOpenGLFunctions_1_4_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_4_DeprecatedBackend(self) -> QOpenGLFunctions_1_4_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_4_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_1_4_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_4_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_4_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_4_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_4_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_4_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_4_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_4_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_4_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_1_4_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_3_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_3_CoreBackend {
    return QOpenGLFunctions_1_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_3_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_3_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_3_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_3_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_3_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_3_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_3_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_3_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_3_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_1_3_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_1_3_CoreBackend::QOpenGLFunctions_1_3_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_3_CoreBackend {
  pub fn NewQOpenGLFunctions_1_3_CoreBackend<T: QOpenGLFunctions_1_3_CoreBackend_NewQOpenGLFunctions_1_3_CoreBackend>(value: T) -> QOpenGLFunctions_1_3_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_1_3_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_3_CoreBackend_NewQOpenGLFunctions_1_3_CoreBackend {
  fn NewQOpenGLFunctions_1_3_CoreBackend(self) -> QOpenGLFunctions_1_3_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_3_CoreBackend::QOpenGLFunctions_1_3_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_3_CoreBackend_NewQOpenGLFunctions_1_3_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_3_CoreBackend(self) -> QOpenGLFunctions_1_3_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_3_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_1_3_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_3_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionStatus {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLVersionStatus {
    return QOpenGLVersionStatus{qclsinst: qthis};
  }
}
  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
impl /*struct*/ QOpenGLVersionStatus {
  pub fn NewQOpenGLVersionStatus<T: QOpenGLVersionStatus_NewQOpenGLVersionStatus>(value: T) -> QOpenGLVersionStatus {
    let rsthis = value.NewQOpenGLVersionStatus();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionStatus_NewQOpenGLVersionStatus {
  fn NewQOpenGLVersionStatus(self) -> QOpenGLVersionStatus;
}

  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
impl<'a> /*trait*/ QOpenGLVersionStatus_NewQOpenGLVersionStatus for () {
  fn NewQOpenGLVersionStatus(self) -> QOpenGLVersionStatus {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLVersionStatusC1Ev()};
    unsafe {_ZN20QOpenGLVersionStatusC1Ev(qthis)};
    let rsthis = QOpenGLVersionStatus{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_0_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_0_CoreBackend {
    return QOpenGLFunctions_1_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_0_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_0_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_0_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_0_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_0_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_0_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_0_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_1_0_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_1_0_CoreBackend::QOpenGLFunctions_1_0_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_0_CoreBackend {
  pub fn NewQOpenGLFunctions_1_0_CoreBackend<T: QOpenGLFunctions_1_0_CoreBackend_NewQOpenGLFunctions_1_0_CoreBackend>(value: T) -> QOpenGLFunctions_1_0_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_1_0_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_0_CoreBackend_NewQOpenGLFunctions_1_0_CoreBackend {
  fn NewQOpenGLFunctions_1_0_CoreBackend(self) -> QOpenGLFunctions_1_0_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_0_CoreBackend::QOpenGLFunctions_1_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_0_CoreBackend_NewQOpenGLFunctions_1_0_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_0_CoreBackend(self) -> QOpenGLFunctions_1_0_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_0_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_1_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_0_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_1_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_3_1_CoreBackend {
    return QOpenGLFunctions_3_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_3_1_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_1_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_1_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_3_1_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_3_1_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_1_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_1_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_3_1_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_1_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_3_1_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_3_1_CoreBackend::QOpenGLFunctions_3_1_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_3_1_CoreBackend {
  pub fn NewQOpenGLFunctions_3_1_CoreBackend<T: QOpenGLFunctions_3_1_CoreBackend_NewQOpenGLFunctions_3_1_CoreBackend>(value: T) -> QOpenGLFunctions_3_1_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_3_1_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_1_CoreBackend_NewQOpenGLFunctions_3_1_CoreBackend {
  fn NewQOpenGLFunctions_3_1_CoreBackend(self) -> QOpenGLFunctions_3_1_CoreBackend;
}

  // proto:  void QOpenGLFunctions_3_1_CoreBackend::QOpenGLFunctions_3_1_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_1_CoreBackend_NewQOpenGLFunctions_3_1_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_3_1_CoreBackend(self) -> QOpenGLFunctions_3_1_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_1_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_3_1_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_3_1_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_1_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_1_CoreBackend {
    return QOpenGLFunctions_1_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_1_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_1_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
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
    let rsthis = QOpenGLFunctions_1_1_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
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

impl /*struct*/ QOpenGLFunctions_1_4_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_1_4_CoreBackend {
    return QOpenGLFunctions_1_4_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_1_4_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_4_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_4_CoreBackend::QOpenGLFunctions_1_4_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_4_CoreBackend {
  pub fn NewQOpenGLFunctions_1_4_CoreBackend<T: QOpenGLFunctions_1_4_CoreBackend_NewQOpenGLFunctions_1_4_CoreBackend>(value: T) -> QOpenGLFunctions_1_4_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_1_4_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_4_CoreBackend_NewQOpenGLFunctions_1_4_CoreBackend {
  fn NewQOpenGLFunctions_1_4_CoreBackend(self) -> QOpenGLFunctions_1_4_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_4_CoreBackend::QOpenGLFunctions_1_4_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_4_CoreBackend_NewQOpenGLFunctions_1_4_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_4_CoreBackend(self) -> QOpenGLFunctions_1_4_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_4_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_1_4_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_4_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_4_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_4_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_4_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_4_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_4_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_4_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_4_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_1_4_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_0_CoreBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_4_0_CoreBackend {
    return QOpenGLFunctions_4_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_4_0_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_0_CoreBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_0_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_4_0_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_4_0_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_0_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_0_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_0_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_0_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_4_0_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_4_0_CoreBackend::QOpenGLFunctions_4_0_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_0_CoreBackend {
  pub fn NewQOpenGLFunctions_4_0_CoreBackend<T: QOpenGLFunctions_4_0_CoreBackend_NewQOpenGLFunctions_4_0_CoreBackend>(value: T) -> QOpenGLFunctions_4_0_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_4_0_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_0_CoreBackend_NewQOpenGLFunctions_4_0_CoreBackend {
  fn NewQOpenGLFunctions_4_0_CoreBackend(self) -> QOpenGLFunctions_4_0_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_0_CoreBackend::QOpenGLFunctions_4_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_0_CoreBackend_NewQOpenGLFunctions_4_0_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_4_0_CoreBackend(self) -> QOpenGLFunctions_4_0_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_0_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_4_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_0_CoreBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_3_DeprecatedBackend {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLFunctions_3_3_DeprecatedBackend {
    return QOpenGLFunctions_3_3_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLFunctions_3_3_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_3_DeprecatedBackend {
  fn as_ref(&self) -> &QOpenGLVersionFunctionsBackend {
    return &self.qbase;
  }
}
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_3_3_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_3_3_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_3_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_3_3_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_3_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_3_3_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_3_3_DeprecatedBackend::QOpenGLFunctions_3_3_DeprecatedBackend(QOpenGLContext * context);
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

  // proto:  void QOpenGLFunctions_3_3_DeprecatedBackend::QOpenGLFunctions_3_3_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_3_DeprecatedBackend_NewQOpenGLFunctions_3_3_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_3_3_DeprecatedBackend(self) -> QOpenGLFunctions_3_3_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_3_3_DeprecatedBackend{/**/qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

