// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QMetaEnum::value(int index);
  fn _ZNK9QMetaEnum5valueEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  const char * QMetaEnum::name();
  fn _ZNK9QMetaEnum4nameEv(qthis: *mut c_void) -> *const c_char;
  // proto:  bool QMetaEnum::isFlag();
  fn _ZNK9QMetaEnum6isFlagEv(qthis: *mut c_void) -> int8_t;
  // proto:  const char * QMetaEnum::scope();
  fn _ZNK9QMetaEnum5scopeEv(qthis: *mut c_void) -> *const c_char;
  // proto:  int QMetaEnum::keyToValue(const char * key, bool * ok);
  fn _ZNK9QMetaEnum10keyToValueEPKcPb(qthis: *mut c_void, arg0: *const c_char, arg1: *mut int8_t) -> c_int;
  // proto:  const QMetaObject * QMetaEnum::enclosingMetaObject();
  fn _ZNK9QMetaEnum19enclosingMetaObjectEv(qthis: *mut c_void) ;
  // proto:  QByteArray QMetaEnum::valueToKeys(int value);
  fn _ZNK9QMetaEnum11valueToKeysEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QMetaEnum::NewQMetaEnum();
  fn _ZN9QMetaEnumC1Ev(qthis: *mut c_void) ;
  // proto:  int QMetaEnum::keysToValue(const char * keys, bool * ok);
  fn _ZNK9QMetaEnum11keysToValueEPKcPb(qthis: *mut c_void, arg0: *const c_char, arg1: *mut int8_t) -> c_int;
  // proto:  const char * QMetaEnum::key(int index);
  fn _ZNK9QMetaEnum3keyEi(qthis: *mut c_void, arg0: c_int) -> *const c_char;
  // proto:  const char * QMetaEnum::valueToKey(int value);
  fn _ZNK9QMetaEnum10valueToKeyEi(qthis: *mut c_void, arg0: c_int) -> *const c_char;
  // proto:  int QMetaEnum::keyCount();
  fn _ZNK9QMetaEnum8keyCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMetaEnum::isValid();
  fn _ZNK9QMetaEnum7isValidEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QMetaEnum)=16
pub struct QMetaEnum {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMetaEnum {
  pub fn value<T: QMetaEnum_value>(&mut self, value: T) -> i32 {
    return value.value(self);
    // return 1;
  }
}

pub trait QMetaEnum_value {
  fn value(self, rsthis: &mut QMetaEnum) -> i32;
}

// proto:  int QMetaEnum::value(int index);
impl<'a> /*trait*/ QMetaEnum_value for (i32) {
  fn value(self, rsthis: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum5valueEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum5valueEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn name<T: QMetaEnum_name>(&mut self, value: T) -> String {
    return value.name(self);
    // return 1;
  }
}

pub trait QMetaEnum_name {
  fn name(self, rsthis: &mut QMetaEnum) -> String;
}

// proto:  const char * QMetaEnum::name();
impl<'a> /*trait*/ QMetaEnum_name for () {
  fn name(self, rsthis: &mut QMetaEnum) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum4nameEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum4nameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn isFlag<T: QMetaEnum_isFlag>(&mut self, value: T) -> i8 {
    return value.isFlag(self);
    // return 1;
  }
}

pub trait QMetaEnum_isFlag {
  fn isFlag(self, rsthis: &mut QMetaEnum) -> i8;
}

// proto:  bool QMetaEnum::isFlag();
impl<'a> /*trait*/ QMetaEnum_isFlag for () {
  fn isFlag(self, rsthis: &mut QMetaEnum) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum6isFlagEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum6isFlagEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn scope<T: QMetaEnum_scope>(&mut self, value: T) -> String {
    return value.scope(self);
    // return 1;
  }
}

pub trait QMetaEnum_scope {
  fn scope(self, rsthis: &mut QMetaEnum) -> String;
}

// proto:  const char * QMetaEnum::scope();
impl<'a> /*trait*/ QMetaEnum_scope for () {
  fn scope(self, rsthis: &mut QMetaEnum) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum5scopeEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum5scopeEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn keyToValue<T: QMetaEnum_keyToValue>(&mut self, value: T) -> i32 {
    return value.keyToValue(self);
    // return 1;
  }
}

pub trait QMetaEnum_keyToValue {
  fn keyToValue(self, rsthis: &mut QMetaEnum) -> i32;
}

