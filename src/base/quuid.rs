// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN5QUuidC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK5QUuid9toRfc4122Ev() -> i32;
  fn _ZNK5QUuid8toStringEv() -> i32;
  fn _ZNK5QUuid6isNullEv() -> i32;
  fn _ZN5QUuid12createUuidV5ERKS_RK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QUuid10createUuidEv() -> i32;
  fn _ZN5QUuidC1Ejtthhhhhhhh(qthis: *mut c_void, arg0: c_uint, arg1: c_ushort, arg2: c_ushort, arg3: c_uchar, arg4: c_uchar, arg5: c_uchar, arg6: c_uchar, arg7: c_uchar, arg8: c_uchar, arg9: c_uchar, arg10: c_uchar) -> i32;
  fn _ZN5QUuidC1ERK10QByteArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN5QUuid12createUuidV3ERKS_RK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QUuidC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK5QUuid11toByteArrayEv() -> i32;
  fn _ZN5QUuidC1EPKc(qthis: *mut c_void, arg0: *const c_char) -> i32;
  fn _ZN5QUuid12createUuidV5ERKS_RK10QByteArray(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QUuid11fromRfc4122ERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN5QUuid12createUuidV3ERKS_RK10QByteArray(arg0: *const c_void, arg1: *const c_void) -> i32;
}

// body block begin
// class sizeof(QUuid)=16
pub struct QUuid {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUuid {
  pub fn NewQUuid<T: QUuid_NewQUuid>(value: T) -> QUuid {
    let rsthis = value.NewQUuid();
    return rsthis;
    // return 1;
  }
}

pub trait QUuid_NewQUuid {
  fn NewQUuid(self) -> QUuid;
}

// proto: void QUuid::NewQUuid(const QString & );
impl<'a> /*trait*/ QUuid_NewQUuid for (&'a  QString) {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QUuidC1ERK7QString(qthis, arg0)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn toRfc4122<T: QUuid_toRfc4122>(&mut self, value: T) -> i32 {
    value.toRfc4122(self);
    return 1;
  }
}

pub trait QUuid_toRfc4122 {
  fn toRfc4122(self, this: &mut QUuid) -> i32;
}

// proto: QByteArray QUuid::toRfc4122();
impl<'a> /*trait*/ QUuid_toRfc4122 for () {
  fn toRfc4122(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid9toRfc4122Ev()};
    unsafe {_ZNK5QUuid9toRfc4122Ev()};
    return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn toString<T: QUuid_toString>(&mut self, value: T) -> i32 {
    value.toString(self);
    return 1;
  }
}

pub trait QUuid_toString {
  fn toString(self, this: &mut QUuid) -> i32;
}

// proto: QString QUuid::toString();
impl<'a> /*trait*/ QUuid_toString for () {
  fn toString(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid8toStringEv()};
    unsafe {_ZNK5QUuid8toStringEv()};
    return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn isNull<T: QUuid_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QUuid_isNull {
  fn isNull(self, this: &mut QUuid) -> i32;
}

// proto: bool QUuid::isNull();
impl<'a> /*trait*/ QUuid_isNull for () {
  fn isNull(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid6isNullEv()};
    unsafe {_ZNK5QUuid6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn createUuidV5<T: QUuid_createUuidV5>(&mut self, value: T) -> i32 {
    value.createUuidV5(self);
    return 1;
  }
}

pub trait QUuid_createUuidV5 {
  fn createUuidV5(self, this: &mut QUuid) -> i32;
}

// proto: QUuid QUuid::createUuidV5(const QUuid & ns, const QString & baseData);
impl<'a> /*trait*/ QUuid_createUuidV5 for (&'a  QUuid, &'a  QString) {
  fn createUuidV5(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV5ERKS_RK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QUuid12createUuidV5ERKS_RK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn createUuid<T: QUuid_createUuid>(&mut self, value: T) -> i32 {
    value.createUuid(self);
    return 1;
  }
}

pub trait QUuid_createUuid {
  fn createUuid(self, this: &mut QUuid) -> i32;
}

// proto: QUuid QUuid::createUuid();
impl<'a> /*trait*/ QUuid_createUuid for () {
  fn createUuid(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid10createUuidEv()};
    unsafe {_ZN5QUuid10createUuidEv()};
    return 1;
  }
}

// proto: void QUuid::NewQUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3, uchar b4, uchar b5, uchar b6, uchar b7, uchar b8);
impl<'a> /*trait*/ QUuid_NewQUuid for (u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1Ejtthhhhhhhh()};
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
    unsafe {_ZN5QUuidC1Ejtthhhhhhhh(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QUuid::NewQUuid(const QByteArray & );
impl<'a> /*trait*/ QUuid_NewQUuid for (&'a  QByteArray) {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QUuidC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn createUuidV3<T: QUuid_createUuidV3>(&mut self, value: T) -> i32 {
    value.createUuidV3(self);
    return 1;
  }
}

pub trait QUuid_createUuidV3 {
  fn createUuidV3(self, this: &mut QUuid) -> i32;
}

// proto: QUuid QUuid::createUuidV3(const QUuid & ns, const QString & baseData);
impl<'a> /*trait*/ QUuid_createUuidV3 for (&'a  QUuid, &'a  QString) {
  fn createUuidV3(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV3ERKS_RK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QUuid12createUuidV3ERKS_RK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: void QUuid::NewQUuid();
impl<'a> /*trait*/ QUuid_NewQUuid for () {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1Ev()};
    unsafe {_ZN5QUuidC1Ev(qthis)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn toByteArray<T: QUuid_toByteArray>(&mut self, value: T) -> i32 {
    value.toByteArray(self);
    return 1;
  }
}

pub trait QUuid_toByteArray {
  fn toByteArray(self, this: &mut QUuid) -> i32;
}

// proto: QByteArray QUuid::toByteArray();
impl<'a> /*trait*/ QUuid_toByteArray for () {
  fn toByteArray(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid11toByteArrayEv()};
    unsafe {_ZNK5QUuid11toByteArrayEv()};
    return 1;
  }
}

// proto: void QUuid::NewQUuid(const char * );
impl<'a> /*trait*/ QUuid_NewQUuid for (&'a  String) {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1EPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN5QUuidC1EPKc(qthis, arg0)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QUuid QUuid::createUuidV5(const QUuid & ns, const QByteArray & baseData);
impl<'a> /*trait*/ QUuid_createUuidV5 for (&'a  QUuid, &'a  QByteArray) {
  fn createUuidV5(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV5ERKS_RK10QByteArray()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QUuid12createUuidV5ERKS_RK10QByteArray(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn fromRfc4122<T: QUuid_fromRfc4122>(&mut self, value: T) -> i32 {
    value.fromRfc4122(self);
    return 1;
  }
}

pub trait QUuid_fromRfc4122 {
  fn fromRfc4122(self, this: &mut QUuid) -> i32;
}

// proto: QUuid QUuid::fromRfc4122(const QByteArray & );
impl<'a> /*trait*/ QUuid_fromRfc4122 for (&'a  QByteArray) {
  fn fromRfc4122(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid11fromRfc4122ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QUuid11fromRfc4122ERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: QUuid QUuid::createUuidV3(const QUuid & ns, const QByteArray & baseData);
impl<'a> /*trait*/ QUuid_createUuidV3 for (&'a  QUuid, &'a  QByteArray) {
  fn createUuidV3(self, this: &mut QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV3ERKS_RK10QByteArray()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QUuid12createUuidV3ERKS_RK10QByteArray(arg0, arg1)};
    return 1;
  }
}

