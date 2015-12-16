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
  // proto:  QList<QByteArray> QMetaMethod::parameterTypes();
  fn _ZNK11QMetaMethod14parameterTypesEv(qthis: *mut c_void) ;
  // proto:  QList<QByteArray> QMetaMethod::parameterNames();
  fn _ZNK11QMetaMethod14parameterNamesEv(qthis: *mut c_void) ;
  // proto:  QByteArray QMetaMethod::methodSignature();
  fn _ZNK11QMetaMethod15methodSignatureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const char * QMetaMethod::typeName();
  fn _ZNK11QMetaMethod8typeNameEv(qthis: *mut c_void) -> *const c_char;
  // proto:  int QMetaMethod::attributes();
  fn _ZNK11QMetaMethod10attributesEv(qthis: *mut c_void) -> c_int;
  // proto:  void QMetaMethod::getParameterTypes(int * types);
  fn _ZNK11QMetaMethod17getParameterTypesEPi(qthis: *mut c_void, arg0: *mut c_int) ;
  // proto:  void QMetaMethod::NewQMetaMethod();
  fn _ZN11QMetaMethodC1Ev(qthis: *mut c_void) ;
  // proto:  int QMetaMethod::parameterType(int index);
  fn _ZNK11QMetaMethod13parameterTypeEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QByteArray QMetaMethod::name();
  fn _ZNK11QMetaMethod4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMetaMethod::returnType();
  fn _ZNK11QMetaMethod10returnTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMetaMethod::methodIndex();
  fn _ZNK11QMetaMethod11methodIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMetaMethod::parameterCount();
  fn _ZNK11QMetaMethod14parameterCountEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QMetaMethod::enclosingMetaObject();
  fn _ZNK11QMetaMethod19enclosingMetaObjectEv(qthis: *mut c_void) ;
  // proto:  int QMetaMethod::revision();
  fn _ZNK11QMetaMethod8revisionEv(qthis: *mut c_void) -> c_int;
  // proto:  const char * QMetaMethod::tag();
  fn _ZNK11QMetaMethod3tagEv(qthis: *mut c_void) -> *const c_char;
  // proto:  bool QMetaMethod::isValid();
  fn _ZNK11QMetaMethod7isValidEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QMetaMethod)=16
