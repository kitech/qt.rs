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
  // proto:  const char * QGenericArgument::name();
  fn _ZNK16QGenericArgument4nameEv(qthis: *mut c_void) -> *const c_char;
  // proto:  void * QGenericArgument::data();
  fn _ZNK16QGenericArgument4dataEv(qthis: *mut c_void) ;
  // proto:  void QGenericArgument::NewQGenericArgument(const char * aName, const void * aData);
  fn _ZN16QGenericArgumentC1EPKcPKv(qthis: *mut c_void, arg0: *const c_char, arg1: *const uint8_t) ;
}

// body block begin
// class sizeof(QGenericArgument)=16
pub struct QGenericArgument {
  pub qclsinst: *mut c_void,
}

// proto:  const char * QGenericArgument::name();
impl /*struct*/ QGenericArgument {
  pub fn name<RetType, T: QGenericArgument_name<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QGenericArgument_name<RetType> {
  fn name(self , rsthis: &mut QGenericArgument) -> RetType;
}

// proto:  const char * QGenericArgument::name();
impl<'a> /*trait*/ QGenericArgument_name<String> for () {
  fn name(self , rsthis: &mut QGenericArgument) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QGenericArgument4nameEv()};
    let mut ret = unsafe {_ZNK16QGenericArgument4nameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

// proto:  void * QGenericArgument::data();
impl /*struct*/ QGenericArgument {
  pub fn data<RetType, T: QGenericArgument_data<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QGenericArgument_data<RetType> {
  fn data(self , rsthis: &mut QGenericArgument) -> RetType;
}

// proto:  void * QGenericArgument::data();
impl<'a> /*trait*/ QGenericArgument_data<()> for () {
  fn data(self , rsthis: &mut QGenericArgument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QGenericArgument4dataEv()};
     unsafe {_ZNK16QGenericArgument4dataEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGenericArgument {
  pub fn NewQGenericArgument<T: QGenericArgument_NewQGenericArgument>(value: T) -> QGenericArgument {
    let rsthis = value.NewQGenericArgument();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericArgument_NewQGenericArgument {
  fn NewQGenericArgument(self) -> QGenericArgument;
}

// proto: void QGenericArgument::NewQGenericArgument(const char * aName, const void * aData);
impl<'a> /*trait*/ QGenericArgument_NewQGenericArgument for (&'a  String, &'a  u8) {
  fn NewQGenericArgument(self) -> QGenericArgument {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QGenericArgumentC1EPKcPKv()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as *const uint8_t;
    unsafe {_ZN16QGenericArgumentC1EPKcPKv(qthis, arg0, arg1)};
    let rsthis = QGenericArgument{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

