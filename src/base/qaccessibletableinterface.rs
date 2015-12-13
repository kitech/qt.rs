// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibletablemodelchangeevent::QAccessibleTableModelChangeEvent;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QAccessibleTableInterface::unselectColumn(int column);
  fn _ZN25QAccessibleTableInterface14unselectColumnEi(arg0: c_int) -> i32;
  // proto: QString QAccessibleTableInterface::columnDescription(int column);
  fn _ZNK25QAccessibleTableInterface17columnDescriptionEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTableInterface::selectedCellCount();
  fn _ZNK25QAccessibleTableInterface17selectedCellCountEv() -> i32;
  // proto: QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
  fn _ZNK25QAccessibleTableInterface13selectedCellsEv() -> i32;
  // proto: bool QAccessibleTableInterface::selectRow(int row);
  fn _ZN25QAccessibleTableInterface9selectRowEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTableInterface::selectedRowCount();
  fn _ZNK25QAccessibleTableInterface16selectedRowCountEv() -> i32;
  // proto: void QAccessibleTableInterface::FreeQAccessibleTableInterface();
  fn _ZN25QAccessibleTableInterfaceD0Ev() -> i32;
  // proto: QList<int> QAccessibleTableInterface::selectedColumns();
  fn _ZNK25QAccessibleTableInterface15selectedColumnsEv() -> i32;
  // proto: QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
  fn _ZNK25QAccessibleTableInterface6cellAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QList<int> QAccessibleTableInterface::selectedRows();
  fn _ZNK25QAccessibleTableInterface12selectedRowsEv() -> i32;
  // proto: void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
  fn _ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent(arg0: *mut c_void) -> i32;
  // proto: int QAccessibleTableInterface::columnCount();
  fn _ZNK25QAccessibleTableInterface11columnCountEv() -> i32;
  // proto: bool QAccessibleTableInterface::selectColumn(int column);
  fn _ZN25QAccessibleTableInterface12selectColumnEi(arg0: c_int) -> i32;
  // proto: bool QAccessibleTableInterface::unselectRow(int row);
  fn _ZN25QAccessibleTableInterface11unselectRowEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTableInterface::rowCount();
  fn _ZNK25QAccessibleTableInterface8rowCountEv() -> i32;
  // proto: QString QAccessibleTableInterface::rowDescription(int row);
  fn _ZNK25QAccessibleTableInterface14rowDescriptionEi(arg0: c_int) -> i32;
  // proto: QAccessibleInterface * QAccessibleTableInterface::summary();
  fn _ZNK25QAccessibleTableInterface7summaryEv() -> i32;
  // proto: bool QAccessibleTableInterface::isColumnSelected(int column);
  fn _ZNK25QAccessibleTableInterface16isColumnSelectedEi(arg0: c_int) -> i32;
  // proto: QAccessibleInterface * QAccessibleTableInterface::caption();
  fn _ZNK25QAccessibleTableInterface7captionEv() -> i32;
  // proto: bool QAccessibleTableInterface::isRowSelected(int row);
  fn _ZNK25QAccessibleTableInterface13isRowSelectedEi(arg0: c_int) -> i32;
  // proto: int QAccessibleTableInterface::selectedColumnCount();
  fn _ZNK25QAccessibleTableInterface19selectedColumnCountEv() -> i32;
}