pub struct QMetaMethod {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMetaMethod {
  pub fn parameterTypes<T: QMetaMethod_parameterTypes>(&mut self, value: T)  {
     value.parameterTypes(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterTypes {
  fn parameterTypes(self, rsthis: &mut QMetaMethod) ;
}

// proto:  QList<QByteArray> QMetaMethod::parameterTypes();
impl<'a> /*trait*/ QMetaMethod_parameterTypes for () {
  fn parameterTypes(self, rsthis: &mut QMetaMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterTypesEv()};
     unsafe {_ZNK11QMetaMethod14parameterTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn parameterNames<T: QMetaMethod_parameterNames>(&mut self, value: T)  {
     value.parameterNames(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterNames {
  fn parameterNames(self, rsthis: &mut QMetaMethod) ;
}

// proto:  QList<QByteArray> QMetaMethod::parameterNames();
impl<'a> /*trait*/ QMetaMethod_parameterNames for () {
  fn parameterNames(self, rsthis: &mut QMetaMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterNamesEv()};
     unsafe {_ZNK11QMetaMethod14parameterNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn methodSignature<T: QMetaMethod_methodSignature>(&mut self, value: T) -> QByteArray {
    return value.methodSignature(self);
    // return 1;
  }
}

pub trait QMetaMethod_methodSignature {
  fn methodSignature(self, rsthis: &mut QMetaMethod) -> QByteArray;
}

// proto:  QByteArray QMetaMethod::methodSignature();
impl<'a> /*trait*/ QMetaMethod_methodSignature for () {
  fn methodSignature(self, rsthis: &mut QMetaMethod) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod15methodSignatureEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod15methodSignatureEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn typeName<T: QMetaMethod_typeName>(&mut self, value: T) -> String {
    return value.typeName(self);
    // return 1;
  }
}

pub trait QMetaMethod_typeName {
  fn typeName(self, rsthis: &mut QMetaMethod) -> String;
}

// proto:  const char * QMetaMethod::typeName();
impl<'a> /*trait*/ QMetaMethod_typeName for () {
  fn typeName(self, rsthis: &mut QMetaMethod) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod8typeNameEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod8typeNameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn attributes<T: QMetaMethod_attributes>(&mut self, value: T) -> i32 {
    return value.attributes(self);
    // return 1;
  }
}

pub trait QMetaMethod_attributes {
  fn attributes(self, rsthis: &mut QMetaMethod) -> i32;
}

// proto:  int QMetaMethod::attributes();
impl<'a> /*trait*/ QMetaMethod_attributes for () {
  fn attributes(self, rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod10attributesEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod10attributesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn getParameterTypes<T: QMetaMethod_getParameterTypes>(&mut self, value: T)  {
     value.getParameterTypes(self);
    // return 1;
  }
}

pub trait QMetaMethod_getParameterTypes {
  fn getParameterTypes(self, rsthis: &mut QMetaMethod) ;
}

// proto:  void QMetaMethod::getParameterTypes(int * types);
impl<'a> /*trait*/ QMetaMethod_getParameterTypes for (&'a mut i32) {
  fn getParameterTypes(self, rsthis: &mut QMetaMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod17getParameterTypesEPi()};
    let arg0 = self  as *mut c_int;
     unsafe {_ZNK11QMetaMethod17getParameterTypesEPi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn NewQMetaMethod<T: QMetaMethod_NewQMetaMethod>(value: T) -> QMetaMethod {
    let rsthis = value.NewQMetaMethod();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaMethod_NewQMetaMethod {
  fn NewQMetaMethod(self) -> QMetaMethod;
}

// proto: void QMetaMethod::NewQMetaMethod();
impl<'a> /*trait*/ QMetaMethod_NewQMetaMethod for () {
  fn NewQMetaMethod(self) -> QMetaMethod {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMetaMethodC1Ev()};
    unsafe {_ZN11QMetaMethodC1Ev(qthis)};
    let rsthis = QMetaMethod{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn parameterType<T: QMetaMethod_parameterType>(&mut self, value: T) -> i32 {
    return value.parameterType(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterType {
  fn parameterType(self, rsthis: &mut QMetaMethod) -> i32;
}

// proto:  int QMetaMethod::parameterType(int index);
impl<'a> /*trait*/ QMetaMethod_parameterType for (i32) {
  fn parameterType(self, rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod13parameterTypeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QMetaMethod13parameterTypeEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn name<T: QMetaMethod_name>(&mut self, value: T) -> QByteArray {
    return value.name(self);
    // return 1;
  }
}

pub trait QMetaMethod_name {
  fn name(self, rsthis: &mut QMetaMethod) -> QByteArray;
}

// proto:  QByteArray QMetaMethod::name();
impl<'a> /*trait*/ QMetaMethod_name for () {
  fn name(self, rsthis: &mut QMetaMethod) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod4nameEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn returnType<T: QMetaMethod_returnType>(&mut self, value: T) -> i32 {
    return value.returnType(self);
    // return 1;
  }
}

pub trait QMetaMethod_returnType {
  fn returnType(self, rsthis: &mut QMetaMethod) -> i32;
}

// proto:  int QMetaMethod::returnType();
impl<'a> /*trait*/ QMetaMethod_returnType for () {
  fn returnType(self, rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod10returnTypeEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod10returnTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn methodIndex<T: QMetaMethod_methodIndex>(&mut self, value: T) -> i32 {
    return value.methodIndex(self);
    // return 1;
  }
}

pub trait QMetaMethod_methodIndex {
  fn methodIndex(self, rsthis: &mut QMetaMethod) -> i32;
}

// proto:  int QMetaMethod::methodIndex();
impl<'a> /*trait*/ QMetaMethod_methodIndex for () {
  fn methodIndex(self, rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod11methodIndexEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod11methodIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn parameterCount<T: QMetaMethod_parameterCount>(&mut self, value: T) -> i32 {
    return value.parameterCount(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterCount {
  fn parameterCount(self, rsthis: &mut QMetaMethod) -> i32;
}

// proto:  int QMetaMethod::parameterCount();
impl<'a> /*trait*/ QMetaMethod_parameterCount for () {
  fn parameterCount(self, rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterCountEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod14parameterCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn enclosingMetaObject<T: QMetaMethod_enclosingMetaObject>(&mut self, value: T)  {
     value.enclosingMetaObject(self);
    // return 1;
  }
}

pub trait QMetaMethod_enclosingMetaObject {
  fn enclosingMetaObject(self, rsthis: &mut QMetaMethod) ;
}

// proto:  const QMetaObject * QMetaMethod::enclosingMetaObject();
impl<'a> /*trait*/ QMetaMethod_enclosingMetaObject for () {
  fn enclosingMetaObject(self, rsthis: &mut QMetaMethod)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod19enclosingMetaObjectEv()};
     unsafe {_ZNK11QMetaMethod19enclosingMetaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn revision<T: QMetaMethod_revision>(&mut self, value: T) -> i32 {
    return value.revision(self);
    // return 1;
  }
}

pub trait QMetaMethod_revision {
  fn revision(self, rsthis: &mut QMetaMethod) -> i32;
}

// proto:  int QMetaMethod::revision();
impl<'a> /*trait*/ QMetaMethod_revision for () {
  fn revision(self, rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod8revisionEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn tag<T: QMetaMethod_tag>(&mut self, value: T) -> String {
    return value.tag(self);
    // return 1;
  }
}

pub trait QMetaMethod_tag {
  fn tag(self, rsthis: &mut QMetaMethod) -> String;
}

// proto:  const char * QMetaMethod::tag();
impl<'a> /*trait*/ QMetaMethod_tag for () {
  fn tag(self, rsthis: &mut QMetaMethod) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod3tagEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod3tagEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn isValid<T: QMetaMethod_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QMetaMethod_isValid {
  fn isValid(self, rsthis: &mut QMetaMethod) -> i8;
}

// proto:  bool QMetaMethod::isValid();
impl<'a> /*trait*/ QMetaMethod_isValid for () {
  fn isValid(self, rsthis: &mut QMetaMethod) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod7isValidEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

