// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qtableview.h
// dst-file: /src/widgets/qtableview.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qabstractitemview::QAbstractItemView; // 773
use std::ops::Deref;
use super::qheaderview::QHeaderView; // 773
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::core::qitemselectionmodel::QItemSelectionModel; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QTableView::resizeRowsToContents();
  fn _ZN10QTableView20resizeRowsToContentsEv(qthis: *mut c_void);
  // proto:  void QTableView::setRowHeight(int row, int height);
  fn _ZN10QTableView12setRowHeightEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  QHeaderView * QTableView::verticalHeader();
  fn _ZNK10QTableView14verticalHeaderEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableView::setSpan(int row, int column, int rowSpan, int columnSpan);
  fn _ZN10QTableView7setSpanEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QTableView::setSortingEnabled(bool enable);
  fn _ZN10QTableView17setSortingEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTableView::setColumnWidth(int column, int width);
  fn _ZN10QTableView14setColumnWidthEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QTableView::setWordWrap(bool on);
  fn _ZN10QTableView11setWordWrapEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTableView::doItemsLayout();
  fn _ZN10QTableView13doItemsLayoutEv(qthis: *mut c_void);
  // proto:  void QTableView::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN10QTableView17setSelectionModelEP19QItemSelectionModel(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTableView::setHorizontalHeader(QHeaderView * header);
  fn _ZN10QTableView19setHorizontalHeaderEP11QHeaderView(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTableView::setRowHidden(int row, bool hide);
  fn _ZN10QTableView12setRowHiddenEib(qthis: *mut c_void, arg0: c_int, arg1: c_char);
  // proto:  int QTableView::rowViewportPosition(int row);
  fn _ZNK10QTableView19rowViewportPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTableView::columnAt(int x);
  fn _ZNK10QTableView8columnAtEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QTableView::isRowHidden(int row);
  fn _ZNK10QTableView11isRowHiddenEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QTableView::showColumn(int column);
  fn _ZN10QTableView10showColumnEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTableView::resizeRowToContents(int row);
  fn _ZN10QTableView19resizeRowToContentsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTableView::setRootIndex(const QModelIndex & index);
  fn _ZN10QTableView12setRootIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTableView::setColumnHidden(int column, bool hide);
  fn _ZN10QTableView15setColumnHiddenEib(qthis: *mut c_void, arg0: c_int, arg1: c_char);
  // proto:  void QTableView::hideRow(int row);
  fn _ZN10QTableView7hideRowEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTableView::resizeColumnsToContents();
  fn _ZN10QTableView23resizeColumnsToContentsEv(qthis: *mut c_void);
  // proto:  bool QTableView::wordWrap();
  fn _ZNK10QTableView8wordWrapEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTableView::setShowGrid(bool show);
  fn _ZN10QTableView11setShowGridEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QTableView::isColumnHidden(int column);
  fn _ZNK10QTableView14isColumnHiddenEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QTableView::selectRow(int row);
  fn _ZN10QTableView9selectRowEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const QMetaObject * QTableView::metaObject();
  fn _ZNK10QTableView10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QTableView::isCornerButtonEnabled();
  fn _ZNK10QTableView21isCornerButtonEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTableView::selectColumn(int column);
  fn _ZN10QTableView12selectColumnEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTableView::~QTableView();
  fn _ZN10QTableViewD0Ev(qthis: *mut c_void);
  // proto:  void QTableView::resizeColumnToContents(int column);
  fn _ZN10QTableView22resizeColumnToContentsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTableView::QTableView(QWidget * parent);
  fn _ZN10QTableViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTableView::sortByColumn(int column);
  fn _ZN10QTableView12sortByColumnEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QTableView::columnSpan(int row, int column);
  fn _ZNK10QTableView10columnSpanEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  int QTableView::columnWidth(int column);
  fn _ZNK10QTableView11columnWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTableView::columnViewportPosition(int column);
  fn _ZNK10QTableView22columnViewportPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTableView::rowHeight(int row);
  fn _ZNK10QTableView9rowHeightEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTableView::QTableView(const QTableView & );
  fn _ZN10QTableViewC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTableView::rowAt(int y);
  fn _ZNK10QTableView5rowAtEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTableView::rowSpan(int row, int column);
  fn _ZNK10QTableView7rowSpanEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  void QTableView::setCornerButtonEnabled(bool enable);
  fn _ZN10QTableView22setCornerButtonEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QRect QTableView::visualRect(const QModelIndex & index);
  fn _ZNK10QTableView10visualRectERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QTableView::indexAt(const QPoint & p);
  fn _ZNK10QTableView7indexAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QHeaderView * QTableView::horizontalHeader();
  fn _ZNK10QTableView16horizontalHeaderEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTableView::isSortingEnabled();
  fn _ZNK10QTableView16isSortingEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTableView::clearSpans();
  fn _ZN10QTableView10clearSpansEv(qthis: *mut c_void);
  // proto:  void QTableView::setVerticalHeader(QHeaderView * header);
  fn _ZN10QTableView17setVerticalHeaderEP11QHeaderView(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTableView::showGrid();
  fn _ZNK10QTableView8showGridEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTableView::showRow(int row);
  fn _ZN10QTableView7showRowEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTableView::hideColumn(int column);
  fn _ZN10QTableView10hideColumnEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QTableView)=1
pub struct QTableView {
  qbase: QAbstractItemView,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTableView {
  pub fn inheritFrom(qthis: *mut c_void) -> QTableView {
    return QTableView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTableView {
  type Target = QAbstractItemView;

  fn deref(&self) -> &QAbstractItemView {
    return &self.qbase;
  }
}
impl AsRef<QAbstractItemView> for QTableView {
  fn as_ref(&self) -> &QAbstractItemView {
    return &self.qbase;
  }
}
  // proto:  void QTableView::resizeRowsToContents();
impl /*struct*/ QTableView {
  pub fn resizeRowsToContents<RetType, T: QTableView_resizeRowsToContents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resizeRowsToContents(self);
    // return 1;
  }
}

pub trait QTableView_resizeRowsToContents<RetType> {
  fn resizeRowsToContents(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::resizeRowsToContents();
impl<'a> /*trait*/ QTableView_resizeRowsToContents<()> for () {
  fn resizeRowsToContents(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView20resizeRowsToContentsEv()};
     unsafe {_ZN10QTableView20resizeRowsToContentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTableView::setRowHeight(int row, int height);
impl /*struct*/ QTableView {
  pub fn setRowHeight<RetType, T: QTableView_setRowHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRowHeight(self);
    // return 1;
  }
}

pub trait QTableView_setRowHeight<RetType> {
  fn setRowHeight(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setRowHeight(int row, int height);
impl<'a> /*trait*/ QTableView_setRowHeight<()> for (i32, i32) {
  fn setRowHeight(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12setRowHeightEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTableView12setRowHeightEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QHeaderView * QTableView::verticalHeader();
impl /*struct*/ QTableView {
  pub fn verticalHeader<RetType, T: QTableView_verticalHeader<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.verticalHeader(self);
    // return 1;
  }
}

pub trait QTableView_verticalHeader<RetType> {
  fn verticalHeader(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  QHeaderView * QTableView::verticalHeader();
impl<'a> /*trait*/ QTableView_verticalHeader<QHeaderView> for () {
  fn verticalHeader(self , rsthis: &mut QTableView) -> QHeaderView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView14verticalHeaderEv()};
    let mut ret = unsafe {_ZNK10QTableView14verticalHeaderEv(rsthis.qclsinst)};
    let mut ret1 = QHeaderView::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableView::setSpan(int row, int column, int rowSpan, int columnSpan);
impl /*struct*/ QTableView {
  pub fn setSpan<RetType, T: QTableView_setSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSpan(self);
    // return 1;
  }
}

pub trait QTableView_setSpan<RetType> {
  fn setSpan(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setSpan(int row, int column, int rowSpan, int columnSpan);
impl<'a> /*trait*/ QTableView_setSpan<()> for (i32, i32, i32, i32) {
  fn setSpan(self , rsthis: &mut QTableView) -> () {
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

  // proto:  void QTableView::setSortingEnabled(bool enable);
impl /*struct*/ QTableView {
  pub fn setSortingEnabled<RetType, T: QTableView_setSortingEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableView_setSortingEnabled<RetType> {
  fn setSortingEnabled(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QTableView_setSortingEnabled<()> for (i8) {
  fn setSortingEnabled(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView17setSortingEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTableView17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::setColumnWidth(int column, int width);
impl /*struct*/ QTableView {
  pub fn setColumnWidth<RetType, T: QTableView_setColumnWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColumnWidth(self);
    // return 1;
  }
}

pub trait QTableView_setColumnWidth<RetType> {
  fn setColumnWidth(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setColumnWidth(int column, int width);
impl<'a> /*trait*/ QTableView_setColumnWidth<()> for (i32, i32) {
  fn setColumnWidth(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView14setColumnWidthEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QTableView14setColumnWidthEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTableView::setWordWrap(bool on);
impl /*struct*/ QTableView {
  pub fn setWordWrap<RetType, T: QTableView_setWordWrap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWordWrap(self);
    // return 1;
  }
}

pub trait QTableView_setWordWrap<RetType> {
  fn setWordWrap(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setWordWrap(bool on);
impl<'a> /*trait*/ QTableView_setWordWrap<()> for (i8) {
  fn setWordWrap(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView11setWordWrapEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTableView11setWordWrapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::doItemsLayout();
impl /*struct*/ QTableView {
  pub fn doItemsLayout<RetType, T: QTableView_doItemsLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout(self);
    // return 1;
  }
}

pub trait QTableView_doItemsLayout<RetType> {
  fn doItemsLayout(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::doItemsLayout();
impl<'a> /*trait*/ QTableView_doItemsLayout<()> for () {
  fn doItemsLayout(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView13doItemsLayoutEv()};
     unsafe {_ZN10QTableView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTableView::setSelectionModel(QItemSelectionModel * selectionModel);
impl /*struct*/ QTableView {
  pub fn setSelectionModel<RetType, T: QTableView_setSelectionModel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel(self);
    // return 1;
  }
}

pub trait QTableView_setSelectionModel<RetType> {
  fn setSelectionModel(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QTableView_setSelectionModel<()> for (QItemSelectionModel) {
  fn setSelectionModel(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTableView17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::setHorizontalHeader(QHeaderView * header);
impl /*struct*/ QTableView {
  pub fn setHorizontalHeader<RetType, T: QTableView_setHorizontalHeader<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeader(self);
    // return 1;
  }
}

pub trait QTableView_setHorizontalHeader<RetType> {
  fn setHorizontalHeader(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setHorizontalHeader(QHeaderView * header);
impl<'a> /*trait*/ QTableView_setHorizontalHeader<()> for (QHeaderView) {
  fn setHorizontalHeader(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView19setHorizontalHeaderEP11QHeaderView()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTableView19setHorizontalHeaderEP11QHeaderView(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::setRowHidden(int row, bool hide);
impl /*struct*/ QTableView {
  pub fn setRowHidden<RetType, T: QTableView_setRowHidden<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRowHidden(self);
    // return 1;
  }
}

pub trait QTableView_setRowHidden<RetType> {
  fn setRowHidden(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setRowHidden(int row, bool hide);
impl<'a> /*trait*/ QTableView_setRowHidden<()> for (i32, i8) {
  fn setRowHidden(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12setRowHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN10QTableView12setRowHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTableView::rowViewportPosition(int row);
impl /*struct*/ QTableView {
  pub fn rowViewportPosition<RetType, T: QTableView_rowViewportPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowViewportPosition(self);
    // return 1;
  }
}

pub trait QTableView_rowViewportPosition<RetType> {
  fn rowViewportPosition(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  int QTableView::rowViewportPosition(int row);
impl<'a> /*trait*/ QTableView_rowViewportPosition<i32> for (i32) {
  fn rowViewportPosition(self , rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView19rowViewportPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView19rowViewportPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTableView::columnAt(int x);
impl /*struct*/ QTableView {
  pub fn columnAt<RetType, T: QTableView_columnAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnAt(self);
    // return 1;
  }
}

pub trait QTableView_columnAt<RetType> {
  fn columnAt(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  int QTableView::columnAt(int x);
impl<'a> /*trait*/ QTableView_columnAt<i32> for (i32) {
  fn columnAt(self , rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView8columnAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView8columnAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTableView::isRowHidden(int row);
impl /*struct*/ QTableView {
  pub fn isRowHidden<RetType, T: QTableView_isRowHidden<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRowHidden(self);
    // return 1;
  }
}

pub trait QTableView_isRowHidden<RetType> {
  fn isRowHidden(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  bool QTableView::isRowHidden(int row);
impl<'a> /*trait*/ QTableView_isRowHidden<i8> for (i32) {
  fn isRowHidden(self , rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView11isRowHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView11isRowHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTableView::showColumn(int column);
impl /*struct*/ QTableView {
  pub fn showColumn<RetType, T: QTableView_showColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showColumn(self);
    // return 1;
  }
}

pub trait QTableView_showColumn<RetType> {
  fn showColumn(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::showColumn(int column);
impl<'a> /*trait*/ QTableView_showColumn<()> for (i32) {
  fn showColumn(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView10showColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView10showColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::resizeRowToContents(int row);
impl /*struct*/ QTableView {
  pub fn resizeRowToContents<RetType, T: QTableView_resizeRowToContents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resizeRowToContents(self);
    // return 1;
  }
}

pub trait QTableView_resizeRowToContents<RetType> {
  fn resizeRowToContents(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::resizeRowToContents(int row);
impl<'a> /*trait*/ QTableView_resizeRowToContents<()> for (i32) {
  fn resizeRowToContents(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView19resizeRowToContentsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView19resizeRowToContentsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::setRootIndex(const QModelIndex & index);
impl /*struct*/ QTableView {
  pub fn setRootIndex<RetType, T: QTableView_setRootIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex(self);
    // return 1;
  }
}

pub trait QTableView_setRootIndex<RetType> {
  fn setRootIndex(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QTableView_setRootIndex<()> for (QModelIndex) {
  fn setRootIndex(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTableView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::setColumnHidden(int column, bool hide);
impl /*struct*/ QTableView {
  pub fn setColumnHidden<RetType, T: QTableView_setColumnHidden<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColumnHidden(self);
    // return 1;
  }
}

pub trait QTableView_setColumnHidden<RetType> {
  fn setColumnHidden(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setColumnHidden(int column, bool hide);
impl<'a> /*trait*/ QTableView_setColumnHidden<()> for (i32, i8) {
  fn setColumnHidden(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView15setColumnHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN10QTableView15setColumnHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTableView::hideRow(int row);
impl /*struct*/ QTableView {
  pub fn hideRow<RetType, T: QTableView_hideRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hideRow(self);
    // return 1;
  }
}

pub trait QTableView_hideRow<RetType> {
  fn hideRow(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::hideRow(int row);
impl<'a> /*trait*/ QTableView_hideRow<()> for (i32) {
  fn hideRow(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView7hideRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView7hideRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::resizeColumnsToContents();
impl /*struct*/ QTableView {
  pub fn resizeColumnsToContents<RetType, T: QTableView_resizeColumnsToContents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resizeColumnsToContents(self);
    // return 1;
  }
}

pub trait QTableView_resizeColumnsToContents<RetType> {
  fn resizeColumnsToContents(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::resizeColumnsToContents();
impl<'a> /*trait*/ QTableView_resizeColumnsToContents<()> for () {
  fn resizeColumnsToContents(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView23resizeColumnsToContentsEv()};
     unsafe {_ZN10QTableView23resizeColumnsToContentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTableView::wordWrap();
impl /*struct*/ QTableView {
  pub fn wordWrap<RetType, T: QTableView_wordWrap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.wordWrap(self);
    // return 1;
  }
}

pub trait QTableView_wordWrap<RetType> {
  fn wordWrap(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  bool QTableView::wordWrap();
impl<'a> /*trait*/ QTableView_wordWrap<i8> for () {
  fn wordWrap(self , rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView8wordWrapEv()};
    let mut ret = unsafe {_ZNK10QTableView8wordWrapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTableView::setShowGrid(bool show);
impl /*struct*/ QTableView {
  pub fn setShowGrid<RetType, T: QTableView_setShowGrid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setShowGrid(self);
    // return 1;
  }
}

pub trait QTableView_setShowGrid<RetType> {
  fn setShowGrid(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setShowGrid(bool show);
impl<'a> /*trait*/ QTableView_setShowGrid<()> for (i8) {
  fn setShowGrid(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView11setShowGridEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTableView11setShowGridEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTableView::isColumnHidden(int column);
impl /*struct*/ QTableView {
  pub fn isColumnHidden<RetType, T: QTableView_isColumnHidden<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isColumnHidden(self);
    // return 1;
  }
}

pub trait QTableView_isColumnHidden<RetType> {
  fn isColumnHidden(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  bool QTableView::isColumnHidden(int column);
impl<'a> /*trait*/ QTableView_isColumnHidden<i8> for (i32) {
  fn isColumnHidden(self , rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView14isColumnHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView14isColumnHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTableView::selectRow(int row);
impl /*struct*/ QTableView {
  pub fn selectRow<RetType, T: QTableView_selectRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectRow(self);
    // return 1;
  }
}

pub trait QTableView_selectRow<RetType> {
  fn selectRow(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::selectRow(int row);
impl<'a> /*trait*/ QTableView_selectRow<()> for (i32) {
  fn selectRow(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView9selectRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView9selectRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTableView::metaObject();
impl /*struct*/ QTableView {
  pub fn metaObject<RetType, T: QTableView_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTableView_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  const QMetaObject * QTableView::metaObject();
impl<'a> /*trait*/ QTableView_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView10metaObjectEv()};
     unsafe {_ZNK10QTableView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTableView::isCornerButtonEnabled();
impl /*struct*/ QTableView {
  pub fn isCornerButtonEnabled<RetType, T: QTableView_isCornerButtonEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCornerButtonEnabled(self);
    // return 1;
  }
}

pub trait QTableView_isCornerButtonEnabled<RetType> {
  fn isCornerButtonEnabled(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  bool QTableView::isCornerButtonEnabled();
impl<'a> /*trait*/ QTableView_isCornerButtonEnabled<i8> for () {
  fn isCornerButtonEnabled(self , rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView21isCornerButtonEnabledEv()};
    let mut ret = unsafe {_ZNK10QTableView21isCornerButtonEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTableView::selectColumn(int column);
impl /*struct*/ QTableView {
  pub fn selectColumn<RetType, T: QTableView_selectColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectColumn(self);
    // return 1;
  }
}

pub trait QTableView_selectColumn<RetType> {
  fn selectColumn(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::selectColumn(int column);
impl<'a> /*trait*/ QTableView_selectColumn<()> for (i32) {
  fn selectColumn(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12selectColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView12selectColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::~QTableView();
impl /*struct*/ QTableView {
  pub fn FreeQTableView<RetType, T: QTableView_FreeQTableView<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTableView(self);
    // return 1;
  }
}

pub trait QTableView_FreeQTableView<RetType> {
  fn FreeQTableView(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::~QTableView();
impl<'a> /*trait*/ QTableView_FreeQTableView<()> for () {
  fn FreeQTableView(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableViewD0Ev()};
     unsafe {_ZN10QTableViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTableView::resizeColumnToContents(int column);
impl /*struct*/ QTableView {
  pub fn resizeColumnToContents<RetType, T: QTableView_resizeColumnToContents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resizeColumnToContents(self);
    // return 1;
  }
}

pub trait QTableView_resizeColumnToContents<RetType> {
  fn resizeColumnToContents(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::resizeColumnToContents(int column);
impl<'a> /*trait*/ QTableView_resizeColumnToContents<()> for (i32) {
  fn resizeColumnToContents(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView22resizeColumnToContentsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView22resizeColumnToContentsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::QTableView(QWidget * parent);
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

  // proto:  void QTableView::QTableView(QWidget * parent);
impl<'a> /*trait*/ QTableView_NewQTableView for (QWidget) {
  fn NewQTableView(self) -> QTableView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableViewC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTableViewC1EP7QWidget(qthis, arg0)};
    let rsthis = QTableView{/**/qbase: QAbstractItemView::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTableView::sortByColumn(int column);
impl /*struct*/ QTableView {
  pub fn sortByColumn<RetType, T: QTableView_sortByColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sortByColumn(self);
    // return 1;
  }
}

pub trait QTableView_sortByColumn<RetType> {
  fn sortByColumn(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::sortByColumn(int column);
impl<'a> /*trait*/ QTableView_sortByColumn<()> for (i32) {
  fn sortByColumn(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView12sortByColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView12sortByColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTableView::columnSpan(int row, int column);
impl /*struct*/ QTableView {
  pub fn columnSpan<RetType, T: QTableView_columnSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnSpan(self);
    // return 1;
  }
}

pub trait QTableView_columnSpan<RetType> {
  fn columnSpan(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  int QTableView::columnSpan(int row, int column);
impl<'a> /*trait*/ QTableView_columnSpan<i32> for (i32, i32) {
  fn columnSpan(self , rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView10columnSpanEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QTableView10columnSpanEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTableView::columnWidth(int column);
impl /*struct*/ QTableView {
  pub fn columnWidth<RetType, T: QTableView_columnWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnWidth(self);
    // return 1;
  }
}

pub trait QTableView_columnWidth<RetType> {
  fn columnWidth(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  int QTableView::columnWidth(int column);
impl<'a> /*trait*/ QTableView_columnWidth<i32> for (i32) {
  fn columnWidth(self , rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView11columnWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView11columnWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTableView::columnViewportPosition(int column);
impl /*struct*/ QTableView {
  pub fn columnViewportPosition<RetType, T: QTableView_columnViewportPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnViewportPosition(self);
    // return 1;
  }
}

pub trait QTableView_columnViewportPosition<RetType> {
  fn columnViewportPosition(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  int QTableView::columnViewportPosition(int column);
impl<'a> /*trait*/ QTableView_columnViewportPosition<i32> for (i32) {
  fn columnViewportPosition(self , rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView22columnViewportPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView22columnViewportPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTableView::rowHeight(int row);
impl /*struct*/ QTableView {
  pub fn rowHeight<RetType, T: QTableView_rowHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowHeight(self);
    // return 1;
  }
}

pub trait QTableView_rowHeight<RetType> {
  fn rowHeight(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  int QTableView::rowHeight(int row);
impl<'a> /*trait*/ QTableView_rowHeight<i32> for (i32) {
  fn rowHeight(self , rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView9rowHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView9rowHeightEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTableView::QTableView(const QTableView & );
impl<'a> /*trait*/ QTableView_NewQTableView for (QTableView) {
  fn NewQTableView(self) -> QTableView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableViewC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTableViewC1ERKS_(qthis, arg0)};
    let rsthis = QTableView{/**/qbase: QAbstractItemView::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTableView::rowAt(int y);
impl /*struct*/ QTableView {
  pub fn rowAt<RetType, T: QTableView_rowAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowAt(self);
    // return 1;
  }
}

pub trait QTableView_rowAt<RetType> {
  fn rowAt(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  int QTableView::rowAt(int y);
impl<'a> /*trait*/ QTableView_rowAt<i32> for (i32) {
  fn rowAt(self , rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView5rowAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTableView5rowAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTableView::rowSpan(int row, int column);
impl /*struct*/ QTableView {
  pub fn rowSpan<RetType, T: QTableView_rowSpan<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowSpan(self);
    // return 1;
  }
}

pub trait QTableView_rowSpan<RetType> {
  fn rowSpan(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  int QTableView::rowSpan(int row, int column);
impl<'a> /*trait*/ QTableView_rowSpan<i32> for (i32, i32) {
  fn rowSpan(self , rsthis: &mut QTableView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView7rowSpanEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QTableView7rowSpanEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTableView::setCornerButtonEnabled(bool enable);
impl /*struct*/ QTableView {
  pub fn setCornerButtonEnabled<RetType, T: QTableView_setCornerButtonEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCornerButtonEnabled(self);
    // return 1;
  }
}

pub trait QTableView_setCornerButtonEnabled<RetType> {
  fn setCornerButtonEnabled(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setCornerButtonEnabled(bool enable);
impl<'a> /*trait*/ QTableView_setCornerButtonEnabled<()> for (i8) {
  fn setCornerButtonEnabled(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView22setCornerButtonEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTableView22setCornerButtonEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QTableView::visualRect(const QModelIndex & index);
impl /*struct*/ QTableView {
  pub fn visualRect<RetType, T: QTableView_visualRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.visualRect(self);
    // return 1;
  }
}

pub trait QTableView_visualRect<RetType> {
  fn visualRect(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  QRect QTableView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QTableView_visualRect<QRect> for (QModelIndex) {
  fn visualRect(self , rsthis: &mut QTableView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTableView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QTableView::indexAt(const QPoint & p);
impl /*struct*/ QTableView {
  pub fn indexAt<RetType, T: QTableView_indexAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexAt(self);
    // return 1;
  }
}

pub trait QTableView_indexAt<RetType> {
  fn indexAt(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  QModelIndex QTableView::indexAt(const QPoint & p);
impl<'a> /*trait*/ QTableView_indexAt<QModelIndex> for (QPoint) {
  fn indexAt(self , rsthis: &mut QTableView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTableView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QHeaderView * QTableView::horizontalHeader();
impl /*struct*/ QTableView {
  pub fn horizontalHeader<RetType, T: QTableView_horizontalHeader<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.horizontalHeader(self);
    // return 1;
  }
}

pub trait QTableView_horizontalHeader<RetType> {
  fn horizontalHeader(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  QHeaderView * QTableView::horizontalHeader();
impl<'a> /*trait*/ QTableView_horizontalHeader<QHeaderView> for () {
  fn horizontalHeader(self , rsthis: &mut QTableView) -> QHeaderView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView16horizontalHeaderEv()};
    let mut ret = unsafe {_ZNK10QTableView16horizontalHeaderEv(rsthis.qclsinst)};
    let mut ret1 = QHeaderView::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTableView::isSortingEnabled();
impl /*struct*/ QTableView {
  pub fn isSortingEnabled<RetType, T: QTableView_isSortingEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableView_isSortingEnabled<RetType> {
  fn isSortingEnabled(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  bool QTableView::isSortingEnabled();
impl<'a> /*trait*/ QTableView_isSortingEnabled<i8> for () {
  fn isSortingEnabled(self , rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView16isSortingEnabledEv()};
    let mut ret = unsafe {_ZNK10QTableView16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTableView::clearSpans();
impl /*struct*/ QTableView {
  pub fn clearSpans<RetType, T: QTableView_clearSpans<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearSpans(self);
    // return 1;
  }
}

pub trait QTableView_clearSpans<RetType> {
  fn clearSpans(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::clearSpans();
impl<'a> /*trait*/ QTableView_clearSpans<()> for () {
  fn clearSpans(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView10clearSpansEv()};
     unsafe {_ZN10QTableView10clearSpansEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTableView::setVerticalHeader(QHeaderView * header);
impl /*struct*/ QTableView {
  pub fn setVerticalHeader<RetType, T: QTableView_setVerticalHeader<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeader(self);
    // return 1;
  }
}

pub trait QTableView_setVerticalHeader<RetType> {
  fn setVerticalHeader(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::setVerticalHeader(QHeaderView * header);
impl<'a> /*trait*/ QTableView_setVerticalHeader<()> for (QHeaderView) {
  fn setVerticalHeader(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView17setVerticalHeaderEP11QHeaderView()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTableView17setVerticalHeaderEP11QHeaderView(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTableView::showGrid();
impl /*struct*/ QTableView {
  pub fn showGrid<RetType, T: QTableView_showGrid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showGrid(self);
    // return 1;
  }
}

pub trait QTableView_showGrid<RetType> {
  fn showGrid(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  bool QTableView::showGrid();
impl<'a> /*trait*/ QTableView_showGrid<i8> for () {
  fn showGrid(self , rsthis: &mut QTableView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTableView8showGridEv()};
    let mut ret = unsafe {_ZNK10QTableView8showGridEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTableView::showRow(int row);
impl /*struct*/ QTableView {
  pub fn showRow<RetType, T: QTableView_showRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showRow(self);
    // return 1;
  }
}

pub trait QTableView_showRow<RetType> {
  fn showRow(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::showRow(int row);
impl<'a> /*trait*/ QTableView_showRow<()> for (i32) {
  fn showRow(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView7showRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView7showRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableView::hideColumn(int column);
impl /*struct*/ QTableView {
  pub fn hideColumn<RetType, T: QTableView_hideColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hideColumn(self);
    // return 1;
  }
}

pub trait QTableView_hideColumn<RetType> {
  fn hideColumn(self , rsthis: &mut QTableView) -> RetType;
}

  // proto:  void QTableView::hideColumn(int column);
impl<'a> /*trait*/ QTableView_hideColumn<()> for (i32) {
  fn hideColumn(self , rsthis: &mut QTableView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTableView10hideColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTableView10hideColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

