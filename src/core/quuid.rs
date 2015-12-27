// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtCore/quuid.h
// dst-file: /src/core/quuid.rs
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
use super::qstring::QString; // 773
use super::qbytearray::QByteArray; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QUuid_Class_Size() -> c_int;
  // proto:  void QUuid::QUuid(const QString & );
  fn dector_ZN5QUuidC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN5QUuidC1ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QUuid::toRfc4122();
  fn _ZNK5QUuid9toRfc4122Ev(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QUuid::toString();
  fn _ZNK5QUuid8toStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QUuid::isNull();
  fn _ZNK5QUuid6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QString & baseData);
  fn demth_ZN5QUuid12createUuidV5ERKS_RK7QString(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QUuid QUuid::createUuid();
  fn _ZN5QUuid10createUuidEv() -> *mut c_void;
  // proto:  void QUuid::QUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3, uchar b4, uchar b5, uchar b6, uchar b7, uchar b8);
  fn dector_ZN5QUuidC1Ejtthhhhhhhh(arg0: c_uint, arg1: c_ushort, arg2: c_ushort, arg3: c_uchar, arg4: c_uchar, arg5: c_uchar, arg6: c_uchar, arg7: c_uchar, arg8: c_uchar, arg9: c_uchar, arg10: c_uchar) -> *mut c_void;
  fn _ZN5QUuidC1Ejtthhhhhhhh(qthis: u64 /* *mut c_void*/, arg0: c_uint, arg1: c_ushort, arg2: c_ushort, arg3: c_uchar, arg4: c_uchar, arg5: c_uchar, arg6: c_uchar, arg7: c_uchar, arg8: c_uchar, arg9: c_uchar, arg10: c_uchar);
  // proto:  void QUuid::QUuid(const QByteArray & );
  fn dector_ZN5QUuidC1ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  fn _ZN5QUuidC1ERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QString & baseData);
  fn demth_ZN5QUuid12createUuidV3ERKS_RK7QString(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QUuid::QUuid();
  fn dector_ZN5QUuidC1Ev() -> *mut c_void;
  fn _ZN5QUuidC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QUuid::toByteArray();
  fn _ZNK5QUuid11toByteArrayEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QUuid::QUuid(const char * );
  fn dector_ZN5QUuidC1EPKc(arg0: *mut c_char) -> *mut c_void;
  fn _ZN5QUuidC1EPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QByteArray & baseData);
  fn _ZN5QUuid12createUuidV5ERKS_RK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QUuid QUuid::fromRfc4122(const QByteArray & );
  fn _ZN5QUuid11fromRfc4122ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QByteArray & baseData);
  fn _ZN5QUuid12createUuidV3ERKS_RK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QUuid)=16
#[derive(Default)]
pub struct QUuid {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QUuid {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QUuid {
    return QUuid{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QUuid::QUuid(const QString & );
impl /*struct*/ QUuid {
  pub fn New<T: QUuid_New>(value: T) -> QUuid {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QUuid_New {
  fn New(self) -> QUuid;
}

  // proto:  void QUuid::QUuid(const QString & );
impl<'a> /*trait*/ QUuid_New for (&'a QString) {
  fn New(self) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1ERK7QString()};
    let ctysz: c_int = unsafe{QUuid_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN5QUuidC1ERK7QString(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QUuidC1ERK7QString(arg0)} as u64;
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QUuid::toRfc4122();
impl /*struct*/ QUuid {
  pub fn toRfc4122<RetType, T: QUuid_toRfc4122<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toRfc4122(self);
    // return 1;
  }
}

pub trait QUuid_toRfc4122<RetType> {
  fn toRfc4122(self , rsthis: & QUuid) -> RetType;
}

  // proto:  QByteArray QUuid::toRfc4122();
impl<'a> /*trait*/ QUuid_toRfc4122<QByteArray> for () {
  fn toRfc4122(self , rsthis: & QUuid) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid9toRfc4122Ev()};
    let mut ret = unsafe {_ZNK5QUuid9toRfc4122Ev(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QUuid::toString();
impl /*struct*/ QUuid {
  pub fn toString<RetType, T: QUuid_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QUuid_toString<RetType> {
  fn toString(self , rsthis: & QUuid) -> RetType;
}

  // proto:  QString QUuid::toString();
impl<'a> /*trait*/ QUuid_toString<QString> for () {
  fn toString(self , rsthis: & QUuid) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid8toStringEv()};
    let mut ret = unsafe {_ZNK5QUuid8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QUuid::isNull();
impl /*struct*/ QUuid {
  pub fn isNull<RetType, T: QUuid_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QUuid_isNull<RetType> {
  fn isNull(self , rsthis: & QUuid) -> RetType;
}

  // proto:  bool QUuid::isNull();
impl<'a> /*trait*/ QUuid_isNull<i8> for () {
  fn isNull(self , rsthis: & QUuid) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid6isNullEv()};
    let mut ret = unsafe {_ZNK5QUuid6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QString & baseData);
impl /*struct*/ QUuid {
  pub fn createUuidV5_s<RetType, T: QUuid_createUuidV5_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.createUuidV5_s();
    // return 1;
  }
}

pub trait QUuid_createUuidV5_s<RetType> {
  fn createUuidV5_s(self ) -> RetType;
}

  // proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QString & baseData);
impl<'a> /*trait*/ QUuid_createUuidV5_s<QUuid> for (&'a QUuid, &'a QString) {
  fn createUuidV5_s(self ) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV5ERKS_RK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN5QUuid12createUuidV5ERKS_RK7QString(arg0, arg1)};
    let mut ret1 = QUuid::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QUuid QUuid::createUuid();
impl /*struct*/ QUuid {
  pub fn createUuid_s<RetType, T: QUuid_createUuid_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.createUuid_s();
    // return 1;
  }
}

pub trait QUuid_createUuid_s<RetType> {
  fn createUuid_s(self ) -> RetType;
}

  // proto: static QUuid QUuid::createUuid();
impl<'a> /*trait*/ QUuid_createUuid_s<QUuid> for () {
  fn createUuid_s(self ) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid10createUuidEv()};
    let mut ret = unsafe {_ZN5QUuid10createUuidEv()};
    let mut ret1 = QUuid::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUuid::QUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3, uchar b4, uchar b5, uchar b6, uchar b7, uchar b8);
impl<'a> /*trait*/ QUuid_New for (u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) {
  fn New(self) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1Ejtthhhhhhhh()};
    let ctysz: c_int = unsafe{QUuid_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_ushort;
    let arg2 = self.2  as c_ushort;
    let arg3 = self.3  as c_uchar;
    let arg4 = self.4  as c_uchar;
    let arg5 = self.5  as c_uchar;
    let arg6 = self.6  as c_uchar;
    let arg7 = self.7  as c_uchar;
    let arg8 = self.8  as c_uchar;
    let arg9 = self.9  as c_uchar;
    let arg10 = self.10  as c_uchar;
    // unsafe {_ZN5QUuidC1Ejtthhhhhhhh(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)};
    let qthis: u64 = unsafe {dector_ZN5QUuidC1Ejtthhhhhhhh(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)} as u64;
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUuid::QUuid(const QByteArray & );
impl<'a> /*trait*/ QUuid_New for (&'a QByteArray) {
  fn New(self) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1ERK10QByteArray()};
    let ctysz: c_int = unsafe{QUuid_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN5QUuidC1ERK10QByteArray(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QUuidC1ERK10QByteArray(arg0)} as u64;
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QString & baseData);
impl /*struct*/ QUuid {
  pub fn createUuidV3_s<RetType, T: QUuid_createUuidV3_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.createUuidV3_s();
    // return 1;
  }
}

