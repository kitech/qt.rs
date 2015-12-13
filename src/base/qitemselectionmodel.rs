// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmodelindex::QModelIndex;
use super::qitemselection::QItemSelection;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK19QItemSelectionModel10metaObjectEv() -> i32;
  fn _ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK19QItemSelectionModel9selectionEv() -> i32;
  fn _ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN19QItemSelectionModel14clearSelectionEv() -> i32;
  fn _ZNK19QItemSelectionModel12currentIndexEv() -> i32;
  fn _ZN19QItemSelectionModelC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(arg0: c_int, arg1: *const c_void) -> i32;
  fn _ZN19QItemSelectionModel5resetEv() -> i32;
  fn _ZN19QItemSelectionModel17clearCurrentIndexEv() -> i32;
  fn _ZNK19QItemSelectionModel15selectedIndexesEv() -> i32;
  fn _ZNK19QItemSelectionModel15selectedColumnsEi(arg0: c_int) -> i32;
  fn _ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(arg0: c_int, arg1: *const c_void) -> i32;
  fn _ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(arg0: c_int, arg1: *const c_void) -> i32;
  fn _ZN19QItemSelectionModelD0Ev() -> i32;
  fn _ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(arg0: c_int, arg1: *const c_void) -> i32;
  fn _ZN19QItemSelectionModel5clearEv() -> i32;
  fn _ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK19QItemSelectionModel12hasSelectionEv() -> i32;
  fn _ZN19QItemSelectionModel5modelEv() -> i32;
  fn _ZNK19QItemSelectionModel12selectedRowsEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QItemSelectionModel)=1
pub struct QItemSelectionModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemSelectionModel {
  pub fn currentColumnChanged<T: QItemSelectionModel_currentColumnChanged>(&mut self, value: T) -> i32 {
    value.currentColumnChanged(self);
    return 1;
  }
}

