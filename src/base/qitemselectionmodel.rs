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
  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
  fn _ZNK19QItemSelectionModel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QItemSelection QItemSelectionModel::selection();
  fn _ZNK19QItemSelectionModel9selectionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
  fn _ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
  fn _ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QItemSelectionModel::clearSelection();
  fn _ZN19QItemSelectionModel14clearSelectionEv(qthis: *mut c_void) ;
  // proto:  QModelIndex QItemSelectionModel::currentIndex();
  fn _ZNK19QItemSelectionModel12currentIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QItemSelectionModel::NewQItemSelectionModel(const QItemSelectionModel & );
  fn _ZN19QItemSelectionModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> int8_t;
  // proto:  void QItemSelectionModel::reset();
  fn _ZN19QItemSelectionModel5resetEv(qthis: *mut c_void) ;
  // proto:  void QItemSelectionModel::clearCurrentIndex();
  fn _ZN19QItemSelectionModel17clearCurrentIndexEv(qthis: *mut c_void) ;
  // proto:  QList<QModelIndex> QItemSelectionModel::selectedIndexes();
  fn _ZNK19QItemSelectionModel15selectedIndexesEv(qthis: *mut c_void) ;
  // proto:  QList<QModelIndex> QItemSelectionModel::selectedColumns(int row);
  fn _ZNK19QItemSelectionModel15selectedColumnsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> int8_t;
  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> int8_t;
  // proto:  void QItemSelectionModel::FreeQItemSelectionModel();
  fn _ZN19QItemSelectionModelD0Ev(qthis: *mut c_void) ;
  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> int8_t;
  // proto:  void QItemSelectionModel::clear();
  fn _ZN19QItemSelectionModel5clearEv(qthis: *mut c_void) ;
  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QItemSelectionModel::hasSelection();
  fn _ZNK19QItemSelectionModel12hasSelectionEv(qthis: *mut c_void) -> int8_t;
  // proto:  QAbstractItemModel * QItemSelectionModel::model();
  fn _ZN19QItemSelectionModel5modelEv(qthis: *mut c_void) ;
  // proto:  QList<QModelIndex> QItemSelectionModel::selectedRows(int column);
  fn _ZNK19QItemSelectionModel12selectedRowsEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QItemSelectionModel)=1
