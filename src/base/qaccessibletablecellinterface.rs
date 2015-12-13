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
  // proto: int QAccessibleTableCellInterface::columnIndex();
  fn _ZNK29QAccessibleTableCellInterface11columnIndexEv() -> i32;
  // proto: void QAccessibleTableCellInterface::FreeQAccessibleTableCellInterface();
  fn _ZN29QAccessibleTableCellInterfaceD0Ev() -> i32;
  // proto: int QAccessibleTableCellInterface::columnExtent();
  fn _ZNK29QAccessibleTableCellInterface12columnExtentEv() -> i32;
  // proto: int QAccessibleTableCellInterface::rowIndex();
  fn _ZNK29QAccessibleTableCellInterface8rowIndexEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleTableCellInterface::table();
  fn _ZNK29QAccessibleTableCellInterface5tableEv() -> i32;
  // proto: int QAccessibleTableCellInterface::rowExtent();
  fn _ZNK29QAccessibleTableCellInterface9rowExtentEv() -> i32;
  // proto: QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
  fn _ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv() -> i32;
  // proto: QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
  fn _ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv() -> i32;
  // proto: bool QAccessibleTableCellInterface::isSelected();
  fn _ZNK29QAccessibleTableCellInterface10isSelectedEv() -> i32;
}

// body block begin
// class sizeof(QAccessibleTableCellInterface)=8
pub struct QAccessibleTableCellInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnIndex<T: QAccessibleTableCellInterface_columnIndex>(&mut self, value: T) -> i32 {
    value.columnIndex(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnIndex {
  fn columnIndex(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: int QAccessibleTableCellInterface::columnIndex();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnIndex for () {
  fn columnIndex(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface11columnIndexEv()};
    unsafe {_ZNK29QAccessibleTableCellInterface11columnIndexEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn FreeQAccessibleTableCellInterface<T: QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleTableCellInterface(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface {
  fn FreeQAccessibleTableCellInterface(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: void QAccessibleTableCellInterface::FreeQAccessibleTableCellInterface();
impl<'a> /*trait*/ QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface for () {
  fn FreeQAccessibleTableCellInterface(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QAccessibleTableCellInterfaceD0Ev()};
    unsafe {_ZN29QAccessibleTableCellInterfaceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnExtent<T: QAccessibleTableCellInterface_columnExtent>(&mut self, value: T) -> i32 {
    value.columnExtent(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnExtent {
  fn columnExtent(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: int QAccessibleTableCellInterface::columnExtent();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnExtent for () {
  fn columnExtent(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface12columnExtentEv()};
    unsafe {_ZNK29QAccessibleTableCellInterface12columnExtentEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowIndex<T: QAccessibleTableCellInterface_rowIndex>(&mut self, value: T) -> i32 {
    value.rowIndex(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowIndex {
  fn rowIndex(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: int QAccessibleTableCellInterface::rowIndex();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowIndex for () {
  fn rowIndex(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface8rowIndexEv()};
    unsafe {_ZNK29QAccessibleTableCellInterface8rowIndexEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn table<T: QAccessibleTableCellInterface_table>(&mut self, value: T) -> i32 {
    value.table(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_table {
  fn table(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: QAccessibleInterface * QAccessibleTableCellInterface::table();
impl<'a> /*trait*/ QAccessibleTableCellInterface_table for () {
  fn table(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface5tableEv()};
    unsafe {_ZNK29QAccessibleTableCellInterface5tableEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowExtent<T: QAccessibleTableCellInterface_rowExtent>(&mut self, value: T) -> i32 {
    value.rowExtent(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowExtent {
  fn rowExtent(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: int QAccessibleTableCellInterface::rowExtent();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowExtent for () {
  fn rowExtent(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface9rowExtentEv()};
    unsafe {_ZNK29QAccessibleTableCellInterface9rowExtentEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowHeaderCells<T: QAccessibleTableCellInterface_rowHeaderCells>(&mut self, value: T) -> i32 {
    value.rowHeaderCells(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowHeaderCells {
  fn rowHeaderCells(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowHeaderCells for () {
  fn rowHeaderCells(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv()};
    unsafe {_ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnHeaderCells<T: QAccessibleTableCellInterface_columnHeaderCells>(&mut self, value: T) -> i32 {
    value.columnHeaderCells(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnHeaderCells {
  fn columnHeaderCells(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnHeaderCells for () {
  fn columnHeaderCells(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv()};
    unsafe {_ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn isSelected<T: QAccessibleTableCellInterface_isSelected>(&mut self, value: T) -> i32 {
    value.isSelected(self);
    return 1;
  }
}

pub trait QAccessibleTableCellInterface_isSelected {
  fn isSelected(self, this: &mut QAccessibleTableCellInterface) -> i32;
}

// proto: bool QAccessibleTableCellInterface::isSelected();
impl<'a> /*trait*/ QAccessibleTableCellInterface_isSelected for () {
  fn isSelected(self, this: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface10isSelectedEv()};
    unsafe {_ZNK29QAccessibleTableCellInterface10isSelectedEv()};
    return 1;
  }
}

