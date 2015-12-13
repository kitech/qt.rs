// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qheaderview::QHeaderView;
use super::qitemselectionmodel::QItemSelectionModel;
use super::qmodelindex::QModelIndex;
use super::qwidget::QWidget;
use super::qrect::QRect;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTableView::resizeRowsToContents();
  fn _ZN10QTableView20resizeRowsToContentsEv(qthis: *mut c_void) ;
  // proto:  void QTableView::setRowHeight(int row, int height);
  fn _ZN10QTableView12setRowHeightEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QHeaderView * QTableView::verticalHeader();
  fn _ZNK10QTableView14verticalHeaderEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableView::setSpan(int row, int column, int rowSpan, int columnSpan);
  fn _ZN10QTableView7setSpanEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QTableView::setSortingEnabled(bool enable);
  fn _ZN10QTableView17setSortingEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTableView::setColumnWidth(int column, int width);
  fn _ZN10QTableView14setColumnWidthEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableView::setWordWrap(bool on);
  fn _ZN10QTableView11setWordWrapEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTableView::doItemsLayout();
  fn _ZN10QTableView13doItemsLayoutEv(qthis: *mut c_void) ;
  // proto:  void QTableView::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN10QTableView17setSelectionModelEP19QItemSelectionModel(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableView::setHorizontalHeader(QHeaderView * header);
  fn _ZN10QTableView19setHorizontalHeaderEP11QHeaderView(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableView::setRowHidden(int row, bool hide);
  fn _ZN10QTableView12setRowHiddenEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  int QTableView::rowViewportPosition(int row);
  fn _ZNK10QTableView19rowViewportPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTableView::columnAt(int x);
  fn _ZNK10QTableView8columnAtEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QTableView::isRowHidden(int row);
  fn _ZNK10QTableView11isRowHiddenEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QTableView::showColumn(int column);
  fn _ZN10QTableView10showColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableView::resizeRowToContents(int row);
  fn _ZN10QTableView19resizeRowToContentsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableView::setRootIndex(const QModelIndex & index);
  fn _ZN10QTableView12setRootIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableView::setColumnHidden(int column, bool hide);
  fn _ZN10QTableView15setColumnHiddenEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QTableView::hideRow(int row);
  fn _ZN10QTableView7hideRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableView::resizeColumnsToContents();
  fn _ZN10QTableView23resizeColumnsToContentsEv(qthis: *mut c_void) ;
  // proto:  bool QTableView::wordWrap();
  fn _ZNK10QTableView8wordWrapEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTableView::setShowGrid(bool show);
  fn _ZN10QTableView11setShowGridEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QTableView::isColumnHidden(int column);
  fn _ZNK10QTableView14isColumnHiddenEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QTableView::selectRow(int row);
  fn _ZN10QTableView9selectRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QMetaObject * QTableView::metaObject();
  fn _ZNK10QTableView10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QTableView::isCornerButtonEnabled();
  fn _ZNK10QTableView21isCornerButtonEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTableView::selectColumn(int column);
  fn _ZN10QTableView12selectColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableView::FreeQTableView();
  fn _ZN10QTableViewD0Ev(qthis: *mut c_void) ;
  // proto:  void QTableView::resizeColumnToContents(int column);
  fn _ZN10QTableView22resizeColumnToContentsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableView::NewQTableView(QWidget * parent);
  fn _ZN10QTableViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableView::sortByColumn(int column);
  fn _ZN10QTableView12sortByColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTableView::columnSpan(int row, int column);
  fn _ZNK10QTableView10columnSpanEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  int QTableView::columnWidth(int column);
  fn _ZNK10QTableView11columnWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTableView::columnViewportPosition(int column);
  fn _ZNK10QTableView22columnViewportPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTableView::rowHeight(int row);
  fn _ZNK10QTableView9rowHeightEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTableView::NewQTableView(const QTableView & );
  fn _ZN10QTableViewC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTableView::rowAt(int y);
  fn _ZNK10QTableView5rowAtEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTableView::rowSpan(int row, int column);
  fn _ZNK10QTableView7rowSpanEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  void QTableView::setCornerButtonEnabled(bool enable);
  fn _ZN10QTableView22setCornerButtonEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QRect QTableView::visualRect(const QModelIndex & index);
  fn _ZNK10QTableView10visualRectERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QTableView::indexAt(const QPoint & p);
  fn _ZNK10QTableView7indexAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QHeaderView * QTableView::horizontalHeader();
  fn _ZNK10QTableView16horizontalHeaderEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTableView::isSortingEnabled();
  fn _ZNK10QTableView16isSortingEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTableView::clearSpans();
  fn _ZN10QTableView10clearSpansEv(qthis: *mut c_void) ;
  // proto:  void QTableView::setVerticalHeader(QHeaderView * header);
  fn _ZN10QTableView17setVerticalHeaderEP11QHeaderView(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTableView::showGrid();
  fn _ZNK10QTableView8showGridEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTableView::showRow(int row);
  fn _ZN10QTableView7showRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableView::hideColumn(int column);
  fn _ZN10QTableView10hideColumnEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QTableView)=1
pub struct QTableView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTableView {
  pub fn resizeRowsToContents<T: QTableView_resizeRowsToContents>(&mut self, value: T)  {
     value.resizeRowsToContents(self);
    // return 1;
  }
}

pub trait QTableView_resizeRowsToContents {
  fn resizeRowsToContents(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::resizeRowsToContents();
impl<'a> /*trait*/ QTableView_resizeRowsToContents for () {
  fn resizeRowsToContents(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView20resizeRowsToContentsEv()};
     unsafe {_ZN10QTableView20resizeRowsToContentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setRowHeight<T: QTableView_setRowHeight>(&mut self, value: T)  {
     value.setRowHeight(self);
    // return 1;
  }
}

pub trait QTableView_setRowHeight {
  fn setRowHeight(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setRowHeight(int row, int height);
impl<'a> /*trait*/ QTableView_setRowHeight for (i32, i32) {
  fn setRowHeight(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12setRowHeightEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTableView12setRowHeightEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn verticalHeader<T: QTableView_verticalHeader>(&mut self, value: T) -> QHeaderView {
    return value.verticalHeader(self);
    // return 1;
  }
}

pub trait QTableView_verticalHeader {
  fn verticalHeader(self, rsthis: &mut QTableView) -> QHeaderView;
}

// proto:  QHeaderView * QTableView::verticalHeader();
impl<'a> /*trait*/ QTableView_verticalHeader for () {
  fn verticalHeader(self, rsthis: &mut QTableView) -> QHeaderView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView14verticalHeaderEv()};
    let mut ret = unsafe {_ZNK10QTableView14verticalHeaderEv(rsthis.qclsinst)};
    let mut ret1 = QHeaderView{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setSpan<T: QTableView_setSpan>(&mut self, value: T)  {
     value.setSpan(self);
    // return 1;
  }
}

pub trait QTableView_setSpan {
  fn setSpan(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setSpan(int row, int column, int rowSpan, int columnSpan);
impl<'a> /*trait*/ QTableView_setSpan for (i32, i32, i32, i32) {
  fn setSpan(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView7setSpanEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN10QTableView7setSpanEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setSortingEnabled<T: QTableView_setSortingEnabled>(&mut self, value: T)  {
     value.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableView_setSortingEnabled {
  fn setSortingEnabled(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QTableView_setSortingEnabled for (i8) {
  fn setSortingEnabled(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView17setSortingEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTableView17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setColumnWidth<T: QTableView_setColumnWidth>(&mut self, value: T)  {
     value.setColumnWidth(self);
    // return 1;
  }
}

pub trait QTableView_setColumnWidth {
  fn setColumnWidth(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setColumnWidth(int column, int width);
impl<'a> /*trait*/ QTableView_setColumnWidth for (i32, i32) {
  fn setColumnWidth(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView14setColumnWidthEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTableView14setColumnWidthEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setWordWrap<T: QTableView_setWordWrap>(&mut self, value: T)  {
     value.setWordWrap(self);
    // return 1;
  }
}

pub trait QTableView_setWordWrap {
  fn setWordWrap(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setWordWrap(bool on);
impl<'a> /*trait*/ QTableView_setWordWrap for (i8) {
  fn setWordWrap(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView11setWordWrapEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTableView11setWordWrapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn doItemsLayout<T: QTableView_doItemsLayout>(&mut self, value: T)  {
     value.doItemsLayout(self);
    // return 1;
  }
}

pub trait QTableView_doItemsLayout {
  fn doItemsLayout(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::doItemsLayout();
impl<'a> /*trait*/ QTableView_doItemsLayout for () {
  fn doItemsLayout(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView13doItemsLayoutEv()};
     unsafe {_ZN10QTableView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setSelectionModel<T: QTableView_setSelectionModel>(&mut self, value: T)  {
     value.setSelectionModel(self);
    // return 1;
  }
}

pub trait QTableView_setSelectionModel {
  fn setSelectionModel(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QTableView_setSelectionModel for (&'a mut QItemSelectionModel) {
  fn setSelectionModel(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTableView17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setHorizontalHeader<T: QTableView_setHorizontalHeader>(&mut self, value: T)  {
     value.setHorizontalHeader(self);
    // return 1;
  }
}

pub trait QTableView_setHorizontalHeader {
  fn setHorizontalHeader(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setHorizontalHeader(QHeaderView * header);
impl<'a> /*trait*/ QTableView_setHorizontalHeader for (&'a mut QHeaderView) {
  fn setHorizontalHeader(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView19setHorizontalHeaderEP11QHeaderView()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTableView19setHorizontalHeaderEP11QHeaderView(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setRowHidden<T: QTableView_setRowHidden>(&mut self, value: T)  {
     value.setRowHidden(self);
    // return 1;
  }
}

pub trait QTableView_setRowHidden {
  fn setRowHidden(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setRowHidden(int row, bool hide);
impl<'a> /*trait*/ QTableView_setRowHidden for (i32, i8) {
  fn setRowHidden(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12setRowHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN10QTableView12setRowHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn rowViewportPosition<T: QTableView_rowViewportPosition>(&mut self, value: T) -> i32 {
    return value.rowViewportPosition(self);
    // return 1;
  }
}

pub trait QTableView_rowViewportPosition {
  fn rowViewportPosition(self, rsthis: &mut QTableView) -> i32;
}

// proto:  int QTableView::rowViewportPosition(int row);
impl<'a> /*trait*/ QTableView_rowViewportPosition for (i32) {
  fn rowViewportPosition(self, rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView19rowViewportPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView19rowViewportPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn columnAt<T: QTableView_columnAt>(&mut self, value: T) -> i32 {
    return value.columnAt(self);
    // return 1;
  }
}

pub trait QTableView_columnAt {
  fn columnAt(self, rsthis: &mut QTableView) -> i32;
}

// proto:  int QTableView::columnAt(int x);
impl<'a> /*trait*/ QTableView_columnAt for (i32) {
  fn columnAt(self, rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView8columnAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView8columnAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn isRowHidden<T: QTableView_isRowHidden>(&mut self, value: T) -> i8 {
    return value.isRowHidden(self);
    // return 1;
  }
}

pub trait QTableView_isRowHidden {
  fn isRowHidden(self, rsthis: &mut QTableView) -> i8;
}

// proto:  bool QTableView::isRowHidden(int row);
impl<'a> /*trait*/ QTableView_isRowHidden for (i32) {
  fn isRowHidden(self, rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView11isRowHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView11isRowHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn showColumn<T: QTableView_showColumn>(&mut self, value: T)  {
     value.showColumn(self);
    // return 1;
  }
}

pub trait QTableView_showColumn {
  fn showColumn(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::showColumn(int column);
impl<'a> /*trait*/ QTableView_showColumn for (i32) {
  fn showColumn(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView10showColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView10showColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn resizeRowToContents<T: QTableView_resizeRowToContents>(&mut self, value: T)  {
     value.resizeRowToContents(self);
    // return 1;
  }
}

pub trait QTableView_resizeRowToContents {
  fn resizeRowToContents(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::resizeRowToContents(int row);
impl<'a> /*trait*/ QTableView_resizeRowToContents for (i32) {
  fn resizeRowToContents(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView19resizeRowToContentsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView19resizeRowToContentsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setRootIndex<T: QTableView_setRootIndex>(&mut self, value: T)  {
     value.setRootIndex(self);
    // return 1;
  }
}

pub trait QTableView_setRootIndex {
  fn setRootIndex(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QTableView_setRootIndex for (&'a  QModelIndex) {
  fn setRootIndex(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTableView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setColumnHidden<T: QTableView_setColumnHidden>(&mut self, value: T)  {
     value.setColumnHidden(self);
    // return 1;
  }
}

pub trait QTableView_setColumnHidden {
  fn setColumnHidden(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setColumnHidden(int column, bool hide);
impl<'a> /*trait*/ QTableView_setColumnHidden for (i32, i8) {
  fn setColumnHidden(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView15setColumnHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN10QTableView15setColumnHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn hideRow<T: QTableView_hideRow>(&mut self, value: T)  {
     value.hideRow(self);
    // return 1;
  }
}

pub trait QTableView_hideRow {
  fn hideRow(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::hideRow(int row);
impl<'a> /*trait*/ QTableView_hideRow for (i32) {
  fn hideRow(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView7hideRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView7hideRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn resizeColumnsToContents<T: QTableView_resizeColumnsToContents>(&mut self, value: T)  {
     value.resizeColumnsToContents(self);
    // return 1;
  }
}

pub trait QTableView_resizeColumnsToContents {
  fn resizeColumnsToContents(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::resizeColumnsToContents();
impl<'a> /*trait*/ QTableView_resizeColumnsToContents for () {
  fn resizeColumnsToContents(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView23resizeColumnsToContentsEv()};
     unsafe {_ZN10QTableView23resizeColumnsToContentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn wordWrap<T: QTableView_wordWrap>(&mut self, value: T) -> i8 {
    return value.wordWrap(self);
    // return 1;
  }
}

pub trait QTableView_wordWrap {
  fn wordWrap(self, rsthis: &mut QTableView) -> i8;
}

// proto:  bool QTableView::wordWrap();
impl<'a> /*trait*/ QTableView_wordWrap for () {
  fn wordWrap(self, rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView8wordWrapEv()};
    let mut ret = unsafe {_ZNK10QTableView8wordWrapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setShowGrid<T: QTableView_setShowGrid>(&mut self, value: T)  {
     value.setShowGrid(self);
    // return 1;
  }
}

pub trait QTableView_setShowGrid {
  fn setShowGrid(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setShowGrid(bool show);
impl<'a> /*trait*/ QTableView_setShowGrid for (i8) {
  fn setShowGrid(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView11setShowGridEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTableView11setShowGridEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn isColumnHidden<T: QTableView_isColumnHidden>(&mut self, value: T) -> i8 {
    return value.isColumnHidden(self);
    // return 1;
  }
}

pub trait QTableView_isColumnHidden {
  fn isColumnHidden(self, rsthis: &mut QTableView) -> i8;
}

// proto:  bool QTableView::isColumnHidden(int column);
impl<'a> /*trait*/ QTableView_isColumnHidden for (i32) {
  fn isColumnHidden(self, rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView14isColumnHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView14isColumnHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn selectRow<T: QTableView_selectRow>(&mut self, value: T)  {
     value.selectRow(self);
    // return 1;
  }
}

pub trait QTableView_selectRow {
  fn selectRow(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::selectRow(int row);
impl<'a> /*trait*/ QTableView_selectRow for (i32) {
  fn selectRow(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView9selectRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView9selectRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn metaObject<T: QTableView_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTableView_metaObject {
  fn metaObject(self, rsthis: &mut QTableView) ;
}

// proto:  const QMetaObject * QTableView::metaObject();
impl<'a> /*trait*/ QTableView_metaObject for () {
  fn metaObject(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView10metaObjectEv()};
     unsafe {_ZNK10QTableView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn isCornerButtonEnabled<T: QTableView_isCornerButtonEnabled>(&mut self, value: T) -> i8 {
    return value.isCornerButtonEnabled(self);
    // return 1;
  }
}

pub trait QTableView_isCornerButtonEnabled {
  fn isCornerButtonEnabled(self, rsthis: &mut QTableView) -> i8;
}

// proto:  bool QTableView::isCornerButtonEnabled();
impl<'a> /*trait*/ QTableView_isCornerButtonEnabled for () {
  fn isCornerButtonEnabled(self, rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView21isCornerButtonEnabledEv()};
    let mut ret = unsafe {_ZNK10QTableView21isCornerButtonEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn selectColumn<T: QTableView_selectColumn>(&mut self, value: T)  {
     value.selectColumn(self);
    // return 1;
  }
}

pub trait QTableView_selectColumn {
  fn selectColumn(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::selectColumn(int column);
impl<'a> /*trait*/ QTableView_selectColumn for (i32) {
  fn selectColumn(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12selectColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView12selectColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn FreeQTableView<T: QTableView_FreeQTableView>(&mut self, value: T)  {
     value.FreeQTableView(self);
    // return 1;
  }
}

pub trait QTableView_FreeQTableView {
  fn FreeQTableView(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::FreeQTableView();
impl<'a> /*trait*/ QTableView_FreeQTableView for () {
  fn FreeQTableView(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableViewD0Ev()};
     unsafe {_ZN10QTableViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn resizeColumnToContents<T: QTableView_resizeColumnToContents>(&mut self, value: T)  {
     value.resizeColumnToContents(self);
    // return 1;
  }
}

pub trait QTableView_resizeColumnToContents {
  fn resizeColumnToContents(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::resizeColumnToContents(int column);
impl<'a> /*trait*/ QTableView_resizeColumnToContents for (i32) {
  fn resizeColumnToContents(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView22resizeColumnToContentsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView22resizeColumnToContentsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn NewQTableView<T: QTableView_NewQTableView>(value: T) -> QTableView {
    let rsthis = value.NewQTableView();
    return rsthis;
    // return 1;
  }
}

pub trait QTableView_NewQTableView {
  fn NewQTableView(self) -> QTableView;
}

// proto: void QTableView::NewQTableView(QWidget * parent);
impl<'a> /*trait*/ QTableView_NewQTableView for (&'a mut QWidget) {
  fn NewQTableView(self) -> QTableView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableViewC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTableViewC1EP7QWidget(qthis, arg0)};
    let rsthis = QTableView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn sortByColumn<T: QTableView_sortByColumn>(&mut self, value: T)  {
     value.sortByColumn(self);
    // return 1;
  }
}

pub trait QTableView_sortByColumn {
  fn sortByColumn(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::sortByColumn(int column);
impl<'a> /*trait*/ QTableView_sortByColumn for (i32) {
  fn sortByColumn(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12sortByColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView12sortByColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn columnSpan<T: QTableView_columnSpan>(&mut self, value: T) -> i32 {
    return value.columnSpan(self);
    // return 1;
  }
}

pub trait QTableView_columnSpan {
  fn columnSpan(self, rsthis: &mut QTableView) -> i32;
}

// proto:  int QTableView::columnSpan(int row, int column);
impl<'a> /*trait*/ QTableView_columnSpan for (i32, i32) {
  fn columnSpan(self, rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView10columnSpanEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QTableView10columnSpanEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn columnWidth<T: QTableView_columnWidth>(&mut self, value: T) -> i32 {
    return value.columnWidth(self);
    // return 1;
  }
}

pub trait QTableView_columnWidth {
  fn columnWidth(self, rsthis: &mut QTableView) -> i32;
}

// proto:  int QTableView::columnWidth(int column);
impl<'a> /*trait*/ QTableView_columnWidth for (i32) {
  fn columnWidth(self, rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView11columnWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView11columnWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn columnViewportPosition<T: QTableView_columnViewportPosition>(&mut self, value: T) -> i32 {
    return value.columnViewportPosition(self);
    // return 1;
  }
}

pub trait QTableView_columnViewportPosition {
  fn columnViewportPosition(self, rsthis: &mut QTableView) -> i32;
}

// proto:  int QTableView::columnViewportPosition(int column);
impl<'a> /*trait*/ QTableView_columnViewportPosition for (i32) {
  fn columnViewportPosition(self, rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView22columnViewportPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView22columnViewportPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn rowHeight<T: QTableView_rowHeight>(&mut self, value: T) -> i32 {
    return value.rowHeight(self);
    // return 1;
  }
}

pub trait QTableView_rowHeight {
  fn rowHeight(self, rsthis: &mut QTableView) -> i32;
}

// proto:  int QTableView::rowHeight(int row);
impl<'a> /*trait*/ QTableView_rowHeight for (i32) {
  fn rowHeight(self, rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView9rowHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView9rowHeightEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QTableView::NewQTableView(const QTableView & );
impl<'a> /*trait*/ QTableView_NewQTableView for (&'a  QTableView) {
  fn NewQTableView(self) -> QTableView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableViewC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTableViewC1ERKS_(qthis, arg0)};
    let rsthis = QTableView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn rowAt<T: QTableView_rowAt>(&mut self, value: T) -> i32 {
    return value.rowAt(self);
    // return 1;
  }
}

pub trait QTableView_rowAt {
  fn rowAt(self, rsthis: &mut QTableView) -> i32;
}

// proto:  int QTableView::rowAt(int y);
impl<'a> /*trait*/ QTableView_rowAt for (i32) {
  fn rowAt(self, rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView5rowAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView5rowAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn rowSpan<T: QTableView_rowSpan>(&mut self, value: T) -> i32 {
    return value.rowSpan(self);
    // return 1;
  }
}

pub trait QTableView_rowSpan {
  fn rowSpan(self, rsthis: &mut QTableView) -> i32;
}

// proto:  int QTableView::rowSpan(int row, int column);
impl<'a> /*trait*/ QTableView_rowSpan for (i32, i32) {
  fn rowSpan(self, rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView7rowSpanEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QTableView7rowSpanEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setCornerButtonEnabled<T: QTableView_setCornerButtonEnabled>(&mut self, value: T)  {
     value.setCornerButtonEnabled(self);
    // return 1;
  }
}

pub trait QTableView_setCornerButtonEnabled {
  fn setCornerButtonEnabled(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setCornerButtonEnabled(bool enable);
impl<'a> /*trait*/ QTableView_setCornerButtonEnabled for (i8) {
  fn setCornerButtonEnabled(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView22setCornerButtonEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTableView22setCornerButtonEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn visualRect<T: QTableView_visualRect>(&mut self, value: T) -> QRect {
    return value.visualRect(self);
    // return 1;
  }
}

pub trait QTableView_visualRect {
  fn visualRect(self, rsthis: &mut QTableView) -> QRect;
}

// proto:  QRect QTableView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QTableView_visualRect for (&'a  QModelIndex) {
  fn visualRect(self, rsthis: &mut QTableView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTableView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn indexAt<T: QTableView_indexAt>(&mut self, value: T) -> QModelIndex {
    return value.indexAt(self);
    // return 1;
  }
}

pub trait QTableView_indexAt {
  fn indexAt(self, rsthis: &mut QTableView) -> QModelIndex;
}

// proto:  QModelIndex QTableView::indexAt(const QPoint & p);
impl<'a> /*trait*/ QTableView_indexAt for (&'a  QPoint) {
  fn indexAt(self, rsthis: &mut QTableView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTableView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn horizontalHeader<T: QTableView_horizontalHeader>(&mut self, value: T) -> QHeaderView {
    return value.horizontalHeader(self);
    // return 1;
  }
}

pub trait QTableView_horizontalHeader {
  fn horizontalHeader(self, rsthis: &mut QTableView) -> QHeaderView;
}

// proto:  QHeaderView * QTableView::horizontalHeader();
impl<'a> /*trait*/ QTableView_horizontalHeader for () {
  fn horizontalHeader(self, rsthis: &mut QTableView) -> QHeaderView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView16horizontalHeaderEv()};
    let mut ret = unsafe {_ZNK10QTableView16horizontalHeaderEv(rsthis.qclsinst)};
    let mut ret1 = QHeaderView{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn isSortingEnabled<T: QTableView_isSortingEnabled>(&mut self, value: T) -> i8 {
    return value.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableView_isSortingEnabled {
  fn isSortingEnabled(self, rsthis: &mut QTableView) -> i8;
}

// proto:  bool QTableView::isSortingEnabled();
impl<'a> /*trait*/ QTableView_isSortingEnabled for () {
  fn isSortingEnabled(self, rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView16isSortingEnabledEv()};
    let mut ret = unsafe {_ZNK10QTableView16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn clearSpans<T: QTableView_clearSpans>(&mut self, value: T)  {
     value.clearSpans(self);
    // return 1;
  }
}

pub trait QTableView_clearSpans {
  fn clearSpans(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::clearSpans();
impl<'a> /*trait*/ QTableView_clearSpans for () {
  fn clearSpans(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView10clearSpansEv()};
     unsafe {_ZN10QTableView10clearSpansEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn setVerticalHeader<T: QTableView_setVerticalHeader>(&mut self, value: T)  {
     value.setVerticalHeader(self);
    // return 1;
  }
}

pub trait QTableView_setVerticalHeader {
  fn setVerticalHeader(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::setVerticalHeader(QHeaderView * header);
impl<'a> /*trait*/ QTableView_setVerticalHeader for (&'a mut QHeaderView) {
  fn setVerticalHeader(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView17setVerticalHeaderEP11QHeaderView()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTableView17setVerticalHeaderEP11QHeaderView(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn showGrid<T: QTableView_showGrid>(&mut self, value: T) -> i8 {
    return value.showGrid(self);
    // return 1;
  }
}

pub trait QTableView_showGrid {
  fn showGrid(self, rsthis: &mut QTableView) -> i8;
}

// proto:  bool QTableView::showGrid();
impl<'a> /*trait*/ QTableView_showGrid for () {
  fn showGrid(self, rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView8showGridEv()};
    let mut ret = unsafe {_ZNK10QTableView8showGridEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn showRow<T: QTableView_showRow>(&mut self, value: T)  {
     value.showRow(self);
    // return 1;
  }
}

pub trait QTableView_showRow {
  fn showRow(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::showRow(int row);
impl<'a> /*trait*/ QTableView_showRow for (i32) {
  fn showRow(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView7showRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView7showRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableView {
  pub fn hideColumn<T: QTableView_hideColumn>(&mut self, value: T)  {
     value.hideColumn(self);
    // return 1;
  }
}

pub trait QTableView_hideColumn {
  fn hideColumn(self, rsthis: &mut QTableView) ;
}

// proto:  void QTableView::hideColumn(int column);
impl<'a> /*trait*/ QTableView_hideColumn for (i32) {
  fn hideColumn(self, rsthis: &mut QTableView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView10hideColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView10hideColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

