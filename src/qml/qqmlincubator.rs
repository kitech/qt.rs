// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlincubator.h
// dst-file: /src/qml/qqmlincubator.rs
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
use super::qqmlengine::QQmlEngine; // 773
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlIncubationController_Class_Size() -> c_int;
  // proto:  int QQmlIncubationController::incubatingObjectCount();
  fn _ZNK24QQmlIncubationController21incubatingObjectCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QQmlEngine * QQmlIncubationController::engine();
  fn _ZNK24QQmlIncubationController6engineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQmlIncubationController::QQmlIncubationController();
  fn _ZN24QQmlIncubationControllerC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlIncubationController::incubateWhile(volatile bool * flag, int msecs);
  fn _ZN24QQmlIncubationController13incubateWhileEPVbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int);
  // proto:  void QQmlIncubationController::QQmlIncubationController(const QQmlIncubationController & );
  fn _ZN24QQmlIncubationControllerC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlIncubationController::~QQmlIncubationController();
  fn _ZN24QQmlIncubationControllerD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlIncubationController::incubateFor(int msecs);
  fn _ZN24QQmlIncubationController11incubateForEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QQmlIncubator_Class_Size() -> c_int;
  // proto:  void QQmlIncubator::forceCompletion();
  fn _ZN13QQmlIncubator15forceCompletionEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlIncubator::isNull();
  fn _ZNK13QQmlIncubator6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlIncubator::clear();
  fn _ZN13QQmlIncubator5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlIncubator::isLoading();
  fn _ZNK13QQmlIncubator9isLoadingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QObject * QQmlIncubator::object();
  fn _ZNK13QQmlIncubator6objectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QQmlError> QQmlIncubator::errors();
  fn _ZNK13QQmlIncubator6errorsEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlIncubator::isError();
  fn _ZNK13QQmlIncubator7isErrorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlIncubator::~QQmlIncubator();
  fn _ZN13QQmlIncubatorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlIncubator::isReady();
  fn _ZNK13QQmlIncubator7isReadyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlIncubator::QQmlIncubator(const QQmlIncubator & );
  fn _ZN13QQmlIncubatorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlIncubationController)=16
#[derive(Default)]
pub struct QQmlIncubationController {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QQmlIncubator)=16
#[derive(Default)]
pub struct QQmlIncubator {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlIncubationController {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlIncubationController {
    return QQmlIncubationController{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QQmlIncubationController::incubatingObjectCount();
impl /*struct*/ QQmlIncubationController {
  pub fn incubatingObjectCount<RetType, T: QQmlIncubationController_incubatingObjectCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.incubatingObjectCount(self);
    // return 1;
  }
}

pub trait QQmlIncubationController_incubatingObjectCount<RetType> {
  fn incubatingObjectCount(self , rsthis: & QQmlIncubationController) -> RetType;
}

  // proto:  int QQmlIncubationController::incubatingObjectCount();
impl<'a> /*trait*/ QQmlIncubationController_incubatingObjectCount<i32> for () {
  fn incubatingObjectCount(self , rsthis: & QQmlIncubationController) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QQmlIncubationController21incubatingObjectCountEv()};
    let mut ret = unsafe {_ZNK24QQmlIncubationController21incubatingObjectCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QQmlEngine * QQmlIncubationController::engine();
impl /*struct*/ QQmlIncubationController {
  pub fn engine<RetType, T: QQmlIncubationController_engine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.engine(self);
    // return 1;
  }
}

pub trait QQmlIncubationController_engine<RetType> {
  fn engine(self , rsthis: & QQmlIncubationController) -> RetType;
}

