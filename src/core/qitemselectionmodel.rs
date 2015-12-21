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
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
  fn _ZNK19QItemSelectionModel10metaObjectEv(qthis: *mut c_void);
  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QItemSelection QItemSelectionModel::selection();
  fn _ZNK19QItemSelectionModel9selectionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
  fn _ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
  fn _ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QItemSelectionModel::clearSelection();
  fn _ZN19QItemSelectionModel14clearSelectionEv(qthis: *mut c_void);
  // proto:  QModelIndex QItemSelectionModel::currentIndex();
  fn _ZNK19QItemSelectionModel12currentIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
  fn _ZN19QItemSelectionModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::reset();
  fn _ZN19QItemSelectionModel5resetEv(qthis: *mut c_void);
  // proto:  void QItemSelectionModel::clearCurrentIndex();
  fn _ZN19QItemSelectionModel17clearCurrentIndexEv(qthis: *mut c_void);
  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
  fn _ZNK19QItemSelectionModel15selectedIndexesEv(qthis: *mut c_void);
  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
  fn _ZNK19QItemSelectionModel15selectedColumnsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::~QItemSelectionModel();
  fn _ZN19QItemSelectionModelD0Ev(qthis: *mut c_void);
  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
  fn _ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QItemSelectionModel::clear();
  fn _ZN19QItemSelectionModel5clearEv(qthis: *mut c_void);
  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
  fn _ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QItemSelectionModel::hasSelection();
  fn _ZNK19QItemSelectionModel12hasSelectionEv(qthis: *mut c_void) -> c_char;
  // proto:  QAbstractItemModel * QItemSelectionModel::model();
  fn _ZN19QItemSelectionModel5modelEv(qthis: *mut c_void);
  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
  fn _ZNK19QItemSelectionModel12selectedRowsEi(qthis: *mut c_void, arg0: c_int);
}

