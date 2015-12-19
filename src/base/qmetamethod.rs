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

// proto:  QList<QByteArray> QMetaMethod::parameterTypes();
impl /*struct*/ QMetaMethod {
  pub fn parameterTypes<RetType, T: QMetaMethod_parameterTypes<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.parameterTypes(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterTypes<RetType> {
  fn parameterTypes(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  QList<QByteArray> QMetaMethod::parameterTypes();
impl<'a> /*trait*/ QMetaMethod_parameterTypes<()> for () {
  fn parameterTypes(self , rsthis: &mut QMetaMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterTypesEv()};
     unsafe {_ZNK11QMetaMethod14parameterTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QList<QByteArray> QMetaMethod::parameterNames();
impl /*struct*/ QMetaMethod {
  pub fn parameterNames<RetType, T: QMetaMethod_parameterNames<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.parameterNames(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterNames<RetType> {
  fn parameterNames(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  QList<QByteArray> QMetaMethod::parameterNames();
impl<'a> /*trait*/ QMetaMethod_parameterNames<()> for () {
  fn parameterNames(self , rsthis: &mut QMetaMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterNamesEv()};
     unsafe {_ZNK11QMetaMethod14parameterNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QByteArray QMetaMethod::methodSignature();
impl /*struct*/ QMetaMethod {
  pub fn methodSignature<RetType, T: QMetaMethod_methodSignature<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.methodSignature(self);
    // return 1;
  }
}

pub trait QMetaMethod_methodSignature<RetType> {
  fn methodSignature(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  QByteArray QMetaMethod::methodSignature();
impl<'a> /*trait*/ QMetaMethod_methodSignature<QByteArray> for () {
  fn methodSignature(self , rsthis: &mut QMetaMethod) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod15methodSignatureEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod15methodSignatureEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const char * QMetaMethod::typeName();
impl /*struct*/ QMetaMethod {
  pub fn typeName<RetType, T: QMetaMethod_typeName<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.typeName(self);
    // return 1;
  }
}

pub trait QMetaMethod_typeName<RetType> {
  fn typeName(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  const char * QMetaMethod::typeName();
impl<'a> /*trait*/ QMetaMethod_typeName<String> for () {
  fn typeName(self , rsthis: &mut QMetaMethod) -> String {
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
  pub fn attributes<RetType, T: QMetaMethod_attributes<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.attributes(self);
    // return 1;
  }
}

pub trait QMetaMethod_attributes<RetType> {
  fn attributes(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  int QMetaMethod::attributes();
impl<'a> /*trait*/ QMetaMethod_attributes<i32> for () {
  fn attributes(self , rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod10attributesEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod10attributesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QMetaMethod::getParameterTypes(int * types);
impl /*struct*/ QMetaMethod {
  pub fn getParameterTypes<RetType, T: QMetaMethod_getParameterTypes<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getParameterTypes(self);
    // return 1;
  }
}

pub trait QMetaMethod_getParameterTypes<RetType> {
  fn getParameterTypes(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  void QMetaMethod::getParameterTypes(int * types);
impl<'a> /*trait*/ QMetaMethod_getParameterTypes<()> for (&'a mut i32) {
  fn getParameterTypes(self , rsthis: &mut QMetaMethod) -> () {
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

// proto:  int QMetaMethod::parameterType(int index);
impl /*struct*/ QMetaMethod {
  pub fn parameterType<RetType, T: QMetaMethod_parameterType<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.parameterType(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterType<RetType> {
  fn parameterType(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  int QMetaMethod::parameterType(int index);
impl<'a> /*trait*/ QMetaMethod_parameterType<i32> for (i32) {
  fn parameterType(self , rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod13parameterTypeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QMetaMethod13parameterTypeEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QByteArray QMetaMethod::name();
impl /*struct*/ QMetaMethod {
  pub fn name<RetType, T: QMetaMethod_name<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QMetaMethod_name<RetType> {
  fn name(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  QByteArray QMetaMethod::name();
impl<'a> /*trait*/ QMetaMethod_name<QByteArray> for () {
  fn name(self , rsthis: &mut QMetaMethod) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod4nameEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QMetaMethod::returnType();
impl /*struct*/ QMetaMethod {
  pub fn returnType<RetType, T: QMetaMethod_returnType<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.returnType(self);
    // return 1;
  }
}

pub trait QMetaMethod_returnType<RetType> {
  fn returnType(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  int QMetaMethod::returnType();
impl<'a> /*trait*/ QMetaMethod_returnType<i32> for () {
  fn returnType(self , rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod10returnTypeEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod10returnTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QMetaMethod::methodIndex();
impl /*struct*/ QMetaMethod {
  pub fn methodIndex<RetType, T: QMetaMethod_methodIndex<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.methodIndex(self);
    // return 1;
  }
}

pub trait QMetaMethod_methodIndex<RetType> {
  fn methodIndex(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  int QMetaMethod::methodIndex();
impl<'a> /*trait*/ QMetaMethod_methodIndex<i32> for () {
  fn methodIndex(self , rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod11methodIndexEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod11methodIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QMetaMethod::parameterCount();
impl /*struct*/ QMetaMethod {
  pub fn parameterCount<RetType, T: QMetaMethod_parameterCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.parameterCount(self);
    // return 1;
  }
}

pub trait QMetaMethod_parameterCount<RetType> {
  fn parameterCount(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  int QMetaMethod::parameterCount();
impl<'a> /*trait*/ QMetaMethod_parameterCount<i32> for () {
  fn parameterCount(self , rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterCountEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod14parameterCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  const QMetaObject * QMetaMethod::enclosingMetaObject();
impl /*struct*/ QMetaMethod {
  pub fn enclosingMetaObject<RetType, T: QMetaMethod_enclosingMetaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.enclosingMetaObject(self);
    // return 1;
  }
}

pub trait QMetaMethod_enclosingMetaObject<RetType> {
  fn enclosingMetaObject(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  const QMetaObject * QMetaMethod::enclosingMetaObject();
impl<'a> /*trait*/ QMetaMethod_enclosingMetaObject<()> for () {
  fn enclosingMetaObject(self , rsthis: &mut QMetaMethod) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod19enclosingMetaObjectEv()};
     unsafe {_ZNK11QMetaMethod19enclosingMetaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QMetaMethod::revision();
impl /*struct*/ QMetaMethod {
  pub fn revision<RetType, T: QMetaMethod_revision<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.revision(self);
    // return 1;
  }
}

pub trait QMetaMethod_revision<RetType> {
  fn revision(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  int QMetaMethod::revision();
impl<'a> /*trait*/ QMetaMethod_revision<i32> for () {
  fn revision(self , rsthis: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod8revisionEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  const char * QMetaMethod::tag();
impl /*struct*/ QMetaMethod {
  pub fn tag<RetType, T: QMetaMethod_tag<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.tag(self);
    // return 1;
  }
}

pub trait QMetaMethod_tag<RetType> {
  fn tag(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  const char * QMetaMethod::tag();
impl<'a> /*trait*/ QMetaMethod_tag<String> for () {
  fn tag(self , rsthis: &mut QMetaMethod) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod3tagEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod3tagEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

// proto:  bool QMetaMethod::isValid();
impl /*struct*/ QMetaMethod {
  pub fn isValid<RetType, T: QMetaMethod_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QMetaMethod_isValid<RetType> {
  fn isValid(self , rsthis: &mut QMetaMethod) -> RetType;
}

// proto:  bool QMetaMethod::isValid();
impl<'a> /*trait*/ QMetaMethod_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QMetaMethod) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod7isValidEv()};
    let mut ret = unsafe {_ZNK11QMetaMethod7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

