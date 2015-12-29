// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
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
// use super::qopenglversionfunctions::QAbstractOpenGLFunctions; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractOpenGLFunctionsPrivate_Class_Size() -> c_int;
  // proto: static void QAbstractOpenGLFunctionsPrivate::insertFunctionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v, QOpenGLVersionFunctionsBackend * backend);
  fn _ZN31QAbstractOpenGLFunctionsPrivate22insertFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatusP30QOpenGLVersionFunctionsBackend(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto: static void QAbstractOpenGLFunctionsPrivate::insertExternalFunctions(QOpenGLContext * context, QAbstractOpenGLFunctions * f);
  fn _ZN31QAbstractOpenGLFunctionsPrivate23insertExternalFunctionsEP14QOpenGLContextP24QAbstractOpenGLFunctions(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctionsPrivate::get(QAbstractOpenGLFunctions * q);
  fn _ZN31QAbstractOpenGLFunctionsPrivate3getEP24QAbstractOpenGLFunctions(arg0: *mut c_void);
  // proto: static QOpenGLVersionFunctionsBackend * QAbstractOpenGLFunctionsPrivate::functionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v);
  fn _ZN31QAbstractOpenGLFunctionsPrivate16functionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QAbstractOpenGLFunctionsPrivate::removeFunctionsBackend(QOpenGLContext * context, const QOpenGLVersionStatus & v);
  fn _ZN31QAbstractOpenGLFunctionsPrivate22removeFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QAbstractOpenGLFunctionsPrivate::removeExternalFunctions(QOpenGLContext * context, QAbstractOpenGLFunctions * f);
  fn _ZN31QAbstractOpenGLFunctionsPrivate23removeExternalFunctionsEP14QOpenGLContextP24QAbstractOpenGLFunctions(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAbstractOpenGLFunctionsPrivate::QAbstractOpenGLFunctionsPrivate();
  fn dector_ZN31QAbstractOpenGLFunctionsPrivateC1Ev() -> *mut c_void;
  fn _ZN31QAbstractOpenGLFunctionsPrivateC1Ev(qthis: u64 /* *mut c_void*/);
  fn QOpenGLFunctions_4_5_DeprecatedBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_4_5_DeprecatedBackend::QOpenGLFunctions_4_5_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_4_5_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_4_5_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_4_5_DeprecatedBackend13versionStatusEv();
  fn QOpenGLFunctions_1_2_DeprecatedBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_1_2_DeprecatedBackend::QOpenGLFunctions_1_2_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_2_DeprecatedBackend13versionStatusEv();
  fn QOpenGLFunctions_4_1_CoreBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_4_1_CoreBackend::QOpenGLFunctions_4_1_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_4_1_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_4_1_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_1_CoreBackend13versionStatusEv();
  fn QOpenGLFunctions_3_3_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_3_3_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_3_3_CoreBackend::QOpenGLFunctions_3_3_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_3_3_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_3_3_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_1_5_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_5_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_5_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_5_CoreBackend::QOpenGLFunctions_1_5_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_1_5_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_1_5_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_4_5_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_5_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_5_CoreBackend::QOpenGLFunctions_4_5_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_4_4_CoreBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_4_4_CoreBackend::QOpenGLFunctions_4_4_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_4_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_4_CoreBackend13versionStatusEv();
  fn QOpenGLFunctions_4_3_CoreBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_4_3_CoreBackend::QOpenGLFunctions_4_3_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_4_3_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_4_3_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_3_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_3_CoreBackend13versionStatusEv();
  fn QOpenGLFunctions_3_0_DeprecatedBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_3_0_DeprecatedBackend::QOpenGLFunctions_3_0_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_3_0_DeprecatedBackend13versionStatusEv();
  fn QOpenGLFunctions_2_1_CoreBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_2_1_CoreBackend::QOpenGLFunctions_2_1_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_2_1_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_2_1_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_2_1_CoreBackend13versionStatusEv();
  fn QOpenGLFunctions_1_0_DeprecatedBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_0_DeprecatedBackend13versionStatusEv();
  fn QOpenGLFunctions_3_0_CoreBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_3_0_CoreBackend::QOpenGLFunctions_3_0_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_3_0_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_3_0_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_3_0_CoreBackend13versionStatusEv();
  fn QOpenGLFunctions_1_2_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_2_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_2_CoreBackend::QOpenGLFunctions_1_2_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_1_2_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_1_2_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_1_1_DeprecatedBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_1_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_1_DeprecatedBackend::QOpenGLFunctions_1_1_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_1_1_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_1_1_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_4_2_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_2_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_2_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_2_CoreBackend::QOpenGLFunctions_4_2_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_4_2_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_4_2_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_2_0_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_2_0_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_2_0_CoreBackend::QOpenGLFunctions_2_0_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_3_2_CoreBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_3_2_CoreBackend::QOpenGLFunctions_3_2_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_3_2_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_3_2_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_2_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_3_2_CoreBackend13versionStatusEv();
  fn QOpenGLVersionFunctionsBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
  fn dector_ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QAbstractOpenGLFunctions_Class_Size() -> c_int;
  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
  fn _ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractOpenGLFunctions::QAbstractOpenGLFunctions();
  fn dector_ZN24QAbstractOpenGLFunctionsC1Ev() -> *mut c_void;
  fn _ZN24QAbstractOpenGLFunctionsC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
  fn _ZN24QAbstractOpenGLFunctions6d_funcEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
  fn _ZN24QAbstractOpenGLFunctionsD0Ev(qthis: u64 /* *mut c_void*/);
  fn QOpenGLFunctions_2_0_DeprecatedBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_2_0_DeprecatedBackend::QOpenGLFunctions_2_0_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_2_0_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_2_0_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_2_0_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_2_0_DeprecatedBackend13versionStatusEv();
  fn QOpenGLFunctions_1_3_DeprecatedBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_3_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_3_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_3_DeprecatedBackend::QOpenGLFunctions_1_3_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_1_3_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_1_3_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_1_4_DeprecatedBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_1_4_DeprecatedBackend::QOpenGLFunctions_1_4_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_1_4_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_1_4_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_4_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_4_DeprecatedBackend13versionStatusEv();
  fn QOpenGLFunctions_1_3_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_3_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_3_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_3_CoreBackend::QOpenGLFunctions_1_3_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_1_3_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_1_3_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLVersionStatus_Class_Size() -> c_int;
  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
  fn dector_ZN20QOpenGLVersionStatusC1Ev() -> *mut c_void;
  fn _ZN20QOpenGLVersionStatusC1Ev(qthis: u64 /* *mut c_void*/);
  fn QOpenGLFunctions_1_0_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_0_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_1_0_CoreBackend::QOpenGLFunctions_1_0_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_1_0_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_1_0_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_3_1_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_3_1_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_3_1_CoreBackend::QOpenGLFunctions_3_1_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_3_1_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_3_1_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_1_1_CoreBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_1_1_CoreBackend::QOpenGLFunctions_1_1_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_1_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_1_CoreBackend13versionStatusEv();
  fn QOpenGLFunctions_1_4_CoreBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLFunctions_1_4_CoreBackend::QOpenGLFunctions_1_4_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_1_4_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_1_4_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_4_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_1_4_CoreBackend13versionStatusEv();
  fn QOpenGLFunctions_4_0_CoreBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_0_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_0_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_0_CoreBackend::QOpenGLFunctions_4_0_CoreBackend(QOpenGLContext * context);
  fn dector_ZN32QOpenGLFunctions_4_0_CoreBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN32QOpenGLFunctions_4_0_CoreBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLFunctions_3_3_DeprecatedBackend_Class_Size() -> c_int;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_3_3_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_3_3_DeprecatedBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_3_3_DeprecatedBackend::QOpenGLFunctions_3_3_DeprecatedBackend(QOpenGLContext * context);
  fn dector_ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext(arg0: *mut c_void) -> *mut c_void;
  fn _ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractOpenGLFunctionsPrivate)=16
#[derive(Default)]
pub struct QAbstractOpenGLFunctionsPrivate {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_4_5_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_4_5_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_2_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_2_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_4_1_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_4_1_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_3_3_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_3_3_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_5_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_5_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_4_5_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_4_5_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_4_4_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_4_4_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_4_3_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_4_3_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_3_0_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_3_0_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_2_1_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_2_1_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_0_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_0_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_3_0_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_3_0_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_2_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_2_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_1_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_1_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_4_2_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_4_2_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_2_0_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_2_0_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_3_2_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_3_2_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLVersionFunctionsBackend)=1
#[derive(Default)]
pub struct QOpenGLVersionFunctionsBackend {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractOpenGLFunctions)=16
#[derive(Default)]
pub struct QAbstractOpenGLFunctions {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_2_0_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_2_0_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_3_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_3_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_4_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_4_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_3_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_3_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLVersionStatus)=1
#[derive(Default)]
pub struct QOpenGLVersionStatus {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_0_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_0_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_3_1_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_3_1_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_1_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_1_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_1_4_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_1_4_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_4_0_CoreBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_4_0_CoreBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFunctions_3_3_DeprecatedBackend)=1
#[derive(Default)]
pub struct QOpenGLFunctions_3_3_DeprecatedBackend {
  qbase: QOpenGLVersionFunctionsBackend,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractOpenGLFunctionsPrivate {
    return QAbstractOpenGLFunctionsPrivate{qclsinst: qthis, ..Default::default()};
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
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_insertFunctionsBackend_s<()> for (&'a QOpenGLContext, &'a QOpenGLVersionStatus, &'a QOpenGLVersionFunctionsBackend) {
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

  // proto: static void QAbstractOpenGLFunctionsPrivate::insertExternalFunctions(QOpenGLContext * context, QAbstractOpenGLFunctions * f);
impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn insertExternalFunctions_s<RetType, T: QAbstractOpenGLFunctionsPrivate_insertExternalFunctions_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.insertExternalFunctions_s();
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctionsPrivate_insertExternalFunctions_s<RetType> {
  fn insertExternalFunctions_s(self ) -> RetType;
}

  // proto: static void QAbstractOpenGLFunctionsPrivate::insertExternalFunctions(QOpenGLContext * context, QAbstractOpenGLFunctions * f);
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_insertExternalFunctions_s<()> for (&'a QOpenGLContext, &'a QAbstractOpenGLFunctions) {
  fn insertExternalFunctions_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivate23insertExternalFunctionsEP14QOpenGLContextP24QAbstractOpenGLFunctions()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN31QAbstractOpenGLFunctionsPrivate23insertExternalFunctionsEP14QOpenGLContextP24QAbstractOpenGLFunctions(arg0, arg1)};
    // return 1;
  }
}

  // proto: static QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctionsPrivate::get(QAbstractOpenGLFunctions * q);
impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn get_s<RetType, T: QAbstractOpenGLFunctionsPrivate_get_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.get_s();
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctionsPrivate_get_s<RetType> {
  fn get_s(self ) -> RetType;
}

  // proto: static QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctionsPrivate::get(QAbstractOpenGLFunctions * q);
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_get_s<()> for (&'a QAbstractOpenGLFunctions) {
  fn get_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivate3getEP24QAbstractOpenGLFunctions()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN31QAbstractOpenGLFunctionsPrivate3getEP24QAbstractOpenGLFunctions(arg0)};
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
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_functionsBackend_s<()> for (&'a QOpenGLContext, &'a QOpenGLVersionStatus) {
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
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_removeFunctionsBackend_s<()> for (&'a QOpenGLContext, &'a QOpenGLVersionStatus) {
  fn removeFunctionsBackend_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivate22removeFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN31QAbstractOpenGLFunctionsPrivate22removeFunctionsBackendEP14QOpenGLContextRK20QOpenGLVersionStatus(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QAbstractOpenGLFunctionsPrivate::removeExternalFunctions(QOpenGLContext * context, QAbstractOpenGLFunctions * f);
impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn removeExternalFunctions_s<RetType, T: QAbstractOpenGLFunctionsPrivate_removeExternalFunctions_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeExternalFunctions_s();
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctionsPrivate_removeExternalFunctions_s<RetType> {
  fn removeExternalFunctions_s(self ) -> RetType;
}

  // proto: static void QAbstractOpenGLFunctionsPrivate::removeExternalFunctions(QOpenGLContext * context, QAbstractOpenGLFunctions * f);
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_removeExternalFunctions_s<()> for (&'a QOpenGLContext, &'a QAbstractOpenGLFunctions) {
  fn removeExternalFunctions_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivate23removeExternalFunctionsEP14QOpenGLContextP24QAbstractOpenGLFunctions()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN31QAbstractOpenGLFunctionsPrivate23removeExternalFunctionsEP14QOpenGLContextP24QAbstractOpenGLFunctions(arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractOpenGLFunctionsPrivate::QAbstractOpenGLFunctionsPrivate();
impl /*struct*/ QAbstractOpenGLFunctionsPrivate {
  pub fn New<T: QAbstractOpenGLFunctionsPrivate_New>(value: T) -> QAbstractOpenGLFunctionsPrivate {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctionsPrivate_New {
  fn New(self) -> QAbstractOpenGLFunctionsPrivate;
}

  // proto:  void QAbstractOpenGLFunctionsPrivate::QAbstractOpenGLFunctionsPrivate();
impl<'a> /*trait*/ QAbstractOpenGLFunctionsPrivate_New for () {
  fn New(self) -> QAbstractOpenGLFunctionsPrivate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QAbstractOpenGLFunctionsPrivateC1Ev()};
    let ctysz: c_int = unsafe{QAbstractOpenGLFunctionsPrivate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN31QAbstractOpenGLFunctionsPrivateC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN31QAbstractOpenGLFunctionsPrivateC1Ev()} as u64;
    let rsthis = QAbstractOpenGLFunctionsPrivate{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_5_DeprecatedBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_4_5_DeprecatedBackend {
    return QOpenGLFunctions_4_5_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_4_5_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_5_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_4_5_DeprecatedBackend::QOpenGLFunctions_4_5_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_5_DeprecatedBackend {
  pub fn New<T: QOpenGLFunctions_4_5_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_4_5_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_5_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_4_5_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_4_5_DeprecatedBackend::QOpenGLFunctions_4_5_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_5_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_4_5_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_4_5_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_4_5_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_4_5_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_4_5_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_4_5_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_2_DeprecatedBackend {
    return QOpenGLFunctions_1_2_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_2_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_2_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_2_DeprecatedBackend::QOpenGLFunctions_1_2_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_2_DeprecatedBackend {
  pub fn New<T: QOpenGLFunctions_1_2_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_1_2_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_2_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_1_2_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_2_DeprecatedBackend::QOpenGLFunctions_1_2_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_2_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_2_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_2_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_2_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_4_1_CoreBackend {
    return QOpenGLFunctions_4_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_4_1_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_1_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_4_1_CoreBackend::QOpenGLFunctions_4_1_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_1_CoreBackend {
  pub fn New<T: QOpenGLFunctions_4_1_CoreBackend_New>(value: T) -> QOpenGLFunctions_4_1_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_1_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_4_1_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_1_CoreBackend::QOpenGLFunctions_4_1_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_1_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_4_1_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_1_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_4_1_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_4_1_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_4_1_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_4_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_3_3_CoreBackend {
    return QOpenGLFunctions_3_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_3_3_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_3_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_3_3_CoreBackend_New>(value: T) -> QOpenGLFunctions_3_3_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_3_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_3_3_CoreBackend;
}

  // proto:  void QOpenGLFunctions_3_3_CoreBackend::QOpenGLFunctions_3_3_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_3_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_3_3_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_3_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_3_3_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_3_3_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_3_3_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_3_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_5_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_5_CoreBackend {
    return QOpenGLFunctions_1_5_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_5_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_5_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_1_5_CoreBackend_New>(value: T) -> QOpenGLFunctions_1_5_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_5_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_1_5_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_5_CoreBackend::QOpenGLFunctions_1_5_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_5_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_5_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_5_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_5_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_1_5_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_1_5_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_5_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_5_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_4_5_CoreBackend {
    return QOpenGLFunctions_4_5_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_4_5_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_5_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_4_5_CoreBackend_New>(value: T) -> QOpenGLFunctions_4_5_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_5_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_4_5_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_5_CoreBackend::QOpenGLFunctions_4_5_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_5_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_4_5_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_4_5_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_4_5_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_4_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_4_4_CoreBackend {
    return QOpenGLFunctions_4_4_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_4_4_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_4_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_4_4_CoreBackend::QOpenGLFunctions_4_4_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_4_CoreBackend {
  pub fn New<T: QOpenGLFunctions_4_4_CoreBackend_New>(value: T) -> QOpenGLFunctions_4_4_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_4_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_4_4_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_4_CoreBackend::QOpenGLFunctions_4_4_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_4_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_4_4_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_4_4_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_4_4_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_4_3_CoreBackend {
    return QOpenGLFunctions_4_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_4_3_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_3_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_4_3_CoreBackend::QOpenGLFunctions_4_3_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_3_CoreBackend {
  pub fn New<T: QOpenGLFunctions_4_3_CoreBackend_New>(value: T) -> QOpenGLFunctions_4_3_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_3_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_4_3_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_3_CoreBackend::QOpenGLFunctions_4_3_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_3_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_4_3_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_3_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_4_3_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_4_3_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_4_3_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_4_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_3_0_DeprecatedBackend {
    return QOpenGLFunctions_3_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_3_0_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_0_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_3_0_DeprecatedBackend::QOpenGLFunctions_3_0_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_3_0_DeprecatedBackend {
  pub fn New<T: QOpenGLFunctions_3_0_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_3_0_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_0_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_3_0_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_3_0_DeprecatedBackend::QOpenGLFunctions_3_0_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_0_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_3_0_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_3_0_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_3_0_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_3_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_2_1_CoreBackend {
    return QOpenGLFunctions_2_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_2_1_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_2_1_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_2_1_CoreBackend::QOpenGLFunctions_2_1_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_2_1_CoreBackend {
  pub fn New<T: QOpenGLFunctions_2_1_CoreBackend_New>(value: T) -> QOpenGLFunctions_2_1_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_1_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_2_1_CoreBackend;
}

  // proto:  void QOpenGLFunctions_2_1_CoreBackend::QOpenGLFunctions_2_1_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_2_1_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_2_1_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_2_1_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_2_1_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_2_1_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_2_1_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_2_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_0_DeprecatedBackend {
    return QOpenGLFunctions_1_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_0_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_0_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_0_DeprecatedBackend {
  pub fn New<T: QOpenGLFunctions_1_0_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_1_0_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_0_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_1_0_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_0_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_0_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_0_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_3_0_CoreBackend {
    return QOpenGLFunctions_3_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_3_0_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_0_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_3_0_CoreBackend::QOpenGLFunctions_3_0_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_3_0_CoreBackend {
  pub fn New<T: QOpenGLFunctions_3_0_CoreBackend_New>(value: T) -> QOpenGLFunctions_3_0_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_0_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_3_0_CoreBackend;
}

  // proto:  void QOpenGLFunctions_3_0_CoreBackend::QOpenGLFunctions_3_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_0_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_3_0_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_0_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_3_0_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_3_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_3_0_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_3_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_2_CoreBackend {
    return QOpenGLFunctions_1_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_2_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_2_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_1_2_CoreBackend_New>(value: T) -> QOpenGLFunctions_1_2_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_2_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_1_2_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_2_CoreBackend::QOpenGLFunctions_1_2_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_2_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_2_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_2_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_2_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_1_2_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_1_2_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_1_DeprecatedBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_1_DeprecatedBackend {
    return QOpenGLFunctions_1_1_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_1_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_1_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_1_1_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_1_1_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_1_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_1_1_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_1_DeprecatedBackend::QOpenGLFunctions_1_1_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_1_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_1_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_1_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_1_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_1_1_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_1_1_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_1_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_2_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_4_2_CoreBackend {
    return QOpenGLFunctions_4_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_4_2_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_2_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_4_2_CoreBackend_New>(value: T) -> QOpenGLFunctions_4_2_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_2_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_4_2_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_2_CoreBackend::QOpenGLFunctions_4_2_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_2_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_4_2_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_2_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_4_2_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_4_2_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_4_2_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_4_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_2_0_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_2_0_CoreBackend {
    return QOpenGLFunctions_2_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_2_0_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_2_0_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_2_0_CoreBackend_New>(value: T) -> QOpenGLFunctions_2_0_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_0_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_2_0_CoreBackend;
}

  // proto:  void QOpenGLFunctions_2_0_CoreBackend::QOpenGLFunctions_2_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_2_0_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_2_0_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_2_0_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_2_0_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_2_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_2_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_3_2_CoreBackend {
    return QOpenGLFunctions_3_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_3_2_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_2_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_3_2_CoreBackend::QOpenGLFunctions_3_2_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_3_2_CoreBackend {
  pub fn New<T: QOpenGLFunctions_3_2_CoreBackend_New>(value: T) -> QOpenGLFunctions_3_2_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_2_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_3_2_CoreBackend;
}

  // proto:  void QOpenGLFunctions_3_2_CoreBackend::QOpenGLFunctions_3_2_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_2_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_3_2_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_2_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_3_2_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_3_2_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_3_2_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_3_2_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLVersionFunctionsBackend {
    return QOpenGLVersionFunctionsBackend{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
impl /*struct*/ QOpenGLVersionFunctionsBackend {
  pub fn New<T: QOpenGLVersionFunctionsBackend_New>(value: T) -> QOpenGLVersionFunctionsBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionFunctionsBackend_New {
  fn New(self) -> QOpenGLVersionFunctionsBackend;
}

  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
impl<'a> /*trait*/ QOpenGLVersionFunctionsBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLVersionFunctionsBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLVersionFunctionsBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN30QOpenGLVersionFunctionsBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLVersionFunctionsBackend{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractOpenGLFunctions {
    return QAbstractOpenGLFunctions{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn initializeOpenGLFunctions<RetType, T: QAbstractOpenGLFunctions_initializeOpenGLFunctions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initializeOpenGLFunctions(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_initializeOpenGLFunctions<RetType> {
  fn initializeOpenGLFunctions(self , rsthis: & QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_initializeOpenGLFunctions<i8> for () {
  fn initializeOpenGLFunctions(self , rsthis: & QAbstractOpenGLFunctions) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv()};
    let mut ret = unsafe {_ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractOpenGLFunctions::QAbstractOpenGLFunctions();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn New<T: QAbstractOpenGLFunctions_New>(value: T) -> QAbstractOpenGLFunctions {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_New {
  fn New(self) -> QAbstractOpenGLFunctions;
}

  // proto:  void QAbstractOpenGLFunctions::QAbstractOpenGLFunctions();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_New for () {
  fn New(self) -> QAbstractOpenGLFunctions {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctionsC1Ev()};
    let ctysz: c_int = unsafe{QAbstractOpenGLFunctions_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN24QAbstractOpenGLFunctionsC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN24QAbstractOpenGLFunctionsC1Ev()} as u64;
    let rsthis = QAbstractOpenGLFunctions{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn d_func<RetType, T: QAbstractOpenGLFunctions_d_func<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.d_func(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_d_func<RetType> {
  fn d_func(self , rsthis: & QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_d_func<()> for () {
  fn d_func(self , rsthis: & QAbstractOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctions6d_funcEv()};
     unsafe {_ZN24QAbstractOpenGLFunctions6d_funcEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn Free<RetType, T: QAbstractOpenGLFunctions_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_Free<RetType> {
  fn Free(self , rsthis: & QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_Free<()> for () {
  fn Free(self , rsthis: & QAbstractOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctionsD0Ev()};
     unsafe {_ZN24QAbstractOpenGLFunctionsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_2_0_DeprecatedBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_2_0_DeprecatedBackend {
    return QOpenGLFunctions_2_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_2_0_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_2_0_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_2_0_DeprecatedBackend::QOpenGLFunctions_2_0_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_2_0_DeprecatedBackend {
  pub fn New<T: QOpenGLFunctions_2_0_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_2_0_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_2_0_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_2_0_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_2_0_DeprecatedBackend::QOpenGLFunctions_2_0_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_2_0_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_2_0_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_2_0_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_2_0_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_2_0_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_2_0_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_2_0_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_3_DeprecatedBackend {
    return QOpenGLFunctions_1_3_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_3_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_3_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_1_3_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_1_3_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_3_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_1_3_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_3_DeprecatedBackend::QOpenGLFunctions_1_3_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_3_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_3_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_3_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_3_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_1_3_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_1_3_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_3_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_4_DeprecatedBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_4_DeprecatedBackend {
    return QOpenGLFunctions_1_4_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_4_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_4_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_4_DeprecatedBackend::QOpenGLFunctions_1_4_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_4_DeprecatedBackend {
  pub fn New<T: QOpenGLFunctions_1_4_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_1_4_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_4_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_1_4_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_4_DeprecatedBackend::QOpenGLFunctions_1_4_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_4_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_4_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_4_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_4_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_1_4_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_1_4_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_4_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_3_CoreBackend {
    return QOpenGLFunctions_1_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_3_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_3_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_1_3_CoreBackend_New>(value: T) -> QOpenGLFunctions_1_3_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_3_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_1_3_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_3_CoreBackend::QOpenGLFunctions_1_3_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_3_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_3_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_3_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_3_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_1_3_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_1_3_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_3_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionStatus {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLVersionStatus {
    return QOpenGLVersionStatus{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
impl /*struct*/ QOpenGLVersionStatus {
  pub fn New<T: QOpenGLVersionStatus_New>(value: T) -> QOpenGLVersionStatus {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionStatus_New {
  fn New(self) -> QOpenGLVersionStatus;
}

  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
impl<'a> /*trait*/ QOpenGLVersionStatus_New for () {
  fn New(self) -> QOpenGLVersionStatus {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLVersionStatusC1Ev()};
    let ctysz: c_int = unsafe{QOpenGLVersionStatus_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN20QOpenGLVersionStatusC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN20QOpenGLVersionStatusC1Ev()} as u64;
    let rsthis = QOpenGLVersionStatus{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_0_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_0_CoreBackend {
    return QOpenGLFunctions_1_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_0_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_0_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_1_0_CoreBackend_New>(value: T) -> QOpenGLFunctions_1_0_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_0_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_1_0_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_0_CoreBackend::QOpenGLFunctions_1_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_0_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_0_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_0_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_0_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_1_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_1_0_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_1_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_3_1_CoreBackend {
    return QOpenGLFunctions_3_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_3_1_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_1_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_3_1_CoreBackend_New>(value: T) -> QOpenGLFunctions_3_1_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_1_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_3_1_CoreBackend;
}

  // proto:  void QOpenGLFunctions_3_1_CoreBackend::QOpenGLFunctions_3_1_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_1_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_3_1_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_3_1_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_3_1_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_3_1_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_3_1_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_3_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_1_CoreBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_1_CoreBackend {
    return QOpenGLFunctions_1_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_1_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_1_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_1_CoreBackend::QOpenGLFunctions_1_1_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_1_CoreBackend {
  pub fn New<T: QOpenGLFunctions_1_1_CoreBackend_New>(value: T) -> QOpenGLFunctions_1_1_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_1_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_1_1_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_1_CoreBackend::QOpenGLFunctions_1_1_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_1_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_1_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_1_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_1_1_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_1_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_1_4_CoreBackend {
    return QOpenGLFunctions_1_4_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_1_4_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_1_4_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLFunctions_1_4_CoreBackend::QOpenGLFunctions_1_4_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_4_CoreBackend {
  pub fn New<T: QOpenGLFunctions_1_4_CoreBackend_New>(value: T) -> QOpenGLFunctions_1_4_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_4_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_1_4_CoreBackend;
}

  // proto:  void QOpenGLFunctions_1_4_CoreBackend::QOpenGLFunctions_1_4_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_4_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_1_4_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_1_4_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_1_4_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_1_4_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_1_4_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_1_4_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_4_0_CoreBackend {
    return QOpenGLFunctions_4_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_4_0_CoreBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_4_0_CoreBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_4_0_CoreBackend_New>(value: T) -> QOpenGLFunctions_4_0_CoreBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_0_CoreBackend_New {
  fn New(self) -> QOpenGLFunctions_4_0_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_0_CoreBackend::QOpenGLFunctions_4_0_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_0_CoreBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_4_0_CoreBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_0_CoreBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_4_0_CoreBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN32QOpenGLFunctions_4_0_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN32QOpenGLFunctions_4_0_CoreBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_4_0_CoreBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_3_3_DeprecatedBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFunctions_3_3_DeprecatedBackend {
    return QOpenGLFunctions_3_3_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLFunctions_3_3_DeprecatedBackend {
  type Target = QOpenGLVersionFunctionsBackend;

  fn deref(&self) -> &QOpenGLVersionFunctionsBackend {
    return & self.qbase;
  }
}
impl AsRef<QOpenGLVersionFunctionsBackend> for QOpenGLFunctions_3_3_DeprecatedBackend {
  fn as_ref(& self) -> & QOpenGLVersionFunctionsBackend {
    return & self.qbase;
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
  pub fn New<T: QOpenGLFunctions_3_3_DeprecatedBackend_New>(value: T) -> QOpenGLFunctions_3_3_DeprecatedBackend {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_3_3_DeprecatedBackend_New {
  fn New(self) -> QOpenGLFunctions_3_3_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_3_3_DeprecatedBackend::QOpenGLFunctions_3_3_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_3_3_DeprecatedBackend_New for (&'a QOpenGLContext) {
  fn New(self) -> QOpenGLFunctions_3_3_DeprecatedBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLFunctions_3_3_DeprecatedBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN38QOpenGLFunctions_3_3_DeprecatedBackendC1EP14QOpenGLContext(arg0)} as u64;
    let rsthis = QOpenGLFunctions_3_3_DeprecatedBackend{qbase: QOpenGLVersionFunctionsBackend::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

