// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QModelIndex::QModelIndex();
  fn _ZN11QModelIndexC1Ev(qthis: *mut c_void);
  // proto:  int QModelIndex::column();
  fn _ZNK11QModelIndex6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  quintptr QModelIndex::internalId();
  fn _ZNK11QModelIndex10internalIdEv(qthis: *mut c_void) -> c_int;
  // proto:  QModelIndex QModelIndex::child(int row, int column);
  fn _ZNK11QModelIndex5childEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void * QModelIndex::internalPointer();
  fn _ZNK11QModelIndex15internalPointerEv(qthis: *mut c_void);
  // proto:  bool QModelIndex::isValid();
  fn _ZNK11QModelIndex7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QModelIndex QModelIndex::parent();
  fn _ZNK11QModelIndex6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QModelIndex::sibling(int row, int column);
  fn _ZNK11QModelIndex7siblingEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  const QAbstractItemModel * QModelIndex::model();
  fn _ZNK11QModelIndex5modelEv(qthis: *mut c_void);
  // proto:  QVariant QModelIndex::data(int role);
  fn _ZNK11QModelIndex4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QModelIndex::row();
  fn _ZNK11QModelIndex3rowEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QModelIndex)=24
pub struct QModelIndex {
  pub qclsinst: *mut c_void,
}

  // proto:  void QModelIndex::QModelIndex();
impl /*struct*/ QModelIndex {
  pub fn NewQModelIndex<T: QModelIndex_NewQModelIndex>(value: T) -> QModelIndex {
    let rsthis = value.NewQModelIndex();
    return rsthis;
    // return 1;
  }
}

pub trait QModelIndex_NewQModelIndex {
  fn NewQModelIndex(self) -> QModelIndex;
}

  // proto:  void QModelIndex::QModelIndex();
impl<'a> /*trait*/ QModelIndex_NewQModelIndex for () {
  fn NewQModelIndex(self) -> QModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QModelIndexC1Ev()};
    unsafe {_ZN11QModelIndexC1Ev(qthis)};
    let rsthis = QModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QModelIndex::column();
impl /*struct*/ QModelIndex {
  pub fn column<RetType, T: QModelIndex_column<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QModelIndex_column<RetType> {
  fn column(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  int QModelIndex::column();
impl<'a> /*trait*/ QModelIndex_column<i32> for () {
  fn column(self , rsthis: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex6columnEv()};
    let mut ret = unsafe {_ZNK11QModelIndex6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  quintptr QModelIndex::internalId();
impl /*struct*/ QModelIndex {
  pub fn internalId<RetType, T: QModelIndex_internalId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.internalId(self);
    // return 1;
  }
}

pub trait QModelIndex_internalId<RetType> {
  fn internalId(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  quintptr QModelIndex::internalId();
impl<'a> /*trait*/ QModelIndex_internalId<i32> for () {
  fn internalId(self , rsthis: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex10internalIdEv()};
    let mut ret = unsafe {_ZNK11QModelIndex10internalIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QModelIndex QModelIndex::child(int row, int column);
impl /*struct*/ QModelIndex {
  pub fn child<RetType, T: QModelIndex_child<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QModelIndex_child<RetType> {
  fn child(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  QModelIndex QModelIndex::child(int row, int column);
impl<'a> /*trait*/ QModelIndex_child<QModelIndex> for (i32, i32) {
  fn child(self , rsthis: &mut QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QModelIndex5childEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void * QModelIndex::internalPointer();
impl /*struct*/ QModelIndex {
  pub fn internalPointer<RetType, T: QModelIndex_internalPointer<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.internalPointer(self);
    // return 1;
  }
}

pub trait QModelIndex_internalPointer<RetType> {
  fn internalPointer(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  void * QModelIndex::internalPointer();
impl<'a> /*trait*/ QModelIndex_internalPointer<()> for () {
  fn internalPointer(self , rsthis: &mut QModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex15internalPointerEv()};
     unsafe {_ZNK11QModelIndex15internalPointerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QModelIndex::isValid();
impl /*struct*/ QModelIndex {
  pub fn isValid<RetType, T: QModelIndex_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QModelIndex_isValid<RetType> {
  fn isValid(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  bool QModelIndex::isValid();
impl<'a> /*trait*/ QModelIndex_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QModelIndex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex7isValidEv()};
    let mut ret = unsafe {_ZNK11QModelIndex7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QModelIndex QModelIndex::parent();
impl /*struct*/ QModelIndex {
  pub fn parent<RetType, T: QModelIndex_parent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QModelIndex_parent<RetType> {
  fn parent(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  QModelIndex QModelIndex::parent();
impl<'a> /*trait*/ QModelIndex_parent<QModelIndex> for () {
  fn parent(self , rsthis: &mut QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex6parentEv()};
    let mut ret = unsafe {_ZNK11QModelIndex6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QModelIndex::sibling(int row, int column);
impl /*struct*/ QModelIndex {
  pub fn sibling<RetType, T: QModelIndex_sibling<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QModelIndex_sibling<RetType> {
  fn sibling(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  QModelIndex QModelIndex::sibling(int row, int column);
impl<'a> /*trait*/ QModelIndex_sibling<QModelIndex> for (i32, i32) {
  fn sibling(self , rsthis: &mut QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex7siblingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QModelIndex7siblingEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QAbstractItemModel * QModelIndex::model();
impl /*struct*/ QModelIndex {
  pub fn model<RetType, T: QModelIndex_model<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QModelIndex_model<RetType> {
  fn model(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  const QAbstractItemModel * QModelIndex::model();
impl<'a> /*trait*/ QModelIndex_model<()> for () {
  fn model(self , rsthis: &mut QModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex5modelEv()};
     unsafe {_ZNK11QModelIndex5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QModelIndex::data(int role);
impl /*struct*/ QModelIndex {
  pub fn data<RetType, T: QModelIndex_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QModelIndex_data<RetType> {
  fn data(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  QVariant QModelIndex::data(int role);
impl<'a> /*trait*/ QModelIndex_data<QVariant> for (i32) {
  fn data(self , rsthis: &mut QModelIndex) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QModelIndex4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QModelIndex::row();
impl /*struct*/ QModelIndex {
  pub fn row<RetType, T: QModelIndex_row<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QModelIndex_row<RetType> {
  fn row(self , rsthis: &mut QModelIndex) -> RetType;
}

  // proto:  int QModelIndex::row();
impl<'a> /*trait*/ QModelIndex_row<i32> for () {
  fn row(self , rsthis: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex3rowEv()};
    let mut ret = unsafe {_ZNK11QModelIndex3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