// proto:  int QMetaEnum::keyToValue(const char * key, bool * ok);
impl<'a> /*trait*/ QMetaEnum_keyToValue for (&'a  String, &'a mut i8) {
  fn keyToValue(self, rsthis: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum10keyToValueEPKcPb()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK9QMetaEnum10keyToValueEPKcPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn enclosingMetaObject<T: QMetaEnum_enclosingMetaObject>(&mut self, value: T)  {
     value.enclosingMetaObject(self);
    // return 1;
  }
}

pub trait QMetaEnum_enclosingMetaObject {
  fn enclosingMetaObject(self, rsthis: &mut QMetaEnum) ;
}

// proto:  const QMetaObject * QMetaEnum::enclosingMetaObject();
impl<'a> /*trait*/ QMetaEnum_enclosingMetaObject for () {
  fn enclosingMetaObject(self, rsthis: &mut QMetaEnum)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum19enclosingMetaObjectEv()};
     unsafe {_ZNK9QMetaEnum19enclosingMetaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn valueToKeys<T: QMetaEnum_valueToKeys>(&mut self, value: T) -> QByteArray {
    return value.valueToKeys(self);
    // return 1;
  }
}

pub trait QMetaEnum_valueToKeys {
  fn valueToKeys(self, rsthis: &mut QMetaEnum) -> QByteArray;
}

// proto:  QByteArray QMetaEnum::valueToKeys(int value);
impl<'a> /*trait*/ QMetaEnum_valueToKeys for (i32) {
  fn valueToKeys(self, rsthis: &mut QMetaEnum) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum11valueToKeysEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum11valueToKeysEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn NewQMetaEnum<T: QMetaEnum_NewQMetaEnum>(value: T) -> QMetaEnum {
    let rsthis = value.NewQMetaEnum();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaEnum_NewQMetaEnum {
  fn NewQMetaEnum(self) -> QMetaEnum;
}

// proto: void QMetaEnum::NewQMetaEnum();
impl<'a> /*trait*/ QMetaEnum_NewQMetaEnum for () {
  fn NewQMetaEnum(self) -> QMetaEnum {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMetaEnumC1Ev()};
    unsafe {_ZN9QMetaEnumC1Ev(qthis)};
    let rsthis = QMetaEnum{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn keysToValue<T: QMetaEnum_keysToValue>(&mut self, value: T) -> i32 {
    return value.keysToValue(self);
    // return 1;
  }
}

pub trait QMetaEnum_keysToValue {
  fn keysToValue(self, rsthis: &mut QMetaEnum) -> i32;
}

// proto:  int QMetaEnum::keysToValue(const char * keys, bool * ok);
impl<'a> /*trait*/ QMetaEnum_keysToValue for (&'a  String, &'a mut i8) {
  fn keysToValue(self, rsthis: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum11keysToValueEPKcPb()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *mut int8_t;
    let mut ret = unsafe {_ZNK9QMetaEnum11keysToValueEPKcPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn key<T: QMetaEnum_key>(&mut self, value: T) -> String {
    return value.key(self);
    // return 1;
  }
}

pub trait QMetaEnum_key {
  fn key(self, rsthis: &mut QMetaEnum) -> String;
}

// proto:  const char * QMetaEnum::key(int index);
impl<'a> /*trait*/ QMetaEnum_key for (i32) {
  fn key(self, rsthis: &mut QMetaEnum) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum3keyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum3keyEi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn valueToKey<T: QMetaEnum_valueToKey>(&mut self, value: T) -> String {
    return value.valueToKey(self);
    // return 1;
  }
}

pub trait QMetaEnum_valueToKey {
  fn valueToKey(self, rsthis: &mut QMetaEnum) -> String;
}

// proto:  const char * QMetaEnum::valueToKey(int value);
impl<'a> /*trait*/ QMetaEnum_valueToKey for (i32) {
  fn valueToKey(self, rsthis: &mut QMetaEnum) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum10valueToKeyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum10valueToKeyEi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn keyCount<T: QMetaEnum_keyCount>(&mut self, value: T) -> i32 {
    return value.keyCount(self);
    // return 1;
  }
}

pub trait QMetaEnum_keyCount {
  fn keyCount(self, rsthis: &mut QMetaEnum) -> i32;
}

// proto:  int QMetaEnum::keyCount();
impl<'a> /*trait*/ QMetaEnum_keyCount for () {
  fn keyCount(self, rsthis: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum8keyCountEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum8keyCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn isValid<T: QMetaEnum_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QMetaEnum_isValid {
  fn isValid(self, rsthis: &mut QMetaEnum) -> i8;
}

// proto:  bool QMetaEnum::isValid();
impl<'a> /*trait*/ QMetaEnum_isValid for () {
  fn isValid(self, rsthis: &mut QMetaEnum) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum7isValidEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