pub trait QUuid_createUuidV3_s<RetType> {
  fn createUuidV3_s(self ) -> RetType;
}

  // proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QString & baseData);
impl<'a> /*trait*/ QUuid_createUuidV3_s<QUuid> for (&'a QUuid, &'a QString) {
  fn createUuidV3_s(self ) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV3ERKS_RK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN5QUuid12createUuidV3ERKS_RK7QString(arg0, arg1)};
    let mut ret1 = QUuid::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUuid::QUuid();
impl<'a> /*trait*/ QUuid_New for () {
  fn New(self) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1Ev()};
    let ctysz: c_int = unsafe{QUuid_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN5QUuidC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN5QUuidC1Ev()} as u64;
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QUuid::toByteArray();
impl /*struct*/ QUuid {
  pub fn toByteArray<RetType, T: QUuid_toByteArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toByteArray(self);
    // return 1;
  }
}

pub trait QUuid_toByteArray<RetType> {
  fn toByteArray(self , rsthis: & QUuid) -> RetType;
}

  // proto:  QByteArray QUuid::toByteArray();
impl<'a> /*trait*/ QUuid_toByteArray<QByteArray> for () {
  fn toByteArray(self , rsthis: & QUuid) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid11toByteArrayEv()};
    let mut ret = unsafe {_ZNK5QUuid11toByteArrayEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUuid::QUuid(const char * );
impl<'a> /*trait*/ QUuid_New for (&'a  String) {
  fn New(self) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1EPKc()};
    let ctysz: c_int = unsafe{QUuid_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut c_char;
    // unsafe {_ZN5QUuidC1EPKc(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QUuidC1EPKc(arg0)} as u64;
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QByteArray & baseData);
impl<'a> /*trait*/ QUuid_createUuidV5_s<QUuid> for (&'a QUuid, &'a QByteArray) {
  fn createUuidV5_s(self ) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV5ERKS_RK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QUuid12createUuidV5ERKS_RK10QByteArray(arg0, arg1)};
    let mut ret1 = QUuid::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QUuid QUuid::fromRfc4122(const QByteArray & );
impl /*struct*/ QUuid {
  pub fn fromRfc4122_s<RetType, T: QUuid_fromRfc4122_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRfc4122_s();
    // return 1;
  }
}

pub trait QUuid_fromRfc4122_s<RetType> {
  fn fromRfc4122_s(self ) -> RetType;
}

  // proto: static QUuid QUuid::fromRfc4122(const QByteArray & );
impl<'a> /*trait*/ QUuid_fromRfc4122_s<QUuid> for (&'a QByteArray) {
  fn fromRfc4122_s(self ) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid11fromRfc4122ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QUuid11fromRfc4122ERK10QByteArray(arg0)};
    let mut ret1 = QUuid::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QByteArray & baseData);
impl<'a> /*trait*/ QUuid_createUuidV3_s<QUuid> for (&'a QUuid, &'a QByteArray) {
  fn createUuidV3_s(self ) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV3ERKS_RK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QUuid12createUuidV3ERKS_RK10QByteArray(arg0, arg1)};
    let mut ret1 = QUuid::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

