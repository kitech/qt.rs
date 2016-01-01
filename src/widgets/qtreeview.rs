// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtWidgets/qtreeview.h
// dst-file: /src/widgets/qtreeview.rs
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
use super::qwidget::QWidget; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qitemselectionmodel::QItemSelectionModel; // 771
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTreeView_Class_Size() -> c_int;
  // proto:  void QTreeView::setHeader(QHeaderView * header);
  fn _ZN9QTreeView9setHeaderEP11QHeaderView(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTreeView::isAnimated();
  fn _ZNK9QTreeView10isAnimatedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTreeView::isExpanded(const QModelIndex & index);
  fn _ZNK9QTreeView10isExpandedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QTreeView::setColumnHidden(int column, bool hide);
  fn _ZN9QTreeView15setColumnHiddenEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  void QTreeView::setIndentation(int i);
  fn _ZN9QTreeView14setIndentationEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  const QMetaObject * QTreeView::metaObject();
  fn _ZNK9QTreeView10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTreeView::reset();
  fn _ZN9QTreeView5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTreeView::setExpandsOnDoubleClick(bool enable);
  fn _ZN9QTreeView23setExpandsOnDoubleClickEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTreeView::setFirstColumnSpanned(int row, const QModelIndex & parent, bool span);
  fn _ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_char);
  // proto:  void QTreeView::sortByColumn(int column);
  fn _ZN9QTreeView12sortByColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTreeView::setRowHidden(int row, const QModelIndex & parent, bool hide);
  fn _ZN9QTreeView12setRowHiddenEiRK11QModelIndexb(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_char);
  // proto:  void QTreeView::expand(const QModelIndex & index);
  fn _ZN9QTreeView6expandERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTreeView::autoExpandDelay();
  fn _ZNK9QTreeView15autoExpandDelayEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTreeView::QTreeView(QWidget * parent);
  fn dector_ZN9QTreeViewC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QTreeViewC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTreeView::~QTreeView();
  fn _ZN9QTreeViewD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTreeView::indentation();
  fn _ZNK9QTreeView11indentationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTreeView::columnViewportPosition(int column);
  fn _ZNK9QTreeView22columnViewportPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  bool QTreeView::expandsOnDoubleClick();
  fn _ZNK9QTreeView20expandsOnDoubleClickEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTreeView::isSortingEnabled();
  fn _ZNK9QTreeView16isSortingEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTreeView::showColumn(int column);
  fn _ZN9QTreeView10showColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRect QTreeView::visualRect(const QModelIndex & index);
  fn _ZNK9QTreeView10visualRectERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeView::collapse(const QModelIndex & index);
  fn _ZN9QTreeView8collapseERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTreeView::setWordWrap(bool on);
  fn _ZN9QTreeView11setWordWrapEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QModelIndex QTreeView::indexAbove(const QModelIndex & index);
  fn _ZNK9QTreeView10indexAboveERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTreeView::rootIsDecorated();
  fn _ZNK9QTreeView15rootIsDecoratedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTreeView::collapseAll();
  fn _ZN9QTreeView11collapseAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTreeView::setHeaderHidden(bool hide);
  fn _ZN9QTreeView15setHeaderHiddenEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QTreeView::allColumnsShowFocus();
  fn _ZNK9QTreeView19allColumnsShowFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTreeView::columnWidth(int column);
  fn _ZNK9QTreeView11columnWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QTreeView::resizeColumnToContents(int column);
  fn _ZN9QTreeView22resizeColumnToContentsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTreeView::setAutoExpandDelay(int delay);
  fn _ZN9QTreeView18setAutoExpandDelayEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTreeView::setAllColumnsShowFocus(bool enable);
  fn _ZN9QTreeView22setAllColumnsShowFocusEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QTreeView::isFirstColumnSpanned(int row, const QModelIndex & parent);
  fn _ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QTreeView::hideColumn(int column);
  fn _ZN9QTreeView10hideColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QTreeView::treePosition();
  fn _ZNK9QTreeView12treePositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTreeView::setExpanded(const QModelIndex & index, bool expand);
  fn _ZN9QTreeView11setExpandedERK11QModelIndexb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  void QTreeView::resetIndentation();
  fn _ZN9QTreeView16resetIndentationEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTreeView::isRowHidden(int row, const QModelIndex & parent);
  fn _ZNK9QTreeView11isRowHiddenEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QTreeView::collapsed(const QModelIndex & index);
  fn _ZN9QTreeView9collapsedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTreeView::QTreeView(const QTreeView & );
  fn dector_ZN9QTreeViewC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QTreeViewC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTreeView::selectAll();
  fn _ZN9QTreeView9selectAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTreeView::wordWrap();
  fn _ZNK9QTreeView8wordWrapEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTreeView::doItemsLayout();
  fn _ZN9QTreeView13doItemsLayoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTreeView::setTreePosition(int logicalIndex);
  fn _ZN9QTreeView15setTreePositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTreeView::keyboardSearch(const QString & search);
  fn _ZN9QTreeView14keyboardSearchERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTreeView::setRootIndex(const QModelIndex & index);
  fn _ZN9QTreeView12setRootIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTreeView::setItemsExpandable(bool enable);
  fn _ZN9QTreeView18setItemsExpandableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTreeView::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN9QTreeView17setSelectionModelEP19QItemSelectionModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QHeaderView * QTreeView::header();
  fn _ZNK9QTreeView6headerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTreeView::setAnimated(bool enable);
  fn _ZN9QTreeView11setAnimatedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTreeView::setSortingEnabled(bool enable);
  fn _ZN9QTreeView17setSortingEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QTreeView::itemsExpandable();
  fn _ZNK9QTreeView15itemsExpandableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTreeView::setRootIsDecorated(bool show);
  fn _ZN9QTreeView18setRootIsDecoratedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTreeView::expanded(const QModelIndex & index);
  fn _ZN9QTreeView8expandedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTreeView::isHeaderHidden();
  fn _ZNK9QTreeView14isHeaderHiddenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTreeView::columnAt(int x);
  fn _ZNK9QTreeView8columnAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QTreeView::setModel(QAbstractItemModel * model);
  fn _ZN9QTreeView8setModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTreeView::isColumnHidden(int column);
  fn _ZNK9QTreeView14isColumnHiddenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  bool QTreeView::uniformRowHeights();
  fn _ZNK9QTreeView17uniformRowHeightsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTreeView::setUniformRowHeights(bool uniform);
  fn _ZN9QTreeView20setUniformRowHeightsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTreeView::expandToDepth(int depth);
  fn _ZN9QTreeView13expandToDepthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QModelIndex QTreeView::indexBelow(const QModelIndex & index);
  fn _ZNK9QTreeView10indexBelowERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeView::expandAll();
  fn _ZN9QTreeView9expandAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  QModelIndex QTreeView::indexAt(const QPoint & p);
  fn _ZNK9QTreeView7indexAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeView::setColumnWidth(int column, int width);
  fn _ZN9QTreeView14setColumnWidthEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  fn QTreeView_SlotProxy_connect__ZN9QTreeView9collapsedERK11QModelIndex(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTreeView_SlotProxy_connect__ZN9QTreeView8expandedERK11QModelIndex(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTreeView)=1
#[derive(Default)]
pub struct QTreeView {
  qbase: QAbstractItemView,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _collapsed_1: QTreeView_collapsed_signal,
  pub _expanded_1: QTreeView_expanded_signal,
}

impl /*struct*/ QTreeView {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTreeView {
    return QTreeView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTreeView {
  type Target = QAbstractItemView;

  fn deref(&self) -> &QAbstractItemView {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemView> for QTreeView {
  fn as_ref(& self) -> & QAbstractItemView {
    return & self.qbase;
  }
}
  // proto:  void QTreeView::setHeader(QHeaderView * header);
impl /*struct*/ QTreeView {
  pub fn setHeader<RetType, T: QTreeView_setHeader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeader(self);
    // return 1;
  }
}

pub trait QTreeView_setHeader<RetType> {
  fn setHeader(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setHeader(QHeaderView * header);
impl<'a> /*trait*/ QTreeView_setHeader<()> for (&'a QHeaderView) {
  fn setHeader(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9setHeaderEP11QHeaderView()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView9setHeaderEP11QHeaderView(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTreeView::isAnimated();
impl /*struct*/ QTreeView {
  pub fn isAnimated<RetType, T: QTreeView_isAnimated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAnimated(self);
    // return 1;
  }
}

pub trait QTreeView_isAnimated<RetType> {
  fn isAnimated(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::isAnimated();
impl<'a> /*trait*/ QTreeView_isAnimated<i8> for () {
  fn isAnimated(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10isAnimatedEv()};
    let mut ret = unsafe {_ZNK9QTreeView10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTreeView::isExpanded(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn isExpanded<RetType, T: QTreeView_isExpanded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isExpanded(self);
    // return 1;
  }
}

pub trait QTreeView_isExpanded<RetType> {
  fn isExpanded(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::isExpanded(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_isExpanded<i8> for (&'a QModelIndex) {
  fn isExpanded(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10isExpandedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView10isExpandedERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeView::setColumnHidden(int column, bool hide);
impl /*struct*/ QTreeView {
  pub fn setColumnHidden<RetType, T: QTreeView_setColumnHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnHidden(self);
    // return 1;
  }
}

pub trait QTreeView_setColumnHidden<RetType> {
  fn setColumnHidden(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setColumnHidden(int column, bool hide);
impl<'a> /*trait*/ QTreeView_setColumnHidden<()> for (i32, i8) {
  fn setColumnHidden(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setColumnHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN9QTreeView15setColumnHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeView::setIndentation(int i);
impl /*struct*/ QTreeView {
  pub fn setIndentation<RetType, T: QTreeView_setIndentation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIndentation(self);
    // return 1;
  }
}

pub trait QTreeView_setIndentation<RetType> {
  fn setIndentation(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setIndentation(int i);
impl<'a> /*trait*/ QTreeView_setIndentation<()> for (i32) {
  fn setIndentation(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14setIndentationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView14setIndentationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTreeView::metaObject();
impl /*struct*/ QTreeView {
  pub fn metaObject<RetType, T: QTreeView_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTreeView_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  const QMetaObject * QTreeView::metaObject();
impl<'a> /*trait*/ QTreeView_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10metaObjectEv()};
     unsafe {_ZNK9QTreeView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeView::reset();
impl /*struct*/ QTreeView {
  pub fn reset<RetType, T: QTreeView_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QTreeView_reset<RetType> {
  fn reset(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::reset();
impl<'a> /*trait*/ QTreeView_reset<()> for () {
  fn reset(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView5resetEv()};
     unsafe {_ZN9QTreeView5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeView::setExpandsOnDoubleClick(bool enable);
impl /*struct*/ QTreeView {
  pub fn setExpandsOnDoubleClick<RetType, T: QTreeView_setExpandsOnDoubleClick<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpandsOnDoubleClick(self);
    // return 1;
  }
}

pub trait QTreeView_setExpandsOnDoubleClick<RetType> {
  fn setExpandsOnDoubleClick(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setExpandsOnDoubleClick(bool enable);
impl<'a> /*trait*/ QTreeView_setExpandsOnDoubleClick<()> for (i8) {
  fn setExpandsOnDoubleClick(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView23setExpandsOnDoubleClickEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView23setExpandsOnDoubleClickEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setFirstColumnSpanned(int row, const QModelIndex & parent, bool span);
impl /*struct*/ QTreeView {
  pub fn setFirstColumnSpanned<RetType, T: QTreeView_setFirstColumnSpanned<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFirstColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeView_setFirstColumnSpanned<RetType> {
  fn setFirstColumnSpanned(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setFirstColumnSpanned(int row, const QModelIndex & parent, bool span);
impl<'a> /*trait*/ QTreeView_setFirstColumnSpanned<()> for (i32, &'a QModelIndex, i8) {
  fn setFirstColumnSpanned(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_char;
     unsafe {_ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QTreeView::sortByColumn(int column);
impl /*struct*/ QTreeView {
  pub fn sortByColumn<RetType, T: QTreeView_sortByColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sortByColumn(self);
    // return 1;
  }
}

pub trait QTreeView_sortByColumn<RetType> {
  fn sortByColumn(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::sortByColumn(int column);
impl<'a> /*trait*/ QTreeView_sortByColumn<()> for (i32) {
  fn sortByColumn(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12sortByColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView12sortByColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setRowHidden(int row, const QModelIndex & parent, bool hide);
impl /*struct*/ QTreeView {
  pub fn setRowHidden<RetType, T: QTreeView_setRowHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowHidden(self);
    // return 1;
  }
}

pub trait QTreeView_setRowHidden<RetType> {
  fn setRowHidden(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setRowHidden(int row, const QModelIndex & parent, bool hide);
impl<'a> /*trait*/ QTreeView_setRowHidden<()> for (i32, &'a QModelIndex, i8) {
  fn setRowHidden(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12setRowHiddenEiRK11QModelIndexb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_char;
     unsafe {_ZN9QTreeView12setRowHiddenEiRK11QModelIndexb(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QTreeView::expand(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn expand<RetType, T: QTreeView_expand<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expand(self);
    // return 1;
  }
}

pub trait QTreeView_expand<RetType> {
  fn expand(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::expand(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_expand<()> for (&'a QModelIndex) {
  fn expand(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView6expandERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView6expandERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTreeView::autoExpandDelay();
impl /*struct*/ QTreeView {
  pub fn autoExpandDelay<RetType, T: QTreeView_autoExpandDelay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoExpandDelay(self);
    // return 1;
  }
}

pub trait QTreeView_autoExpandDelay<RetType> {
  fn autoExpandDelay(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  int QTreeView::autoExpandDelay();
impl<'a> /*trait*/ QTreeView_autoExpandDelay<i32> for () {
  fn autoExpandDelay(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15autoExpandDelayEv()};
    let mut ret = unsafe {_ZNK9QTreeView15autoExpandDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeView::QTreeView(QWidget * parent);
impl /*struct*/ QTreeView {
  pub fn new<T: QTreeView_new>(value: T) -> QTreeView {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeView_new {
  fn new(self) -> QTreeView;
}

  // proto:  void QTreeView::QTreeView(QWidget * parent);
impl<'a> /*trait*/ QTreeView_new for (&'a QWidget) {
  fn new(self) -> QTreeView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeViewC1EP7QWidget()};
    let ctysz: c_int = unsafe{QTreeView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QTreeViewC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QTreeViewC1EP7QWidget(arg0)} as u64;
    let rsthis = QTreeView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeView::~QTreeView();
impl /*struct*/ QTreeView {
  pub fn free<RetType, T: QTreeView_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTreeView_free<RetType> {
  fn free(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::~QTreeView();
impl<'a> /*trait*/ QTreeView_free<()> for () {
  fn free(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeViewD0Ev()};
     unsafe {_ZN9QTreeViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTreeView::indentation();
impl /*struct*/ QTreeView {
  pub fn indentation<RetType, T: QTreeView_indentation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indentation(self);
    // return 1;
  }
}

pub trait QTreeView_indentation<RetType> {
  fn indentation(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  int QTreeView::indentation();
impl<'a> /*trait*/ QTreeView_indentation<i32> for () {
  fn indentation(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11indentationEv()};
    let mut ret = unsafe {_ZNK9QTreeView11indentationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTreeView::columnViewportPosition(int column);
impl /*struct*/ QTreeView {
  pub fn columnViewportPosition<RetType, T: QTreeView_columnViewportPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnViewportPosition(self);
    // return 1;
  }
}

pub trait QTreeView_columnViewportPosition<RetType> {
  fn columnViewportPosition(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  int QTreeView::columnViewportPosition(int column);
impl<'a> /*trait*/ QTreeView_columnViewportPosition<i32> for (i32) {
  fn columnViewportPosition(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView22columnViewportPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTreeView22columnViewportPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTreeView::expandsOnDoubleClick();
impl /*struct*/ QTreeView {
  pub fn expandsOnDoubleClick<RetType, T: QTreeView_expandsOnDoubleClick<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expandsOnDoubleClick(self);
    // return 1;
  }
}

pub trait QTreeView_expandsOnDoubleClick<RetType> {
  fn expandsOnDoubleClick(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::expandsOnDoubleClick();
impl<'a> /*trait*/ QTreeView_expandsOnDoubleClick<i8> for () {
  fn expandsOnDoubleClick(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView20expandsOnDoubleClickEv()};
    let mut ret = unsafe {_ZNK9QTreeView20expandsOnDoubleClickEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTreeView::isSortingEnabled();
impl /*struct*/ QTreeView {
  pub fn isSortingEnabled<RetType, T: QTreeView_isSortingEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QTreeView_isSortingEnabled<RetType> {
  fn isSortingEnabled(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::isSortingEnabled();
impl<'a> /*trait*/ QTreeView_isSortingEnabled<i8> for () {
  fn isSortingEnabled(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView16isSortingEnabledEv()};
    let mut ret = unsafe {_ZNK9QTreeView16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeView::showColumn(int column);
impl /*struct*/ QTreeView {
  pub fn showColumn<RetType, T: QTreeView_showColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showColumn(self);
    // return 1;
  }
}

pub trait QTreeView_showColumn<RetType> {
  fn showColumn(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::showColumn(int column);
impl<'a> /*trait*/ QTreeView_showColumn<()> for (i32) {
  fn showColumn(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView10showColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView10showColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QTreeView::visualRect(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn visualRect<RetType, T: QTreeView_visualRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualRect(self);
    // return 1;
  }
}

pub trait QTreeView_visualRect<RetType> {
  fn visualRect(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  QRect QTreeView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_visualRect<QRect> for (&'a QModelIndex) {
  fn visualRect(self , rsthis: & QTreeView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeView::collapse(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn collapse<RetType, T: QTreeView_collapse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.collapse(self);
    // return 1;
  }
}

pub trait QTreeView_collapse<RetType> {
  fn collapse(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::collapse(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_collapse<()> for (&'a QModelIndex) {
  fn collapse(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView8collapseERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView8collapseERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setWordWrap(bool on);
impl /*struct*/ QTreeView {
  pub fn setWordWrap<RetType, T: QTreeView_setWordWrap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWordWrap(self);
    // return 1;
  }
}

pub trait QTreeView_setWordWrap<RetType> {
  fn setWordWrap(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setWordWrap(bool on);
impl<'a> /*trait*/ QTreeView_setWordWrap<()> for (i8) {
  fn setWordWrap(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setWordWrapEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView11setWordWrapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QTreeView::indexAbove(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn indexAbove<RetType, T: QTreeView_indexAbove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexAbove(self);
    // return 1;
  }
}

pub trait QTreeView_indexAbove<RetType> {
  fn indexAbove(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  QModelIndex QTreeView::indexAbove(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_indexAbove<QModelIndex> for (&'a QModelIndex) {
  fn indexAbove(self , rsthis: & QTreeView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10indexAboveERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView10indexAboveERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTreeView::rootIsDecorated();
impl /*struct*/ QTreeView {
  pub fn rootIsDecorated<RetType, T: QTreeView_rootIsDecorated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootIsDecorated(self);
    // return 1;
  }
}

pub trait QTreeView_rootIsDecorated<RetType> {
  fn rootIsDecorated(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::rootIsDecorated();
impl<'a> /*trait*/ QTreeView_rootIsDecorated<i8> for () {
  fn rootIsDecorated(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15rootIsDecoratedEv()};
    let mut ret = unsafe {_ZNK9QTreeView15rootIsDecoratedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeView::collapseAll();
impl /*struct*/ QTreeView {
  pub fn collapseAll<RetType, T: QTreeView_collapseAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.collapseAll(self);
    // return 1;
  }
}

pub trait QTreeView_collapseAll<RetType> {
  fn collapseAll(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::collapseAll();
impl<'a> /*trait*/ QTreeView_collapseAll<()> for () {
  fn collapseAll(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11collapseAllEv()};
     unsafe {_ZN9QTreeView11collapseAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeView::setHeaderHidden(bool hide);
impl /*struct*/ QTreeView {
  pub fn setHeaderHidden<RetType, T: QTreeView_setHeaderHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeaderHidden(self);
    // return 1;
  }
}

pub trait QTreeView_setHeaderHidden<RetType> {
  fn setHeaderHidden(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setHeaderHidden(bool hide);
impl<'a> /*trait*/ QTreeView_setHeaderHidden<()> for (i8) {
  fn setHeaderHidden(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setHeaderHiddenEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView15setHeaderHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTreeView::allColumnsShowFocus();
impl /*struct*/ QTreeView {
  pub fn allColumnsShowFocus<RetType, T: QTreeView_allColumnsShowFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allColumnsShowFocus(self);
    // return 1;
  }
}

pub trait QTreeView_allColumnsShowFocus<RetType> {
  fn allColumnsShowFocus(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::allColumnsShowFocus();
impl<'a> /*trait*/ QTreeView_allColumnsShowFocus<i8> for () {
  fn allColumnsShowFocus(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView19allColumnsShowFocusEv()};
    let mut ret = unsafe {_ZNK9QTreeView19allColumnsShowFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTreeView::columnWidth(int column);
impl /*struct*/ QTreeView {
  pub fn columnWidth<RetType, T: QTreeView_columnWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnWidth(self);
    // return 1;
  }
}

pub trait QTreeView_columnWidth<RetType> {
  fn columnWidth(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  int QTreeView::columnWidth(int column);
impl<'a> /*trait*/ QTreeView_columnWidth<i32> for (i32) {
  fn columnWidth(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11columnWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTreeView11columnWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeView::resizeColumnToContents(int column);
impl /*struct*/ QTreeView {
  pub fn resizeColumnToContents<RetType, T: QTreeView_resizeColumnToContents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resizeColumnToContents(self);
    // return 1;
  }
}

pub trait QTreeView_resizeColumnToContents<RetType> {
  fn resizeColumnToContents(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::resizeColumnToContents(int column);
impl<'a> /*trait*/ QTreeView_resizeColumnToContents<()> for (i32) {
  fn resizeColumnToContents(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView22resizeColumnToContentsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView22resizeColumnToContentsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setAutoExpandDelay(int delay);
impl /*struct*/ QTreeView {
  pub fn setAutoExpandDelay<RetType, T: QTreeView_setAutoExpandDelay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoExpandDelay(self);
    // return 1;
  }
}

pub trait QTreeView_setAutoExpandDelay<RetType> {
  fn setAutoExpandDelay(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setAutoExpandDelay(int delay);
impl<'a> /*trait*/ QTreeView_setAutoExpandDelay<()> for (i32) {
  fn setAutoExpandDelay(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setAutoExpandDelayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView18setAutoExpandDelayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setAllColumnsShowFocus(bool enable);
impl /*struct*/ QTreeView {
  pub fn setAllColumnsShowFocus<RetType, T: QTreeView_setAllColumnsShowFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAllColumnsShowFocus(self);
    // return 1;
  }
}

pub trait QTreeView_setAllColumnsShowFocus<RetType> {
  fn setAllColumnsShowFocus(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setAllColumnsShowFocus(bool enable);
impl<'a> /*trait*/ QTreeView_setAllColumnsShowFocus<()> for (i8) {
  fn setAllColumnsShowFocus(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView22setAllColumnsShowFocusEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView22setAllColumnsShowFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTreeView::isFirstColumnSpanned(int row, const QModelIndex & parent);
impl /*struct*/ QTreeView {
  pub fn isFirstColumnSpanned<RetType, T: QTreeView_isFirstColumnSpanned<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFirstColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeView_isFirstColumnSpanned<RetType> {
  fn isFirstColumnSpanned(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::isFirstColumnSpanned(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QTreeView_isFirstColumnSpanned<i8> for (i32, &'a QModelIndex) {
  fn isFirstColumnSpanned(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeView::hideColumn(int column);
impl /*struct*/ QTreeView {
  pub fn hideColumn<RetType, T: QTreeView_hideColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hideColumn(self);
    // return 1;
  }
}

pub trait QTreeView_hideColumn<RetType> {
  fn hideColumn(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::hideColumn(int column);
impl<'a> /*trait*/ QTreeView_hideColumn<()> for (i32) {
  fn hideColumn(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView10hideColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView10hideColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTreeView::treePosition();
impl /*struct*/ QTreeView {
  pub fn treePosition<RetType, T: QTreeView_treePosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.treePosition(self);
    // return 1;
  }
}

pub trait QTreeView_treePosition<RetType> {
  fn treePosition(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  int QTreeView::treePosition();
impl<'a> /*trait*/ QTreeView_treePosition<i32> for () {
  fn treePosition(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView12treePositionEv()};
    let mut ret = unsafe {_ZNK9QTreeView12treePositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeView::setExpanded(const QModelIndex & index, bool expand);
impl /*struct*/ QTreeView {
  pub fn setExpanded<RetType, T: QTreeView_setExpanded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpanded(self);
    // return 1;
  }
}

pub trait QTreeView_setExpanded<RetType> {
  fn setExpanded(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setExpanded(const QModelIndex & index, bool expand);
impl<'a> /*trait*/ QTreeView_setExpanded<()> for (&'a QModelIndex, i8) {
  fn setExpanded(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setExpandedERK11QModelIndexb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN9QTreeView11setExpandedERK11QModelIndexb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeView::resetIndentation();
impl /*struct*/ QTreeView {
  pub fn resetIndentation<RetType, T: QTreeView_resetIndentation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetIndentation(self);
    // return 1;
  }
}

pub trait QTreeView_resetIndentation<RetType> {
  fn resetIndentation(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::resetIndentation();
impl<'a> /*trait*/ QTreeView_resetIndentation<()> for () {
  fn resetIndentation(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView16resetIndentationEv()};
     unsafe {_ZN9QTreeView16resetIndentationEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTreeView::isRowHidden(int row, const QModelIndex & parent);
impl /*struct*/ QTreeView {
  pub fn isRowHidden<RetType, T: QTreeView_isRowHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRowHidden(self);
    // return 1;
  }
}

pub trait QTreeView_isRowHidden<RetType> {
  fn isRowHidden(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::isRowHidden(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QTreeView_isRowHidden<i8> for (i32, &'a QModelIndex) {
  fn isRowHidden(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11isRowHiddenEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView11isRowHiddenEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeView::collapsed(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn collapsed<RetType, T: QTreeView_collapsed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.collapsed(self);
    // return 1;
  }
}

pub trait QTreeView_collapsed<RetType> {
  fn collapsed(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::collapsed(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_collapsed<()> for (&'a QModelIndex) {
  fn collapsed(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9collapsedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView9collapsedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::QTreeView(const QTreeView & );
impl<'a> /*trait*/ QTreeView_new for (&'a QTreeView) {
  fn new(self) -> QTreeView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeViewC1ERKS_()};
    let ctysz: c_int = unsafe{QTreeView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QTreeViewC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QTreeViewC1ERKS_(arg0)} as u64;
    let rsthis = QTreeView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeView::selectAll();
impl /*struct*/ QTreeView {
  pub fn selectAll<RetType, T: QTreeView_selectAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectAll(self);
    // return 1;
  }
}

pub trait QTreeView_selectAll<RetType> {
  fn selectAll(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::selectAll();
impl<'a> /*trait*/ QTreeView_selectAll<()> for () {
  fn selectAll(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9selectAllEv()};
     unsafe {_ZN9QTreeView9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTreeView::wordWrap();
impl /*struct*/ QTreeView {
  pub fn wordWrap<RetType, T: QTreeView_wordWrap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wordWrap(self);
    // return 1;
  }
}

pub trait QTreeView_wordWrap<RetType> {
  fn wordWrap(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::wordWrap();
impl<'a> /*trait*/ QTreeView_wordWrap<i8> for () {
  fn wordWrap(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView8wordWrapEv()};
    let mut ret = unsafe {_ZNK9QTreeView8wordWrapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeView::doItemsLayout();
impl /*struct*/ QTreeView {
  pub fn doItemsLayout<RetType, T: QTreeView_doItemsLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout(self);
    // return 1;
  }
}

pub trait QTreeView_doItemsLayout<RetType> {
  fn doItemsLayout(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::doItemsLayout();
impl<'a> /*trait*/ QTreeView_doItemsLayout<()> for () {
  fn doItemsLayout(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView13doItemsLayoutEv()};
     unsafe {_ZN9QTreeView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeView::setTreePosition(int logicalIndex);
impl /*struct*/ QTreeView {
  pub fn setTreePosition<RetType, T: QTreeView_setTreePosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTreePosition(self);
    // return 1;
  }
}

pub trait QTreeView_setTreePosition<RetType> {
  fn setTreePosition(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setTreePosition(int logicalIndex);
impl<'a> /*trait*/ QTreeView_setTreePosition<()> for (i32) {
  fn setTreePosition(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setTreePositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView15setTreePositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::keyboardSearch(const QString & search);
impl /*struct*/ QTreeView {
  pub fn keyboardSearch<RetType, T: QTreeView_keyboardSearch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardSearch(self);
    // return 1;
  }
}

pub trait QTreeView_keyboardSearch<RetType> {
  fn keyboardSearch(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::keyboardSearch(const QString & search);
impl<'a> /*trait*/ QTreeView_keyboardSearch<()> for (&'a QString) {
  fn keyboardSearch(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14keyboardSearchERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView14keyboardSearchERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setRootIndex(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn setRootIndex<RetType, T: QTreeView_setRootIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex(self);
    // return 1;
  }
}

pub trait QTreeView_setRootIndex<RetType> {
  fn setRootIndex(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_setRootIndex<()> for (&'a QModelIndex) {
  fn setRootIndex(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setItemsExpandable(bool enable);
impl /*struct*/ QTreeView {
  pub fn setItemsExpandable<RetType, T: QTreeView_setItemsExpandable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemsExpandable(self);
    // return 1;
  }
}

pub trait QTreeView_setItemsExpandable<RetType> {
  fn setItemsExpandable(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setItemsExpandable(bool enable);
impl<'a> /*trait*/ QTreeView_setItemsExpandable<()> for (i8) {
  fn setItemsExpandable(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setItemsExpandableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView18setItemsExpandableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setSelectionModel(QItemSelectionModel * selectionModel);
impl /*struct*/ QTreeView {
  pub fn setSelectionModel<RetType, T: QTreeView_setSelectionModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel(self);
    // return 1;
  }
}

pub trait QTreeView_setSelectionModel<RetType> {
  fn setSelectionModel(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QTreeView_setSelectionModel<()> for (&'a QItemSelectionModel) {
  fn setSelectionModel(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QHeaderView * QTreeView::header();
impl /*struct*/ QTreeView {
  pub fn header<RetType, T: QTreeView_header<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.header(self);
    // return 1;
  }
}

pub trait QTreeView_header<RetType> {
  fn header(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  QHeaderView * QTreeView::header();
impl<'a> /*trait*/ QTreeView_header<QHeaderView> for () {
  fn header(self , rsthis: & QTreeView) -> QHeaderView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView6headerEv()};
    let mut ret = unsafe {_ZNK9QTreeView6headerEv(rsthis.qclsinst)};
    let mut ret1 = QHeaderView::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeView::setAnimated(bool enable);
impl /*struct*/ QTreeView {
  pub fn setAnimated<RetType, T: QTreeView_setAnimated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAnimated(self);
    // return 1;
  }
}

pub trait QTreeView_setAnimated<RetType> {
  fn setAnimated(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setAnimated(bool enable);
impl<'a> /*trait*/ QTreeView_setAnimated<()> for (i8) {
  fn setAnimated(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setAnimatedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::setSortingEnabled(bool enable);
impl /*struct*/ QTreeView {
  pub fn setSortingEnabled<RetType, T: QTreeView_setSortingEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QTreeView_setSortingEnabled<RetType> {
  fn setSortingEnabled(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QTreeView_setSortingEnabled<()> for (i8) {
  fn setSortingEnabled(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView17setSortingEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTreeView::itemsExpandable();
impl /*struct*/ QTreeView {
  pub fn itemsExpandable<RetType, T: QTreeView_itemsExpandable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemsExpandable(self);
    // return 1;
  }
}

pub trait QTreeView_itemsExpandable<RetType> {
  fn itemsExpandable(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::itemsExpandable();
impl<'a> /*trait*/ QTreeView_itemsExpandable<i8> for () {
  fn itemsExpandable(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15itemsExpandableEv()};
    let mut ret = unsafe {_ZNK9QTreeView15itemsExpandableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeView::setRootIsDecorated(bool show);
impl /*struct*/ QTreeView {
  pub fn setRootIsDecorated<RetType, T: QTreeView_setRootIsDecorated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootIsDecorated(self);
    // return 1;
  }
}

pub trait QTreeView_setRootIsDecorated<RetType> {
  fn setRootIsDecorated(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setRootIsDecorated(bool show);
impl<'a> /*trait*/ QTreeView_setRootIsDecorated<()> for (i8) {
  fn setRootIsDecorated(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setRootIsDecoratedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView18setRootIsDecoratedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::expanded(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn expanded<RetType, T: QTreeView_expanded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expanded(self);
    // return 1;
  }
}

pub trait QTreeView_expanded<RetType> {
  fn expanded(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::expanded(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_expanded<()> for (&'a QModelIndex) {
  fn expanded(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView8expandedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView8expandedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTreeView::isHeaderHidden();
impl /*struct*/ QTreeView {
  pub fn isHeaderHidden<RetType, T: QTreeView_isHeaderHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isHeaderHidden(self);
    // return 1;
  }
}

pub trait QTreeView_isHeaderHidden<RetType> {
  fn isHeaderHidden(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::isHeaderHidden();
impl<'a> /*trait*/ QTreeView_isHeaderHidden<i8> for () {
  fn isHeaderHidden(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView14isHeaderHiddenEv()};
    let mut ret = unsafe {_ZNK9QTreeView14isHeaderHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTreeView::columnAt(int x);
impl /*struct*/ QTreeView {
  pub fn columnAt<RetType, T: QTreeView_columnAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnAt(self);
    // return 1;
  }
}

pub trait QTreeView_columnAt<RetType> {
  fn columnAt(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  int QTreeView::columnAt(int x);
impl<'a> /*trait*/ QTreeView_columnAt<i32> for (i32) {
  fn columnAt(self , rsthis: & QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView8columnAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTreeView8columnAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeView::setModel(QAbstractItemModel * model);
impl /*struct*/ QTreeView {
  pub fn setModel<RetType, T: QTreeView_setModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModel(self);
    // return 1;
  }
}

pub trait QTreeView_setModel<RetType> {
  fn setModel(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setModel(QAbstractItemModel * model);
impl<'a> /*trait*/ QTreeView_setModel<()> for (&'a QAbstractItemModel) {
  fn setModel(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView8setModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView8setModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTreeView::isColumnHidden(int column);
impl /*struct*/ QTreeView {
  pub fn isColumnHidden<RetType, T: QTreeView_isColumnHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isColumnHidden(self);
    // return 1;
  }
}

pub trait QTreeView_isColumnHidden<RetType> {
  fn isColumnHidden(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::isColumnHidden(int column);
impl<'a> /*trait*/ QTreeView_isColumnHidden<i8> for (i32) {
  fn isColumnHidden(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView14isColumnHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTreeView14isColumnHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTreeView::uniformRowHeights();
impl /*struct*/ QTreeView {
  pub fn uniformRowHeights<RetType, T: QTreeView_uniformRowHeights<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.uniformRowHeights(self);
    // return 1;
  }
}

pub trait QTreeView_uniformRowHeights<RetType> {
  fn uniformRowHeights(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  bool QTreeView::uniformRowHeights();
impl<'a> /*trait*/ QTreeView_uniformRowHeights<i8> for () {
  fn uniformRowHeights(self , rsthis: & QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView17uniformRowHeightsEv()};
    let mut ret = unsafe {_ZNK9QTreeView17uniformRowHeightsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeView::setUniformRowHeights(bool uniform);
impl /*struct*/ QTreeView {
  pub fn setUniformRowHeights<RetType, T: QTreeView_setUniformRowHeights<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUniformRowHeights(self);
    // return 1;
  }
}

pub trait QTreeView_setUniformRowHeights<RetType> {
  fn setUniformRowHeights(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setUniformRowHeights(bool uniform);
impl<'a> /*trait*/ QTreeView_setUniformRowHeights<()> for (i8) {
  fn setUniformRowHeights(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView20setUniformRowHeightsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTreeView20setUniformRowHeightsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeView::expandToDepth(int depth);
impl /*struct*/ QTreeView {
  pub fn expandToDepth<RetType, T: QTreeView_expandToDepth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expandToDepth(self);
    // return 1;
  }
}

pub trait QTreeView_expandToDepth<RetType> {
  fn expandToDepth(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::expandToDepth(int depth);
impl<'a> /*trait*/ QTreeView_expandToDepth<()> for (i32) {
  fn expandToDepth(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView13expandToDepthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView13expandToDepthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QTreeView::indexBelow(const QModelIndex & index);
impl /*struct*/ QTreeView {
  pub fn indexBelow<RetType, T: QTreeView_indexBelow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexBelow(self);
    // return 1;
  }
}

pub trait QTreeView_indexBelow<RetType> {
  fn indexBelow(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  QModelIndex QTreeView::indexBelow(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_indexBelow<QModelIndex> for (&'a QModelIndex) {
  fn indexBelow(self , rsthis: & QTreeView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10indexBelowERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView10indexBelowERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeView::expandAll();
impl /*struct*/ QTreeView {
  pub fn expandAll<RetType, T: QTreeView_expandAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expandAll(self);
    // return 1;
  }
}

pub trait QTreeView_expandAll<RetType> {
  fn expandAll(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::expandAll();
impl<'a> /*trait*/ QTreeView_expandAll<()> for () {
  fn expandAll(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9expandAllEv()};
     unsafe {_ZN9QTreeView9expandAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QTreeView::indexAt(const QPoint & p);
impl /*struct*/ QTreeView {
  pub fn indexAt<RetType, T: QTreeView_indexAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexAt(self);
    // return 1;
  }
}

pub trait QTreeView_indexAt<RetType> {
  fn indexAt(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  QModelIndex QTreeView::indexAt(const QPoint & p);
impl<'a> /*trait*/ QTreeView_indexAt<QModelIndex> for (&'a QPoint) {
  fn indexAt(self , rsthis: & QTreeView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeView::setColumnWidth(int column, int width);
impl /*struct*/ QTreeView {
  pub fn setColumnWidth<RetType, T: QTreeView_setColumnWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnWidth(self);
    // return 1;
  }
}

pub trait QTreeView_setColumnWidth<RetType> {
  fn setColumnWidth(self , rsthis: & QTreeView) -> RetType;
}

  // proto:  void QTreeView::setColumnWidth(int column, int width);
impl<'a> /*trait*/ QTreeView_setColumnWidth<()> for (i32, i32) {
  fn setColumnWidth(self , rsthis: & QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14setColumnWidthEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QTreeView14setColumnWidthEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

#[derive(Default)] // for QTreeView_collapsed
pub struct QTreeView_collapsed_signal{poi:u64}
impl /* struct */ QTreeView {
  pub fn collapsed_1(&self) -> QTreeView_collapsed_signal {
     return QTreeView_collapsed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTreeView_collapsed_signal {
  pub fn connect<T: QTreeView_collapsed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTreeView_collapsed_signal_connect {
  fn connect(self, sigthis: QTreeView_collapsed_signal);
}

#[derive(Default)] // for QTreeView_expanded
pub struct QTreeView_expanded_signal{poi:u64}
impl /* struct */ QTreeView {
  pub fn expanded_1(&self) -> QTreeView_expanded_signal {
     return QTreeView_expanded_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTreeView_expanded_signal {
  pub fn connect<T: QTreeView_expanded_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTreeView_expanded_signal_connect {
  fn connect(self, sigthis: QTreeView_expanded_signal);
}

// collapsed(const class QModelIndex &)
extern fn QTreeView_collapsed_signal_connect_cb_0(rsfptr:fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTreeView_collapsed_signal_connect_cb_box_0(rsfptr_raw:*mut Fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QTreeView_collapsed_signal_connect for fn(QModelIndex) {
  fn connect(self, sigthis: QTreeView_collapsed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTreeView_collapsed_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTreeView_SlotProxy_connect__ZN9QTreeView9collapsedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTreeView_collapsed_signal_connect for Box<Fn(QModelIndex)> {
  fn connect(self, sigthis: QTreeView_collapsed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTreeView_collapsed_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTreeView_SlotProxy_connect__ZN9QTreeView9collapsedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
// expanded(const class QModelIndex &)
extern fn QTreeView_expanded_signal_connect_cb_1(rsfptr:fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTreeView_expanded_signal_connect_cb_box_1(rsfptr_raw:*mut Fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QTreeView_expanded_signal_connect for fn(QModelIndex) {
  fn connect(self, sigthis: QTreeView_expanded_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTreeView_expanded_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTreeView_SlotProxy_connect__ZN9QTreeView8expandedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTreeView_expanded_signal_connect for Box<Fn(QModelIndex)> {
  fn connect(self, sigthis: QTreeView_expanded_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTreeView_expanded_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTreeView_SlotProxy_connect__ZN9QTreeView8expandedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
// <= body block end