pub struct QItemSelectionModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemSelectionModel {
  pub fn currentColumnChanged<T: QItemSelectionModel_currentColumnChanged>(&mut self, value: T)  {
     value.currentColumnChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentColumnChanged {
  fn currentColumnChanged(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentColumnChanged for (&'a  QModelIndex, &'a  QModelIndex) {
  fn currentColumnChanged(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn metaObject<T: QItemSelectionModel_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_metaObject {
  fn metaObject(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  const QMetaObject * QItemSelectionModel::metaObject();
impl<'a> /*trait*/ QItemSelectionModel_metaObject for () {
  fn metaObject(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10metaObjectEv()};
     unsafe {_ZNK19QItemSelectionModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn currentChanged<T: QItemSelectionModel_currentChanged>(&mut self, value: T)  {
     value.currentChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentChanged {
  fn currentChanged(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentChanged for (&'a  QModelIndex, &'a  QModelIndex) {
  fn currentChanged(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selection<T: QItemSelectionModel_selection>(&mut self, value: T) -> QItemSelection {
    return value.selection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selection {
  fn selection(self, rsthis: &mut QItemSelectionModel) -> QItemSelection;
}

// proto:  const QItemSelection QItemSelectionModel::selection();
impl<'a> /*trait*/ QItemSelectionModel_selection for () {
  fn selection(self, rsthis: &mut QItemSelectionModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel9selectionEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel9selectionEv(rsthis.qclsinst)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn isSelected<T: QItemSelectionModel_isSelected>(&mut self, value: T) -> i8 {
    return value.isSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isSelected {
  fn isSelected(self, rsthis: &mut QItemSelectionModel) -> i8;
}

// proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionModel_isSelected for (&'a  QModelIndex) {
  fn isSelected(self, rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selectionChanged<T: QItemSelectionModel_selectionChanged>(&mut self, value: T)  {
     value.selectionChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectionChanged {
  fn selectionChanged(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
impl<'a> /*trait*/ QItemSelectionModel_selectionChanged for (&'a  QItemSelection, &'a  QItemSelection) {
  fn selectionChanged(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn clearSelection<T: QItemSelectionModel_clearSelection>(&mut self, value: T)  {
     value.clearSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clearSelection {
  fn clearSelection(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::clearSelection();
impl<'a> /*trait*/ QItemSelectionModel_clearSelection for () {
  fn clearSelection(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14clearSelectionEv()};
     unsafe {_ZN19QItemSelectionModel14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn currentIndex<T: QItemSelectionModel_currentIndex>(&mut self, value: T) -> QModelIndex {
    return value.currentIndex(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentIndex {
  fn currentIndex(self, rsthis: &mut QItemSelectionModel) -> QModelIndex;
}

// proto:  QModelIndex QItemSelectionModel::currentIndex();
impl<'a> /*trait*/ QItemSelectionModel_currentIndex for () {
  fn currentIndex(self, rsthis: &mut QItemSelectionModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12currentIndexEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel12currentIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QItemSelectionModelC1ERKS_(qthis, arg0)};
    let rsthis = QItemSelectionModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn rowIntersectsSelection<T: QItemSelectionModel_rowIntersectsSelection>(&mut self, value: T) -> i8 {
    return value.rowIntersectsSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_rowIntersectsSelection {
  fn rowIntersectsSelection(self, rsthis: &mut QItemSelectionModel) -> i8;
}

// proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_rowIntersectsSelection for (i32, &'a  QModelIndex) {
  fn rowIntersectsSelection(self, rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn reset<T: QItemSelectionModel_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_reset {
  fn reset(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::reset();
impl<'a> /*trait*/ QItemSelectionModel_reset for () {
  fn reset(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5resetEv()};
     unsafe {_ZN19QItemSelectionModel5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn clearCurrentIndex<T: QItemSelectionModel_clearCurrentIndex>(&mut self, value: T)  {
     value.clearCurrentIndex(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clearCurrentIndex {
  fn clearCurrentIndex(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::clearCurrentIndex();
impl<'a> /*trait*/ QItemSelectionModel_clearCurrentIndex for () {
  fn clearCurrentIndex(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17clearCurrentIndexEv()};
     unsafe {_ZN19QItemSelectionModel17clearCurrentIndexEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selectedIndexes<T: QItemSelectionModel_selectedIndexes>(&mut self, value: T)  {
     value.selectedIndexes(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedIndexes {
  fn selectedIndexes(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  QList<QModelIndex> QItemSelectionModel::selectedIndexes();
impl<'a> /*trait*/ QItemSelectionModel_selectedIndexes for () {
  fn selectedIndexes(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedIndexesEv()};
     unsafe {_ZNK19QItemSelectionModel15selectedIndexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selectedColumns<T: QItemSelectionModel_selectedColumns>(&mut self, value: T)  {
     value.selectedColumns(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedColumns {
  fn selectedColumns(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  QList<QModelIndex> QItemSelectionModel::selectedColumns(int row);
impl<'a> /*trait*/ QItemSelectionModel_selectedColumns for (i32) {
  fn selectedColumns(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QItemSelectionModel15selectedColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn isColumnSelected<T: QItemSelectionModel_isColumnSelected>(&mut self, value: T) -> i8 {
    return value.isColumnSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isColumnSelected {
  fn isColumnSelected(self, rsthis: &mut QItemSelectionModel) -> i8;
}

// proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isColumnSelected for (i32, &'a  QModelIndex) {
  fn isColumnSelected(self, rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn columnIntersectsSelection<T: QItemSelectionModel_columnIntersectsSelection>(&mut self, value: T) -> i8 {
    return value.columnIntersectsSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_columnIntersectsSelection {
  fn columnIntersectsSelection(self, rsthis: &mut QItemSelectionModel) -> i8;
}

// proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_columnIntersectsSelection for (i32, &'a  QModelIndex) {
  fn columnIntersectsSelection(self, rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn FreeQItemSelectionModel<T: QItemSelectionModel_FreeQItemSelectionModel>(&mut self, value: T)  {
     value.FreeQItemSelectionModel(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_FreeQItemSelectionModel {
  fn FreeQItemSelectionModel(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::FreeQItemSelectionModel();
impl<'a> /*trait*/ QItemSelectionModel_FreeQItemSelectionModel for () {
  fn FreeQItemSelectionModel(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelD0Ev()};
     unsafe {_ZN19QItemSelectionModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn isRowSelected<T: QItemSelectionModel_isRowSelected>(&mut self, value: T) -> i8 {
    return value.isRowSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isRowSelected {
  fn isRowSelected(self, rsthis: &mut QItemSelectionModel) -> i8;
}

// proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isRowSelected for (i32, &'a  QModelIndex) {
  fn isRowSelected(self, rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn clear<T: QItemSelectionModel_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clear {
  fn clear(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::clear();
impl<'a> /*trait*/ QItemSelectionModel_clear for () {
  fn clear(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5clearEv()};
     unsafe {_ZN19QItemSelectionModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn currentRowChanged<T: QItemSelectionModel_currentRowChanged>(&mut self, value: T)  {
     value.currentRowChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentRowChanged {
  fn currentRowChanged(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentRowChanged for (&'a  QModelIndex, &'a  QModelIndex) {
  fn currentRowChanged(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn hasSelection<T: QItemSelectionModel_hasSelection>(&mut self, value: T) -> i8 {
    return value.hasSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_hasSelection {
  fn hasSelection(self, rsthis: &mut QItemSelectionModel) -> i8;
}

// proto:  bool QItemSelectionModel::hasSelection();
impl<'a> /*trait*/ QItemSelectionModel_hasSelection for () {
  fn hasSelection(self, rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12hasSelectionEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel12hasSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn model<T: QItemSelectionModel_model>(&mut self, value: T)  {
     value.model(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_model {
  fn model(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  QAbstractItemModel * QItemSelectionModel::model();
impl<'a> /*trait*/ QItemSelectionModel_model for () {
  fn model(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5modelEv()};
     unsafe {_ZN19QItemSelectionModel5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemSelectionModel {
  pub fn selectedRows<T: QItemSelectionModel_selectedRows>(&mut self, value: T)  {
     value.selectedRows(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedRows {
  fn selectedRows(self, rsthis: &mut QItemSelectionModel) ;
}

// proto:  QList<QModelIndex> QItemSelectionModel::selectedRows(int column);
impl<'a> /*trait*/ QItemSelectionModel_selectedRows for (i32) {
  fn selectedRows(self, rsthis: &mut QItemSelectionModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12selectedRowsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QItemSelectionModel12selectedRowsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

