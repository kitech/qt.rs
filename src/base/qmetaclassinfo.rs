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
  // proto:  void QMetaClassInfo::NewQMetaClassInfo();
  fn _ZN14QMetaClassInfoC1Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QMetaClassInfo::enclosingMetaObject();
  fn _ZNK14QMetaClassInfo19enclosingMetaObjectEv(qthis: *mut c_void) ;
  // proto:  const char * QMetaClassInfo::name();
  fn _ZNK14QMetaClassInfo4nameEv(qthis: *mut c_void) -> *const c_char;
  // proto:  const char * QMetaClassInfo::value();
  fn _ZNK14QMetaClassInfo5valueEv(qthis: *mut c_void) -> *const c_char;
}

// body block begin
// class sizeof(QMetaClassInfo)=16
pub struct QMetaClassInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMetaClassInfo {
  pub fn NewQMetaClassInfo<T: QMetaClassInfo_NewQMetaClassInfo>(value: T) -> QMetaClassInfo {
    let rsthis = value.NewQMetaClassInfo();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaClassInfo_NewQMetaClassInfo {
  fn NewQMetaClassInfo(self) -> QMetaClassInfo;
}

// proto: void QMetaClassInfo::NewQMetaClassInfo();
impl<'a> /*trait*/ QMetaClassInfo_NewQMetaClassInfo for () {
  fn NewQMetaClassInfo(self) -> QMetaClassInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QMetaClassInfoC1Ev()};
    unsafe {_ZN14QMetaClassInfoC1Ev(qthis)};
    let rsthis = QMetaClassInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QMetaObject * QMetaClassInfo::enclosingMetaObject();
impl /*struct*/ QMetaClassInfo {
  pub fn enclosingMetaObject<RetType, T: QMetaClassInfo_enclosingMetaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.enclosingMetaObject(self);
    // return 1;
  }
}

pub trait QMetaClassInfo_enclosingMetaObject<RetType> {
  fn enclosingMetaObject(self , rsthis: &mut QMetaClassInfo) -> RetType;
}

// proto:  const QMetaObject * QMetaClassInfo::enclosingMetaObject();
impl<'a> /*trait*/ QMetaClassInfo_enclosingMetaObject<()> for () {
  fn enclosingMetaObject(self , rsthis: &mut QMetaClassInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMetaClassInfo19enclosingMetaObjectEv()};
     unsafe {_ZNK14QMetaClassInfo19enclosingMetaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const char * QMetaClassInfo::name();
impl /*struct*/ QMetaClassInfo {
  pub fn name<RetType, T: QMetaClassInfo_name<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QMetaClassInfo_name<RetType> {
  fn name(self , rsthis: &mut QMetaClassInfo) -> RetType;
}

// proto:  const char * QMetaClassInfo::name();
impl<'a> /*trait*/ QMetaClassInfo_name<String> for () {
  fn name(self , rsthis: &mut QMetaClassInfo) -> String {
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
  pub fn value<RetType, T: QMetaClassInfo_value<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QMetaClassInfo_value<RetType> {
  fn value(self , rsthis: &mut QMetaClassInfo) -> RetType;
}

// proto:  const char * QMetaClassInfo::value();
impl<'a> /*trait*/ QMetaClassInfo_value<String> for () {
  fn value(self , rsthis: &mut QMetaClassInfo) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMetaClassInfo5valueEv()};
    let mut ret = unsafe {_ZNK14QMetaClassInfo5valueEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