// body block begin
// class sizeof(QItemSelectionModel)=1
pub struct QItemSelectionModel {
  pub qclsinst: *mut c_void,
}

  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentColumnChanged<RetType, T: QItemSelectionModel_currentColumnChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentColumnChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentColumnChanged<RetType> {
  fn currentColumnChanged(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentColumnChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentColumnChanged<()> for (QModelIndex, QModelIndex) {
  fn currentColumnChanged(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel20currentColumnChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
impl /*struct*/ QItemSelectionModel {
  pub fn metaObject<RetType, T: QItemSelectionModel_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  const QMetaObject * QItemSelectionModel::metaObject();
impl<'a> /*trait*/ QItemSelectionModel_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10metaObjectEv()};
     unsafe {_ZNK19QItemSelectionModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentChanged<RetType, T: QItemSelectionModel_currentChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentChanged<RetType> {
  fn currentChanged(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentChanged<()> for (QModelIndex, QModelIndex) {
  fn currentChanged(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel14currentChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QItemSelection QItemSelectionModel::selection();
impl /*struct*/ QItemSelectionModel {
  pub fn selection<RetType, T: QItemSelectionModel_selection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selection<RetType> {
  fn selection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  const QItemSelection QItemSelectionModel::selection();
impl<'a> /*trait*/ QItemSelectionModel_selection<QItemSelection> for () {
  fn selection(self , rsthis: &mut QItemSelectionModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel9selectionEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel9selectionEv(rsthis.qclsinst)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
impl /*struct*/ QItemSelectionModel {
  pub fn isSelected<RetType, T: QItemSelectionModel_isSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isSelected<RetType> {
  fn isSelected(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isSelected(const QModelIndex & index);
impl<'a> /*trait*/ QItemSelectionModel_isSelected<i8> for (QModelIndex) {
  fn isSelected(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel10isSelectedERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
impl /*struct*/ QItemSelectionModel {
  pub fn selectionChanged<RetType, T: QItemSelectionModel_selectionChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::selectionChanged(const QItemSelection & selected, const QItemSelection & deselected);
impl<'a> /*trait*/ QItemSelectionModel_selectionChanged<()> for (QItemSelection, QItemSelection) {
  fn selectionChanged(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel16selectionChangedERK14QItemSelectionS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::clearSelection();
impl /*struct*/ QItemSelectionModel {
  pub fn clearSelection<RetType, T: QItemSelectionModel_clearSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clearSelection<RetType> {
  fn clearSelection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clearSelection();
impl<'a> /*trait*/ QItemSelectionModel_clearSelection<()> for () {
  fn clearSelection(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel14clearSelectionEv()};
     unsafe {_ZN19QItemSelectionModel14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QItemSelectionModel::currentIndex();
impl /*struct*/ QItemSelectionModel {
  pub fn currentIndex<RetType, T: QItemSelectionModel_currentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentIndex<RetType> {
  fn currentIndex(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndex QItemSelectionModel::currentIndex();
impl<'a> /*trait*/ QItemSelectionModel_currentIndex<QModelIndex> for () {
  fn currentIndex(self , rsthis: &mut QItemSelectionModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12currentIndexEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel12currentIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
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

  // proto:  void QItemSelectionModel::QItemSelectionModel(const QItemSelectionModel & );
impl<'a> /*trait*/ QItemSelectionModel_NewQItemSelectionModel for (QItemSelectionModel) {
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

  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn rowIntersectsSelection<RetType, T: QItemSelectionModel_rowIntersectsSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowIntersectsSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_rowIntersectsSelection<RetType> {
  fn rowIntersectsSelection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::rowIntersectsSelection(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_rowIntersectsSelection<i8> for (i32, QModelIndex) {
  fn rowIntersectsSelection(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel22rowIntersectsSelectionEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::reset();
impl /*struct*/ QItemSelectionModel {
  pub fn reset<RetType, T: QItemSelectionModel_reset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_reset<RetType> {
  fn reset(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::reset();
impl<'a> /*trait*/ QItemSelectionModel_reset<()> for () {
  fn reset(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5resetEv()};
     unsafe {_ZN19QItemSelectionModel5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::clearCurrentIndex();
impl /*struct*/ QItemSelectionModel {
  pub fn clearCurrentIndex<RetType, T: QItemSelectionModel_clearCurrentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearCurrentIndex(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clearCurrentIndex<RetType> {
  fn clearCurrentIndex(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clearCurrentIndex();
impl<'a> /*trait*/ QItemSelectionModel_clearCurrentIndex<()> for () {
  fn clearCurrentIndex(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17clearCurrentIndexEv()};
     unsafe {_ZN19QItemSelectionModel17clearCurrentIndexEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
impl /*struct*/ QItemSelectionModel {
  pub fn selectedIndexes<RetType, T: QItemSelectionModel_selectedIndexes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedIndexes(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedIndexes<RetType> {
  fn selectedIndexes(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedIndexes();
impl<'a> /*trait*/ QItemSelectionModel_selectedIndexes<()> for () {
  fn selectedIndexes(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedIndexesEv()};
     unsafe {_ZNK19QItemSelectionModel15selectedIndexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
impl /*struct*/ QItemSelectionModel {
  pub fn selectedColumns<RetType, T: QItemSelectionModel_selectedColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedColumns(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedColumns<RetType> {
  fn selectedColumns(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedColumns(int row);
impl<'a> /*trait*/ QItemSelectionModel_selectedColumns<()> for (i32) {
  fn selectedColumns(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel15selectedColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QItemSelectionModel15selectedColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn isColumnSelected<RetType, T: QItemSelectionModel_isColumnSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isColumnSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isColumnSelected<RetType> {
  fn isColumnSelected(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isColumnSelected(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isColumnSelected<i8> for (i32, QModelIndex) {
  fn isColumnSelected(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel16isColumnSelectedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn columnIntersectsSelection<RetType, T: QItemSelectionModel_columnIntersectsSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnIntersectsSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_columnIntersectsSelection<RetType> {
  fn columnIntersectsSelection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::columnIntersectsSelection(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_columnIntersectsSelection<i8> for (i32, QModelIndex) {
  fn columnIntersectsSelection(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel25columnIntersectsSelectionEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::~QItemSelectionModel();
impl /*struct*/ QItemSelectionModel {
  pub fn FreeQItemSelectionModel<RetType, T: QItemSelectionModel_FreeQItemSelectionModel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQItemSelectionModel(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_FreeQItemSelectionModel<RetType> {
  fn FreeQItemSelectionModel(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::~QItemSelectionModel();
impl<'a> /*trait*/ QItemSelectionModel_FreeQItemSelectionModel<()> for () {
  fn FreeQItemSelectionModel(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModelD0Ev()};
     unsafe {_ZN19QItemSelectionModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
impl /*struct*/ QItemSelectionModel {
  pub fn isRowSelected<RetType, T: QItemSelectionModel_isRowSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRowSelected(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_isRowSelected<RetType> {
  fn isRowSelected(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::isRowSelected(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QItemSelectionModel_isRowSelected<i8> for (i32, QModelIndex) {
  fn isRowSelected(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QItemSelectionModel13isRowSelectedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::clear();
impl /*struct*/ QItemSelectionModel {
  pub fn clear<RetType, T: QItemSelectionModel_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_clear<RetType> {
  fn clear(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::clear();
impl<'a> /*trait*/ QItemSelectionModel_clear<()> for () {
  fn clear(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5clearEv()};
     unsafe {_ZN19QItemSelectionModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
impl /*struct*/ QItemSelectionModel {
  pub fn currentRowChanged<RetType, T: QItemSelectionModel_currentRowChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentRowChanged(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_currentRowChanged<RetType> {
  fn currentRowChanged(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  void QItemSelectionModel::currentRowChanged(const QModelIndex & current, const QModelIndex & previous);
impl<'a> /*trait*/ QItemSelectionModel_currentRowChanged<()> for (QModelIndex, QModelIndex) {
  fn currentRowChanged(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN19QItemSelectionModel17currentRowChangedERK11QModelIndexS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QItemSelectionModel::hasSelection();
impl /*struct*/ QItemSelectionModel {
  pub fn hasSelection<RetType, T: QItemSelectionModel_hasSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasSelection(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_hasSelection<RetType> {
  fn hasSelection(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  bool QItemSelectionModel::hasSelection();
impl<'a> /*trait*/ QItemSelectionModel_hasSelection<i8> for () {
  fn hasSelection(self , rsthis: &mut QItemSelectionModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12hasSelectionEv()};
    let mut ret = unsafe {_ZNK19QItemSelectionModel12hasSelectionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QItemSelectionModel::model();
impl /*struct*/ QItemSelectionModel {
  pub fn model<RetType, T: QItemSelectionModel_model<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_model<RetType> {
  fn model(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QAbstractItemModel * QItemSelectionModel::model();
impl<'a> /*trait*/ QItemSelectionModel_model<()> for () {
  fn model(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QItemSelectionModel5modelEv()};
     unsafe {_ZN19QItemSelectionModel5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
impl /*struct*/ QItemSelectionModel {
  pub fn selectedRows<RetType, T: QItemSelectionModel_selectedRows<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedRows(self);
    // return 1;
  }
}

pub trait QItemSelectionModel_selectedRows<RetType> {
  fn selectedRows(self , rsthis: &mut QItemSelectionModel) -> RetType;
}

  // proto:  QModelIndexList QItemSelectionModel::selectedRows(int column);
impl<'a> /*trait*/ QItemSelectionModel_selectedRows<()> for (i32) {
  fn selectedRows(self , rsthis: &mut QItemSelectionModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QItemSelectionModel12selectedRowsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QItemSelectionModel12selectedRowsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