  // proto:  QQmlEngine * QQmlIncubationController::engine();
impl<'a> /*trait*/ QQmlIncubationController_engine<QQmlEngine> for () {
  fn engine(self , rsthis: & QQmlIncubationController) -> QQmlEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QQmlIncubationController6engineEv()};
    let mut ret = unsafe {_ZNK24QQmlIncubationController6engineEv(rsthis.qclsinst)};
    let mut ret1 = QQmlEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlIncubationController::QQmlIncubationController();
impl /*struct*/ QQmlIncubationController {
  pub fn new<T: QQmlIncubationController_new>(value: T) -> QQmlIncubationController {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlIncubationController_new {
  fn new(self) -> QQmlIncubationController;
}

  // proto:  void QQmlIncubationController::QQmlIncubationController();
impl<'a> /*trait*/ QQmlIncubationController_new for () {
  fn new(self) -> QQmlIncubationController {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QQmlIncubationControllerC2Ev()};
    let ctysz: c_int = unsafe{QQmlIncubationController_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN24QQmlIncubationControllerC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlIncubationController{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlIncubationController::incubateWhile(volatile bool * flag, int msecs);
impl /*struct*/ QQmlIncubationController {
  pub fn incubateWhile<RetType, T: QQmlIncubationController_incubateWhile<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.incubateWhile(self);
    // return 1;
  }
}

pub trait QQmlIncubationController_incubateWhile<RetType> {
  fn incubateWhile(self , rsthis: & QQmlIncubationController) -> RetType;
}

  // proto:  void QQmlIncubationController::incubateWhile(volatile bool * flag, int msecs);
impl<'a> /*trait*/ QQmlIncubationController_incubateWhile<()> for (&'a mut Vec<i8>, i32) {
  fn incubateWhile(self , rsthis: & QQmlIncubationController) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QQmlIncubationController13incubateWhileEPVbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN24QQmlIncubationController13incubateWhileEPVbi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QQmlIncubationController::QQmlIncubationController(const QQmlIncubationController & );
impl<'a> /*trait*/ QQmlIncubationController_new for (&'a QQmlIncubationController) {
  fn new(self) -> QQmlIncubationController {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QQmlIncubationControllerC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlIncubationController_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QQmlIncubationControllerC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlIncubationController{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlIncubationController::~QQmlIncubationController();
impl /*struct*/ QQmlIncubationController {
  pub fn free<RetType, T: QQmlIncubationController_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlIncubationController_free<RetType> {
  fn free(self , rsthis: & QQmlIncubationController) -> RetType;
}

  // proto:  void QQmlIncubationController::~QQmlIncubationController();
impl<'a> /*trait*/ QQmlIncubationController_free<()> for () {
  fn free(self , rsthis: & QQmlIncubationController) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QQmlIncubationControllerD2Ev()};
     unsafe {_ZN24QQmlIncubationControllerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlIncubationController::incubateFor(int msecs);
impl /*struct*/ QQmlIncubationController {
  pub fn incubateFor<RetType, T: QQmlIncubationController_incubateFor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.incubateFor(self);
    // return 1;
  }
}

pub trait QQmlIncubationController_incubateFor<RetType> {
  fn incubateFor(self , rsthis: & QQmlIncubationController) -> RetType;
}

  // proto:  void QQmlIncubationController::incubateFor(int msecs);
impl<'a> /*trait*/ QQmlIncubationController_incubateFor<()> for (i32) {
  fn incubateFor(self , rsthis: & QQmlIncubationController) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QQmlIncubationController11incubateForEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN24QQmlIncubationController11incubateForEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QQmlIncubator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlIncubator {
    return QQmlIncubator{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlIncubator::forceCompletion();
impl /*struct*/ QQmlIncubator {
  pub fn forceCompletion<RetType, T: QQmlIncubator_forceCompletion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.forceCompletion(self);
    // return 1;
  }
}

pub trait QQmlIncubator_forceCompletion<RetType> {
  fn forceCompletion(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  void QQmlIncubator::forceCompletion();
impl<'a> /*trait*/ QQmlIncubator_forceCompletion<()> for () {
  fn forceCompletion(self , rsthis: & QQmlIncubator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlIncubator15forceCompletionEv()};
     unsafe {_ZN13QQmlIncubator15forceCompletionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlIncubator::isNull();
impl /*struct*/ QQmlIncubator {
  pub fn isNull<RetType, T: QQmlIncubator_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QQmlIncubator_isNull<RetType> {
  fn isNull(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  bool QQmlIncubator::isNull();
impl<'a> /*trait*/ QQmlIncubator_isNull<i8> for () {
  fn isNull(self , rsthis: & QQmlIncubator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlIncubator6isNullEv()};
    let mut ret = unsafe {_ZNK13QQmlIncubator6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlIncubator::clear();
impl /*struct*/ QQmlIncubator {
  pub fn clear<RetType, T: QQmlIncubator_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QQmlIncubator_clear<RetType> {
  fn clear(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  void QQmlIncubator::clear();
impl<'a> /*trait*/ QQmlIncubator_clear<()> for () {
  fn clear(self , rsthis: & QQmlIncubator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlIncubator5clearEv()};
     unsafe {_ZN13QQmlIncubator5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlIncubator::isLoading();
impl /*struct*/ QQmlIncubator {
  pub fn isLoading<RetType, T: QQmlIncubator_isLoading<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLoading(self);
    // return 1;
  }
}

pub trait QQmlIncubator_isLoading<RetType> {
  fn isLoading(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  bool QQmlIncubator::isLoading();
impl<'a> /*trait*/ QQmlIncubator_isLoading<i8> for () {
  fn isLoading(self , rsthis: & QQmlIncubator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlIncubator9isLoadingEv()};
    let mut ret = unsafe {_ZNK13QQmlIncubator9isLoadingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QObject * QQmlIncubator::object();
impl /*struct*/ QQmlIncubator {
  pub fn object<RetType, T: QQmlIncubator_object<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QQmlIncubator_object<RetType> {
  fn object(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  QObject * QQmlIncubator::object();
impl<'a> /*trait*/ QQmlIncubator_object<QObject> for () {
  fn object(self , rsthis: & QQmlIncubator) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlIncubator6objectEv()};
    let mut ret = unsafe {_ZNK13QQmlIncubator6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QQmlError> QQmlIncubator::errors();
impl /*struct*/ QQmlIncubator {
  pub fn errors<RetType, T: QQmlIncubator_errors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errors(self);
    // return 1;
  }
}

pub trait QQmlIncubator_errors<RetType> {
  fn errors(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  QList<QQmlError> QQmlIncubator::errors();
impl<'a> /*trait*/ QQmlIncubator_errors<()> for () {
  fn errors(self , rsthis: & QQmlIncubator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlIncubator6errorsEv()};
     unsafe {_ZNK13QQmlIncubator6errorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlIncubator::isError();
impl /*struct*/ QQmlIncubator {
  pub fn isError<RetType, T: QQmlIncubator_isError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isError(self);
    // return 1;
  }
}

pub trait QQmlIncubator_isError<RetType> {
  fn isError(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  bool QQmlIncubator::isError();
impl<'a> /*trait*/ QQmlIncubator_isError<i8> for () {
  fn isError(self , rsthis: & QQmlIncubator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlIncubator7isErrorEv()};
    let mut ret = unsafe {_ZNK13QQmlIncubator7isErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlIncubator::~QQmlIncubator();
impl /*struct*/ QQmlIncubator {
  pub fn free<RetType, T: QQmlIncubator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlIncubator_free<RetType> {
  fn free(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  void QQmlIncubator::~QQmlIncubator();
impl<'a> /*trait*/ QQmlIncubator_free<()> for () {
  fn free(self , rsthis: & QQmlIncubator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlIncubatorD2Ev()};
     unsafe {_ZN13QQmlIncubatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlIncubator::isReady();
impl /*struct*/ QQmlIncubator {
  pub fn isReady<RetType, T: QQmlIncubator_isReady<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReady(self);
    // return 1;
  }
}

pub trait QQmlIncubator_isReady<RetType> {
  fn isReady(self , rsthis: & QQmlIncubator) -> RetType;
}

  // proto:  bool QQmlIncubator::isReady();
impl<'a> /*trait*/ QQmlIncubator_isReady<i8> for () {
  fn isReady(self , rsthis: & QQmlIncubator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlIncubator7isReadyEv()};
    let mut ret = unsafe {_ZNK13QQmlIncubator7isReadyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlIncubator::QQmlIncubator(const QQmlIncubator & );
impl /*struct*/ QQmlIncubator {
  pub fn new<T: QQmlIncubator_new>(value: T) -> QQmlIncubator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlIncubator_new {
  fn new(self) -> QQmlIncubator;
}

  // proto:  void QQmlIncubator::QQmlIncubator(const QQmlIncubator & );
impl<'a> /*trait*/ QQmlIncubator_new for (&'a QQmlIncubator) {
  fn new(self) -> QQmlIncubator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlIncubatorC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlIncubator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QQmlIncubatorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlIncubator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

