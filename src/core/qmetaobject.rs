// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qmetaobject.h
// dst-file: /src/core/qmetaobject.rs
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
use super::qbytearray::QByteArray; // 773
use super::qobject::QObject; // 773
use super::qobjectdefs::QGenericReturnArgument; // 773
use super::qvariant::QVariant; // 773
// use super::qmetaobject::QMetaEnum; // 773
// use super::qmetaobject::QMetaMethod; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMetaEnum_Class_Size() -> c_int;
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
  // proto:  QByteArray QMetaEnum::valueToKeys(int value);
  fn _ZNK9QMetaEnum11valueToKeysEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QMetaEnum::keysToValue(const char * keys, bool * ok);
  fn _ZNK9QMetaEnum11keysToValueEPKcPb(qthis: *mut c_void, arg0: *mut c_char, arg1: *mut c_char) -> c_int;
  // proto:  const char * QMetaEnum::key(int index);
  fn _ZNK9QMetaEnum3keyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_char;
  // proto:  const char * QMetaEnum::valueToKey(int value);
  fn _ZNK9QMetaEnum10valueToKeyEi(qthis: *mut c_void, arg0: c_int) -> *mut c_char;
  // proto:  int QMetaEnum::keyCount();
  fn _ZNK9QMetaEnum8keyCountEv(qthis: *mut c_void) -> c_int;
  fn QMetaClassInfo_Class_Size() -> c_int;
  // proto:  const char * QMetaClassInfo::name();
  fn _ZNK14QMetaClassInfo4nameEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  const char * QMetaClassInfo::value();
  fn _ZNK14QMetaClassInfo5valueEv(qthis: *mut c_void) -> *mut c_char;
  fn QMetaMethod_Class_Size() -> c_int;
  // proto:  void QMetaMethod::getParameterTypes(int * types);
  fn _ZNK11QMetaMethod17getParameterTypesEPi(qthis: *mut c_void, arg0: *mut c_int);
  // proto:  int QMetaMethod::returnType();
  fn _ZNK11QMetaMethod10returnTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QByteArray> QMetaMethod::parameterNames();
  fn _ZNK11QMetaMethod14parameterNamesEv(qthis: *mut c_void);
  // proto:  int QMetaMethod::methodIndex();
  fn _ZNK11QMetaMethod11methodIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  const char * QMetaMethod::tag();
  fn _ZNK11QMetaMethod3tagEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  int QMetaMethod::parameterCount();
  fn _ZNK11QMetaMethod14parameterCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMetaMethod::revision();
  fn _ZNK11QMetaMethod8revisionEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QByteArray> QMetaMethod::parameterTypes();
  fn _ZNK11QMetaMethod14parameterTypesEv(qthis: *mut c_void);
  // proto:  QByteArray QMetaMethod::name();
  fn _ZNK11QMetaMethod4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QMetaMethod::methodSignature();
  fn _ZNK11QMetaMethod15methodSignatureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const char * QMetaMethod::typeName();
  fn _ZNK11QMetaMethod8typeNameEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  int QMetaMethod::attributes();
  fn _ZNK11QMetaMethod10attributesEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMetaMethod::parameterType(int index);
  fn _ZNK11QMetaMethod13parameterTypeEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  fn QMetaProperty_Class_Size() -> c_int;
  // proto:  bool QMetaProperty::isEnumType();
  fn _ZNK13QMetaProperty10isEnumTypeEv(qthis: *mut c_void) -> c_char;
  // proto:  void QMetaProperty::QMetaProperty();
  fn dector_ZN13QMetaPropertyC1Ev() -> *mut c_void;
  fn _ZN13QMetaPropertyC1Ev(qthis: *mut c_void);
  // proto:  QVariant QMetaProperty::readOnGadget(const void * gadget);
  fn _ZNK13QMetaProperty12readOnGadgetEPKv(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QMetaProperty::resetOnGadget(void * gadget);
  fn _ZNK13QMetaProperty13resetOnGadgetEPv(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QMetaProperty::propertyIndex();
  fn _ZNK13QMetaProperty13propertyIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMetaProperty::isStored(const QObject * obj);
  fn _ZNK13QMetaProperty8isStoredEPK7QObject(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QMetaEnum QMetaProperty::enumerator();
  fn _ZNK13QMetaProperty10enumeratorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMetaProperty::write(QObject * obj, const QVariant & value);
  fn _ZNK13QMetaProperty5writeEP7QObjectRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  bool QMetaProperty::isResettable();
  fn _ZNK13QMetaProperty12isResettableEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QMetaProperty::isEditable(const QObject * obj);
  fn _ZNK13QMetaProperty10isEditableEPK7QObject(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QMetaProperty::hasStdCppSet();
  fn _ZNK13QMetaProperty12hasStdCppSetEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QMetaProperty::hasNotifySignal();
  fn _ZNK13QMetaProperty15hasNotifySignalEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QMetaProperty::isConstant();
  fn _ZNK13QMetaProperty10isConstantEv(qthis: *mut c_void) -> c_char;
  // proto:  const char * QMetaProperty::typeName();
  fn _ZNK13QMetaProperty8typeNameEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  bool QMetaProperty::isReadable();
  fn _ZNK13QMetaProperty10isReadableEv(qthis: *mut c_void) -> c_char;
  // proto:  int QMetaProperty::userType();
  fn _ZNK13QMetaProperty8userTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMetaProperty::isWritable();
  fn _ZNK13QMetaProperty10isWritableEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QMetaProperty::writeOnGadget(void * gadget, const QVariant & value);
  fn _ZNK13QMetaProperty13writeOnGadgetEPvRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  int QMetaProperty::notifySignalIndex();
  fn _ZNK13QMetaProperty17notifySignalIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMetaProperty::isUser(const QObject * obj);
  fn _ZNK13QMetaProperty6isUserEPK7QObject(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QMetaProperty::isFlagType();
  fn _ZNK13QMetaProperty10isFlagTypeEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QMetaProperty::isFinal();
  fn _ZNK13QMetaProperty7isFinalEv(qthis: *mut c_void) -> c_char;
  // proto:  const char * QMetaProperty::name();
  fn _ZNK13QMetaProperty4nameEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  bool QMetaProperty::reset(QObject * obj);
  fn _ZNK13QMetaProperty5resetEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QMetaProperty::revision();
  fn _ZNK13QMetaProperty8revisionEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMetaProperty::isScriptable(const QObject * obj);
  fn _ZNK13QMetaProperty12isScriptableEPK7QObject(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QVariant QMetaProperty::read(const QObject * obj);
  fn _ZNK13QMetaProperty4readEPK7QObject(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMetaMethod QMetaProperty::notifySignal();
  fn _ZNK13QMetaProperty12notifySignalEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMetaProperty::isDesignable(const QObject * obj);
  fn _ZNK13QMetaProperty12isDesignableEPK7QObject(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QMetaEnum)=16
pub struct QMetaEnum {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QMetaClassInfo)=16
pub struct QMetaClassInfo {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QMetaMethod)=16
pub struct QMetaMethod {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QMetaProperty)=32
pub struct QMetaProperty {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMetaEnum {
  pub fn inheritFrom(qthis: *mut c_void) -> QMetaEnum {
    return QMetaEnum{qclsinst: qthis};
  }
}
  // proto:  int QMetaEnum::value(int index);
impl /*struct*/ QMetaEnum {
  pub fn value<RetType, T: QMetaEnum_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QMetaEnum_value<RetType> {
  fn value(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  int QMetaEnum::value(int index);
impl<'a> /*trait*/ QMetaEnum_value<i32> for (i32) {
  fn value(self , rsthis: & QMetaEnum) -> i32 {
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
  pub fn name<RetType, T: QMetaEnum_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QMetaEnum_name<RetType> {
  fn name(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  const char * QMetaEnum::name();
impl<'a> /*trait*/ QMetaEnum_name<String> for () {
  fn name(self , rsthis: & QMetaEnum) -> String {
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
  pub fn isFlag<RetType, T: QMetaEnum_isFlag<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFlag(self);
    // return 1;
  }
}

pub trait QMetaEnum_isFlag<RetType> {
  fn isFlag(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  bool QMetaEnum::isFlag();
impl<'a> /*trait*/ QMetaEnum_isFlag<i8> for () {
  fn isFlag(self , rsthis: & QMetaEnum) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum6isFlagEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum6isFlagEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const char * QMetaEnum::scope();
impl /*struct*/ QMetaEnum {
  pub fn scope<RetType, T: QMetaEnum_scope<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scope(self);
    // return 1;
  }
}

pub trait QMetaEnum_scope<RetType> {
  fn scope(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  const char * QMetaEnum::scope();
impl<'a> /*trait*/ QMetaEnum_scope<String> for () {
  fn scope(self , rsthis: & QMetaEnum) -> String {
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
  pub fn keyToValue<RetType, T: QMetaEnum_keyToValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyToValue(self);
    // return 1;
  }
}

pub trait QMetaEnum_keyToValue<RetType> {
  fn keyToValue(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  int QMetaEnum::keyToValue(const char * key, bool * ok);
impl<'a> /*trait*/ QMetaEnum_keyToValue<i32> for (&'a  String, &'a mut Vec<i8>) {
  fn keyToValue(self , rsthis: & QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum10keyToValueEPKcPb()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK9QMetaEnum10keyToValueEPKcPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QByteArray QMetaEnum::valueToKeys(int value);
impl /*struct*/ QMetaEnum {
  pub fn valueToKeys<RetType, T: QMetaEnum_valueToKeys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueToKeys(self);
    // return 1;
  }
}

pub trait QMetaEnum_valueToKeys<RetType> {
  fn valueToKeys(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  QByteArray QMetaEnum::valueToKeys(int value);
impl<'a> /*trait*/ QMetaEnum_valueToKeys<QByteArray> for (i32) {
  fn valueToKeys(self , rsthis: & QMetaEnum) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum11valueToKeysEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QMetaEnum11valueToKeysEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMetaEnum::keysToValue(const char * keys, bool * ok);
impl /*struct*/ QMetaEnum {
  pub fn keysToValue<RetType, T: QMetaEnum_keysToValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keysToValue(self);
    // return 1;
  }
}

pub trait QMetaEnum_keysToValue<RetType> {
  fn keysToValue(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  int QMetaEnum::keysToValue(const char * keys, bool * ok);
impl<'a> /*trait*/ QMetaEnum_keysToValue<i32> for (&'a  String, &'a mut Vec<i8>) {
  fn keysToValue(self , rsthis: & QMetaEnum) -> i32 {
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
  pub fn key<RetType, T: QMetaEnum_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QMetaEnum_key<RetType> {
  fn key(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  const char * QMetaEnum::key(int index);
impl<'a> /*trait*/ QMetaEnum_key<String> for (i32) {
  fn key(self , rsthis: & QMetaEnum) -> String {
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
  pub fn valueToKey<RetType, T: QMetaEnum_valueToKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueToKey(self);
    // return 1;
  }
}

pub trait QMetaEnum_valueToKey<RetType> {
  fn valueToKey(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  const char * QMetaEnum::valueToKey(int value);
impl<'a> /*trait*/ QMetaEnum_valueToKey<String> for (i32) {
  fn valueToKey(self , rsthis: & QMetaEnum) -> String {
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
  pub fn keyCount<RetType, T: QMetaEnum_keyCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyCount(self);
    // return 1;
  }
}

pub trait QMetaEnum_keyCount<RetType> {
  fn keyCount(self , rsthis: & QMetaEnum) -> RetType;
}

  // proto:  int QMetaEnum::keyCount();
impl<'a> /*trait*/ QMetaEnum_keyCount<i32> for () {
  fn keyCount(self , rsthis: & QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMetaEnum8keyCountEv()};
    let mut ret = unsafe {_ZNK9QMetaEnum8keyCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaClassInfo {
  pub fn inheritFrom(qthis: *mut c_void) -> QMetaClassInfo {
    return QMetaClassInfo{qclsinst: qthis};
  }
}
  // proto:  const char * QMetaClassInfo::name();
impl /*struct*/ QMetaClassInfo {
  pub fn name<RetType, T: QMetaClassInfo_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QMetaClassInfo_name<RetType> {
  fn name(self , rsthis: & QMetaClassInfo) -> RetType;
}

  // proto:  const char * QMetaClassInfo::name();
impl<'a> /*trait*/ QMetaClassInfo_name<String> for () {
  fn name(self , rsthis: & QMetaClassInfo) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMetaClassInfo4nameEv()};
    let mut ret = unsafe {_ZNK14QMetaClassInfo4nameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  const char * QMetaClassInfo::value();
impl /*struct*/ QMetaClassInfo {
  pub fn value<RetType, T: QMetaClassInfo_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QMetaClassInfo_value<RetType> {
  fn value(self , rsthis: & QMetaClassInfo) -> RetType;
}

  // proto:  const char * QMetaClassInfo::value();
impl<'a> /*trait*/ QMetaClassInfo_value<String> for () {
  fn value(self , rsthis: & QMetaClassInfo) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMetaClassInfo5valueEv()};
    let mut ret = unsafe {_ZNK14QMetaClassInfo5valueEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn inheritFrom(qthis: *mut c_void) -> QMetaMethod {
    return QMetaMethod{qclsinst: qthis};
  }
}
  // proto:  void QMetaMethod::getParameterTypes(int * types);
impl /*struct*/ QMetaMethod {
  pub fn getParameterTypes<RetType, T: QMetaMethod_getParameterTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getParameterTypes(self);
    // return 1;
  }
}

pub trait QMetaMethod_getParameterTypes<RetType> {
  fn getParameterTypes(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  void QMetaMethod::getParameterTypes(int * types);
impl<'a> /*trait*/ QMetaMethod_getParameterTypes<()> for (&'a mut Vec<i32>) {
  fn getParameterTypes(self , rsthis: & QMetaMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod17getParameterTypesEPi()};
    let arg0 = self.as_ptr()  as *mut c_int;
     unsafe {_ZNK11QMetaMethod17getParameterTypesEPi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QMetaMethod::returnType();
impl /*struct*/ QMetaMethod {
  pub fn returnType<RetType, T: QMetaMethod_returnType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.returnType(self);
    // return 1;
  }
}

pub trait QMetaMethod_returnType<RetType> {
  fn returnType(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  int QMetaMethod::returnType();
impl<'a> /*trait*/ QMetaMethod_returnType<i32> for () {
  fn returnType(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod10returnTypeEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod10returnTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QMetaMethod::parameterNames();
impl /*struct*/ QMetaMethod {
  pub fn parameterNames<RetType, T: QMetaMethod_parameterNames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parameterNames(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterNames<RetType> {
  fn parameterNames(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  QList<QByteArray> QMetaMethod::parameterNames();
impl<'a> /*trait*/ QMetaMethod_parameterNames<()> for () {
  fn parameterNames(self , rsthis: & QMetaMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterNamesEv()};
     unsafe {_ZNK11QMetaMethod14parameterNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QMetaMethod::methodIndex();
impl /*struct*/ QMetaMethod {
  pub fn methodIndex<RetType, T: QMetaMethod_methodIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.methodIndex(self);
    // return 1;
  }
}

pub trait QMetaMethod_methodIndex<RetType> {
  fn methodIndex(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  int QMetaMethod::methodIndex();
impl<'a> /*trait*/ QMetaMethod_methodIndex<i32> for () {
  fn methodIndex(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod11methodIndexEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod11methodIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const char * QMetaMethod::tag();
impl /*struct*/ QMetaMethod {
  pub fn tag<RetType, T: QMetaMethod_tag<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tag(self);
    // return 1;
  }
}

pub trait QMetaMethod_tag<RetType> {
  fn tag(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  const char * QMetaMethod::tag();
impl<'a> /*trait*/ QMetaMethod_tag<String> for () {
  fn tag(self , rsthis: & QMetaMethod) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod3tagEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod3tagEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  int QMetaMethod::parameterCount();
impl /*struct*/ QMetaMethod {
  pub fn parameterCount<RetType, T: QMetaMethod_parameterCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parameterCount(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterCount<RetType> {
  fn parameterCount(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  int QMetaMethod::parameterCount();
impl<'a> /*trait*/ QMetaMethod_parameterCount<i32> for () {
  fn parameterCount(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterCountEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod14parameterCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QMetaMethod::revision();
impl /*struct*/ QMetaMethod {
  pub fn revision<RetType, T: QMetaMethod_revision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.revision(self);
    // return 1;
  }
}

pub trait QMetaMethod_revision<RetType> {
  fn revision(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  int QMetaMethod::revision();
impl<'a> /*trait*/ QMetaMethod_revision<i32> for () {
  fn revision(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod8revisionEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QMetaMethod::parameterTypes();
impl /*struct*/ QMetaMethod {
  pub fn parameterTypes<RetType, T: QMetaMethod_parameterTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parameterTypes(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterTypes<RetType> {
  fn parameterTypes(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  QList<QByteArray> QMetaMethod::parameterTypes();
impl<'a> /*trait*/ QMetaMethod_parameterTypes<()> for () {
  fn parameterTypes(self , rsthis: & QMetaMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterTypesEv()};
     unsafe {_ZNK11QMetaMethod14parameterTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QMetaMethod::name();
impl /*struct*/ QMetaMethod {
  pub fn name<RetType, T: QMetaMethod_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QMetaMethod_name<RetType> {
  fn name(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  QByteArray QMetaMethod::name();
impl<'a> /*trait*/ QMetaMethod_name<QByteArray> for () {
  fn name(self , rsthis: & QMetaMethod) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod4nameEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QMetaMethod::methodSignature();
impl /*struct*/ QMetaMethod {
  pub fn methodSignature<RetType, T: QMetaMethod_methodSignature<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.methodSignature(self);
    // return 1;
  }
}

pub trait QMetaMethod_methodSignature<RetType> {
  fn methodSignature(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  QByteArray QMetaMethod::methodSignature();
impl<'a> /*trait*/ QMetaMethod_methodSignature<QByteArray> for () {
  fn methodSignature(self , rsthis: & QMetaMethod) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod15methodSignatureEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod15methodSignatureEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const char * QMetaMethod::typeName();
impl /*struct*/ QMetaMethod {
  pub fn typeName<RetType, T: QMetaMethod_typeName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.typeName(self);
    // return 1;
  }
}

pub trait QMetaMethod_typeName<RetType> {
  fn typeName(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  const char * QMetaMethod::typeName();
impl<'a> /*trait*/ QMetaMethod_typeName<String> for () {
  fn typeName(self , rsthis: & QMetaMethod) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod8typeNameEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod8typeNameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  int QMetaMethod::attributes();
impl /*struct*/ QMetaMethod {
  pub fn attributes<RetType, T: QMetaMethod_attributes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.attributes(self);
    // return 1;
  }
}

pub trait QMetaMethod_attributes<RetType> {
  fn attributes(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  int QMetaMethod::attributes();
impl<'a> /*trait*/ QMetaMethod_attributes<i32> for () {
  fn attributes(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod10attributesEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod10attributesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QMetaMethod::parameterType(int index);
impl /*struct*/ QMetaMethod {
  pub fn parameterType<RetType, T: QMetaMethod_parameterType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parameterType(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterType<RetType> {
  fn parameterType(self , rsthis: & QMetaMethod) -> RetType;
}

  // proto:  int QMetaMethod::parameterType(int index);
impl<'a> /*trait*/ QMetaMethod_parameterType<i32> for (i32) {
  fn parameterType(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod13parameterTypeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QMetaMethod13parameterTypeEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaProperty {
  pub fn inheritFrom(qthis: *mut c_void) -> QMetaProperty {
    return QMetaProperty{qclsinst: qthis};
  }
}
  // proto:  bool QMetaProperty::isEnumType();
impl /*struct*/ QMetaProperty {
  pub fn isEnumType<RetType, T: QMetaProperty_isEnumType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnumType(self);
    // return 1;
  }
}

pub trait QMetaProperty_isEnumType<RetType> {
  fn isEnumType(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isEnumType();
impl<'a> /*trait*/ QMetaProperty_isEnumType<i8> for () {
  fn isEnumType(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty10isEnumTypeEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty10isEnumTypeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMetaProperty::QMetaProperty();
impl /*struct*/ QMetaProperty {
  pub fn New<T: QMetaProperty_New>(value: T) -> QMetaProperty {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaProperty_New {
  fn New(self) -> QMetaProperty;
}

  // proto:  void QMetaProperty::QMetaProperty();
impl<'a> /*trait*/ QMetaProperty_New for () {
  fn New(self) -> QMetaProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMetaPropertyC1Ev()};
    let ctysz: c_int = unsafe{QMetaProperty_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN13QMetaPropertyC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN13QMetaPropertyC1Ev()};
    let rsthis = QMetaProperty{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariant QMetaProperty::readOnGadget(const void * gadget);
impl /*struct*/ QMetaProperty {
  pub fn readOnGadget<RetType, T: QMetaProperty_readOnGadget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readOnGadget(self);
    // return 1;
  }
}

pub trait QMetaProperty_readOnGadget<RetType> {
  fn readOnGadget(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  QVariant QMetaProperty::readOnGadget(const void * gadget);
impl<'a> /*trait*/ QMetaProperty_readOnGadget<QVariant> for (*mut c_void) {
  fn readOnGadget(self , rsthis: & QMetaProperty) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty12readOnGadgetEPKv()};
    let arg0 = self  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty12readOnGadgetEPKv(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::resetOnGadget(void * gadget);
impl /*struct*/ QMetaProperty {
  pub fn resetOnGadget<RetType, T: QMetaProperty_resetOnGadget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetOnGadget(self);
    // return 1;
  }
}

pub trait QMetaProperty_resetOnGadget<RetType> {
  fn resetOnGadget(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::resetOnGadget(void * gadget);
impl<'a> /*trait*/ QMetaProperty_resetOnGadget<i8> for (*mut c_void) {
  fn resetOnGadget(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty13resetOnGadgetEPv()};
    let arg0 = self  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty13resetOnGadgetEPv(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QMetaProperty::propertyIndex();
impl /*struct*/ QMetaProperty {
  pub fn propertyIndex<RetType, T: QMetaProperty_propertyIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.propertyIndex(self);
    // return 1;
  }
}

pub trait QMetaProperty_propertyIndex<RetType> {
  fn propertyIndex(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  int QMetaProperty::propertyIndex();
impl<'a> /*trait*/ QMetaProperty_propertyIndex<i32> for () {
  fn propertyIndex(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty13propertyIndexEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty13propertyIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isStored(const QObject * obj);
impl /*struct*/ QMetaProperty {
  pub fn isStored<RetType, T: QMetaProperty_isStored<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isStored(self);
    // return 1;
  }
}

pub trait QMetaProperty_isStored<RetType> {
  fn isStored(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isStored(const QObject * obj);
impl<'a> /*trait*/ QMetaProperty_isStored<i8> for (&'a QObject) {
  fn isStored(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty8isStoredEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty8isStoredEPK7QObject(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QMetaEnum QMetaProperty::enumerator();
impl /*struct*/ QMetaProperty {
  pub fn enumerator<RetType, T: QMetaProperty_enumerator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.enumerator(self);
    // return 1;
  }
}

pub trait QMetaProperty_enumerator<RetType> {
  fn enumerator(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  QMetaEnum QMetaProperty::enumerator();
impl<'a> /*trait*/ QMetaProperty_enumerator<QMetaEnum> for () {
  fn enumerator(self , rsthis: & QMetaProperty) -> QMetaEnum {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty10enumeratorEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty10enumeratorEv(rsthis.qclsinst)};
    let mut ret1 = QMetaEnum::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::write(QObject * obj, const QVariant & value);
impl /*struct*/ QMetaProperty {
  pub fn write<RetType, T: QMetaProperty_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QMetaProperty_write<RetType> {
  fn write(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::write(QObject * obj, const QVariant & value);
impl<'a> /*trait*/ QMetaProperty_write<i8> for (&'a QObject, &'a QVariant) {
  fn write(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty5writeEP7QObjectRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty5writeEP7QObjectRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isResettable();
impl /*struct*/ QMetaProperty {
  pub fn isResettable<RetType, T: QMetaProperty_isResettable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isResettable(self);
    // return 1;
  }
}

pub trait QMetaProperty_isResettable<RetType> {
  fn isResettable(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isResettable();
impl<'a> /*trait*/ QMetaProperty_isResettable<i8> for () {
  fn isResettable(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty12isResettableEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty12isResettableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isEditable(const QObject * obj);
impl /*struct*/ QMetaProperty {
  pub fn isEditable<RetType, T: QMetaProperty_isEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEditable(self);
    // return 1;
  }
}

pub trait QMetaProperty_isEditable<RetType> {
  fn isEditable(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isEditable(const QObject * obj);
impl<'a> /*trait*/ QMetaProperty_isEditable<i8> for (&'a QObject) {
  fn isEditable(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty10isEditableEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty10isEditableEPK7QObject(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::hasStdCppSet();
impl /*struct*/ QMetaProperty {
  pub fn hasStdCppSet<RetType, T: QMetaProperty_hasStdCppSet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasStdCppSet(self);
    // return 1;
  }
}

pub trait QMetaProperty_hasStdCppSet<RetType> {
  fn hasStdCppSet(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::hasStdCppSet();
impl<'a> /*trait*/ QMetaProperty_hasStdCppSet<i8> for () {
  fn hasStdCppSet(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty12hasStdCppSetEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty12hasStdCppSetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::hasNotifySignal();
impl /*struct*/ QMetaProperty {
  pub fn hasNotifySignal<RetType, T: QMetaProperty_hasNotifySignal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasNotifySignal(self);
    // return 1;
  }
}

pub trait QMetaProperty_hasNotifySignal<RetType> {
  fn hasNotifySignal(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::hasNotifySignal();
impl<'a> /*trait*/ QMetaProperty_hasNotifySignal<i8> for () {
  fn hasNotifySignal(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty15hasNotifySignalEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty15hasNotifySignalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isConstant();
impl /*struct*/ QMetaProperty {
  pub fn isConstant<RetType, T: QMetaProperty_isConstant<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isConstant(self);
    // return 1;
  }
}

pub trait QMetaProperty_isConstant<RetType> {
  fn isConstant(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isConstant();
impl<'a> /*trait*/ QMetaProperty_isConstant<i8> for () {
  fn isConstant(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty10isConstantEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty10isConstantEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const char * QMetaProperty::typeName();
impl /*struct*/ QMetaProperty {
  pub fn typeName<RetType, T: QMetaProperty_typeName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.typeName(self);
    // return 1;
  }
}

pub trait QMetaProperty_typeName<RetType> {
  fn typeName(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  const char * QMetaProperty::typeName();
impl<'a> /*trait*/ QMetaProperty_typeName<String> for () {
  fn typeName(self , rsthis: & QMetaProperty) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty8typeNameEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty8typeNameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isReadable();
impl /*struct*/ QMetaProperty {
  pub fn isReadable<RetType, T: QMetaProperty_isReadable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadable(self);
    // return 1;
  }
}

pub trait QMetaProperty_isReadable<RetType> {
  fn isReadable(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isReadable();
impl<'a> /*trait*/ QMetaProperty_isReadable<i8> for () {
  fn isReadable(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty10isReadableEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QMetaProperty::userType();
impl /*struct*/ QMetaProperty {
  pub fn userType<RetType, T: QMetaProperty_userType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.userType(self);
    // return 1;
  }
}

pub trait QMetaProperty_userType<RetType> {
  fn userType(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  int QMetaProperty::userType();
impl<'a> /*trait*/ QMetaProperty_userType<i32> for () {
  fn userType(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty8userTypeEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty8userTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isWritable();
impl /*struct*/ QMetaProperty {
  pub fn isWritable<RetType, T: QMetaProperty_isWritable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWritable(self);
    // return 1;
  }
}

pub trait QMetaProperty_isWritable<RetType> {
  fn isWritable(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isWritable();
impl<'a> /*trait*/ QMetaProperty_isWritable<i8> for () {
  fn isWritable(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty10isWritableEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::writeOnGadget(void * gadget, const QVariant & value);
impl /*struct*/ QMetaProperty {
  pub fn writeOnGadget<RetType, T: QMetaProperty_writeOnGadget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeOnGadget(self);
    // return 1;
  }
}

pub trait QMetaProperty_writeOnGadget<RetType> {
  fn writeOnGadget(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::writeOnGadget(void * gadget, const QVariant & value);
impl<'a> /*trait*/ QMetaProperty_writeOnGadget<i8> for (*mut c_void, &'a QVariant) {
  fn writeOnGadget(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty13writeOnGadgetEPvRK8QVariant()};
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty13writeOnGadgetEPvRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QMetaProperty::notifySignalIndex();
impl /*struct*/ QMetaProperty {
  pub fn notifySignalIndex<RetType, T: QMetaProperty_notifySignalIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notifySignalIndex(self);
    // return 1;
  }
}

pub trait QMetaProperty_notifySignalIndex<RetType> {
  fn notifySignalIndex(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  int QMetaProperty::notifySignalIndex();
impl<'a> /*trait*/ QMetaProperty_notifySignalIndex<i32> for () {
  fn notifySignalIndex(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty17notifySignalIndexEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty17notifySignalIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isUser(const QObject * obj);
impl /*struct*/ QMetaProperty {
  pub fn isUser<RetType, T: QMetaProperty_isUser<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUser(self);
    // return 1;
  }
}

pub trait QMetaProperty_isUser<RetType> {
  fn isUser(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isUser(const QObject * obj);
impl<'a> /*trait*/ QMetaProperty_isUser<i8> for (&'a QObject) {
  fn isUser(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty6isUserEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty6isUserEPK7QObject(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isFlagType();
impl /*struct*/ QMetaProperty {
  pub fn isFlagType<RetType, T: QMetaProperty_isFlagType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFlagType(self);
    // return 1;
  }
}

pub trait QMetaProperty_isFlagType<RetType> {
  fn isFlagType(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isFlagType();
impl<'a> /*trait*/ QMetaProperty_isFlagType<i8> for () {
  fn isFlagType(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty10isFlagTypeEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty10isFlagTypeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isFinal();
impl /*struct*/ QMetaProperty {
  pub fn isFinal<RetType, T: QMetaProperty_isFinal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFinal(self);
    // return 1;
  }
}

pub trait QMetaProperty_isFinal<RetType> {
  fn isFinal(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isFinal();
impl<'a> /*trait*/ QMetaProperty_isFinal<i8> for () {
  fn isFinal(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty7isFinalEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty7isFinalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const char * QMetaProperty::name();
impl /*struct*/ QMetaProperty {
  pub fn name<RetType, T: QMetaProperty_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QMetaProperty_name<RetType> {
  fn name(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  const char * QMetaProperty::name();
impl<'a> /*trait*/ QMetaProperty_name<String> for () {
  fn name(self , rsthis: & QMetaProperty) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty4nameEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty4nameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  bool QMetaProperty::reset(QObject * obj);
impl /*struct*/ QMetaProperty {
  pub fn reset<RetType, T: QMetaProperty_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QMetaProperty_reset<RetType> {
  fn reset(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::reset(QObject * obj);
impl<'a> /*trait*/ QMetaProperty_reset<i8> for (&'a QObject) {
  fn reset(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty5resetEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty5resetEP7QObject(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QMetaProperty::revision();
impl /*struct*/ QMetaProperty {
  pub fn revision<RetType, T: QMetaProperty_revision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.revision(self);
    // return 1;
  }
}

pub trait QMetaProperty_revision<RetType> {
  fn revision(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  int QMetaProperty::revision();
impl<'a> /*trait*/ QMetaProperty_revision<i32> for () {
  fn revision(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty8revisionEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isScriptable(const QObject * obj);
impl /*struct*/ QMetaProperty {
  pub fn isScriptable<RetType, T: QMetaProperty_isScriptable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isScriptable(self);
    // return 1;
  }
}

pub trait QMetaProperty_isScriptable<RetType> {
  fn isScriptable(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isScriptable(const QObject * obj);
impl<'a> /*trait*/ QMetaProperty_isScriptable<i8> for (&'a QObject) {
  fn isScriptable(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty12isScriptableEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty12isScriptableEPK7QObject(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVariant QMetaProperty::read(const QObject * obj);
impl /*struct*/ QMetaProperty {
  pub fn read<RetType, T: QMetaProperty_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QMetaProperty_read<RetType> {
  fn read(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  QVariant QMetaProperty::read(const QObject * obj);
impl<'a> /*trait*/ QMetaProperty_read<QVariant> for (&'a QObject) {
  fn read(self , rsthis: & QMetaProperty) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty4readEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty4readEPK7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QMetaMethod QMetaProperty::notifySignal();
impl /*struct*/ QMetaProperty {
  pub fn notifySignal<RetType, T: QMetaProperty_notifySignal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notifySignal(self);
    // return 1;
  }
}

pub trait QMetaProperty_notifySignal<RetType> {
  fn notifySignal(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  QMetaMethod QMetaProperty::notifySignal();
impl<'a> /*trait*/ QMetaProperty_notifySignal<QMetaMethod> for () {
  fn notifySignal(self , rsthis: & QMetaProperty) -> QMetaMethod {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty12notifySignalEv()};
    let mut ret = unsafe {_ZNK13QMetaProperty12notifySignalEv(rsthis.qclsinst)};
    let mut ret1 = QMetaMethod::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMetaProperty::isDesignable(const QObject * obj);
impl /*struct*/ QMetaProperty {
  pub fn isDesignable<RetType, T: QMetaProperty_isDesignable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDesignable(self);
    // return 1;
  }
}

pub trait QMetaProperty_isDesignable<RetType> {
  fn isDesignable(self , rsthis: & QMetaProperty) -> RetType;
}

  // proto:  bool QMetaProperty::isDesignable(const QObject * obj);
impl<'a> /*trait*/ QMetaProperty_isDesignable<i8> for (&'a QObject) {
  fn isDesignable(self , rsthis: & QMetaProperty) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMetaProperty12isDesignableEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QMetaProperty12isDesignableEPK7QObject(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

