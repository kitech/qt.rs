// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qheaderview::QHeaderView;
use super::qmodelindex::QModelIndex;
use super::qwidget::QWidget;
use super::qrect::QRect;
use super::qstring::QString;
use super::qitemselectionmodel::QItemSelectionModel;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTreeView::setHeader(QHeaderView * header);
  fn _ZN9QTreeView9setHeaderEP11QHeaderView(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTreeView::isAnimated();
  fn _ZNK9QTreeView10isAnimatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTreeView::isExpanded(const QModelIndex & index);
  fn _ZNK9QTreeView10isExpandedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QTreeView::setColumnHidden(int column, bool hide);
  fn _ZN9QTreeView15setColumnHiddenEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QTreeView::setIndentation(int i);
  fn _ZN9QTreeView14setIndentationEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QMetaObject * QTreeView::metaObject();
  fn _ZNK9QTreeView10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTreeView::reset();
  fn _ZN9QTreeView5resetEv(qthis: *mut c_void) ;
  // proto:  void QTreeView::setExpandsOnDoubleClick(bool enable);
  fn _ZN9QTreeView23setExpandsOnDoubleClickEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTreeView::setFirstColumnSpanned(int row, const QModelIndex & parent, bool span);
  fn _ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: int8_t) ;
  // proto:  void QTreeView::sortByColumn(int column);
  fn _ZN9QTreeView12sortByColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTreeView::setRowHidden(int row, const QModelIndex & parent, bool hide);
  fn _ZN9QTreeView12setRowHiddenEiRK11QModelIndexb(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: int8_t) ;
  // proto:  void QTreeView::expand(const QModelIndex & index);
  fn _ZN9QTreeView6expandERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTreeView::autoExpandDelay();
  fn _ZNK9QTreeView15autoExpandDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTreeView::NewQTreeView(QWidget * parent);
  fn _ZN9QTreeViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeView::FreeQTreeView();
  fn _ZN9QTreeViewD0Ev(qthis: *mut c_void) ;
  // proto:  int QTreeView::indentation();
  fn _ZNK9QTreeView11indentationEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTreeView::columnViewportPosition(int column);
  fn _ZNK9QTreeView22columnViewportPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QTreeView::expandsOnDoubleClick();
  fn _ZNK9QTreeView20expandsOnDoubleClickEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTreeView::isSortingEnabled();
  fn _ZNK9QTreeView16isSortingEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTreeView::showColumn(int column);
  fn _ZN9QTreeView10showColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRect QTreeView::visualRect(const QModelIndex & index);
  fn _ZNK9QTreeView10visualRectERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeView::collapse(const QModelIndex & index);
  fn _ZN9QTreeView8collapseERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeView::setWordWrap(bool on);
  fn _ZN9QTreeView11setWordWrapEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QModelIndex QTreeView::indexAbove(const QModelIndex & index);
  fn _ZNK9QTreeView10indexAboveERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTreeView::rootIsDecorated();
  fn _ZNK9QTreeView15rootIsDecoratedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTreeView::collapseAll();
  fn _ZN9QTreeView11collapseAllEv(qthis: *mut c_void) ;
  // proto:  void QTreeView::setHeaderHidden(bool hide);
  fn _ZN9QTreeView15setHeaderHiddenEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QTreeView::allColumnsShowFocus();
  fn _ZNK9QTreeView19allColumnsShowFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTreeView::columnWidth(int column);
  fn _ZNK9QTreeView11columnWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTreeView::resizeColumnToContents(int column);
  fn _ZN9QTreeView22resizeColumnToContentsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTreeView::setAutoExpandDelay(int delay);
  fn _ZN9QTreeView18setAutoExpandDelayEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTreeView::setAllColumnsShowFocus(bool enable);
  fn _ZN9QTreeView22setAllColumnsShowFocusEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QTreeView::isFirstColumnSpanned(int row, const QModelIndex & parent);
  fn _ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> int8_t;
  // proto:  void QTreeView::hideColumn(int column);
  fn _ZN9QTreeView10hideColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTreeView::treePosition();
  fn _ZNK9QTreeView12treePositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTreeView::setExpanded(const QModelIndex & index, bool expand);
  fn _ZN9QTreeView11setExpandedERK11QModelIndexb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QTreeView::resetIndentation();
  fn _ZN9QTreeView16resetIndentationEv(qthis: *mut c_void) ;
  // proto:  bool QTreeView::isRowHidden(int row, const QModelIndex & parent);
  fn _ZNK9QTreeView11isRowHiddenEiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> int8_t;
  // proto:  void QTreeView::collapsed(const QModelIndex & index);
  fn _ZN9QTreeView9collapsedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeView::NewQTreeView(const QTreeView & );
  fn _ZN9QTreeViewC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeView::selectAll();
  fn _ZN9QTreeView9selectAllEv(qthis: *mut c_void) ;
  // proto:  bool QTreeView::wordWrap();
  fn _ZNK9QTreeView8wordWrapEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTreeView::doItemsLayout();
  fn _ZN9QTreeView13doItemsLayoutEv(qthis: *mut c_void) ;
  // proto:  void QTreeView::setTreePosition(int logicalIndex);
  fn _ZN9QTreeView15setTreePositionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTreeView::keyboardSearch(const QString & search);
  fn _ZN9QTreeView14keyboardSearchERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeView::setRootIndex(const QModelIndex & index);
  fn _ZN9QTreeView12setRootIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTreeView::setItemsExpandable(bool enable);
  fn _ZN9QTreeView18setItemsExpandableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTreeView::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN9QTreeView17setSelectionModelEP19QItemSelectionModel(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QHeaderView * QTreeView::header();
  fn _ZNK9QTreeView6headerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeView::setAnimated(bool enable);
  fn _ZN9QTreeView11setAnimatedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTreeView::setSortingEnabled(bool enable);
  fn _ZN9QTreeView17setSortingEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QTreeView::itemsExpandable();
  fn _ZNK9QTreeView15itemsExpandableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTreeView::setRootIsDecorated(bool show);
  fn _ZN9QTreeView18setRootIsDecoratedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTreeView::expanded(const QModelIndex & index);
  fn _ZN9QTreeView8expandedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTreeView::isHeaderHidden();
  fn _ZNK9QTreeView14isHeaderHiddenEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTreeView::columnAt(int x);
  fn _ZNK9QTreeView8columnAtEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  bool QTreeView::isColumnHidden(int column);
  fn _ZNK9QTreeView14isColumnHiddenEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  bool QTreeView::uniformRowHeights();
  fn _ZNK9QTreeView17uniformRowHeightsEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTreeView::setUniformRowHeights(bool uniform);
  fn _ZN9QTreeView20setUniformRowHeightsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTreeView::expandToDepth(int depth);
  fn _ZN9QTreeView13expandToDepthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QModelIndex QTreeView::indexBelow(const QModelIndex & index);
  fn _ZNK9QTreeView10indexBelowERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeView::expandAll();
  fn _ZN9QTreeView9expandAllEv(qthis: *mut c_void) ;
  // proto:  QModelIndex QTreeView::indexAt(const QPoint & p);
  fn _ZNK9QTreeView7indexAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeView::setColumnWidth(int column, int width);
  fn _ZN9QTreeView14setColumnWidthEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
}