// body block begin
// class sizeof(QAccessibleTableInterface)=8
pub struct QAccessibleTableInterface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectColumn<T: QAccessibleTableInterface_unselectColumn>(&mut self, value: T) -> i32 {
    value.unselectColumn(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_unselectColumn {
  fn unselectColumn(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: bool QAccessibleTableInterface::unselectColumn(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_unselectColumn for (i32) {
  fn unselectColumn(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface14unselectColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN25QAccessibleTableInterface14unselectColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn columnDescription<T: QAccessibleTableInterface_columnDescription>(&mut self, value: T) -> i32 {
    value.columnDescription(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_columnDescription {
  fn columnDescription(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: QString QAccessibleTableInterface::columnDescription(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_columnDescription for (i32) {
  fn columnDescription(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface17columnDescriptionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK25QAccessibleTableInterface17columnDescriptionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCellCount<T: QAccessibleTableInterface_selectedCellCount>(&mut self, value: T) -> i32 {
    value.selectedCellCount(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_selectedCellCount {
  fn selectedCellCount(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: int QAccessibleTableInterface::selectedCellCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCellCount for () {
  fn selectedCellCount(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface17selectedCellCountEv()};
    unsafe {_ZNK25QAccessibleTableInterface17selectedCellCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCells<T: QAccessibleTableInterface_selectedCells>(&mut self, value: T) -> i32 {
    value.selectedCells(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_selectedCells {
  fn selectedCells(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCells for () {
  fn selectedCells(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface13selectedCellsEv()};
    unsafe {_ZNK25QAccessibleTableInterface13selectedCellsEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectRow<T: QAccessibleTableInterface_selectRow>(&mut self, value: T) -> i32 {
    value.selectRow(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_selectRow {
  fn selectRow(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: bool QAccessibleTableInterface::selectRow(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_selectRow for (i32) {
  fn selectRow(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface9selectRowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN25QAccessibleTableInterface9selectRowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRowCount<T: QAccessibleTableInterface_selectedRowCount>(&mut self, value: T) -> i32 {
    value.selectedRowCount(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_selectedRowCount {
  fn selectedRowCount(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: int QAccessibleTableInterface::selectedRowCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRowCount for () {
  fn selectedRowCount(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface16selectedRowCountEv()};
    unsafe {_ZNK25QAccessibleTableInterface16selectedRowCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn FreeQAccessibleTableInterface<T: QAccessibleTableInterface_FreeQAccessibleTableInterface>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleTableInterface(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_FreeQAccessibleTableInterface {
  fn FreeQAccessibleTableInterface(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: void QAccessibleTableInterface::FreeQAccessibleTableInterface();
impl<'a> /*trait*/ QAccessibleTableInterface_FreeQAccessibleTableInterface for () {
  fn FreeQAccessibleTableInterface(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterfaceD0Ev()};
    unsafe {_ZN25QAccessibleTableInterfaceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumns<T: QAccessibleTableInterface_selectedColumns>(&mut self, value: T) -> i32 {
    value.selectedColumns(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_selectedColumns {
  fn selectedColumns(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: QList<int> QAccessibleTableInterface::selectedColumns();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumns for () {
  fn selectedColumns(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface15selectedColumnsEv()};
    unsafe {_ZNK25QAccessibleTableInterface15selectedColumnsEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn cellAt<T: QAccessibleTableInterface_cellAt>(&mut self, value: T) -> i32 {
    value.cellAt(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_cellAt {
  fn cellAt(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
impl<'a> /*trait*/ QAccessibleTableInterface_cellAt for (i32, i32) {
  fn cellAt(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface6cellAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK25QAccessibleTableInterface6cellAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRows<T: QAccessibleTableInterface_selectedRows>(&mut self, value: T) -> i32 {
    value.selectedRows(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_selectedRows {
  fn selectedRows(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: QList<int> QAccessibleTableInterface::selectedRows();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRows for () {
  fn selectedRows(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface12selectedRowsEv()};
    unsafe {_ZNK25QAccessibleTableInterface12selectedRowsEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn modelChange<T: QAccessibleTableInterface_modelChange>(&mut self, value: T) -> i32 {
    value.modelChange(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_modelChange {
  fn modelChange(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
impl<'a> /*trait*/ QAccessibleTableInterface_modelChange for (&'a mut QAccessibleTableModelChangeEvent) {
  fn modelChange(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn columnCount<T: QAccessibleTableInterface_columnCount>(&mut self, value: T) -> i32 {
    value.columnCount(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_columnCount {
  fn columnCount(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: int QAccessibleTableInterface::columnCount();
impl<'a> /*trait*/ QAccessibleTableInterface_columnCount for () {
  fn columnCount(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface11columnCountEv()};
    unsafe {_ZNK25QAccessibleTableInterface11columnCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectColumn<T: QAccessibleTableInterface_selectColumn>(&mut self, value: T) -> i32 {
    value.selectColumn(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_selectColumn {
  fn selectColumn(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: bool QAccessibleTableInterface::selectColumn(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_selectColumn for (i32) {
  fn selectColumn(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface12selectColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN25QAccessibleTableInterface12selectColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectRow<T: QAccessibleTableInterface_unselectRow>(&mut self, value: T) -> i32 {
    value.unselectRow(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_unselectRow {
  fn unselectRow(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: bool QAccessibleTableInterface::unselectRow(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_unselectRow for (i32) {
  fn unselectRow(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface11unselectRowEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN25QAccessibleTableInterface11unselectRowEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn rowCount<T: QAccessibleTableInterface_rowCount>(&mut self, value: T) -> i32 {
    value.rowCount(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_rowCount {
  fn rowCount(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: int QAccessibleTableInterface::rowCount();
impl<'a> /*trait*/ QAccessibleTableInterface_rowCount for () {
  fn rowCount(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface8rowCountEv()};
    unsafe {_ZNK25QAccessibleTableInterface8rowCountEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn rowDescription<T: QAccessibleTableInterface_rowDescription>(&mut self, value: T) -> i32 {
    value.rowDescription(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_rowDescription {
  fn rowDescription(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: QString QAccessibleTableInterface::rowDescription(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_rowDescription for (i32) {
  fn rowDescription(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface14rowDescriptionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK25QAccessibleTableInterface14rowDescriptionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn summary<T: QAccessibleTableInterface_summary>(&mut self, value: T) -> i32 {
    value.summary(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_summary {
  fn summary(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: QAccessibleInterface * QAccessibleTableInterface::summary();
impl<'a> /*trait*/ QAccessibleTableInterface_summary for () {
  fn summary(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface7summaryEv()};
    unsafe {_ZNK25QAccessibleTableInterface7summaryEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn isColumnSelected<T: QAccessibleTableInterface_isColumnSelected>(&mut self, value: T) -> i32 {
    value.isColumnSelected(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_isColumnSelected {
  fn isColumnSelected(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: bool QAccessibleTableInterface::isColumnSelected(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_isColumnSelected for (i32) {
  fn isColumnSelected(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface16isColumnSelectedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK25QAccessibleTableInterface16isColumnSelectedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn caption<T: QAccessibleTableInterface_caption>(&mut self, value: T) -> i32 {
    value.caption(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_caption {
  fn caption(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: QAccessibleInterface * QAccessibleTableInterface::caption();
impl<'a> /*trait*/ QAccessibleTableInterface_caption for () {
  fn caption(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface7captionEv()};
    unsafe {_ZNK25QAccessibleTableInterface7captionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn isRowSelected<T: QAccessibleTableInterface_isRowSelected>(&mut self, value: T) -> i32 {
    value.isRowSelected(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_isRowSelected {
  fn isRowSelected(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: bool QAccessibleTableInterface::isRowSelected(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_isRowSelected for (i32) {
  fn isRowSelected(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface13isRowSelectedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK25QAccessibleTableInterface13isRowSelectedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumnCount<T: QAccessibleTableInterface_selectedColumnCount>(&mut self, value: T) -> i32 {
    value.selectedColumnCount(self);
    return 1;
  }
}

pub trait QAccessibleTableInterface_selectedColumnCount {
  fn selectedColumnCount(self, this: &mut QAccessibleTableInterface) -> i32;
}

// proto: int QAccessibleTableInterface::selectedColumnCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumnCount for () {
  fn selectedColumnCount(self, this: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface19selectedColumnCountEv()};
    unsafe {_ZNK25QAccessibleTableInterface19selectedColumnCountEv()};
    return 1;
  }
}

