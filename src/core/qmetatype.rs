// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qmetatype.h
// dst-file: /src/core/qmetatype.rs
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
use super::qobjectdefs::*; // 773
use super::qbytearray::*; // 773
use super::qdatastream::*; // 773
use super::qdebug::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMetaType_Class_Size() -> c_int;
  // proto: static void QMetaType::destroy(int type, void * data);
  fn C_ZN9QMetaType7destroyEiPv(arg0: c_int, arg1: *mut c_void);
  // proto: static bool QMetaType::hasRegisteredConverterFunction(int fromTypeId, int toTypeId);
  fn C_ZN9QMetaType30hasRegisteredConverterFunctionEii(arg0: c_int, arg1: c_int) -> c_char;
  // proto:  const QMetaObject * QMetaType::metaObject();
  fn C_ZNK9QMetaType10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QMetaType::hasRegisteredDebugStreamOperator(int typeId);
  fn C_ZN9QMetaType32hasRegisteredDebugStreamOperatorEi(arg0: c_int) -> c_char;
  // proto: static void * QMetaType::create(int type, const void * copy);
  fn C_ZN9QMetaType6createEiPKv(arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QMetaType::destroy(void * data);
  fn C_ZNK9QMetaType7destroyEPv(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static int QMetaType::registerTypedef(const char * typeName, int aliasId);
  fn C_ZN9QMetaType15registerTypedefEPKci(arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto: static void QMetaType::destruct(int type, void * where);
  fn C_ZN9QMetaType8destructEiPv(arg0: c_int, arg1: *mut c_void);
  // proto:  bool QMetaType::isValid();
  fn C_ZNK9QMetaType7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static void * QMetaType::construct(int type, void * where, const void * copy);
  fn C_ZN9QMetaType9constructEiPvPKv(arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto: static bool QMetaType::equals(const void * lhs, const void * rhs, int typeId, int * result);
  fn C_ZN9QMetaType6equalsEPKvS1_iPi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_int) -> c_char;
  // proto:  void * QMetaType::construct(void * where, const void * copy);
  fn C_ZNK9QMetaType9constructEPvPKv(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static bool QMetaType::isRegistered(int type);
  fn C_ZN9QMetaType12isRegisteredEi(arg0: c_int) -> c_char;
  // proto: static bool QMetaType::unregisterType(int type);
  fn C_ZN9QMetaType14unregisterTypeEi(arg0: c_int) -> c_char;
  // proto: static const QMetaObject * QMetaType::metaObjectForType(int type);
  fn C_ZN9QMetaType17metaObjectForTypeEi(arg0: c_int) -> *mut c_void;
  // proto: static bool QMetaType::load(QDataStream & stream, int type, void * data);
  fn C_ZN9QMetaType4loadER11QDataStreamiPv(arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void * QMetaType::create(const void * copy);
  fn C_ZNK9QMetaType6createEPKv(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto: static int QMetaType::sizeOf(int type);
  fn C_ZN9QMetaType6sizeOfEi(arg0: c_int) -> c_int;
  // proto: static bool QMetaType::hasRegisteredComparators(int typeId);
  fn C_ZN9QMetaType24hasRegisteredComparatorsEi(arg0: c_int) -> c_char;
  // proto: static bool QMetaType::save(QDataStream & stream, int type, const void * data);
  fn C_ZN9QMetaType4saveER11QDataStreamiPKv(arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QMetaType::destruct(void * data);
  fn C_ZNK9QMetaType8destructEPv(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMetaType::~QMetaType();
  fn C_ZN9QMetaTypeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static int QMetaType::type(const char * typeName);
  fn C_ZN9QMetaType4typeEPKc(arg0: *mut c_char) -> c_int;
  // proto: static int QMetaType::type(const ::QByteArray & typeName);
  fn C_ZN9QMetaType4typeERK10QByteArray(arg0: *mut c_void) -> c_int;
  // proto: static bool QMetaType::debugStream(QDebug & dbg, const void * rhs, int typeId);
  fn C_ZN9QMetaType11debugStreamER6QDebugPKvi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  int QMetaType::sizeOf();
  fn C_ZNK9QMetaType6sizeOfEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static const char * QMetaType::typeName(int type);
  fn C_ZN9QMetaType8typeNameEi(arg0: c_int) -> *mut c_char;
  // proto: static bool QMetaType::convert(const void * from, int fromTypeId, void * to, int toTypeId);
  fn C_ZN9QMetaType7convertEPKviPvi(arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> c_char;
  // proto:  void QMetaType::QMetaType(const int type);
  fn C_ZN9QMetaTypeC2Ei(arg0: c_int) -> u64;
  // proto: static int QMetaType::registerNormalizedTypedef(const ::QByteArray & normalizedTypeName, int aliasId);
  fn C_ZN9QMetaType25registerNormalizedTypedefERK10QByteArrayi(arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto: static bool QMetaType::compare(const void * lhs, const void * rhs, int typeId, int * result);
  fn C_ZN9QMetaType7compareEPKvS1_iPi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_int) -> c_char;
  // proto:  bool QMetaType::isRegistered();
  fn C_ZNK9QMetaType12isRegisteredEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QMetaType)=80
#[derive(Default)]
pub struct QMetaType {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMetaType {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMetaType {
    return QMetaType{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static void QMetaType::destroy(int type, void * data);
impl /*struct*/ QMetaType {
  pub fn destroy_s<RetType, T: QMetaType_destroy_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.destroy_s();
    // return 1;
  }
}

pub trait QMetaType_destroy_s<RetType> {
  fn destroy_s(self ) -> RetType;
}

  // proto: static void QMetaType::destroy(int type, void * data);
impl<'a> /*trait*/ QMetaType_destroy_s<()> for (i32, *mut c_void) {
  fn destroy_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType7destroyEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
     unsafe {C_ZN9QMetaType7destroyEiPv(arg0, arg1)};
    // return 1;
  }
}

  // proto: static bool QMetaType::hasRegisteredConverterFunction(int fromTypeId, int toTypeId);
impl /*struct*/ QMetaType {
  pub fn hasRegisteredConverterFunction_s<RetType, T: QMetaType_hasRegisteredConverterFunction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasRegisteredConverterFunction_s();
    // return 1;
  }
}

pub trait QMetaType_hasRegisteredConverterFunction_s<RetType> {
  fn hasRegisteredConverterFunction_s(self ) -> RetType;
}

  // proto: static bool QMetaType::hasRegisteredConverterFunction(int fromTypeId, int toTypeId);
impl<'a> /*trait*/ QMetaType_hasRegisteredConverterFunction_s<i8> for (i32, i32) {
  fn hasRegisteredConverterFunction_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType30hasRegisteredConverterFunctionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType30hasRegisteredConverterFunctionEii(arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QMetaType::metaObject();
impl /*struct*/ QMetaType {
  pub fn metaObject<RetType, T: QMetaType_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMetaType_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  const QMetaObject * QMetaType::metaObject();
impl<'a> /*trait*/ QMetaType_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QMetaType) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK9QMetaType10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QMetaType10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QMetaType::hasRegisteredDebugStreamOperator(int typeId);
impl /*struct*/ QMetaType {
  pub fn hasRegisteredDebugStreamOperator_s<RetType, T: QMetaType_hasRegisteredDebugStreamOperator_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasRegisteredDebugStreamOperator_s();
    // return 1;
  }
}

pub trait QMetaType_hasRegisteredDebugStreamOperator_s<RetType> {
  fn hasRegisteredDebugStreamOperator_s(self ) -> RetType;
}

  // proto: static bool QMetaType::hasRegisteredDebugStreamOperator(int typeId);
impl<'a> /*trait*/ QMetaType_hasRegisteredDebugStreamOperator_s<i8> for (i32) {
  fn hasRegisteredDebugStreamOperator_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType32hasRegisteredDebugStreamOperatorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType32hasRegisteredDebugStreamOperatorEi(arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static void * QMetaType::create(int type, const void * copy);
impl /*struct*/ QMetaType {
  pub fn create_s<RetType, T: QMetaType_create_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.create_s();
    // return 1;
  }
}

pub trait QMetaType_create_s<RetType> {
  fn create_s(self ) -> RetType;
}

  // proto: static void * QMetaType::create(int type, const void * copy);
impl<'a> /*trait*/ QMetaType_create_s<*mut c_void> for (i32, *mut c_void) {
  fn create_s(self ) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType6createEiPKv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
    let mut ret = unsafe {C_ZN9QMetaType6createEiPKv(arg0, arg1)};
    return ret as *mut c_void; // 1
    // return 1;
  }
}

  // proto:  void QMetaType::destroy(void * data);
impl /*struct*/ QMetaType {
  pub fn destroy<RetType, T: QMetaType_destroy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QMetaType_destroy<RetType> {
  fn destroy(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  void QMetaType::destroy(void * data);
impl<'a> /*trait*/ QMetaType_destroy<()> for (*mut c_void) {
  fn destroy(self , rsthis: & QMetaType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK9QMetaType7destroyEPv()};
    let arg0 = self  as *mut c_void;
     unsafe {C_ZNK9QMetaType7destroyEPv(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static int QMetaType::registerTypedef(const char * typeName, int aliasId);
impl /*struct*/ QMetaType {
  pub fn registerTypedef_s<RetType, T: QMetaType_registerTypedef_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerTypedef_s();
    // return 1;
  }
}

pub trait QMetaType_registerTypedef_s<RetType> {
  fn registerTypedef_s(self ) -> RetType;
}

  // proto: static int QMetaType::registerTypedef(const char * typeName, int aliasId);
impl<'a> /*trait*/ QMetaType_registerTypedef_s<i32> for (&'a  String, i32) {
  fn registerTypedef_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType15registerTypedefEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType15registerTypedefEPKci(arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static void QMetaType::destruct(int type, void * where);
impl /*struct*/ QMetaType {
  pub fn destruct_s<RetType, T: QMetaType_destruct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.destruct_s();
    // return 1;
  }
}

pub trait QMetaType_destruct_s<RetType> {
  fn destruct_s(self ) -> RetType;
}

  // proto: static void QMetaType::destruct(int type, void * where);
impl<'a> /*trait*/ QMetaType_destruct_s<()> for (i32, *mut c_void) {
  fn destruct_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType8destructEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
     unsafe {C_ZN9QMetaType8destructEiPv(arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QMetaType::isValid();
impl /*struct*/ QMetaType {
  pub fn isValid<RetType, T: QMetaType_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QMetaType_isValid<RetType> {
  fn isValid(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  bool QMetaType::isValid();
impl<'a> /*trait*/ QMetaType_isValid<i8> for () {
  fn isValid(self , rsthis: & QMetaType) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK9QMetaType7isValidEv()};
    let mut ret = unsafe {C_ZNK9QMetaType7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static void * QMetaType::construct(int type, void * where, const void * copy);
impl /*struct*/ QMetaType {
  pub fn construct_s<RetType, T: QMetaType_construct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.construct_s();
    // return 1;
  }
}

pub trait QMetaType_construct_s<RetType> {
  fn construct_s(self ) -> RetType;
}

  // proto: static void * QMetaType::construct(int type, void * where, const void * copy);
impl<'a> /*trait*/ QMetaType_construct_s<*mut c_void> for (i32, *mut c_void, *mut c_void) {
  fn construct_s(self ) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType9constructEiPvPKv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2  as *mut c_void;
    let mut ret = unsafe {C_ZN9QMetaType9constructEiPvPKv(arg0, arg1, arg2)};
    return ret as *mut c_void; // 1
    // return 1;
  }
}

  // proto: static bool QMetaType::equals(const void * lhs, const void * rhs, int typeId, int * result);
impl /*struct*/ QMetaType {
  pub fn equals_s<RetType, T: QMetaType_equals_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.equals_s();
    // return 1;
  }
}

pub trait QMetaType_equals_s<RetType> {
  fn equals_s(self ) -> RetType;
}

  // proto: static bool QMetaType::equals(const void * lhs, const void * rhs, int typeId, int * result);
impl<'a> /*trait*/ QMetaType_equals_s<i8> for (*mut c_void, *mut c_void, i32, &'a mut Vec<i32>) {
  fn equals_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType6equalsEPKvS1_iPi()};
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
    let mut ret = unsafe {C_ZN9QMetaType6equalsEPKvS1_iPi(arg0, arg1, arg2, arg3)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void * QMetaType::construct(void * where, const void * copy);
impl /*struct*/ QMetaType {
  pub fn construct<RetType, T: QMetaType_construct<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.construct(self);
    // return 1;
  }
}

pub trait QMetaType_construct<RetType> {
  fn construct(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  void * QMetaType::construct(void * where, const void * copy);
impl<'a> /*trait*/ QMetaType_construct<*mut c_void> for (*mut c_void, *mut c_void) {
  fn construct(self , rsthis: & QMetaType) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK9QMetaType9constructEPvPKv()};
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QMetaType9constructEPvPKv(rsthis.qclsinst, arg0, arg1)};
    return ret as *mut c_void; // 1
    // return 1;
  }
}

  // proto: static bool QMetaType::isRegistered(int type);
impl /*struct*/ QMetaType {
  pub fn isRegistered_s<RetType, T: QMetaType_isRegistered_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isRegistered_s();
    // return 1;
  }
}

pub trait QMetaType_isRegistered_s<RetType> {
  fn isRegistered_s(self ) -> RetType;
}

  // proto: static bool QMetaType::isRegistered(int type);
impl<'a> /*trait*/ QMetaType_isRegistered_s<i8> for (i32) {
  fn isRegistered_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType12isRegisteredEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType12isRegisteredEi(arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static bool QMetaType::unregisterType(int type);
impl /*struct*/ QMetaType {
  pub fn unregisterType_s<RetType, T: QMetaType_unregisterType_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.unregisterType_s();
    // return 1;
  }
}

pub trait QMetaType_unregisterType_s<RetType> {
  fn unregisterType_s(self ) -> RetType;
}

  // proto: static bool QMetaType::unregisterType(int type);
impl<'a> /*trait*/ QMetaType_unregisterType_s<i8> for (i32) {
  fn unregisterType_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType14unregisterTypeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType14unregisterTypeEi(arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static const QMetaObject * QMetaType::metaObjectForType(int type);
impl /*struct*/ QMetaType {
  pub fn metaObjectForType_s<RetType, T: QMetaType_metaObjectForType_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.metaObjectForType_s();
    // return 1;
  }
}

pub trait QMetaType_metaObjectForType_s<RetType> {
  fn metaObjectForType_s(self ) -> RetType;
}

  // proto: static const QMetaObject * QMetaType::metaObjectForType(int type);
impl<'a> /*trait*/ QMetaType_metaObjectForType_s<QMetaObject> for (i32) {
  fn metaObjectForType_s(self ) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType17metaObjectForTypeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType17metaObjectForTypeEi(arg0)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QMetaType::load(QDataStream & stream, int type, void * data);
impl /*struct*/ QMetaType {
  pub fn load_s<RetType, T: QMetaType_load_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.load_s();
    // return 1;
  }
}

pub trait QMetaType_load_s<RetType> {
  fn load_s(self ) -> RetType;
}

  // proto: static bool QMetaType::load(QDataStream & stream, int type, void * data);
impl<'a> /*trait*/ QMetaType_load_s<i8> for (&'a QDataStream, i32, *mut c_void) {
  fn load_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType4loadER11QDataStreamiPv()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_void;
    let mut ret = unsafe {C_ZN9QMetaType4loadER11QDataStreamiPv(arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void * QMetaType::create(const void * copy);
impl /*struct*/ QMetaType {
  pub fn create<RetType, T: QMetaType_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QMetaType_create<RetType> {
  fn create(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  void * QMetaType::create(const void * copy);
impl<'a> /*trait*/ QMetaType_create<*mut c_void> for (*mut c_void) {
  fn create(self , rsthis: & QMetaType) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK9QMetaType6createEPKv()};
    let arg0 = self  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QMetaType6createEPKv(rsthis.qclsinst, arg0)};
    return ret as *mut c_void; // 1
    // return 1;
  }
}

  // proto: static int QMetaType::sizeOf(int type);
impl /*struct*/ QMetaType {
  pub fn sizeOf_s<RetType, T: QMetaType_sizeOf_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sizeOf_s();
    // return 1;
  }
}

pub trait QMetaType_sizeOf_s<RetType> {
  fn sizeOf_s(self ) -> RetType;
}

  // proto: static int QMetaType::sizeOf(int type);
impl<'a> /*trait*/ QMetaType_sizeOf_s<i32> for (i32) {
  fn sizeOf_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType6sizeOfEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType6sizeOfEi(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static bool QMetaType::hasRegisteredComparators(int typeId);
impl /*struct*/ QMetaType {
  pub fn hasRegisteredComparators_s<RetType, T: QMetaType_hasRegisteredComparators_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasRegisteredComparators_s();
    // return 1;
  }
}

pub trait QMetaType_hasRegisteredComparators_s<RetType> {
  fn hasRegisteredComparators_s(self ) -> RetType;
}

  // proto: static bool QMetaType::hasRegisteredComparators(int typeId);
impl<'a> /*trait*/ QMetaType_hasRegisteredComparators_s<i8> for (i32) {
  fn hasRegisteredComparators_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType24hasRegisteredComparatorsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType24hasRegisteredComparatorsEi(arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static bool QMetaType::save(QDataStream & stream, int type, const void * data);
impl /*struct*/ QMetaType {
  pub fn save_s<RetType, T: QMetaType_save_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.save_s();
    // return 1;
  }
}

pub trait QMetaType_save_s<RetType> {
  fn save_s(self ) -> RetType;
}

  // proto: static bool QMetaType::save(QDataStream & stream, int type, const void * data);
impl<'a> /*trait*/ QMetaType_save_s<i8> for (&'a QDataStream, i32, *mut c_void) {
  fn save_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType4saveER11QDataStreamiPKv()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_void;
    let mut ret = unsafe {C_ZN9QMetaType4saveER11QDataStreamiPKv(arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMetaType::destruct(void * data);
impl /*struct*/ QMetaType {
  pub fn destruct<RetType, T: QMetaType_destruct<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destruct(self);
    // return 1;
  }
}

pub trait QMetaType_destruct<RetType> {
  fn destruct(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  void QMetaType::destruct(void * data);
impl<'a> /*trait*/ QMetaType_destruct<()> for (*mut c_void) {
  fn destruct(self , rsthis: & QMetaType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK9QMetaType8destructEPv()};
    let arg0 = self  as *mut c_void;
     unsafe {C_ZNK9QMetaType8destructEPv(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMetaType::~QMetaType();
impl /*struct*/ QMetaType {
  pub fn free<RetType, T: QMetaType_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMetaType_free<RetType> {
  fn free(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  void QMetaType::~QMetaType();
impl<'a> /*trait*/ QMetaType_free<()> for () {
  fn free(self , rsthis: & QMetaType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaTypeD2Ev()};
     unsafe {C_ZN9QMetaTypeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static int QMetaType::type(const char * typeName);
impl /*struct*/ QMetaType {
  pub fn type_s<RetType, T: QMetaType_type_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.type_s();
    // return 1;
  }
}

pub trait QMetaType_type_s<RetType> {
  fn type_s(self ) -> RetType;
}

  // proto: static int QMetaType::type(const char * typeName);
impl<'a> /*trait*/ QMetaType_type_s<i32> for (&'a  String) {
  fn type_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType4typeEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN9QMetaType4typeEPKc(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static int QMetaType::type(const ::QByteArray & typeName);
impl<'a> /*trait*/ QMetaType_type_s<i32> for (&'a QByteArray) {
  fn type_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType4typeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN9QMetaType4typeERK10QByteArray(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static bool QMetaType::debugStream(QDebug & dbg, const void * rhs, int typeId);
impl /*struct*/ QMetaType {
  pub fn debugStream_s<RetType, T: QMetaType_debugStream_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.debugStream_s();
    // return 1;
  }
}

pub trait QMetaType_debugStream_s<RetType> {
  fn debugStream_s(self ) -> RetType;
}

  // proto: static bool QMetaType::debugStream(QDebug & dbg, const void * rhs, int typeId);
impl<'a> /*trait*/ QMetaType_debugStream_s<i8> for (&'a QDebug, *mut c_void, i32) {
  fn debugStream_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType11debugStreamER6QDebugPKvi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType11debugStreamER6QDebugPKvi(arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QMetaType::sizeOf();
impl /*struct*/ QMetaType {
  pub fn sizeOf<RetType, T: QMetaType_sizeOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeOf(self);
    // return 1;
  }
}

pub trait QMetaType_sizeOf<RetType> {
  fn sizeOf(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  int QMetaType::sizeOf();
impl<'a> /*trait*/ QMetaType_sizeOf<i32> for () {
  fn sizeOf(self , rsthis: & QMetaType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK9QMetaType6sizeOfEv()};
    let mut ret = unsafe {C_ZNK9QMetaType6sizeOfEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static const char * QMetaType::typeName(int type);
impl /*struct*/ QMetaType {
  pub fn typeName_s<RetType, T: QMetaType_typeName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.typeName_s();
    // return 1;
  }
}

pub trait QMetaType_typeName_s<RetType> {
  fn typeName_s(self ) -> RetType;
}

  // proto: static const char * QMetaType::typeName(int type);
impl<'a> /*trait*/ QMetaType_typeName_s<String> for (i32) {
  fn typeName_s(self ) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType8typeNameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType8typeNameEi(arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static bool QMetaType::convert(const void * from, int fromTypeId, void * to, int toTypeId);
impl /*struct*/ QMetaType {
  pub fn convert_s<RetType, T: QMetaType_convert_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.convert_s();
    // return 1;
  }
}

pub trait QMetaType_convert_s<RetType> {
  fn convert_s(self ) -> RetType;
}

  // proto: static bool QMetaType::convert(const void * from, int fromTypeId, void * to, int toTypeId);
impl<'a> /*trait*/ QMetaType_convert_s<i8> for (*mut c_void, i32, *mut c_void, i32) {
  fn convert_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType7convertEPKviPvi()};
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType7convertEPKviPvi(arg0, arg1, arg2, arg3)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMetaType::QMetaType(const int type);
impl /*struct*/ QMetaType {
  pub fn new<T: QMetaType_new>(value: T) -> QMetaType {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaType_new {
  fn new(self) -> QMetaType;
}

  // proto:  void QMetaType::QMetaType(const int type);
impl<'a> /*trait*/ QMetaType_new for (i32) {
  fn new(self) -> QMetaType {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaTypeC2Ei()};
    let ctysz: c_int = unsafe{QMetaType_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN9QMetaTypeC2Ei(arg0)};
    let rsthis = QMetaType{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static int QMetaType::registerNormalizedTypedef(const ::QByteArray & normalizedTypeName, int aliasId);
impl /*struct*/ QMetaType {
  pub fn registerNormalizedTypedef_s<RetType, T: QMetaType_registerNormalizedTypedef_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerNormalizedTypedef_s();
    // return 1;
  }
}

pub trait QMetaType_registerNormalizedTypedef_s<RetType> {
  fn registerNormalizedTypedef_s(self ) -> RetType;
}

  // proto: static int QMetaType::registerNormalizedTypedef(const ::QByteArray & normalizedTypeName, int aliasId);
impl<'a> /*trait*/ QMetaType_registerNormalizedTypedef_s<i32> for (&'a QByteArray, i32) {
  fn registerNormalizedTypedef_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType25registerNormalizedTypedefERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN9QMetaType25registerNormalizedTypedefERK10QByteArrayi(arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static bool QMetaType::compare(const void * lhs, const void * rhs, int typeId, int * result);
impl /*struct*/ QMetaType {
  pub fn compare_s<RetType, T: QMetaType_compare_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_s();
    // return 1;
  }
}

pub trait QMetaType_compare_s<RetType> {
  fn compare_s(self ) -> RetType;
}

  // proto: static bool QMetaType::compare(const void * lhs, const void * rhs, int typeId, int * result);
impl<'a> /*trait*/ QMetaType_compare_s<i8> for (*mut c_void, *mut c_void, i32, &'a mut Vec<i32>) {
  fn compare_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN9QMetaType7compareEPKvS1_iPi()};
    let arg0 = self.0  as *mut c_void;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
    let mut ret = unsafe {C_ZN9QMetaType7compareEPKvS1_iPi(arg0, arg1, arg2, arg3)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QMetaType::isRegistered();
impl /*struct*/ QMetaType {
  pub fn isRegistered<RetType, T: QMetaType_isRegistered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRegistered(self);
    // return 1;
  }
}

pub trait QMetaType_isRegistered<RetType> {
  fn isRegistered(self , rsthis: & QMetaType) -> RetType;
}

  // proto:  bool QMetaType::isRegistered();
impl<'a> /*trait*/ QMetaType_isRegistered<i8> for () {
  fn isRegistered(self , rsthis: & QMetaType) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK9QMetaType12isRegisteredEv()};
    let mut ret = unsafe {C_ZNK9QMetaType12isRegisteredEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

// <= body block end

