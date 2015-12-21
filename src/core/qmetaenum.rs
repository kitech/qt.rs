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
  fn _ZNK9QMetaEnum4nameEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  bool QMetaEnum::isFlag();
  fn _ZNK9QMetaEnum6isFlagEv(qthis: *mut c_void) -> c_char;
  // proto:  const char * QMetaEnum::scope();
  fn _ZNK9QMetaEnum5scopeEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  int QMetaEnum::keyToValue(const char * key, bool * ok);
  fn _ZNK9QMetaEnum10keyToValueEPKcPb(qthis: *mut c_void, arg0: *mut c_char, arg1: *mut c_char) -> c_int;
  // proto:  const QMetaObject * QMetaEnum::enclosingMetaObject();
  fn _ZNK9QMetaEnum19enclosingMetaObjectEv(qthis: *mut c_void);
  // proto:  QByteArray QMetaEnum::valueToKeys(int value);
  fn _ZNK9QMetaEnum11valueToKeysEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QMetaEnum::QMetaEnum();
  fn _ZN9QMetaEnumC1Ev(qthis: *mut c_void);
  // proto:  int QMetaEnum::keysToValue(const char * keys, bool * ok);
  fn _ZNK9QMetaEnum11keysToValueEPKcPb(qthis: *mut c_void, arg0: *mut c_char, arg1: *mut c_char) -> c_int;
  // proto:  const char * QMetaEnum::key(int index);
  fn _ZNK9QMetaEnum3keyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_char;
  // proto:  const char * QMetaEnum::valueToKey(int value);
  fn _ZNK9QMetaEnum10valueToKeyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_char;
  // proto:  int QMetaEnum::keyCount();
  fn _ZNK9QMetaEnum8keyCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMetaEnum::isValid();
  fn _ZNK9QMetaEnum7isValidEv(qthis: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QMetaEnum)=16
pub struct QMetaEnum {
  pub qclsinst: *mut c_void,
}

  // proto:  int QMetaEnum::value(int index);
