// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmodelindex::QModelIndex;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QModelIndex QPersistentModelIndex::sibling(int row, int column);
  fn _ZNK21QPersistentModelIndex7siblingEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QVariant QPersistentModelIndex::data(int role);
  fn _ZNK21QPersistentModelIndex4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QModelIndex QPersistentModelIndex::parent();
  fn _ZNK21QPersistentModelIndex6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPersistentModelIndex::NewQPersistentModelIndex(const QPersistentModelIndex & other);
  fn _ZN21QPersistentModelIndexC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void * QPersistentModelIndex::internalPointer();
  fn _ZNK21QPersistentModelIndex15internalPointerEv(qthis: *mut c_void) ;
  // proto:  int QPersistentModelIndex::row();
  fn _ZNK21QPersistentModelIndex3rowEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPersistentModelIndex::internalId();
  fn _ZNK21QPersistentModelIndex10internalIdEv(qthis: *mut c_void) -> c_int;
  // proto:  const QAbstractItemModel * QPersistentModelIndex::model();
  fn _ZNK21QPersistentModelIndex5modelEv(qthis: *mut c_void) ;
  // proto:  void QPersistentModelIndex::FreeQPersistentModelIndex();
  fn _ZN21QPersistentModelIndexD0Ev(qthis: *mut c_void) ;
  // proto:  void QPersistentModelIndex::NewQPersistentModelIndex(const QModelIndex & index);
  fn _ZN21QPersistentModelIndexC1ERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPersistentModelIndex::NewQPersistentModelIndex();
  fn _ZN21QPersistentModelIndexC1Ev(qthis: *mut c_void) ;
  // proto:  int QPersistentModelIndex::column();
  fn _ZNK21QPersistentModelIndex6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPersistentModelIndex::swap(QPersistentModelIndex & other);
  fn _ZN21QPersistentModelIndex4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QPersistentModelIndex::child(int row, int column);
  fn _ZNK21QPersistentModelIndex5childEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QPersistentModelIndex::isValid();
  fn _ZNK21QPersistentModelIndex7isValidEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QPersistentModelIndex)=8
