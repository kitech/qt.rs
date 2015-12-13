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
  fn _ZNK11QMetaMethod14parameterTypesEv() -> i32;
  fn _ZNK11QMetaMethod14parameterNamesEv() -> i32;
  fn _ZNK11QMetaMethod15methodSignatureEv() -> i32;
  fn _ZNK11QMetaMethod8typeNameEv() -> i32;
  fn _ZNK11QMetaMethod10attributesEv() -> i32;
  fn _ZNK11QMetaMethod17getParameterTypesEPi(arg0: *mut c_int) -> i32;
  fn _ZN11QMetaMethodC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK11QMetaMethod13parameterTypeEi(arg0: c_int) -> i32;
  fn _ZNK11QMetaMethod4nameEv() -> i32;
  fn _ZNK11QMetaMethod10returnTypeEv() -> i32;
  fn _ZNK11QMetaMethod11methodIndexEv() -> i32;
  fn _ZNK11QMetaMethod14parameterCountEv() -> i32;
  fn _ZNK11QMetaMethod19enclosingMetaObjectEv() -> i32;
  fn _ZNK11QMetaMethod8revisionEv() -> i32;
  fn _ZNK11QMetaMethod3tagEv() -> i32;
  fn _ZNK11QMetaMethod7isValidEv() -> i32;
}

// body block begin
// class sizeof(QMetaMethod)=16
pub struct QMetaMethod {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMetaMethod {
  pub fn parameterTypes<T: QMetaMethod_parameterTypes>(&mut self, value: T) -> i32 {
    value.parameterTypes(self);
    return 1;
  }
}

pub trait QMetaMethod_parameterTypes {
  fn parameterTypes(self, this: &mut QMetaMethod) -> i32;
}

// proto: QList<QByteArray> QMetaMethod::parameterTypes();
impl<'a> /*trait*/ QMetaMethod_parameterTypes for () {
  fn parameterTypes(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterTypesEv()};
    unsafe {_ZNK11QMetaMethod14parameterTypesEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn parameterNames<T: QMetaMethod_parameterNames>(&mut self, value: T) -> i32 {
    value.parameterNames(self);
    return 1;
  }
}

pub trait QMetaMethod_parameterNames {
  fn parameterNames(self, this: &mut QMetaMethod) -> i32;
}

// proto: QList<QByteArray> QMetaMethod::parameterNames();
impl<'a> /*trait*/ QMetaMethod_parameterNames for () {
  fn parameterNames(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterNamesEv()};
    unsafe {_ZNK11QMetaMethod14parameterNamesEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn methodSignature<T: QMetaMethod_methodSignature>(&mut self, value: T) -> i32 {
    value.methodSignature(self);
    return 1;
  }
}

pub trait QMetaMethod_methodSignature {
  fn methodSignature(self, this: &mut QMetaMethod) -> i32;
}

// proto: QByteArray QMetaMethod::methodSignature();
impl<'a> /*trait*/ QMetaMethod_methodSignature for () {
  fn methodSignature(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod15methodSignatureEv()};
    unsafe {_ZNK11QMetaMethod15methodSignatureEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn typeName<T: QMetaMethod_typeName>(&mut self, value: T) -> i32 {
    value.typeName(self);
    return 1;
  }
}

pub trait QMetaMethod_typeName {
  fn typeName(self, this: &mut QMetaMethod) -> i32;
}

// proto: const char * QMetaMethod::typeName();
impl<'a> /*trait*/ QMetaMethod_typeName for () {
  fn typeName(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod8typeNameEv()};
    unsafe {_ZNK11QMetaMethod8typeNameEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn attributes<T: QMetaMethod_attributes>(&mut self, value: T) -> i32 {
    value.attributes(self);
    return 1;
  }
}

pub trait QMetaMethod_attributes {
  fn attributes(self, this: &mut QMetaMethod) -> i32;
}

// proto: int QMetaMethod::attributes();
impl<'a> /*trait*/ QMetaMethod_attributes for () {
  fn attributes(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod10attributesEv()};
    unsafe {_ZNK11QMetaMethod10attributesEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn getParameterTypes<T: QMetaMethod_getParameterTypes>(&mut self, value: T) -> i32 {
    value.getParameterTypes(self);
    return 1;
  }
}

pub trait QMetaMethod_getParameterTypes {
  fn getParameterTypes(self, this: &mut QMetaMethod) -> i32;
}

// proto: void QMetaMethod::getParameterTypes(int * types);
impl<'a> /*trait*/ QMetaMethod_getParameterTypes for (&'a mut i32) {
  fn getParameterTypes(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod17getParameterTypesEPi()};
    let arg0 = self  as *mut c_int;
    unsafe {_ZNK11QMetaMethod17getParameterTypesEPi(arg0)};
    return 1;
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
    value.parameterType(self);
    return 1;
  }
}

