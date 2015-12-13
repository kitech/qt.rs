// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK9QMetaEnum5valueEi(arg0: c_int) -> i32;
  fn _ZNK9QMetaEnum4nameEv() -> i32;
  fn _ZNK9QMetaEnum6isFlagEv() -> i32;
  fn _ZNK9QMetaEnum5scopeEv() -> i32;
  fn _ZNK9QMetaEnum10keyToValueEPKcPb(arg0: *const c_char, arg1: *mut int8_t) -> i32;
  fn _ZNK9QMetaEnum19enclosingMetaObjectEv() -> i32;
  fn _ZNK9QMetaEnum11valueToKeysEi(arg0: c_int) -> i32;
  fn _ZN9QMetaEnumC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK9QMetaEnum11keysToValueEPKcPb(arg0: *const c_char, arg1: *mut int8_t) -> i32;
  fn _ZNK9QMetaEnum3keyEi(arg0: c_int) -> i32;
  fn _ZNK9QMetaEnum10valueToKeyEi(arg0: c_int) -> i32;
  fn _ZNK9QMetaEnum8keyCountEv() -> i32;
  fn _ZNK9QMetaEnum7isValidEv() -> i32;
}

// body block begin
// class sizeof(QMetaEnum)=16
pub struct QMetaEnum {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMetaEnum {
  pub fn value<T: QMetaEnum_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QMetaEnum_value {
  fn value(self, this: &mut QMetaEnum) -> i32;
}

// proto: int QMetaEnum::value(int index);
impl<'a> /*trait*/ QMetaEnum_value for (i32) {
  fn value(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum5valueEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QMetaEnum5valueEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn name<T: QMetaEnum_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QMetaEnum_name {
  fn name(self, this: &mut QMetaEnum) -> i32;
}

// proto: const char * QMetaEnum::name();
impl<'a> /*trait*/ QMetaEnum_name for () {
  fn name(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum4nameEv()};
    unsafe {_ZNK9QMetaEnum4nameEv()};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn isFlag<T: QMetaEnum_isFlag>(&mut self, value: T) -> i32 {
    value.isFlag(self);
    return 1;
  }
}

pub trait QMetaEnum_isFlag {
  fn isFlag(self, this: &mut QMetaEnum) -> i32;
}

// proto: bool QMetaEnum::isFlag();
impl<'a> /*trait*/ QMetaEnum_isFlag for () {
  fn isFlag(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum6isFlagEv()};
    unsafe {_ZNK9QMetaEnum6isFlagEv()};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn scope<T: QMetaEnum_scope>(&mut self, value: T) -> i32 {
    value.scope(self);
    return 1;
  }
}

pub trait QMetaEnum_scope {
  fn scope(self, this: &mut QMetaEnum) -> i32;
}

// proto: const char * QMetaEnum::scope();
impl<'a> /*trait*/ QMetaEnum_scope for () {
  fn scope(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum5scopeEv()};
    unsafe {_ZNK9QMetaEnum5scopeEv()};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn keyToValue<T: QMetaEnum_keyToValue>(&mut self, value: T) -> i32 {
    value.keyToValue(self);
    return 1;
  }
}

pub trait QMetaEnum_keyToValue {
  fn keyToValue(self, this: &mut QMetaEnum) -> i32;
}

// proto: int QMetaEnum::keyToValue(const char * key, bool * ok);
impl<'a> /*trait*/ QMetaEnum_keyToValue for (&'a  String, &'a mut i8) {
  fn keyToValue(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum10keyToValueEPKcPb()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK9QMetaEnum10keyToValueEPKcPb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn enclosingMetaObject<T: QMetaEnum_enclosingMetaObject>(&mut self, value: T) -> i32 {
    value.enclosingMetaObject(self);
    return 1;
  }
}

pub trait QMetaEnum_enclosingMetaObject {
  fn enclosingMetaObject(self, this: &mut QMetaEnum) -> i32;
}

// proto: const QMetaObject * QMetaEnum::enclosingMetaObject();
impl<'a> /*trait*/ QMetaEnum_enclosingMetaObject for () {
  fn enclosingMetaObject(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum19enclosingMetaObjectEv()};
    unsafe {_ZNK9QMetaEnum19enclosingMetaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn valueToKeys<T: QMetaEnum_valueToKeys>(&mut self, value: T) -> i32 {
    value.valueToKeys(self);
    return 1;
  }
}

pub trait QMetaEnum_valueToKeys {
  fn valueToKeys(self, this: &mut QMetaEnum) -> i32;
}

// proto: QByteArray QMetaEnum::valueToKeys(int value);
impl<'a> /*trait*/ QMetaEnum_valueToKeys for (i32) {
  fn valueToKeys(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum11valueToKeysEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QMetaEnum11valueToKeysEi(arg0)};
    return 1;
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
    value.keysToValue(self);
    return 1;
  }
}

pub trait QMetaEnum_keysToValue {
  fn keysToValue(self, this: &mut QMetaEnum) -> i32;
}

// proto: int QMetaEnum::keysToValue(const char * keys, bool * ok);
impl<'a> /*trait*/ QMetaEnum_keysToValue for (&'a  String, &'a mut i8) {
  fn keysToValue(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum11keysToValueEPKcPb()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *mut int8_t;
    unsafe {_ZNK9QMetaEnum11keysToValueEPKcPb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn key<T: QMetaEnum_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QMetaEnum_key {
  fn key(self, this: &mut QMetaEnum) -> i32;
}

// proto: const char * QMetaEnum::key(int index);
impl<'a> /*trait*/ QMetaEnum_key for (i32) {
  fn key(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum3keyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QMetaEnum3keyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn valueToKey<T: QMetaEnum_valueToKey>(&mut self, value: T) -> i32 {
    value.valueToKey(self);
    return 1;
  }
}

pub trait QMetaEnum_valueToKey {
  fn valueToKey(self, this: &mut QMetaEnum) -> i32;
}

// proto: const char * QMetaEnum::valueToKey(int value);
impl<'a> /*trait*/ QMetaEnum_valueToKey for (i32) {
  fn valueToKey(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum10valueToKeyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QMetaEnum10valueToKeyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn keyCount<T: QMetaEnum_keyCount>(&mut self, value: T) -> i32 {
    value.keyCount(self);
    return 1;
  }
}

pub trait QMetaEnum_keyCount {
  fn keyCount(self, this: &mut QMetaEnum) -> i32;
}

// proto: int QMetaEnum::keyCount();
impl<'a> /*trait*/ QMetaEnum_keyCount for () {
  fn keyCount(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum8keyCountEv()};
    unsafe {_ZNK9QMetaEnum8keyCountEv()};
    return 1;
  }
}

impl /*struct*/ QMetaEnum {
  pub fn isValid<T: QMetaEnum_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QMetaEnum_isValid {
  fn isValid(self, this: &mut QMetaEnum) -> i32;
}

// proto: bool QMetaEnum::isValid();
impl<'a> /*trait*/ QMetaEnum_isValid for () {
  fn isValid(self, this: &mut QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum7isValidEv()};
    unsafe {_ZNK9QMetaEnum7isValidEv()};
    return 1;
  }
}