// body block begin
// class sizeof(QTreeView)=1
pub struct QTreeView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTreeView {
  pub fn setHeader<RetType, T: QTreeView_setHeader<RetType>>(&mut self, value: T) -> RetType {
    return value.setHeader(self);
    // return 1;
  }
}

pub trait QTreeView_setHeader<RetType> {
  fn setHeader(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setHeader(QHeaderView * header);
impl<'a> /*trait*/ QTreeView_setHeader<()> for (&'a mut QHeaderView) {
  fn setHeader(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9setHeaderEP11QHeaderView()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView9setHeaderEP11QHeaderView(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isAnimated<RetType, T: QTreeView_isAnimated<RetType>>(&mut self, value: T) -> RetType {
    return value.isAnimated(self);
    // return 1;
  }
}

pub trait QTreeView_isAnimated<RetType> {
  fn isAnimated(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::isAnimated();
impl<'a> /*trait*/ QTreeView_isAnimated<i8> for () {
  fn isAnimated(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10isAnimatedEv()};
    let mut ret = unsafe {_ZNK9QTreeView10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isExpanded<RetType, T: QTreeView_isExpanded<RetType>>(&mut self, value: T) -> RetType {
    return value.isExpanded(self);
    // return 1;
  }
}

pub trait QTreeView_isExpanded<RetType> {
  fn isExpanded(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::isExpanded(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_isExpanded<i8> for (&'a  QModelIndex) {
  fn isExpanded(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10isExpandedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView10isExpandedERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setColumnHidden<RetType, T: QTreeView_setColumnHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.setColumnHidden(self);
    // return 1;
  }
}

pub trait QTreeView_setColumnHidden<RetType> {
  fn setColumnHidden(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setColumnHidden(int column, bool hide);
impl<'a> /*trait*/ QTreeView_setColumnHidden<()> for (i32, i8) {
  fn setColumnHidden(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setColumnHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN9QTreeView15setColumnHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setIndentation<RetType, T: QTreeView_setIndentation<RetType>>(&mut self, value: T) -> RetType {
    return value.setIndentation(self);
    // return 1;
  }
}

pub trait QTreeView_setIndentation<RetType> {
  fn setIndentation(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setIndentation(int i);
impl<'a> /*trait*/ QTreeView_setIndentation<()> for (i32) {
  fn setIndentation(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14setIndentationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView14setIndentationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn metaObject<RetType, T: QTreeView_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QTreeView_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  const QMetaObject * QTreeView::metaObject();
impl<'a> /*trait*/ QTreeView_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10metaObjectEv()};
     unsafe {_ZNK9QTreeView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn reset<RetType, T: QTreeView_reset<RetType>>(&mut self, value: T) -> RetType {
    return value.reset(self);
    // return 1;
  }
}

pub trait QTreeView_reset<RetType> {
  fn reset(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::reset();
impl<'a> /*trait*/ QTreeView_reset<()> for () {
  fn reset(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView5resetEv()};
     unsafe {_ZN9QTreeView5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setExpandsOnDoubleClick<RetType, T: QTreeView_setExpandsOnDoubleClick<RetType>>(&mut self, value: T) -> RetType {
    return value.setExpandsOnDoubleClick(self);
    // return 1;
  }
}

pub trait QTreeView_setExpandsOnDoubleClick<RetType> {
  fn setExpandsOnDoubleClick(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setExpandsOnDoubleClick(bool enable);
impl<'a> /*trait*/ QTreeView_setExpandsOnDoubleClick<()> for (i8) {
  fn setExpandsOnDoubleClick(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView23setExpandsOnDoubleClickEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView23setExpandsOnDoubleClickEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setFirstColumnSpanned<RetType, T: QTreeView_setFirstColumnSpanned<RetType>>(&mut self, value: T) -> RetType {
    return value.setFirstColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeView_setFirstColumnSpanned<RetType> {
  fn setFirstColumnSpanned(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setFirstColumnSpanned(int row, const QModelIndex & parent, bool span);
impl<'a> /*trait*/ QTreeView_setFirstColumnSpanned<()> for (i32, &'a  QModelIndex, i8) {
  fn setFirstColumnSpanned(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as int8_t;
     unsafe {_ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn sortByColumn<RetType, T: QTreeView_sortByColumn<RetType>>(&mut self, value: T) -> RetType {
    return value.sortByColumn(self);
    // return 1;
  }
}

pub trait QTreeView_sortByColumn<RetType> {
  fn sortByColumn(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::sortByColumn(int column);
impl<'a> /*trait*/ QTreeView_sortByColumn<()> for (i32) {
  fn sortByColumn(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12sortByColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView12sortByColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setRowHidden<RetType, T: QTreeView_setRowHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.setRowHidden(self);
    // return 1;
  }
}

pub trait QTreeView_setRowHidden<RetType> {
  fn setRowHidden(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setRowHidden(int row, const QModelIndex & parent, bool hide);
impl<'a> /*trait*/ QTreeView_setRowHidden<()> for (i32, &'a  QModelIndex, i8) {
  fn setRowHidden(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12setRowHiddenEiRK11QModelIndexb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as int8_t;
     unsafe {_ZN9QTreeView12setRowHiddenEiRK11QModelIndexb(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expand<RetType, T: QTreeView_expand<RetType>>(&mut self, value: T) -> RetType {
    return value.expand(self);
    // return 1;
  }
}

pub trait QTreeView_expand<RetType> {
  fn expand(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::expand(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_expand<()> for (&'a  QModelIndex) {
  fn expand(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView6expandERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView6expandERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn autoExpandDelay<RetType, T: QTreeView_autoExpandDelay<RetType>>(&mut self, value: T) -> RetType {
    return value.autoExpandDelay(self);
    // return 1;
  }
}

pub trait QTreeView_autoExpandDelay<RetType> {
  fn autoExpandDelay(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  int QTreeView::autoExpandDelay();
impl<'a> /*trait*/ QTreeView_autoExpandDelay<i32> for () {
  fn autoExpandDelay(self, rsthis: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15autoExpandDelayEv()};
    let mut ret = unsafe {_ZNK9QTreeView15autoExpandDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn NewQTreeView<T: QTreeView_NewQTreeView>(value: T) -> QTreeView {
    let rsthis = value.NewQTreeView();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeView_NewQTreeView {
  fn NewQTreeView(self) -> QTreeView;
}

// proto: void QTreeView::NewQTreeView(QWidget * parent);
impl<'a> /*trait*/ QTreeView_NewQTreeView for (&'a mut QWidget) {
  fn NewQTreeView(self) -> QTreeView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeViewC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTreeViewC1EP7QWidget(qthis, arg0)};
    let rsthis = QTreeView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn FreeQTreeView<RetType, T: QTreeView_FreeQTreeView<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTreeView(self);
    // return 1;
  }
}

pub trait QTreeView_FreeQTreeView<RetType> {
  fn FreeQTreeView(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::FreeQTreeView();
impl<'a> /*trait*/ QTreeView_FreeQTreeView<()> for () {
  fn FreeQTreeView(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeViewD0Ev()};
     unsafe {_ZN9QTreeViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn indentation<RetType, T: QTreeView_indentation<RetType>>(&mut self, value: T) -> RetType {
    return value.indentation(self);
    // return 1;
  }
}

pub trait QTreeView_indentation<RetType> {
  fn indentation(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  int QTreeView::indentation();
impl<'a> /*trait*/ QTreeView_indentation<i32> for () {
  fn indentation(self, rsthis: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11indentationEv()};
    let mut ret = unsafe {_ZNK9QTreeView11indentationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn columnViewportPosition<RetType, T: QTreeView_columnViewportPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.columnViewportPosition(self);
    // return 1;
  }
}

pub trait QTreeView_columnViewportPosition<RetType> {
  fn columnViewportPosition(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  int QTreeView::columnViewportPosition(int column);
impl<'a> /*trait*/ QTreeView_columnViewportPosition<i32> for (i32) {
  fn columnViewportPosition(self, rsthis: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView22columnViewportPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTreeView22columnViewportPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expandsOnDoubleClick<RetType, T: QTreeView_expandsOnDoubleClick<RetType>>(&mut self, value: T) -> RetType {
    return value.expandsOnDoubleClick(self);
    // return 1;
  }
}

pub trait QTreeView_expandsOnDoubleClick<RetType> {
  fn expandsOnDoubleClick(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::expandsOnDoubleClick();
impl<'a> /*trait*/ QTreeView_expandsOnDoubleClick<i8> for () {
  fn expandsOnDoubleClick(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView20expandsOnDoubleClickEv()};
    let mut ret = unsafe {_ZNK9QTreeView20expandsOnDoubleClickEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isSortingEnabled<RetType, T: QTreeView_isSortingEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QTreeView_isSortingEnabled<RetType> {
  fn isSortingEnabled(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::isSortingEnabled();
impl<'a> /*trait*/ QTreeView_isSortingEnabled<i8> for () {
  fn isSortingEnabled(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView16isSortingEnabledEv()};
    let mut ret = unsafe {_ZNK9QTreeView16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn showColumn<RetType, T: QTreeView_showColumn<RetType>>(&mut self, value: T) -> RetType {
    return value.showColumn(self);
    // return 1;
  }
}

pub trait QTreeView_showColumn<RetType> {
  fn showColumn(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::showColumn(int column);
impl<'a> /*trait*/ QTreeView_showColumn<()> for (i32) {
  fn showColumn(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView10showColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView10showColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn visualRect<RetType, T: QTreeView_visualRect<RetType>>(&mut self, value: T) -> RetType {
    return value.visualRect(self);
    // return 1;
  }
}

pub trait QTreeView_visualRect<RetType> {
  fn visualRect(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  QRect QTreeView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_visualRect<QRect> for (&'a  QModelIndex) {
  fn visualRect(self, rsthis: &mut QTreeView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn collapse<RetType, T: QTreeView_collapse<RetType>>(&mut self, value: T) -> RetType {
    return value.collapse(self);
    // return 1;
  }
}

pub trait QTreeView_collapse<RetType> {
  fn collapse(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::collapse(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_collapse<()> for (&'a  QModelIndex) {
  fn collapse(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView8collapseERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView8collapseERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setWordWrap<RetType, T: QTreeView_setWordWrap<RetType>>(&mut self, value: T) -> RetType {
    return value.setWordWrap(self);
    // return 1;
  }
}

pub trait QTreeView_setWordWrap<RetType> {
  fn setWordWrap(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setWordWrap(bool on);
impl<'a> /*trait*/ QTreeView_setWordWrap<()> for (i8) {
  fn setWordWrap(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setWordWrapEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView11setWordWrapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn indexAbove<RetType, T: QTreeView_indexAbove<RetType>>(&mut self, value: T) -> RetType {
    return value.indexAbove(self);
    // return 1;
  }
}

pub trait QTreeView_indexAbove<RetType> {
  fn indexAbove(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  QModelIndex QTreeView::indexAbove(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_indexAbove<QModelIndex> for (&'a  QModelIndex) {
  fn indexAbove(self, rsthis: &mut QTreeView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10indexAboveERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView10indexAboveERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn rootIsDecorated<RetType, T: QTreeView_rootIsDecorated<RetType>>(&mut self, value: T) -> RetType {
    return value.rootIsDecorated(self);
    // return 1;
  }
}

pub trait QTreeView_rootIsDecorated<RetType> {
  fn rootIsDecorated(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::rootIsDecorated();
impl<'a> /*trait*/ QTreeView_rootIsDecorated<i8> for () {
  fn rootIsDecorated(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15rootIsDecoratedEv()};
    let mut ret = unsafe {_ZNK9QTreeView15rootIsDecoratedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn collapseAll<RetType, T: QTreeView_collapseAll<RetType>>(&mut self, value: T) -> RetType {
    return value.collapseAll(self);
    // return 1;
  }
}

pub trait QTreeView_collapseAll<RetType> {
  fn collapseAll(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::collapseAll();
impl<'a> /*trait*/ QTreeView_collapseAll<()> for () {
  fn collapseAll(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11collapseAllEv()};
     unsafe {_ZN9QTreeView11collapseAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setHeaderHidden<RetType, T: QTreeView_setHeaderHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.setHeaderHidden(self);
    // return 1;
  }
}

pub trait QTreeView_setHeaderHidden<RetType> {
  fn setHeaderHidden(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setHeaderHidden(bool hide);
impl<'a> /*trait*/ QTreeView_setHeaderHidden<()> for (i8) {
  fn setHeaderHidden(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setHeaderHiddenEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView15setHeaderHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn allColumnsShowFocus<RetType, T: QTreeView_allColumnsShowFocus<RetType>>(&mut self, value: T) -> RetType {
    return value.allColumnsShowFocus(self);
    // return 1;
  }
}

pub trait QTreeView_allColumnsShowFocus<RetType> {
  fn allColumnsShowFocus(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::allColumnsShowFocus();
impl<'a> /*trait*/ QTreeView_allColumnsShowFocus<i8> for () {
  fn allColumnsShowFocus(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView19allColumnsShowFocusEv()};
    let mut ret = unsafe {_ZNK9QTreeView19allColumnsShowFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn columnWidth<RetType, T: QTreeView_columnWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.columnWidth(self);
    // return 1;
  }
}

pub trait QTreeView_columnWidth<RetType> {
  fn columnWidth(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  int QTreeView::columnWidth(int column);
impl<'a> /*trait*/ QTreeView_columnWidth<i32> for (i32) {
  fn columnWidth(self, rsthis: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11columnWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTreeView11columnWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn resizeColumnToContents<RetType, T: QTreeView_resizeColumnToContents<RetType>>(&mut self, value: T) -> RetType {
    return value.resizeColumnToContents(self);
    // return 1;
  }
}

pub trait QTreeView_resizeColumnToContents<RetType> {
  fn resizeColumnToContents(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::resizeColumnToContents(int column);
impl<'a> /*trait*/ QTreeView_resizeColumnToContents<()> for (i32) {
  fn resizeColumnToContents(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView22resizeColumnToContentsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView22resizeColumnToContentsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setAutoExpandDelay<RetType, T: QTreeView_setAutoExpandDelay<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoExpandDelay(self);
    // return 1;
  }
}

pub trait QTreeView_setAutoExpandDelay<RetType> {
  fn setAutoExpandDelay(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setAutoExpandDelay(int delay);
impl<'a> /*trait*/ QTreeView_setAutoExpandDelay<()> for (i32) {
  fn setAutoExpandDelay(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setAutoExpandDelayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView18setAutoExpandDelayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setAllColumnsShowFocus<RetType, T: QTreeView_setAllColumnsShowFocus<RetType>>(&mut self, value: T) -> RetType {
    return value.setAllColumnsShowFocus(self);
    // return 1;
  }
}

pub trait QTreeView_setAllColumnsShowFocus<RetType> {
  fn setAllColumnsShowFocus(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setAllColumnsShowFocus(bool enable);
impl<'a> /*trait*/ QTreeView_setAllColumnsShowFocus<()> for (i8) {
  fn setAllColumnsShowFocus(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView22setAllColumnsShowFocusEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView22setAllColumnsShowFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isFirstColumnSpanned<RetType, T: QTreeView_isFirstColumnSpanned<RetType>>(&mut self, value: T) -> RetType {
    return value.isFirstColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeView_isFirstColumnSpanned<RetType> {
  fn isFirstColumnSpanned(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::isFirstColumnSpanned(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QTreeView_isFirstColumnSpanned<i8> for (i32, &'a  QModelIndex) {
  fn isFirstColumnSpanned(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn hideColumn<RetType, T: QTreeView_hideColumn<RetType>>(&mut self, value: T) -> RetType {
    return value.hideColumn(self);
    // return 1;
  }
}

pub trait QTreeView_hideColumn<RetType> {
  fn hideColumn(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::hideColumn(int column);
impl<'a> /*trait*/ QTreeView_hideColumn<()> for (i32) {
  fn hideColumn(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView10hideColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView10hideColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn treePosition<RetType, T: QTreeView_treePosition<RetType>>(&mut self, value: T) -> RetType {
    return value.treePosition(self);
    // return 1;
  }
}

pub trait QTreeView_treePosition<RetType> {
  fn treePosition(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  int QTreeView::treePosition();
impl<'a> /*trait*/ QTreeView_treePosition<i32> for () {
  fn treePosition(self, rsthis: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView12treePositionEv()};
    let mut ret = unsafe {_ZNK9QTreeView12treePositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setExpanded<RetType, T: QTreeView_setExpanded<RetType>>(&mut self, value: T) -> RetType {
    return value.setExpanded(self);
    // return 1;
  }
}

pub trait QTreeView_setExpanded<RetType> {
  fn setExpanded(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setExpanded(const QModelIndex & index, bool expand);
impl<'a> /*trait*/ QTreeView_setExpanded<()> for (&'a  QModelIndex, i8) {
  fn setExpanded(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setExpandedERK11QModelIndexb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN9QTreeView11setExpandedERK11QModelIndexb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn resetIndentation<RetType, T: QTreeView_resetIndentation<RetType>>(&mut self, value: T) -> RetType {
    return value.resetIndentation(self);
    // return 1;
  }
}

pub trait QTreeView_resetIndentation<RetType> {
  fn resetIndentation(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::resetIndentation();
impl<'a> /*trait*/ QTreeView_resetIndentation<()> for () {
  fn resetIndentation(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView16resetIndentationEv()};
     unsafe {_ZN9QTreeView16resetIndentationEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isRowHidden<RetType, T: QTreeView_isRowHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.isRowHidden(self);
    // return 1;
  }
}

pub trait QTreeView_isRowHidden<RetType> {
  fn isRowHidden(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::isRowHidden(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QTreeView_isRowHidden<i8> for (i32, &'a  QModelIndex) {
  fn isRowHidden(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11isRowHiddenEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView11isRowHiddenEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn collapsed<RetType, T: QTreeView_collapsed<RetType>>(&mut self, value: T) -> RetType {
    return value.collapsed(self);
    // return 1;
  }
}

pub trait QTreeView_collapsed<RetType> {
  fn collapsed(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::collapsed(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_collapsed<()> for (&'a  QModelIndex) {
  fn collapsed(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9collapsedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView9collapsedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QTreeView::NewQTreeView(const QTreeView & );
impl<'a> /*trait*/ QTreeView_NewQTreeView for (&'a  QTreeView) {
  fn NewQTreeView(self) -> QTreeView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeViewC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTreeViewC1ERKS_(qthis, arg0)};
    let rsthis = QTreeView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn selectAll<RetType, T: QTreeView_selectAll<RetType>>(&mut self, value: T) -> RetType {
    return value.selectAll(self);
    // return 1;
  }
}

pub trait QTreeView_selectAll<RetType> {
  fn selectAll(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::selectAll();
impl<'a> /*trait*/ QTreeView_selectAll<()> for () {
  fn selectAll(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9selectAllEv()};
     unsafe {_ZN9QTreeView9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn wordWrap<RetType, T: QTreeView_wordWrap<RetType>>(&mut self, value: T) -> RetType {
    return value.wordWrap(self);
    // return 1;
  }
}

pub trait QTreeView_wordWrap<RetType> {
  fn wordWrap(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::wordWrap();
impl<'a> /*trait*/ QTreeView_wordWrap<i8> for () {
  fn wordWrap(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView8wordWrapEv()};
    let mut ret = unsafe {_ZNK9QTreeView8wordWrapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn doItemsLayout<RetType, T: QTreeView_doItemsLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.doItemsLayout(self);
    // return 1;
  }
}

pub trait QTreeView_doItemsLayout<RetType> {
  fn doItemsLayout(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::doItemsLayout();
impl<'a> /*trait*/ QTreeView_doItemsLayout<()> for () {
  fn doItemsLayout(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView13doItemsLayoutEv()};
     unsafe {_ZN9QTreeView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setTreePosition<RetType, T: QTreeView_setTreePosition<RetType>>(&mut self, value: T) -> RetType {
    return value.setTreePosition(self);
    // return 1;
  }
}

pub trait QTreeView_setTreePosition<RetType> {
  fn setTreePosition(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setTreePosition(int logicalIndex);
impl<'a> /*trait*/ QTreeView_setTreePosition<()> for (i32) {
  fn setTreePosition(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setTreePositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView15setTreePositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn keyboardSearch<RetType, T: QTreeView_keyboardSearch<RetType>>(&mut self, value: T) -> RetType {
    return value.keyboardSearch(self);
    // return 1;
  }
}

pub trait QTreeView_keyboardSearch<RetType> {
  fn keyboardSearch(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::keyboardSearch(const QString & search);
impl<'a> /*trait*/ QTreeView_keyboardSearch<()> for (&'a  QString) {
  fn keyboardSearch(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14keyboardSearchERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView14keyboardSearchERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setRootIndex<RetType, T: QTreeView_setRootIndex<RetType>>(&mut self, value: T) -> RetType {
    return value.setRootIndex(self);
    // return 1;
  }
}

pub trait QTreeView_setRootIndex<RetType> {
  fn setRootIndex(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_setRootIndex<()> for (&'a  QModelIndex) {
  fn setRootIndex(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setItemsExpandable<RetType, T: QTreeView_setItemsExpandable<RetType>>(&mut self, value: T) -> RetType {
    return value.setItemsExpandable(self);
    // return 1;
  }
}

pub trait QTreeView_setItemsExpandable<RetType> {
  fn setItemsExpandable(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setItemsExpandable(bool enable);
impl<'a> /*trait*/ QTreeView_setItemsExpandable<()> for (i8) {
  fn setItemsExpandable(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setItemsExpandableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView18setItemsExpandableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setSelectionModel<RetType, T: QTreeView_setSelectionModel<RetType>>(&mut self, value: T) -> RetType {
    return value.setSelectionModel(self);
    // return 1;
  }
}

pub trait QTreeView_setSelectionModel<RetType> {
  fn setSelectionModel(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QTreeView_setSelectionModel<()> for (&'a mut QItemSelectionModel) {
  fn setSelectionModel(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn header<RetType, T: QTreeView_header<RetType>>(&mut self, value: T) -> RetType {
    return value.header(self);
    // return 1;
  }
}

pub trait QTreeView_header<RetType> {
  fn header(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  QHeaderView * QTreeView::header();
impl<'a> /*trait*/ QTreeView_header<QHeaderView> for () {
  fn header(self, rsthis: &mut QTreeView) -> QHeaderView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView6headerEv()};
    let mut ret = unsafe {_ZNK9QTreeView6headerEv(rsthis.qclsinst)};
    let mut ret1 = QHeaderView{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setAnimated<RetType, T: QTreeView_setAnimated<RetType>>(&mut self, value: T) -> RetType {
    return value.setAnimated(self);
    // return 1;
  }
}

pub trait QTreeView_setAnimated<RetType> {
  fn setAnimated(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setAnimated(bool enable);
impl<'a> /*trait*/ QTreeView_setAnimated<()> for (i8) {
  fn setAnimated(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setAnimatedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setSortingEnabled<RetType, T: QTreeView_setSortingEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QTreeView_setSortingEnabled<RetType> {
  fn setSortingEnabled(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QTreeView_setSortingEnabled<()> for (i8) {
  fn setSortingEnabled(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView17setSortingEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn itemsExpandable<RetType, T: QTreeView_itemsExpandable<RetType>>(&mut self, value: T) -> RetType {
    return value.itemsExpandable(self);
    // return 1;
  }
}

pub trait QTreeView_itemsExpandable<RetType> {
  fn itemsExpandable(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::itemsExpandable();
impl<'a> /*trait*/ QTreeView_itemsExpandable<i8> for () {
  fn itemsExpandable(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15itemsExpandableEv()};
    let mut ret = unsafe {_ZNK9QTreeView15itemsExpandableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setRootIsDecorated<RetType, T: QTreeView_setRootIsDecorated<RetType>>(&mut self, value: T) -> RetType {
    return value.setRootIsDecorated(self);
    // return 1;
  }
}

pub trait QTreeView_setRootIsDecorated<RetType> {
  fn setRootIsDecorated(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setRootIsDecorated(bool show);
impl<'a> /*trait*/ QTreeView_setRootIsDecorated<()> for (i8) {
  fn setRootIsDecorated(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setRootIsDecoratedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView18setRootIsDecoratedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expanded<RetType, T: QTreeView_expanded<RetType>>(&mut self, value: T) -> RetType {
    return value.expanded(self);
    // return 1;
  }
}

pub trait QTreeView_expanded<RetType> {
  fn expanded(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::expanded(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_expanded<()> for (&'a  QModelIndex) {
  fn expanded(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView8expandedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTreeView8expandedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isHeaderHidden<RetType, T: QTreeView_isHeaderHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.isHeaderHidden(self);
    // return 1;
  }
}

pub trait QTreeView_isHeaderHidden<RetType> {
  fn isHeaderHidden(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::isHeaderHidden();
impl<'a> /*trait*/ QTreeView_isHeaderHidden<i8> for () {
  fn isHeaderHidden(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView14isHeaderHiddenEv()};
    let mut ret = unsafe {_ZNK9QTreeView14isHeaderHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn columnAt<RetType, T: QTreeView_columnAt<RetType>>(&mut self, value: T) -> RetType {
    return value.columnAt(self);
    // return 1;
  }
}

pub trait QTreeView_columnAt<RetType> {
  fn columnAt(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  int QTreeView::columnAt(int x);
impl<'a> /*trait*/ QTreeView_columnAt<i32> for (i32) {
  fn columnAt(self, rsthis: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView8columnAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTreeView8columnAtEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isColumnHidden<RetType, T: QTreeView_isColumnHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.isColumnHidden(self);
    // return 1;
  }
}

pub trait QTreeView_isColumnHidden<RetType> {
  fn isColumnHidden(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::isColumnHidden(int column);
impl<'a> /*trait*/ QTreeView_isColumnHidden<i8> for (i32) {
  fn isColumnHidden(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView14isColumnHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTreeView14isColumnHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn uniformRowHeights<RetType, T: QTreeView_uniformRowHeights<RetType>>(&mut self, value: T) -> RetType {
    return value.uniformRowHeights(self);
    // return 1;
  }
}

pub trait QTreeView_uniformRowHeights<RetType> {
  fn uniformRowHeights(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  bool QTreeView::uniformRowHeights();
impl<'a> /*trait*/ QTreeView_uniformRowHeights<i8> for () {
  fn uniformRowHeights(self, rsthis: &mut QTreeView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView17uniformRowHeightsEv()};
    let mut ret = unsafe {_ZNK9QTreeView17uniformRowHeightsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setUniformRowHeights<RetType, T: QTreeView_setUniformRowHeights<RetType>>(&mut self, value: T) -> RetType {
    return value.setUniformRowHeights(self);
    // return 1;
  }
}

pub trait QTreeView_setUniformRowHeights<RetType> {
  fn setUniformRowHeights(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setUniformRowHeights(bool uniform);
impl<'a> /*trait*/ QTreeView_setUniformRowHeights<()> for (i8) {
  fn setUniformRowHeights(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView20setUniformRowHeightsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTreeView20setUniformRowHeightsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expandToDepth<RetType, T: QTreeView_expandToDepth<RetType>>(&mut self, value: T) -> RetType {
    return value.expandToDepth(self);
    // return 1;
  }
}

pub trait QTreeView_expandToDepth<RetType> {
  fn expandToDepth(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::expandToDepth(int depth);
impl<'a> /*trait*/ QTreeView_expandToDepth<()> for (i32) {
  fn expandToDepth(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView13expandToDepthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTreeView13expandToDepthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn indexBelow<RetType, T: QTreeView_indexBelow<RetType>>(&mut self, value: T) -> RetType {
    return value.indexBelow(self);
    // return 1;
  }
}

pub trait QTreeView_indexBelow<RetType> {
  fn indexBelow(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  QModelIndex QTreeView::indexBelow(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_indexBelow<QModelIndex> for (&'a  QModelIndex) {
  fn indexBelow(self, rsthis: &mut QTreeView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10indexBelowERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView10indexBelowERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expandAll<RetType, T: QTreeView_expandAll<RetType>>(&mut self, value: T) -> RetType {
    return value.expandAll(self);
    // return 1;
  }
}

pub trait QTreeView_expandAll<RetType> {
  fn expandAll(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::expandAll();
impl<'a> /*trait*/ QTreeView_expandAll<()> for () {
  fn expandAll(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9expandAllEv()};
     unsafe {_ZN9QTreeView9expandAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn indexAt<RetType, T: QTreeView_indexAt<RetType>>(&mut self, value: T) -> RetType {
    return value.indexAt(self);
    // return 1;
  }
}

pub trait QTreeView_indexAt<RetType> {
  fn indexAt(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  QModelIndex QTreeView::indexAt(const QPoint & p);
impl<'a> /*trait*/ QTreeView_indexAt<QModelIndex> for (&'a  QPoint) {
  fn indexAt(self, rsthis: &mut QTreeView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QTreeView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setColumnWidth<RetType, T: QTreeView_setColumnWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setColumnWidth(self);
    // return 1;
  }
}

pub trait QTreeView_setColumnWidth<RetType> {
  fn setColumnWidth(self, rsthis: &mut QTreeView) -> RetType;
}

// proto:  void QTreeView::setColumnWidth(int column, int width);
impl<'a> /*trait*/ QTreeView_setColumnWidth<()> for (i32, i32) {
  fn setColumnWidth(self, rsthis: &mut QTreeView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14setColumnWidthEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QTreeView14setColumnWidthEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