pub struct QPersistentModelIndex {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPersistentModelIndex {
  pub fn sibling<T: QPersistentModelIndex_sibling>(&mut self, value: T) -> QModelIndex {
    return value.sibling(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_sibling {
  fn sibling(self, rsthis: &mut QPersistentModelIndex) -> QModelIndex;
}

// proto:  QModelIndex QPersistentModelIndex::sibling(int row, int column);
impl<'a> /*trait*/ QPersistentModelIndex_sibling for (i32, i32) {
  fn sibling(self, rsthis: &mut QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex7siblingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex7siblingEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn data<T: QPersistentModelIndex_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_data {
  fn data(self, rsthis: &mut QPersistentModelIndex) -> QVariant;
}

// proto:  QVariant QPersistentModelIndex::data(int role);
impl<'a> /*trait*/ QPersistentModelIndex_data for (i32) {
  fn data(self, rsthis: &mut QPersistentModelIndex) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn parent<T: QPersistentModelIndex_parent>(&mut self, value: T) -> QModelIndex {
    return value.parent(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_parent {
  fn parent(self, rsthis: &mut QPersistentModelIndex) -> QModelIndex;
}

// proto:  QModelIndex QPersistentModelIndex::parent();
impl<'a> /*trait*/ QPersistentModelIndex_parent for () {
  fn parent(self, rsthis: &mut QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex6parentEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn NewQPersistentModelIndex<T: QPersistentModelIndex_NewQPersistentModelIndex>(value: T) -> QPersistentModelIndex {
    let rsthis = value.NewQPersistentModelIndex();
    return rsthis;
    // return 1;
  }
}

pub trait QPersistentModelIndex_NewQPersistentModelIndex {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex;
}

// proto: void QPersistentModelIndex::NewQPersistentModelIndex(const QPersistentModelIndex & other);
impl<'a> /*trait*/ QPersistentModelIndex_NewQPersistentModelIndex for (&'a  QPersistentModelIndex) {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QPersistentModelIndexC1ERKS_(qthis, arg0)};
    let rsthis = QPersistentModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn internalPointer<T: QPersistentModelIndex_internalPointer>(&mut self, value: T)  {
     value.internalPointer(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_internalPointer {
  fn internalPointer(self, rsthis: &mut QPersistentModelIndex) ;
}

// proto:  void * QPersistentModelIndex::internalPointer();
impl<'a> /*trait*/ QPersistentModelIndex_internalPointer for () {
  fn internalPointer(self, rsthis: &mut QPersistentModelIndex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex15internalPointerEv()};
     unsafe {_ZNK21QPersistentModelIndex15internalPointerEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn row<T: QPersistentModelIndex_row>(&mut self, value: T) -> i32 {
    return value.row(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_row {
  fn row(self, rsthis: &mut QPersistentModelIndex) -> i32;
}

// proto:  int QPersistentModelIndex::row();
impl<'a> /*trait*/ QPersistentModelIndex_row for () {
  fn row(self, rsthis: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex3rowEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn internalId<T: QPersistentModelIndex_internalId>(&mut self, value: T) -> i32 {
    return value.internalId(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_internalId {
  fn internalId(self, rsthis: &mut QPersistentModelIndex) -> i32;
}

// proto:  int QPersistentModelIndex::internalId();
impl<'a> /*trait*/ QPersistentModelIndex_internalId for () {
  fn internalId(self, rsthis: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex10internalIdEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex10internalIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn model<T: QPersistentModelIndex_model>(&mut self, value: T)  {
     value.model(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_model {
  fn model(self, rsthis: &mut QPersistentModelIndex) ;
}

// proto:  const QAbstractItemModel * QPersistentModelIndex::model();
impl<'a> /*trait*/ QPersistentModelIndex_model for () {
  fn model(self, rsthis: &mut QPersistentModelIndex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex5modelEv()};
     unsafe {_ZNK21QPersistentModelIndex5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn FreeQPersistentModelIndex<T: QPersistentModelIndex_FreeQPersistentModelIndex>(&mut self, value: T)  {
     value.FreeQPersistentModelIndex(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_FreeQPersistentModelIndex {
  fn FreeQPersistentModelIndex(self, rsthis: &mut QPersistentModelIndex) ;
}

// proto:  void QPersistentModelIndex::FreeQPersistentModelIndex();
impl<'a> /*trait*/ QPersistentModelIndex_FreeQPersistentModelIndex for () {
  fn FreeQPersistentModelIndex(self, rsthis: &mut QPersistentModelIndex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexD0Ev()};
     unsafe {_ZN21QPersistentModelIndexD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QPersistentModelIndex::NewQPersistentModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QPersistentModelIndex_NewQPersistentModelIndex for (&'a  QModelIndex) {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1ERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QPersistentModelIndexC1ERK11QModelIndex(qthis, arg0)};
    let rsthis = QPersistentModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPersistentModelIndex::NewQPersistentModelIndex();
impl<'a> /*trait*/ QPersistentModelIndex_NewQPersistentModelIndex for () {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1Ev()};
    unsafe {_ZN21QPersistentModelIndexC1Ev(qthis)};
    let rsthis = QPersistentModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn column<T: QPersistentModelIndex_column>(&mut self, value: T) -> i32 {
    return value.column(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_column {
  fn column(self, rsthis: &mut QPersistentModelIndex) -> i32;
}

// proto:  int QPersistentModelIndex::column();
impl<'a> /*trait*/ QPersistentModelIndex_column for () {
  fn column(self, rsthis: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex6columnEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn swap<T: QPersistentModelIndex_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_swap {
  fn swap(self, rsthis: &mut QPersistentModelIndex) ;
}

// proto:  void QPersistentModelIndex::swap(QPersistentModelIndex & other);
impl<'a> /*trait*/ QPersistentModelIndex_swap for (&'a mut QPersistentModelIndex) {
  fn swap(self, rsthis: &mut QPersistentModelIndex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndex4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QPersistentModelIndex4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn child<T: QPersistentModelIndex_child>(&mut self, value: T) -> QModelIndex {
    return value.child(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_child {
  fn child(self, rsthis: &mut QPersistentModelIndex) -> QModelIndex;
}

// proto:  QModelIndex QPersistentModelIndex::child(int row, int column);
impl<'a> /*trait*/ QPersistentModelIndex_child for (i32, i32) {
  fn child(self, rsthis: &mut QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex5childEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn isValid<T: QPersistentModelIndex_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_isValid {
  fn isValid(self, rsthis: &mut QPersistentModelIndex) -> i8;
}

// proto:  bool QPersistentModelIndex::isValid();
impl<'a> /*trait*/ QPersistentModelIndex_isValid for () {
  fn isValid(self, rsthis: &mut QPersistentModelIndex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex7isValidEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