pub trait QMetaMethod_parameterType {
  fn parameterType(self, this: &mut QMetaMethod) -> i32;
}

// proto: int QMetaMethod::parameterType(int index);
impl<'a> /*trait*/ QMetaMethod_parameterType for (i32) {
  fn parameterType(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod13parameterTypeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QMetaMethod13parameterTypeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn name<T: QMetaMethod_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QMetaMethod_name {
  fn name(self, this: &mut QMetaMethod) -> i32;
}

// proto: QByteArray QMetaMethod::name();
impl<'a> /*trait*/ QMetaMethod_name for () {
  fn name(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod4nameEv()};
    unsafe {_ZNK11QMetaMethod4nameEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn returnType<T: QMetaMethod_returnType>(&mut self, value: T) -> i32 {
    value.returnType(self);
    return 1;
  }
}

pub trait QMetaMethod_returnType {
  fn returnType(self, this: &mut QMetaMethod) -> i32;
}

// proto: int QMetaMethod::returnType();
impl<'a> /*trait*/ QMetaMethod_returnType for () {
  fn returnType(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod10returnTypeEv()};
    unsafe {_ZNK11QMetaMethod10returnTypeEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn methodIndex<T: QMetaMethod_methodIndex>(&mut self, value: T) -> i32 {
    value.methodIndex(self);
    return 1;
  }
}

pub trait QMetaMethod_methodIndex {
  fn methodIndex(self, this: &mut QMetaMethod) -> i32;
}

// proto: int QMetaMethod::methodIndex();
impl<'a> /*trait*/ QMetaMethod_methodIndex for () {
  fn methodIndex(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod11methodIndexEv()};
    unsafe {_ZNK11QMetaMethod11methodIndexEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn parameterCount<T: QMetaMethod_parameterCount>(&mut self, value: T) -> i32 {
    value.parameterCount(self);
    return 1;
  }
}

pub trait QMetaMethod_parameterCount {
  fn parameterCount(self, this: &mut QMetaMethod) -> i32;
}

// proto: int QMetaMethod::parameterCount();
impl<'a> /*trait*/ QMetaMethod_parameterCount for () {
  fn parameterCount(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod14parameterCountEv()};
    unsafe {_ZNK11QMetaMethod14parameterCountEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn enclosingMetaObject<T: QMetaMethod_enclosingMetaObject>(&mut self, value: T) -> i32 {
    value.enclosingMetaObject(self);
    return 1;
  }
}

pub trait QMetaMethod_enclosingMetaObject {
  fn enclosingMetaObject(self, this: &mut QMetaMethod) -> i32;
}

// proto: const QMetaObject * QMetaMethod::enclosingMetaObject();
impl<'a> /*trait*/ QMetaMethod_enclosingMetaObject for () {
  fn enclosingMetaObject(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod19enclosingMetaObjectEv()};
    unsafe {_ZNK11QMetaMethod19enclosingMetaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn revision<T: QMetaMethod_revision>(&mut self, value: T) -> i32 {
    value.revision(self);
    return 1;
  }
}

pub trait QMetaMethod_revision {
  fn revision(self, this: &mut QMetaMethod) -> i32;
}

// proto: int QMetaMethod::revision();
impl<'a> /*trait*/ QMetaMethod_revision for () {
  fn revision(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod8revisionEv()};
    unsafe {_ZNK11QMetaMethod8revisionEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn tag<T: QMetaMethod_tag>(&mut self, value: T) -> i32 {
    value.tag(self);
    return 1;
  }
}

pub trait QMetaMethod_tag {
  fn tag(self, this: &mut QMetaMethod) -> i32;
}

// proto: const char * QMetaMethod::tag();
impl<'a> /*trait*/ QMetaMethod_tag for () {
  fn tag(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod3tagEv()};
    unsafe {_ZNK11QMetaMethod3tagEv()};
    return 1;
  }
}

impl /*struct*/ QMetaMethod {
  pub fn isValid<T: QMetaMethod_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QMetaMethod_isValid {
  fn isValid(self, this: &mut QMetaMethod) -> i32;
}

// proto: bool QMetaMethod::isValid();
impl<'a> /*trait*/ QMetaMethod_isValid for () {
  fn isValid(self, this: &mut QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMetaMethod7isValidEv()};
    unsafe {_ZNK11QMetaMethod7isValidEv()};
    return 1;
  }
}

