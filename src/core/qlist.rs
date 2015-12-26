// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtCore/qlist.h
// dst-file: /src/core/qlist.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QListData_Class_Size() -> c_int;
  // proto:  void ** QListData::prepend();
  fn _ZN9QListData7prependEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListData::realloc(int alloc);
  fn _ZN9QListData7reallocEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void ** QListData::end();
  fn _ZNK9QListData3endEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListData::remove(int i, int n);
  fn _ZN9QListData6removeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void ** QListData::append(const QListData & l);
  fn _ZN9QListData6appendERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListData::remove(int i);
  fn _ZN9QListData6removeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void ** QListData::append();
  fn _ZN9QListData6appendEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListData::dispose();
  fn _ZN9QListData7disposeEv(qthis: *mut c_void);
  // proto:  int QListData::size();
  fn _ZNK9QListData4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QListData::isEmpty();
  fn _ZNK9QListData7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void ** QListData::at(int i);
  fn _ZNK9QListData2atEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void ** QListData::erase(void ** xi);
  fn _ZN9QListData5eraseEPPv(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void ** QListData::append(int n);
  fn _ZN9QListData6appendEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QListData::move(int from, int to);
  fn _ZN9QListData4moveEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void ** QListData::begin();
  fn _ZNK9QListData5beginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void ** QListData::insert(int i);
  fn _ZN9QListData6insertEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QListData)=8
pub struct QListData {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QListData {
  pub fn inheritFrom(qthis: *mut c_void) -> QListData {
    return QListData{qclsinst: qthis};
  }
}
  // proto:  void ** QListData::prepend();
impl /*struct*/ QListData {
  pub fn prepend<RetType, T: QListData_prepend<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prepend(self);
    // return 1;
  }
}

pub trait QListData_prepend<RetType> {
  fn prepend(self , rsthis: & QListData) -> RetType;
}

  // proto:  void ** QListData::prepend();
impl<'a> /*trait*/ QListData_prepend<*mut c_void> for () {
  fn prepend(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData7prependEv()};
    let mut ret = unsafe {_ZN9QListData7prependEv(rsthis.qclsinst)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void QListData::realloc(int alloc);
impl /*struct*/ QListData {
  pub fn realloc<RetType, T: QListData_realloc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.realloc(self);
    // return 1;
  }
}

pub trait QListData_realloc<RetType> {
  fn realloc(self , rsthis: & QListData) -> RetType;
}

  // proto:  void QListData::realloc(int alloc);