impl /*struct*/ QMetaEnum {
  pub fn value<RetType, T: QMetaEnum_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QMetaEnum_value<RetType> {
  fn value(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  int QMetaEnum::value(int index);
impl<'a> /*trait*/ QMetaEnum_value<i32> for (i32) {
  fn value(self , rsthis: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum5valueEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum5valueEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const char * QMetaEnum::name();
impl /*struct*/ QMetaEnum {
  pub fn name<RetType, T: QMetaEnum_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QMetaEnum_name<RetType> {
  fn name(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  const char * QMetaEnum::name();
impl<'a> /*trait*/ QMetaEnum_name<String> for () {
  fn name(self , rsthis: &mut QMetaEnum) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum4nameEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum4nameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  bool QMetaEnum::isFlag();
impl /*struct*/ QMetaEnum {
  pub fn isFlag<RetType, T: QMetaEnum_isFlag<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isFlag(self);
    // return 1;
  }
}

pub trait QMetaEnum_isFlag<RetType> {
  fn isFlag(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  bool QMetaEnum::isFlag();
impl<'a> /*trait*/ QMetaEnum_isFlag<i8> for () {
  fn isFlag(self , rsthis: &mut QMetaEnum) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum6isFlagEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum6isFlagEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const char * QMetaEnum::scope();
impl /*struct*/ QMetaEnum {
  pub fn scope<RetType, T: QMetaEnum_scope<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scope(self);
    // return 1;
  }
}

pub trait QMetaEnum_scope<RetType> {
  fn scope(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  const char * QMetaEnum::scope();
impl<'a> /*trait*/ QMetaEnum_scope<String> for () {
  fn scope(self , rsthis: &mut QMetaEnum) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum5scopeEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum5scopeEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  int QMetaEnum::keyToValue(const char * key, bool * ok);
impl /*struct*/ QMetaEnum {
  pub fn keyToValue<RetType, T: QMetaEnum_keyToValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keyToValue(self);
    // return 1;
  }
}

pub trait QMetaEnum_keyToValue<RetType> {
  fn keyToValue(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  int QMetaEnum::keyToValue(const char * key, bool * ok);
impl<'a> /*trait*/ QMetaEnum_keyToValue<i32> for (&'a  String, &'a mut Vec<i8>) {
  fn keyToValue(self , rsthis: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum10keyToValueEPKcPb()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK9QMetaEnum10keyToValueEPKcPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QMetaEnum::enclosingMetaObject();
impl /*struct*/ QMetaEnum {
  pub fn enclosingMetaObject<RetType, T: QMetaEnum_enclosingMetaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.enclosingMetaObject(self);
    // return 1;
  }
}

pub trait QMetaEnum_enclosingMetaObject<RetType> {
  fn enclosingMetaObject(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  const QMetaObject * QMetaEnum::enclosingMetaObject();
impl<'a> /*trait*/ QMetaEnum_enclosingMetaObject<()> for () {
  fn enclosingMetaObject(self , rsthis: &mut QMetaEnum) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum19enclosingMetaObjectEv()};
     unsafe {_ZNK9QMetaEnum19enclosingMetaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QMetaEnum::valueToKeys(int value);
impl /*struct*/ QMetaEnum {
  pub fn valueToKeys<RetType, T: QMetaEnum_valueToKeys<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valueToKeys(self);
    // return 1;
  }
}

pub trait QMetaEnum_valueToKeys<RetType> {
  fn valueToKeys(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  QByteArray QMetaEnum::valueToKeys(int value);
impl<'a> /*trait*/ QMetaEnum_valueToKeys<QByteArray> for (i32) {
  fn valueToKeys(self , rsthis: &mut QMetaEnum) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum11valueToKeysEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum11valueToKeysEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMetaEnum::QMetaEnum();
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

  // proto:  void QMetaEnum::QMetaEnum();
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

  // proto:  int QMetaEnum::keysToValue(const char * keys, bool * ok);
impl /*struct*/ QMetaEnum {
  pub fn keysToValue<RetType, T: QMetaEnum_keysToValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keysToValue(self);
    // return 1;
  }
}

pub trait QMetaEnum_keysToValue<RetType> {
  fn keysToValue(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  int QMetaEnum::keysToValue(const char * keys, bool * ok);
impl<'a> /*trait*/ QMetaEnum_keysToValue<i32> for (&'a  String, &'a mut Vec<i8>) {
  fn keysToValue(self , rsthis: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum11keysToValueEPKcPb()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK9QMetaEnum11keysToValueEPKcPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const char * QMetaEnum::key(int index);
impl /*struct*/ QMetaEnum {
  pub fn key<RetType, T: QMetaEnum_key<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QMetaEnum_key<RetType> {
  fn key(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  const char * QMetaEnum::key(int index);
impl<'a> /*trait*/ QMetaEnum_key<String> for (i32) {
  fn key(self , rsthis: &mut QMetaEnum) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum3keyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum3keyEi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  const char * QMetaEnum::valueToKey(int value);
impl /*struct*/ QMetaEnum {
  pub fn valueToKey<RetType, T: QMetaEnum_valueToKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valueToKey(self);
    // return 1;
  }
}

pub trait QMetaEnum_valueToKey<RetType> {
  fn valueToKey(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  const char * QMetaEnum::valueToKey(int value);
impl<'a> /*trait*/ QMetaEnum_valueToKey<String> for (i32) {
  fn valueToKey(self , rsthis: &mut QMetaEnum) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum10valueToKeyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum10valueToKeyEi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  int QMetaEnum::keyCount();
impl /*struct*/ QMetaEnum {
  pub fn keyCount<RetType, T: QMetaEnum_keyCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keyCount(self);
    // return 1;
  }
}

pub trait QMetaEnum_keyCount<RetType> {
  fn keyCount(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  int QMetaEnum::keyCount();
impl<'a> /*trait*/ QMetaEnum_keyCount<i32> for () {
  fn keyCount(self , rsthis: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum8keyCountEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum8keyCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QMetaEnum::isValid();
impl /*struct*/ QMetaEnum {
  pub fn isValid<RetType, T: QMetaEnum_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QMetaEnum_isValid<RetType> {
  fn isValid(self , rsthis: &mut QMetaEnum) -> RetType;
}

  // proto:  bool QMetaEnum::isValid();
impl<'a> /*trait*/ QMetaEnum_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QMetaEnum) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum7isValidEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

