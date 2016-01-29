// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmllist.h
// dst-file: /src/qml/qqmllist.rs
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
use super::super::core::qobject::QObject; // 771
use super::qqmlengine::QQmlEngine; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlListReference_Class_Size() -> c_int;
  // proto:  void QQmlListReference::QQmlListReference(const QQmlListReference & );
  fn _ZN17QQmlListReferenceC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QQmlListReference::clear();
  fn _ZNK17QQmlListReference5clearEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QQmlListReference::listElementType();
  fn _ZNK17QQmlListReference15listElementTypeEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlListReference::canClear();
  fn _ZNK17QQmlListReference8canClearEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlListReference::isReadable();
  fn _ZNK17QQmlListReference10isReadableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlListReference::QQmlListReference();
  fn _ZN17QQmlListReferenceC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlListReference::canCount();
  fn _ZNK17QQmlListReference8canCountEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QQmlListReference::count();
  fn _ZNK17QQmlListReference5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QQmlListReference::QQmlListReference(QObject * , const char * property, QQmlEngine * );
  fn _ZN17QQmlListReferenceC2EP7QObjectPKcP10QQmlEngine(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char, arg2: *mut c_void);
  // proto:  bool QQmlListReference::canAt();
  fn _ZNK17QQmlListReference5canAtEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlListReference::~QQmlListReference();
  fn _ZN17QQmlListReferenceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QObject * QQmlListReference::at(int );
  fn _ZNK17QQmlListReference2atEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QObject * QQmlListReference::object();
  fn _ZNK17QQmlListReference6objectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QQmlListReference::canAppend();
  fn _ZNK17QQmlListReference9canAppendEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlListReference::isManipulable();
  fn _ZNK17QQmlListReference13isManipulableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlListReference::isValid();
  fn _ZNK17QQmlListReference7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QQmlListReference::append(QObject * );
  fn _ZNK17QQmlListReference6appendEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QQmlListReference)=8