impl<'a> /*trait*/ QListData_realloc<()> for (i32) {
  fn realloc(self , rsthis: & QListData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData7reallocEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QListData7reallocEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void ** QListData::end();
impl /*struct*/ QListData {
  pub fn end<RetType, T: QListData_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QListData_end<RetType> {
  fn end(self , rsthis: & QListData) -> RetType;
}

  // proto:  void ** QListData::end();
impl<'a> /*trait*/ QListData_end<*mut c_void> for () {
  fn end(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListData3endEv()};
    let mut ret = unsafe {_ZNK9QListData3endEv(rsthis.qclsinst)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void QListData::remove(int i, int n);
impl /*struct*/ QListData {
  pub fn remove<RetType, T: QListData_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QListData_remove<RetType> {
  fn remove(self , rsthis: & QListData) -> RetType;
}

  // proto:  void QListData::remove(int i, int n);
impl<'a> /*trait*/ QListData_remove<()> for (i32, i32) {
  fn remove(self , rsthis: & QListData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData6removeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QListData6removeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void ** QListData::append(const QListData & l);
impl /*struct*/ QListData {
  pub fn append<RetType, T: QListData_append<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QListData_append<RetType> {
  fn append(self , rsthis: & QListData) -> RetType;
}

  // proto:  void ** QListData::append(const QListData & l);
impl<'a> /*trait*/ QListData_append<*mut c_void> for (&'a QListData) {
  fn append(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData6appendERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QListData6appendERKS_(rsthis.qclsinst, arg0)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void QListData::remove(int i);
impl<'a> /*trait*/ QListData_remove<()> for (i32) {
  fn remove(self , rsthis: & QListData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData6removeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QListData6removeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void ** QListData::append();
impl<'a> /*trait*/ QListData_append<*mut c_void> for () {
  fn append(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData6appendEv()};
    let mut ret = unsafe {_ZN9QListData6appendEv(rsthis.qclsinst)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void QListData::dispose();
impl /*struct*/ QListData {
  pub fn dispose<RetType, T: QListData_dispose<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dispose(self);
    // return 1;
  }
}

pub trait QListData_dispose<RetType> {
  fn dispose(self , rsthis: & QListData) -> RetType;
}

  // proto:  void QListData::dispose();
impl<'a> /*trait*/ QListData_dispose<()> for () {
  fn dispose(self , rsthis: & QListData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData7disposeEv()};
     unsafe {_ZN9QListData7disposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QListData::size();
impl /*struct*/ QListData {
  pub fn size<RetType, T: QListData_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QListData_size<RetType> {
  fn size(self , rsthis: & QListData) -> RetType;
}

  // proto:  int QListData::size();
impl<'a> /*trait*/ QListData_size<i32> for () {
  fn size(self , rsthis: & QListData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListData4sizeEv()};
    let mut ret = unsafe {_ZNK9QListData4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QListData::isEmpty();
impl /*struct*/ QListData {
  pub fn isEmpty<RetType, T: QListData_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QListData_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QListData) -> RetType;
}

  // proto:  bool QListData::isEmpty();
impl<'a> /*trait*/ QListData_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QListData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListData7isEmptyEv()};
    let mut ret = unsafe {_ZNK9QListData7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void ** QListData::at(int i);
impl /*struct*/ QListData {
  pub fn at<RetType, T: QListData_at<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QListData_at<RetType> {
  fn at(self , rsthis: & QListData) -> RetType;
}

  // proto:  void ** QListData::at(int i);
impl<'a> /*trait*/ QListData_at<*mut c_void> for (i32) {
  fn at(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListData2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QListData2atEi(rsthis.qclsinst, arg0)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void ** QListData::erase(void ** xi);
impl /*struct*/ QListData {
  pub fn erase<RetType, T: QListData_erase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.erase(self);
    // return 1;
  }
}

pub trait QListData_erase<RetType> {
  fn erase(self , rsthis: & QListData) -> RetType;
}

  // proto:  void ** QListData::erase(void ** xi);
impl<'a> /*trait*/ QListData_erase<*mut c_void> for (*mut c_void) {
  fn erase(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData5eraseEPPv()};
    let arg0 = self  as *mut c_void;
    let mut ret = unsafe {_ZN9QListData5eraseEPPv(rsthis.qclsinst, arg0)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void ** QListData::append(int n);
impl<'a> /*trait*/ QListData_append<*mut c_void> for (i32) {
  fn append(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData6appendEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QListData6appendEi(rsthis.qclsinst, arg0)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void QListData::move(int from, int to);
impl /*struct*/ QListData {
  pub fn move_<RetType, T: QListData_move_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.move_(self);
    // return 1;
  }
}

pub trait QListData_move_<RetType> {
  fn move_(self , rsthis: & QListData) -> RetType;
}

  // proto:  void QListData::move(int from, int to);
impl<'a> /*trait*/ QListData_move_<()> for (i32, i32) {
  fn move_(self , rsthis: & QListData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData4moveEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QListData4moveEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void ** QListData::begin();
impl /*struct*/ QListData {
  pub fn begin<RetType, T: QListData_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QListData_begin<RetType> {
  fn begin(self , rsthis: & QListData) -> RetType;
}

  // proto:  void ** QListData::begin();
impl<'a> /*trait*/ QListData_begin<*mut c_void> for () {
  fn begin(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListData5beginEv()};
    let mut ret = unsafe {_ZNK9QListData5beginEv(rsthis.qclsinst)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void ** QListData::insert(int i);
impl /*struct*/ QListData {
  pub fn insert<RetType, T: QListData_insert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QListData_insert<RetType> {
  fn insert(self , rsthis: & QListData) -> RetType;
}

  // proto:  void ** QListData::insert(int i);
impl<'a> /*trait*/ QListData_insert<*mut c_void> for (i32) {
  fn insert(self , rsthis: & QListData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListData6insertEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QListData6insertEi(rsthis.qclsinst, arg0)};
    return ret as *mut c_void;
    // return 1;
  }
}

// <= body block end

