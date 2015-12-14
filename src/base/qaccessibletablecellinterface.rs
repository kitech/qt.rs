// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QAccessibleTableCellInterface::columnIndex();
  fn _ZNK29QAccessibleTableCellInterface11columnIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTableCellInterface::FreeQAccessibleTableCellInterface();
  fn _ZN29QAccessibleTableCellInterfaceD0Ev(qthis: *mut c_void) ;
  // proto:  int QAccessibleTableCellInterface::columnExtent();
  fn _ZNK29QAccessibleTableCellInterface12columnExtentEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAccessibleTableCellInterface::rowIndex();
  fn _ZNK29QAccessibleTableCellInterface8rowIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  QAccessibleInterface * QAccessibleTableCellInterface::table();
  fn _ZNK29QAccessibleTableCellInterface5tableEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleTableCellInterface::rowExtent();
  fn _ZNK29QAccessibleTableCellInterface9rowExtentEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
  fn _ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv(qthis: *mut c_void) ;
  // proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
  fn _ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv(qthis: *mut c_void) ;
  // proto:  bool QAccessibleTableCellInterface::isSelected();
  fn _ZNK29QAccessibleTableCellInterface10isSelectedEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QAccessibleTableCellInterface)=8
pub struct QAccessibleTableCellInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnIndex<T: QAccessibleTableCellInterface_columnIndex>(&mut self, value: T) -> i32 {
    return value.columnIndex(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnIndex {
  fn columnIndex(self, rsthis: &mut QAccessibleTableCellInterface) -> i32;
}

// proto:  int QAccessibleTableCellInterface::columnIndex();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnIndex for () {
  fn columnIndex(self, rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface11columnIndexEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface11columnIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn FreeQAccessibleTableCellInterface<T: QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface>(&mut self, value: T)  {
     value.FreeQAccessibleTableCellInterface(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface {
  fn FreeQAccessibleTableCellInterface(self, rsthis: &mut QAccessibleTableCellInterface) ;
}

// proto:  void QAccessibleTableCellInterface::FreeQAccessibleTableCellInterface();
impl<'a> /*trait*/ QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface for () {
  fn FreeQAccessibleTableCellInterface(self, rsthis: &mut QAccessibleTableCellInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QAccessibleTableCellInterfaceD0Ev()};
     unsafe {_ZN29QAccessibleTableCellInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnExtent<T: QAccessibleTableCellInterface_columnExtent>(&mut self, value: T) -> i32 {
    return value.columnExtent(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnExtent {
  fn columnExtent(self, rsthis: &mut QAccessibleTableCellInterface) -> i32;
}

// proto:  int QAccessibleTableCellInterface::columnExtent();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnExtent for () {
  fn columnExtent(self, rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface12columnExtentEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface12columnExtentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowIndex<T: QAccessibleTableCellInterface_rowIndex>(&mut self, value: T) -> i32 {
    return value.rowIndex(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowIndex {
  fn rowIndex(self, rsthis: &mut QAccessibleTableCellInterface) -> i32;
}

// proto:  int QAccessibleTableCellInterface::rowIndex();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowIndex for () {
  fn rowIndex(self, rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface8rowIndexEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface8rowIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn table<T: QAccessibleTableCellInterface_table>(&mut self, value: T) -> QAccessibleInterface {
    return value.table(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_table {
  fn table(self, rsthis: &mut QAccessibleTableCellInterface) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleTableCellInterface::table();
impl<'a> /*trait*/ QAccessibleTableCellInterface_table for () {
  fn table(self, rsthis: &mut QAccessibleTableCellInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface5tableEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface5tableEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowExtent<T: QAccessibleTableCellInterface_rowExtent>(&mut self, value: T) -> i32 {
    return value.rowExtent(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowExtent {
  fn rowExtent(self, rsthis: &mut QAccessibleTableCellInterface) -> i32;
}

// proto:  int QAccessibleTableCellInterface::rowExtent();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowExtent for () {
  fn rowExtent(self, rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface9rowExtentEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface9rowExtentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowHeaderCells<T: QAccessibleTableCellInterface_rowHeaderCells>(&mut self, value: T)  {
     value.rowHeaderCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowHeaderCells {
  fn rowHeaderCells(self, rsthis: &mut QAccessibleTableCellInterface) ;
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowHeaderCells for () {
  fn rowHeaderCells(self, rsthis: &mut QAccessibleTableCellInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv()};
     unsafe {_ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnHeaderCells<T: QAccessibleTableCellInterface_columnHeaderCells>(&mut self, value: T)  {
     value.columnHeaderCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnHeaderCells {
  fn columnHeaderCells(self, rsthis: &mut QAccessibleTableCellInterface) ;
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnHeaderCells for () {
  fn columnHeaderCells(self, rsthis: &mut QAccessibleTableCellInterface)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv()};
     unsafe {_ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn isSelected<T: QAccessibleTableCellInterface_isSelected>(&mut self, value: T) -> i8 {
    return value.isSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_isSelected {
  fn isSelected(self, rsthis: &mut QAccessibleTableCellInterface) -> i8;
}

// proto:  bool QAccessibleTableCellInterface::isSelected();
impl<'a> /*trait*/ QAccessibleTableCellInterface_isSelected for () {
  fn isSelected(self, rsthis: &mut QAccessibleTableCellInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface10isSelectedEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