#[derive(Default)]
pub struct QQmlListReference {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlListReference {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlListReference {
    return QQmlListReference{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlListReference::QQmlListReference(const QQmlListReference & );
impl /*struct*/ QQmlListReference {
  pub fn new<T: QQmlListReference_new>(value: T) -> QQmlListReference {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlListReference_new {
  fn new(self) -> QQmlListReference;
}

  // proto:  void QQmlListReference::QQmlListReference(const QQmlListReference & );
impl<'a> /*trait*/ QQmlListReference_new for (&'a QQmlListReference) {
  fn new(self) -> QQmlListReference {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQmlListReferenceC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlListReference_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QQmlListReferenceC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlListReference{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlListReference::clear();
impl /*struct*/ QQmlListReference {
  pub fn clear<RetType, T: QQmlListReference_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QQmlListReference_clear<RetType> {
  fn clear(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::clear();
impl<'a> /*trait*/ QQmlListReference_clear<i8> for () {
  fn clear(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference5clearEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference5clearEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlListReference::listElementType();
impl /*struct*/ QQmlListReference {
  pub fn listElementType<RetType, T: QQmlListReference_listElementType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.listElementType(self);
    // return 1;
  }
}

pub trait QQmlListReference_listElementType<RetType> {
  fn listElementType(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  const QMetaObject * QQmlListReference::listElementType();
impl<'a> /*trait*/ QQmlListReference_listElementType<()> for () {
  fn listElementType(self , rsthis: & QQmlListReference) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference15listElementTypeEv()};
     unsafe {_ZNK17QQmlListReference15listElementTypeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlListReference::canClear();
impl /*struct*/ QQmlListReference {
  pub fn canClear<RetType, T: QQmlListReference_canClear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canClear(self);
    // return 1;
  }
}

pub trait QQmlListReference_canClear<RetType> {
  fn canClear(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::canClear();
impl<'a> /*trait*/ QQmlListReference_canClear<i8> for () {
  fn canClear(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference8canClearEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference8canClearEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlListReference::isReadable();
impl /*struct*/ QQmlListReference {
  pub fn isReadable<RetType, T: QQmlListReference_isReadable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadable(self);
    // return 1;
  }
}

pub trait QQmlListReference_isReadable<RetType> {
  fn isReadable(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::isReadable();
impl<'a> /*trait*/ QQmlListReference_isReadable<i8> for () {
  fn isReadable(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference10isReadableEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlListReference::QQmlListReference();
impl<'a> /*trait*/ QQmlListReference_new for () {
  fn new(self) -> QQmlListReference {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQmlListReferenceC2Ev()};
    let ctysz: c_int = unsafe{QQmlListReference_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN17QQmlListReferenceC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlListReference{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlListReference::canCount();
impl /*struct*/ QQmlListReference {
  pub fn canCount<RetType, T: QQmlListReference_canCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canCount(self);
    // return 1;
  }
}

pub trait QQmlListReference_canCount<RetType> {
  fn canCount(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::canCount();
impl<'a> /*trait*/ QQmlListReference_canCount<i8> for () {
  fn canCount(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference8canCountEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference8canCountEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QQmlListReference::count();
impl /*struct*/ QQmlListReference {
  pub fn count<RetType, T: QQmlListReference_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QQmlListReference_count<RetType> {
  fn count(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  int QQmlListReference::count();
impl<'a> /*trait*/ QQmlListReference_count<i32> for () {
  fn count(self , rsthis: & QQmlListReference) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference5countEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QQmlListReference::QQmlListReference(QObject * , const char * property, QQmlEngine * );
impl<'a> /*trait*/ QQmlListReference_new for (&'a QObject, &'a  String, &'a QQmlEngine) {
  fn new(self) -> QQmlListReference {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQmlListReferenceC2EP7QObjectPKcP10QQmlEngine()};
    let ctysz: c_int = unsafe{QQmlListReference_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN17QQmlListReferenceC2EP7QObjectPKcP10QQmlEngine(qthis_ph, arg0, arg1, arg2)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlListReference{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlListReference::canAt();
impl /*struct*/ QQmlListReference {
  pub fn canAt<RetType, T: QQmlListReference_canAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canAt(self);
    // return 1;
  }
}

pub trait QQmlListReference_canAt<RetType> {
  fn canAt(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::canAt();
impl<'a> /*trait*/ QQmlListReference_canAt<i8> for () {
  fn canAt(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference5canAtEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference5canAtEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlListReference::~QQmlListReference();
impl /*struct*/ QQmlListReference {
  pub fn free<RetType, T: QQmlListReference_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlListReference_free<RetType> {
  fn free(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  void QQmlListReference::~QQmlListReference();
impl<'a> /*trait*/ QQmlListReference_free<()> for () {
  fn free(self , rsthis: & QQmlListReference) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQmlListReferenceD2Ev()};
     unsafe {_ZN17QQmlListReferenceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QQmlListReference::at(int );
impl /*struct*/ QQmlListReference {
  pub fn at<RetType, T: QQmlListReference_at<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QQmlListReference_at<RetType> {
  fn at(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  QObject * QQmlListReference::at(int );
impl<'a> /*trait*/ QQmlListReference_at<QObject> for (i32) {
  fn at(self , rsthis: & QQmlListReference) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK17QQmlListReference2atEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QQmlListReference::object();
impl /*struct*/ QQmlListReference {
  pub fn object<RetType, T: QQmlListReference_object<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QQmlListReference_object<RetType> {
  fn object(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  QObject * QQmlListReference::object();
impl<'a> /*trait*/ QQmlListReference_object<QObject> for () {
  fn object(self , rsthis: & QQmlListReference) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference6objectEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQmlListReference::canAppend();
impl /*struct*/ QQmlListReference {
  pub fn canAppend<RetType, T: QQmlListReference_canAppend<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canAppend(self);
    // return 1;
  }
}

pub trait QQmlListReference_canAppend<RetType> {
  fn canAppend(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::canAppend();
impl<'a> /*trait*/ QQmlListReference_canAppend<i8> for () {
  fn canAppend(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference9canAppendEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference9canAppendEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlListReference::isManipulable();
impl /*struct*/ QQmlListReference {
  pub fn isManipulable<RetType, T: QQmlListReference_isManipulable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isManipulable(self);
    // return 1;
  }
}

pub trait QQmlListReference_isManipulable<RetType> {
  fn isManipulable(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::isManipulable();
impl<'a> /*trait*/ QQmlListReference_isManipulable<i8> for () {
  fn isManipulable(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference13isManipulableEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference13isManipulableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlListReference::isValid();
impl /*struct*/ QQmlListReference {
  pub fn isValid<RetType, T: QQmlListReference_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QQmlListReference_isValid<RetType> {
  fn isValid(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::isValid();
impl<'a> /*trait*/ QQmlListReference_isValid<i8> for () {
  fn isValid(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference7isValidEv()};
    let mut ret = unsafe {_ZNK17QQmlListReference7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QQmlListReference::append(QObject * );
impl /*struct*/ QQmlListReference {
  pub fn append<RetType, T: QQmlListReference_append<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QQmlListReference_append<RetType> {
  fn append(self , rsthis: & QQmlListReference) -> RetType;
}

  // proto:  bool QQmlListReference::append(QObject * );
impl<'a> /*trait*/ QQmlListReference_append<i8> for (&'a QObject) {
  fn append(self , rsthis: & QQmlListReference) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQmlListReference6appendEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QQmlListReference6appendEP7QObject(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

