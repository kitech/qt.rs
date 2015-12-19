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

// proto:  int QAccessibleTableCellInterface::columnIndex();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnIndex<RetType, T: QAccessibleTableCellInterface_columnIndex<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnIndex(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnIndex<RetType> {
  fn columnIndex(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  int QAccessibleTableCellInterface::columnIndex();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnIndex<i32> for () {
  fn columnIndex(self , rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface11columnIndexEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface11columnIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QAccessibleTableCellInterface::FreeQAccessibleTableCellInterface();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn FreeQAccessibleTableCellInterface<RetType, T: QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleTableCellInterface(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface<RetType> {
  fn FreeQAccessibleTableCellInterface(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  void QAccessibleTableCellInterface::FreeQAccessibleTableCellInterface();
impl<'a> /*trait*/ QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface<()> for () {
  fn FreeQAccessibleTableCellInterface(self , rsthis: &mut QAccessibleTableCellInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QAccessibleTableCellInterfaceD0Ev()};
     unsafe {_ZN29QAccessibleTableCellInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QAccessibleTableCellInterface::columnExtent();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnExtent<RetType, T: QAccessibleTableCellInterface_columnExtent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnExtent(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnExtent<RetType> {
  fn columnExtent(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  int QAccessibleTableCellInterface::columnExtent();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnExtent<i32> for () {
  fn columnExtent(self , rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface12columnExtentEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface12columnExtentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QAccessibleTableCellInterface::rowIndex();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowIndex<RetType, T: QAccessibleTableCellInterface_rowIndex<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowIndex(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowIndex<RetType> {
  fn rowIndex(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  int QAccessibleTableCellInterface::rowIndex();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowIndex<i32> for () {
  fn rowIndex(self , rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface8rowIndexEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface8rowIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QAccessibleInterface * QAccessibleTableCellInterface::table();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn table<RetType, T: QAccessibleTableCellInterface_table<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.table(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_table<RetType> {
  fn table(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  QAccessibleInterface * QAccessibleTableCellInterface::table();
impl<'a> /*trait*/ QAccessibleTableCellInterface_table<QAccessibleInterface> for () {
  fn table(self , rsthis: &mut QAccessibleTableCellInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface5tableEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface5tableEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QAccessibleTableCellInterface::rowExtent();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowExtent<RetType, T: QAccessibleTableCellInterface_rowExtent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowExtent(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowExtent<RetType> {
  fn rowExtent(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  int QAccessibleTableCellInterface::rowExtent();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowExtent<i32> for () {
  fn rowExtent(self , rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface9rowExtentEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface9rowExtentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowHeaderCells<RetType, T: QAccessibleTableCellInterface_rowHeaderCells<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowHeaderCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowHeaderCells<RetType> {
  fn rowHeaderCells(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowHeaderCells<()> for () {
  fn rowHeaderCells(self , rsthis: &mut QAccessibleTableCellInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv()};
     unsafe {_ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnHeaderCells<RetType, T: QAccessibleTableCellInterface_columnHeaderCells<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnHeaderCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnHeaderCells<RetType> {
  fn columnHeaderCells(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnHeaderCells<()> for () {
  fn columnHeaderCells(self , rsthis: &mut QAccessibleTableCellInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv()};
     unsafe {_ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QAccessibleTableCellInterface::isSelected();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn isSelected<RetType, T: QAccessibleTableCellInterface_isSelected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_isSelected<RetType> {
  fn isSelected(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

// proto:  bool QAccessibleTableCellInterface::isSelected();
impl<'a> /*trait*/ QAccessibleTableCellInterface_isSelected<i8> for () {
  fn isSelected(self , rsthis: &mut QAccessibleTableCellInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface10isSelectedEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