pub trait QItemSelectionModel_currentColumnChanged {
  fn currentColumnChanged(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentColumnChanged for (&'a  QModelIndex, &'a  QModelIndex) {
  fn currentColumnChanged(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn metaObject<T: QItemSelectionModel_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QItemSelectionModel_metaObject {
  fn metaObject(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: const QMetaObject * QItemSelectionModel::metaObject();
impl<'a> /*trait*/ QItemSelectionModel_metaObject for () {
  fn metaObject(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10metaObjectEv()};
    unsafe {_ZNK19QItemSelectionModel10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn currentChanged<T: QItemSelectionModel_currentChanged>(&mut self, value: T) -> i32 {
    value.currentChanged(self);
    return 1;
  }
}

pub trait QItemSelectionModel_currentChanged {
  fn currentChanged(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentChanged for (&'a  QModelIndex, &'a  QModelIndex) {
  fn currentChanged(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selection<T: QItemSelectionModel_selection>(&mut self, value: T) -> i32 {
    value.selection(self);
    return 1;
  }
}

pub trait QItemSelectionModel_selection {
  fn selection(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: const QItemSelection QItemSelectionModel::selection();
impl<'a> /*trait*/ QItemSelectionModel_selection for () {
  fn selection(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel9selectionEv()};
    unsafe {_ZNK19QItemSelectionModel9selectionEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn isSelected<T: QItemSelectionModel_isSelected>(&mut self, value: T) -> i32 {
    value.isSelected(self);
    return 1;
  }
}

pub trait QItemSelectionModel_isSelected {
  fn isSelected(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: bool QItemSelectionModel::isSelected(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionModel_isSelected for (&'a  QModelIndex) {
  fn isSelected(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selectionChanged<T: QItemSelectionModel_selectionChanged>(&mut self, value: T) -> i32 {
    value.selectionChanged(self);
    return 1;
  }
}

pub trait QItemSelectionModel_selectionChanged {
  fn selectionChanged(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
impl<'a> /*trait*/ QItemSelectionModel_selectionChanged for (&'a  QItemSelection, &'a  QItemSelection) {
  fn selectionChanged(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn clearSelection<T: QItemSelectionModel_clearSelection>(&mut self, value: T) -> i32 {
    value.clearSelection(self);
    return 1;
  }
}

pub trait QItemSelectionModel_clearSelection {
  fn clearSelection(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::clearSelection();
impl<'a> /*trait*/ QItemSelectionModel_clearSelection for () {
  fn clearSelection(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14clearSelectionEv()};
    unsafe {_ZN19QItemSelectionModel14clearSelectionEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn currentIndex<T: QItemSelectionModel_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QItemSelectionModel_currentIndex {
  fn currentIndex(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: QModelIndex QItemSelectionModel::currentIndex();
impl<'a> /*trait*/ QItemSelectionModel_currentIndex for () {
  fn currentIndex(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12currentIndexEv()};
    unsafe {_ZNK19QItemSelectionModel12currentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn NewQItemSelectionModel<T: QItemSelectionModel_NewQItemSelectionModel>(value: T) -> QItemSelectionModel {
    let rsthis = value.NewQItemSelectionModel();
    return rsthis;
    // return 1;
  }
}

pub trait QItemSelectionModel_NewQItemSelectionModel {
  fn NewQItemSelectionModel(self) -> QItemSelectionModel;
}

// proto: void QItemSelectionModel::NewQItemSelectionModel(const QItemSelectionModel & );
impl<'a> /*trait*/ QItemSelectionModel_NewQItemSelectionModel for (&'a  QItemSelectionModel) {
  fn NewQItemSelectionModel(self) -> QItemSelectionModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QItemSelectionModelC1ERKS_(qthis, arg0)};
    let rsthis = QItemSelectionModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn rowIntersectsSelection<T: QItemSelectionModel_rowIntersectsSelection>(&mut self, value: T) -> i32 {
    value.rowIntersectsSelection(self);
    return 1;
  }
}

pub trait QItemSelectionModel_rowIntersectsSelection {
  fn rowIntersectsSelection(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_rowIntersectsSelection for (i32, &'a  QModelIndex) {
  fn rowIntersectsSelection(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn reset<T: QItemSelectionModel_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QItemSelectionModel_reset {
  fn reset(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::reset();
impl<'a> /*trait*/ QItemSelectionModel_reset for () {
  fn reset(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5resetEv()};
    unsafe {_ZN19QItemSelectionModel5resetEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn clearCurrentIndex<T: QItemSelectionModel_clearCurrentIndex>(&mut self, value: T) -> i32 {
    value.clearCurrentIndex(self);
    return 1;
  }
}

pub trait QItemSelectionModel_clearCurrentIndex {
  fn clearCurrentIndex(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::clearCurrentIndex();
impl<'a> /*trait*/ QItemSelectionModel_clearCurrentIndex for () {
  fn clearCurrentIndex(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17clearCurrentIndexEv()};
    unsafe {_ZN19QItemSelectionModel17clearCurrentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selectedIndexes<T: QItemSelectionModel_selectedIndexes>(&mut self, value: T) -> i32 {
    value.selectedIndexes(self);
    return 1;
  }
}

pub trait QItemSelectionModel_selectedIndexes {
  fn selectedIndexes(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: QList<QModelIndex> QItemSelectionModel::selectedIndexes();
impl<'a> /*trait*/ QItemSelectionModel_selectedIndexes for () {
  fn selectedIndexes(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedIndexesEv()};
    unsafe {_ZNK19QItemSelectionModel15selectedIndexesEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selectedColumns<T: QItemSelectionModel_selectedColumns>(&mut self, value: T) -> i32 {
    value.selectedColumns(self);
    return 1;
  }
}

pub trait QItemSelectionModel_selectedColumns {
  fn selectedColumns(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: QList<QModelIndex> QItemSelectionModel::selectedColumns(int row);
impl<'a> /*trait*/ QItemSelectionModel_selectedColumns for (i32) {
  fn selectedColumns(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedColumnsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QItemSelectionModel15selectedColumnsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn isColumnSelected<T: QItemSelectionModel_isColumnSelected>(&mut self, value: T) -> i32 {
    value.isColumnSelected(self);
    return 1;
  }
}

pub trait QItemSelectionModel_isColumnSelected {
  fn isColumnSelected(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isColumnSelected for (i32, &'a  QModelIndex) {
  fn isColumnSelected(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn columnIntersectsSelection<T: QItemSelectionModel_columnIntersectsSelection>(&mut self, value: T) -> i32 {
    value.columnIntersectsSelection(self);
    return 1;
  }
}

pub trait QItemSelectionModel_columnIntersectsSelection {
  fn columnIntersectsSelection(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_columnIntersectsSelection for (i32, &'a  QModelIndex) {
  fn columnIntersectsSelection(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn FreeQItemSelectionModel<T: QItemSelectionModel_FreeQItemSelectionModel>(&mut self, value: T) -> i32 {
    value.FreeQItemSelectionModel(self);
    return 1;
  }
}

pub trait QItemSelectionModel_FreeQItemSelectionModel {
  fn FreeQItemSelectionModel(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::FreeQItemSelectionModel();
impl<'a> /*trait*/ QItemSelectionModel_FreeQItemSelectionModel for () {
  fn FreeQItemSelectionModel(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelD0Ev()};
    unsafe {_ZN19QItemSelectionModelD0Ev()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn isRowSelected<T: QItemSelectionModel_isRowSelected>(&mut self, value: T) -> i32 {
    value.isRowSelected(self);
    return 1;
  }
}

pub trait QItemSelectionModel_isRowSelected {
  fn isRowSelected(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isRowSelected for (i32, &'a  QModelIndex) {
  fn isRowSelected(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn clear<T: QItemSelectionModel_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QItemSelectionModel_clear {
  fn clear(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::clear();
impl<'a> /*trait*/ QItemSelectionModel_clear for () {
  fn clear(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5clearEv()};
    unsafe {_ZN19QItemSelectionModel5clearEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn currentRowChanged<T: QItemSelectionModel_currentRowChanged>(&mut self, value: T) -> i32 {
    value.currentRowChanged(self);
    return 1;
  }
}

pub trait QItemSelectionModel_currentRowChanged {
  fn currentRowChanged(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentRowChanged for (&'a  QModelIndex, &'a  QModelIndex) {
  fn currentRowChanged(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn hasSelection<T: QItemSelectionModel_hasSelection>(&mut self, value: T) -> i32 {
    value.hasSelection(self);
    return 1;
  }
}

pub trait QItemSelectionModel_hasSelection {
  fn hasSelection(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: bool QItemSelectionModel::hasSelection();
impl<'a> /*trait*/ QItemSelectionModel_hasSelection for () {
  fn hasSelection(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12hasSelectionEv()};
    unsafe {_ZNK19QItemSelectionModel12hasSelectionEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn model<T: QItemSelectionModel_model>(&mut self, value: T) -> i32 {
    value.model(self);
    return 1;
  }
}

pub trait QItemSelectionModel_model {
  fn model(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: QAbstractItemModel * QItemSelectionModel::model();
impl<'a> /*trait*/ QItemSelectionModel_model for () {
  fn model(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5modelEv()};
    unsafe {_ZN19QItemSelectionModel5modelEv()};
    return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selectedRows<T: QItemSelectionModel_selectedRows>(&mut self, value: T) -> i32 {
    value.selectedRows(self);
    return 1;
  }
}

pub trait QItemSelectionModel_selectedRows {
  fn selectedRows(self, this: &mut QItemSelectionModel) -> i32;
}

// proto: QList<QModelIndex> QItemSelectionModel::selectedRows(int column);
impl<'a> /*trait*/ QItemSelectionModel_selectedRows for (i32) {
  fn selectedRows(self, this: &mut QItemSelectionModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12selectedRowsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QItemSelectionModel12selectedRowsEi(arg0)};
    return 1;
  }
}

