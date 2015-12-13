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
use super::qstring::QString;
use super::qitemselectionmodel::QItemSelectionModel;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTreeView::setHeader(QHeaderView * header);
  fn _ZN9QTreeView9setHeaderEP11QHeaderView(arg0: *mut c_void) -> i32;
  // proto: bool QTreeView::isAnimated();
  fn _ZNK9QTreeView10isAnimatedEv() -> i32;
  // proto: bool QTreeView::isExpanded(const QModelIndex & index);
  fn _ZNK9QTreeView10isExpandedERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QTreeView::setColumnHidden(int column, bool hide);
  fn _ZN9QTreeView15setColumnHiddenEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QTreeView::setIndentation(int i);
  fn _ZN9QTreeView14setIndentationEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QTreeView::metaObject();
  fn _ZNK9QTreeView10metaObjectEv() -> i32;
  // proto: void QTreeView::reset();
  fn _ZN9QTreeView5resetEv() -> i32;
  // proto: void QTreeView::setExpandsOnDoubleClick(bool enable);
  fn _ZN9QTreeView23setExpandsOnDoubleClickEb(arg0: int8_t) -> i32;
  // proto: void QTreeView::setFirstColumnSpanned(int row, const QModelIndex & parent, bool span);
  fn _ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb(arg0: c_int, arg1: *const c_void, arg2: int8_t) -> i32;
  // proto: void QTreeView::sortByColumn(int column);
  fn _ZN9QTreeView12sortByColumnEi(arg0: c_int) -> i32;
  // proto: void QTreeView::setRowHidden(int row, const QModelIndex & parent, bool hide);
  fn _ZN9QTreeView12setRowHiddenEiRK11QModelIndexb(arg0: c_int, arg1: *const c_void, arg2: int8_t) -> i32;
  // proto: void QTreeView::expand(const QModelIndex & index);
  fn _ZN9QTreeView6expandERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: int QTreeView::autoExpandDelay();
  fn _ZNK9QTreeView15autoExpandDelayEv() -> i32;
  // proto: void QTreeView::NewQTreeView(QWidget * parent);
  fn _ZN9QTreeViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QTreeView::FreeQTreeView();
  fn _ZN9QTreeViewD0Ev() -> i32;
  // proto: int QTreeView::indentation();
  fn _ZNK9QTreeView11indentationEv() -> i32;
  // proto: int QTreeView::columnViewportPosition(int column);
  fn _ZNK9QTreeView22columnViewportPositionEi(arg0: c_int) -> i32;
  // proto: bool QTreeView::expandsOnDoubleClick();
  fn _ZNK9QTreeView20expandsOnDoubleClickEv() -> i32;
  // proto: bool QTreeView::isSortingEnabled();
  fn _ZNK9QTreeView16isSortingEnabledEv() -> i32;
  // proto: void QTreeView::showColumn(int column);
  fn _ZN9QTreeView10showColumnEi(arg0: c_int) -> i32;
  // proto: QRect QTreeView::visualRect(const QModelIndex & index);
  fn _ZNK9QTreeView10visualRectERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QTreeView::collapse(const QModelIndex & index);
  fn _ZN9QTreeView8collapseERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QTreeView::setWordWrap(bool on);
  fn _ZN9QTreeView11setWordWrapEb(arg0: int8_t) -> i32;
  // proto: QModelIndex QTreeView::indexAbove(const QModelIndex & index);
  fn _ZNK9QTreeView10indexAboveERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QTreeView::rootIsDecorated();
  fn _ZNK9QTreeView15rootIsDecoratedEv() -> i32;
  // proto: void QTreeView::collapseAll();
  fn _ZN9QTreeView11collapseAllEv() -> i32;
  // proto: void QTreeView::setHeaderHidden(bool hide);
  fn _ZN9QTreeView15setHeaderHiddenEb(arg0: int8_t) -> i32;
  // proto: bool QTreeView::allColumnsShowFocus();
  fn _ZNK9QTreeView19allColumnsShowFocusEv() -> i32;
  // proto: int QTreeView::columnWidth(int column);
  fn _ZNK9QTreeView11columnWidthEi(arg0: c_int) -> i32;
  // proto: void QTreeView::resizeColumnToContents(int column);
  fn _ZN9QTreeView22resizeColumnToContentsEi(arg0: c_int) -> i32;
  // proto: void QTreeView::setAutoExpandDelay(int delay);
  fn _ZN9QTreeView18setAutoExpandDelayEi(arg0: c_int) -> i32;
  // proto: void QTreeView::setAllColumnsShowFocus(bool enable);
  fn _ZN9QTreeView22setAllColumnsShowFocusEb(arg0: int8_t) -> i32;
  // proto: bool QTreeView::isFirstColumnSpanned(int row, const QModelIndex & parent);
  fn _ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeView::hideColumn(int column);
  fn _ZN9QTreeView10hideColumnEi(arg0: c_int) -> i32;
  // proto: int QTreeView::treePosition();
  fn _ZNK9QTreeView12treePositionEv() -> i32;
  // proto: void QTreeView::setExpanded(const QModelIndex & index, bool expand);
  fn _ZN9QTreeView11setExpandedERK11QModelIndexb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: void QTreeView::resetIndentation();
  fn _ZN9QTreeView16resetIndentationEv() -> i32;
  // proto: bool QTreeView::isRowHidden(int row, const QModelIndex & parent);
  fn _ZNK9QTreeView11isRowHiddenEiRK11QModelIndex(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTreeView::collapsed(const QModelIndex & index);
  fn _ZN9QTreeView9collapsedERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QTreeView::NewQTreeView(const QTreeView & );
  fn _ZN9QTreeViewC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTreeView::selectAll();
  fn _ZN9QTreeView9selectAllEv() -> i32;
  // proto: bool QTreeView::wordWrap();
  fn _ZNK9QTreeView8wordWrapEv() -> i32;
  // proto: void QTreeView::doItemsLayout();
  fn _ZN9QTreeView13doItemsLayoutEv() -> i32;
  // proto: void QTreeView::setTreePosition(int logicalIndex);
  fn _ZN9QTreeView15setTreePositionEi(arg0: c_int) -> i32;
  // proto: void QTreeView::keyboardSearch(const QString & search);
  fn _ZN9QTreeView14keyboardSearchERK7QString(arg0: *const c_void) -> i32;
  // proto: void QTreeView::setRootIndex(const QModelIndex & index);
  fn _ZN9QTreeView12setRootIndexERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QTreeView::setItemsExpandable(bool enable);
  fn _ZN9QTreeView18setItemsExpandableEb(arg0: int8_t) -> i32;
  // proto: void QTreeView::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN9QTreeView17setSelectionModelEP19QItemSelectionModel(arg0: *mut c_void) -> i32;
  // proto: QHeaderView * QTreeView::header();
  fn _ZNK9QTreeView6headerEv() -> i32;
  // proto: void QTreeView::setAnimated(bool enable);
  fn _ZN9QTreeView11setAnimatedEb(arg0: int8_t) -> i32;
  // proto: void QTreeView::setSortingEnabled(bool enable);
  fn _ZN9QTreeView17setSortingEnabledEb(arg0: int8_t) -> i32;
  // proto: bool QTreeView::itemsExpandable();
  fn _ZNK9QTreeView15itemsExpandableEv() -> i32;
  // proto: void QTreeView::setRootIsDecorated(bool show);
  fn _ZN9QTreeView18setRootIsDecoratedEb(arg0: int8_t) -> i32;
  // proto: void QTreeView::expanded(const QModelIndex & index);
  fn _ZN9QTreeView8expandedERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QTreeView::isHeaderHidden();
  fn _ZNK9QTreeView14isHeaderHiddenEv() -> i32;
  // proto: int QTreeView::columnAt(int x);
  fn _ZNK9QTreeView8columnAtEi(arg0: c_int) -> i32;
  // proto: bool QTreeView::isColumnHidden(int column);
  fn _ZNK9QTreeView14isColumnHiddenEi(arg0: c_int) -> i32;
  // proto: bool QTreeView::uniformRowHeights();
  fn _ZNK9QTreeView17uniformRowHeightsEv() -> i32;
  // proto: void QTreeView::setUniformRowHeights(bool uniform);
  fn _ZN9QTreeView20setUniformRowHeightsEb(arg0: int8_t) -> i32;
  // proto: void QTreeView::expandToDepth(int depth);
  fn _ZN9QTreeView13expandToDepthEi(arg0: c_int) -> i32;
  // proto: QModelIndex QTreeView::indexBelow(const QModelIndex & index);
  fn _ZNK9QTreeView10indexBelowERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QTreeView::expandAll();
  fn _ZN9QTreeView9expandAllEv() -> i32;
  // proto: QModelIndex QTreeView::indexAt(const QPoint & p);
  fn _ZNK9QTreeView7indexAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QTreeView::setColumnWidth(int column, int width);
  fn _ZN9QTreeView14setColumnWidthEii(arg0: c_int, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QTreeView)=1
pub struct QTreeView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTreeView {
  pub fn setHeader<T: QTreeView_setHeader>(&mut self, value: T) -> i32 {
    value.setHeader(self);
    return 1;
  }
}

pub trait QTreeView_setHeader {
  fn setHeader(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setHeader(QHeaderView * header);
impl<'a> /*trait*/ QTreeView_setHeader for (&'a mut QHeaderView) {
  fn setHeader(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9setHeaderEP11QHeaderView()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTreeView9setHeaderEP11QHeaderView(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isAnimated<T: QTreeView_isAnimated>(&mut self, value: T) -> i32 {
    value.isAnimated(self);
    return 1;
  }
}

pub trait QTreeView_isAnimated {
  fn isAnimated(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::isAnimated();
impl<'a> /*trait*/ QTreeView_isAnimated for () {
  fn isAnimated(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10isAnimatedEv()};
    unsafe {_ZNK9QTreeView10isAnimatedEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isExpanded<T: QTreeView_isExpanded>(&mut self, value: T) -> i32 {
    value.isExpanded(self);
    return 1;
  }
}

pub trait QTreeView_isExpanded {
  fn isExpanded(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::isExpanded(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_isExpanded for (&'a  QModelIndex) {
  fn isExpanded(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10isExpandedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTreeView10isExpandedERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setColumnHidden<T: QTreeView_setColumnHidden>(&mut self, value: T) -> i32 {
    value.setColumnHidden(self);
    return 1;
  }
}

pub trait QTreeView_setColumnHidden {
  fn setColumnHidden(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setColumnHidden(int column, bool hide);
impl<'a> /*trait*/ QTreeView_setColumnHidden for (i32, i8) {
  fn setColumnHidden(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setColumnHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN9QTreeView15setColumnHiddenEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setIndentation<T: QTreeView_setIndentation>(&mut self, value: T) -> i32 {
    value.setIndentation(self);
    return 1;
  }
}

pub trait QTreeView_setIndentation {
  fn setIndentation(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setIndentation(int i);
impl<'a> /*trait*/ QTreeView_setIndentation for (i32) {
  fn setIndentation(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14setIndentationEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTreeView14setIndentationEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn metaObject<T: QTreeView_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTreeView_metaObject {
  fn metaObject(self, this: &mut QTreeView) -> i32;
}

// proto: const QMetaObject * QTreeView::metaObject();
impl<'a> /*trait*/ QTreeView_metaObject for () {
  fn metaObject(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10metaObjectEv()};
    unsafe {_ZNK9QTreeView10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn reset<T: QTreeView_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QTreeView_reset {
  fn reset(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::reset();
impl<'a> /*trait*/ QTreeView_reset for () {
  fn reset(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView5resetEv()};
    unsafe {_ZN9QTreeView5resetEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setExpandsOnDoubleClick<T: QTreeView_setExpandsOnDoubleClick>(&mut self, value: T) -> i32 {
    value.setExpandsOnDoubleClick(self);
    return 1;
  }
}

pub trait QTreeView_setExpandsOnDoubleClick {
  fn setExpandsOnDoubleClick(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setExpandsOnDoubleClick(bool enable);
impl<'a> /*trait*/ QTreeView_setExpandsOnDoubleClick for (i8) {
  fn setExpandsOnDoubleClick(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView23setExpandsOnDoubleClickEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView23setExpandsOnDoubleClickEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setFirstColumnSpanned<T: QTreeView_setFirstColumnSpanned>(&mut self, value: T) -> i32 {
    value.setFirstColumnSpanned(self);
    return 1;
  }
}

pub trait QTreeView_setFirstColumnSpanned {
  fn setFirstColumnSpanned(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setFirstColumnSpanned(int row, const QModelIndex & parent, bool span);
impl<'a> /*trait*/ QTreeView_setFirstColumnSpanned for (i32, &'a  QModelIndex, i8) {
  fn setFirstColumnSpanned(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as int8_t;
    unsafe {_ZN9QTreeView21setFirstColumnSpannedEiRK11QModelIndexb(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn sortByColumn<T: QTreeView_sortByColumn>(&mut self, value: T) -> i32 {
    value.sortByColumn(self);
    return 1;
  }
}

pub trait QTreeView_sortByColumn {
  fn sortByColumn(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::sortByColumn(int column);
impl<'a> /*trait*/ QTreeView_sortByColumn for (i32) {
  fn sortByColumn(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12sortByColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTreeView12sortByColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setRowHidden<T: QTreeView_setRowHidden>(&mut self, value: T) -> i32 {
    value.setRowHidden(self);
    return 1;
  }
}

pub trait QTreeView_setRowHidden {
  fn setRowHidden(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setRowHidden(int row, const QModelIndex & parent, bool hide);
impl<'a> /*trait*/ QTreeView_setRowHidden for (i32, &'a  QModelIndex, i8) {
  fn setRowHidden(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12setRowHiddenEiRK11QModelIndexb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as int8_t;
    unsafe {_ZN9QTreeView12setRowHiddenEiRK11QModelIndexb(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expand<T: QTreeView_expand>(&mut self, value: T) -> i32 {
    value.expand(self);
    return 1;
  }
}

pub trait QTreeView_expand {
  fn expand(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::expand(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_expand for (&'a  QModelIndex) {
  fn expand(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView6expandERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTreeView6expandERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn autoExpandDelay<T: QTreeView_autoExpandDelay>(&mut self, value: T) -> i32 {
    value.autoExpandDelay(self);
    return 1;
  }
}

pub trait QTreeView_autoExpandDelay {
  fn autoExpandDelay(self, this: &mut QTreeView) -> i32;
}

// proto: int QTreeView::autoExpandDelay();
impl<'a> /*trait*/ QTreeView_autoExpandDelay for () {
  fn autoExpandDelay(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15autoExpandDelayEv()};
    unsafe {_ZNK9QTreeView15autoExpandDelayEv()};
    return 1;
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
  pub fn FreeQTreeView<T: QTreeView_FreeQTreeView>(&mut self, value: T) -> i32 {
    value.FreeQTreeView(self);
    return 1;
  }
}

pub trait QTreeView_FreeQTreeView {
  fn FreeQTreeView(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::FreeQTreeView();
impl<'a> /*trait*/ QTreeView_FreeQTreeView for () {
  fn FreeQTreeView(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeViewD0Ev()};
    unsafe {_ZN9QTreeViewD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn indentation<T: QTreeView_indentation>(&mut self, value: T) -> i32 {
    value.indentation(self);
    return 1;
  }
}

pub trait QTreeView_indentation {
  fn indentation(self, this: &mut QTreeView) -> i32;
}

// proto: int QTreeView::indentation();
impl<'a> /*trait*/ QTreeView_indentation for () {
  fn indentation(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11indentationEv()};
    unsafe {_ZNK9QTreeView11indentationEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn columnViewportPosition<T: QTreeView_columnViewportPosition>(&mut self, value: T) -> i32 {
    value.columnViewportPosition(self);
    return 1;
  }
}

pub trait QTreeView_columnViewportPosition {
  fn columnViewportPosition(self, this: &mut QTreeView) -> i32;
}

// proto: int QTreeView::columnViewportPosition(int column);
impl<'a> /*trait*/ QTreeView_columnViewportPosition for (i32) {
  fn columnViewportPosition(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView22columnViewportPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QTreeView22columnViewportPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expandsOnDoubleClick<T: QTreeView_expandsOnDoubleClick>(&mut self, value: T) -> i32 {
    value.expandsOnDoubleClick(self);
    return 1;
  }
}

pub trait QTreeView_expandsOnDoubleClick {
  fn expandsOnDoubleClick(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::expandsOnDoubleClick();
impl<'a> /*trait*/ QTreeView_expandsOnDoubleClick for () {
  fn expandsOnDoubleClick(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView20expandsOnDoubleClickEv()};
    unsafe {_ZNK9QTreeView20expandsOnDoubleClickEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isSortingEnabled<T: QTreeView_isSortingEnabled>(&mut self, value: T) -> i32 {
    value.isSortingEnabled(self);
    return 1;
  }
}

pub trait QTreeView_isSortingEnabled {
  fn isSortingEnabled(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::isSortingEnabled();
impl<'a> /*trait*/ QTreeView_isSortingEnabled for () {
  fn isSortingEnabled(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView16isSortingEnabledEv()};
    unsafe {_ZNK9QTreeView16isSortingEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn showColumn<T: QTreeView_showColumn>(&mut self, value: T) -> i32 {
    value.showColumn(self);
    return 1;
  }
}

pub trait QTreeView_showColumn {
  fn showColumn(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::showColumn(int column);
impl<'a> /*trait*/ QTreeView_showColumn for (i32) {
  fn showColumn(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView10showColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTreeView10showColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn visualRect<T: QTreeView_visualRect>(&mut self, value: T) -> i32 {
    value.visualRect(self);
    return 1;
  }
}

pub trait QTreeView_visualRect {
  fn visualRect(self, this: &mut QTreeView) -> i32;
}

// proto: QRect QTreeView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_visualRect for (&'a  QModelIndex) {
  fn visualRect(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTreeView10visualRectERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn collapse<T: QTreeView_collapse>(&mut self, value: T) -> i32 {
    value.collapse(self);
    return 1;
  }
}

pub trait QTreeView_collapse {
  fn collapse(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::collapse(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_collapse for (&'a  QModelIndex) {
  fn collapse(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView8collapseERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTreeView8collapseERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setWordWrap<T: QTreeView_setWordWrap>(&mut self, value: T) -> i32 {
    value.setWordWrap(self);
    return 1;
  }
}

pub trait QTreeView_setWordWrap {
  fn setWordWrap(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setWordWrap(bool on);
impl<'a> /*trait*/ QTreeView_setWordWrap for (i8) {
  fn setWordWrap(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setWordWrapEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView11setWordWrapEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn indexAbove<T: QTreeView_indexAbove>(&mut self, value: T) -> i32 {
    value.indexAbove(self);
    return 1;
  }
}

pub trait QTreeView_indexAbove {
  fn indexAbove(self, this: &mut QTreeView) -> i32;
}

// proto: QModelIndex QTreeView::indexAbove(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_indexAbove for (&'a  QModelIndex) {
  fn indexAbove(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10indexAboveERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTreeView10indexAboveERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn rootIsDecorated<T: QTreeView_rootIsDecorated>(&mut self, value: T) -> i32 {
    value.rootIsDecorated(self);
    return 1;
  }
}

pub trait QTreeView_rootIsDecorated {
  fn rootIsDecorated(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::rootIsDecorated();
impl<'a> /*trait*/ QTreeView_rootIsDecorated for () {
  fn rootIsDecorated(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15rootIsDecoratedEv()};
    unsafe {_ZNK9QTreeView15rootIsDecoratedEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn collapseAll<T: QTreeView_collapseAll>(&mut self, value: T) -> i32 {
    value.collapseAll(self);
    return 1;
  }
}

pub trait QTreeView_collapseAll {
  fn collapseAll(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::collapseAll();
impl<'a> /*trait*/ QTreeView_collapseAll for () {
  fn collapseAll(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11collapseAllEv()};
    unsafe {_ZN9QTreeView11collapseAllEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setHeaderHidden<T: QTreeView_setHeaderHidden>(&mut self, value: T) -> i32 {
    value.setHeaderHidden(self);
    return 1;
  }
}

pub trait QTreeView_setHeaderHidden {
  fn setHeaderHidden(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setHeaderHidden(bool hide);
impl<'a> /*trait*/ QTreeView_setHeaderHidden for (i8) {
  fn setHeaderHidden(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setHeaderHiddenEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView15setHeaderHiddenEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn allColumnsShowFocus<T: QTreeView_allColumnsShowFocus>(&mut self, value: T) -> i32 {
    value.allColumnsShowFocus(self);
    return 1;
  }
}

pub trait QTreeView_allColumnsShowFocus {
  fn allColumnsShowFocus(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::allColumnsShowFocus();
impl<'a> /*trait*/ QTreeView_allColumnsShowFocus for () {
  fn allColumnsShowFocus(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView19allColumnsShowFocusEv()};
    unsafe {_ZNK9QTreeView19allColumnsShowFocusEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn columnWidth<T: QTreeView_columnWidth>(&mut self, value: T) -> i32 {
    value.columnWidth(self);
    return 1;
  }
}

pub trait QTreeView_columnWidth {
  fn columnWidth(self, this: &mut QTreeView) -> i32;
}

// proto: int QTreeView::columnWidth(int column);
impl<'a> /*trait*/ QTreeView_columnWidth for (i32) {
  fn columnWidth(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11columnWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QTreeView11columnWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn resizeColumnToContents<T: QTreeView_resizeColumnToContents>(&mut self, value: T) -> i32 {
    value.resizeColumnToContents(self);
    return 1;
  }
}

pub trait QTreeView_resizeColumnToContents {
  fn resizeColumnToContents(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::resizeColumnToContents(int column);
impl<'a> /*trait*/ QTreeView_resizeColumnToContents for (i32) {
  fn resizeColumnToContents(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView22resizeColumnToContentsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTreeView22resizeColumnToContentsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setAutoExpandDelay<T: QTreeView_setAutoExpandDelay>(&mut self, value: T) -> i32 {
    value.setAutoExpandDelay(self);
    return 1;
  }
}

pub trait QTreeView_setAutoExpandDelay {
  fn setAutoExpandDelay(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setAutoExpandDelay(int delay);
impl<'a> /*trait*/ QTreeView_setAutoExpandDelay for (i32) {
  fn setAutoExpandDelay(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setAutoExpandDelayEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTreeView18setAutoExpandDelayEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setAllColumnsShowFocus<T: QTreeView_setAllColumnsShowFocus>(&mut self, value: T) -> i32 {
    value.setAllColumnsShowFocus(self);
    return 1;
  }
}

pub trait QTreeView_setAllColumnsShowFocus {
  fn setAllColumnsShowFocus(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setAllColumnsShowFocus(bool enable);
impl<'a> /*trait*/ QTreeView_setAllColumnsShowFocus for (i8) {
  fn setAllColumnsShowFocus(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView22setAllColumnsShowFocusEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView22setAllColumnsShowFocusEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isFirstColumnSpanned<T: QTreeView_isFirstColumnSpanned>(&mut self, value: T) -> i32 {
    value.isFirstColumnSpanned(self);
    return 1;
  }
}

pub trait QTreeView_isFirstColumnSpanned {
  fn isFirstColumnSpanned(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::isFirstColumnSpanned(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QTreeView_isFirstColumnSpanned for (i32, &'a  QModelIndex) {
  fn isFirstColumnSpanned(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK9QTreeView20isFirstColumnSpannedEiRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn hideColumn<T: QTreeView_hideColumn>(&mut self, value: T) -> i32 {
    value.hideColumn(self);
    return 1;
  }
}

pub trait QTreeView_hideColumn {
  fn hideColumn(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::hideColumn(int column);
impl<'a> /*trait*/ QTreeView_hideColumn for (i32) {
  fn hideColumn(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView10hideColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTreeView10hideColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn treePosition<T: QTreeView_treePosition>(&mut self, value: T) -> i32 {
    value.treePosition(self);
    return 1;
  }
}

pub trait QTreeView_treePosition {
  fn treePosition(self, this: &mut QTreeView) -> i32;
}

// proto: int QTreeView::treePosition();
impl<'a> /*trait*/ QTreeView_treePosition for () {
  fn treePosition(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView12treePositionEv()};
    unsafe {_ZNK9QTreeView12treePositionEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setExpanded<T: QTreeView_setExpanded>(&mut self, value: T) -> i32 {
    value.setExpanded(self);
    return 1;
  }
}

pub trait QTreeView_setExpanded {
  fn setExpanded(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setExpanded(const QModelIndex & index, bool expand);
impl<'a> /*trait*/ QTreeView_setExpanded for (&'a  QModelIndex, i8) {
  fn setExpanded(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setExpandedERK11QModelIndexb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN9QTreeView11setExpandedERK11QModelIndexb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn resetIndentation<T: QTreeView_resetIndentation>(&mut self, value: T) -> i32 {
    value.resetIndentation(self);
    return 1;
  }
}

pub trait QTreeView_resetIndentation {
  fn resetIndentation(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::resetIndentation();
impl<'a> /*trait*/ QTreeView_resetIndentation for () {
  fn resetIndentation(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView16resetIndentationEv()};
    unsafe {_ZN9QTreeView16resetIndentationEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isRowHidden<T: QTreeView_isRowHidden>(&mut self, value: T) -> i32 {
    value.isRowHidden(self);
    return 1;
  }
}

pub trait QTreeView_isRowHidden {
  fn isRowHidden(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::isRowHidden(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QTreeView_isRowHidden for (i32, &'a  QModelIndex) {
  fn isRowHidden(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView11isRowHiddenEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK9QTreeView11isRowHiddenEiRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn collapsed<T: QTreeView_collapsed>(&mut self, value: T) -> i32 {
    value.collapsed(self);
    return 1;
  }
}

pub trait QTreeView_collapsed {
  fn collapsed(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::collapsed(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_collapsed for (&'a  QModelIndex) {
  fn collapsed(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9collapsedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTreeView9collapsedERK11QModelIndex(arg0)};
    return 1;
  }
}

// proto: void QTreeView::NewQTreeView(const QTreeView & );
impl<'a> /*trait*/ QTreeView_NewQTreeView for (&'a  QTreeView) {
  fn NewQTreeView(self) -> QTreeView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeViewC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTreeViewC1ERKS_(qthis, arg0)};
    let rsthis = QTreeView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn selectAll<T: QTreeView_selectAll>(&mut self, value: T) -> i32 {
    value.selectAll(self);
    return 1;
  }
}

pub trait QTreeView_selectAll {
  fn selectAll(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::selectAll();
impl<'a> /*trait*/ QTreeView_selectAll for () {
  fn selectAll(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9selectAllEv()};
    unsafe {_ZN9QTreeView9selectAllEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn wordWrap<T: QTreeView_wordWrap>(&mut self, value: T) -> i32 {
    value.wordWrap(self);
    return 1;
  }
}

pub trait QTreeView_wordWrap {
  fn wordWrap(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::wordWrap();
impl<'a> /*trait*/ QTreeView_wordWrap for () {
  fn wordWrap(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView8wordWrapEv()};
    unsafe {_ZNK9QTreeView8wordWrapEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn doItemsLayout<T: QTreeView_doItemsLayout>(&mut self, value: T) -> i32 {
    value.doItemsLayout(self);
    return 1;
  }
}

pub trait QTreeView_doItemsLayout {
  fn doItemsLayout(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::doItemsLayout();
impl<'a> /*trait*/ QTreeView_doItemsLayout for () {
  fn doItemsLayout(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView13doItemsLayoutEv()};
    unsafe {_ZN9QTreeView13doItemsLayoutEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setTreePosition<T: QTreeView_setTreePosition>(&mut self, value: T) -> i32 {
    value.setTreePosition(self);
    return 1;
  }
}

pub trait QTreeView_setTreePosition {
  fn setTreePosition(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setTreePosition(int logicalIndex);
impl<'a> /*trait*/ QTreeView_setTreePosition for (i32) {
  fn setTreePosition(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView15setTreePositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTreeView15setTreePositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn keyboardSearch<T: QTreeView_keyboardSearch>(&mut self, value: T) -> i32 {
    value.keyboardSearch(self);
    return 1;
  }
}

pub trait QTreeView_keyboardSearch {
  fn keyboardSearch(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::keyboardSearch(const QString & search);
impl<'a> /*trait*/ QTreeView_keyboardSearch for (&'a  QString) {
  fn keyboardSearch(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14keyboardSearchERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTreeView14keyboardSearchERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setRootIndex<T: QTreeView_setRootIndex>(&mut self, value: T) -> i32 {
    value.setRootIndex(self);
    return 1;
  }
}

pub trait QTreeView_setRootIndex {
  fn setRootIndex(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_setRootIndex for (&'a  QModelIndex) {
  fn setRootIndex(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTreeView12setRootIndexERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setItemsExpandable<T: QTreeView_setItemsExpandable>(&mut self, value: T) -> i32 {
    value.setItemsExpandable(self);
    return 1;
  }
}

pub trait QTreeView_setItemsExpandable {
  fn setItemsExpandable(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setItemsExpandable(bool enable);
impl<'a> /*trait*/ QTreeView_setItemsExpandable for (i8) {
  fn setItemsExpandable(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setItemsExpandableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView18setItemsExpandableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setSelectionModel<T: QTreeView_setSelectionModel>(&mut self, value: T) -> i32 {
    value.setSelectionModel(self);
    return 1;
  }
}

pub trait QTreeView_setSelectionModel {
  fn setSelectionModel(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QTreeView_setSelectionModel for (&'a mut QItemSelectionModel) {
  fn setSelectionModel(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTreeView17setSelectionModelEP19QItemSelectionModel(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn header<T: QTreeView_header>(&mut self, value: T) -> i32 {
    value.header(self);
    return 1;
  }
}

pub trait QTreeView_header {
  fn header(self, this: &mut QTreeView) -> i32;
}

// proto: QHeaderView * QTreeView::header();
impl<'a> /*trait*/ QTreeView_header for () {
  fn header(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView6headerEv()};
    unsafe {_ZNK9QTreeView6headerEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setAnimated<T: QTreeView_setAnimated>(&mut self, value: T) -> i32 {
    value.setAnimated(self);
    return 1;
  }
}

pub trait QTreeView_setAnimated {
  fn setAnimated(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setAnimated(bool enable);
impl<'a> /*trait*/ QTreeView_setAnimated for (i8) {
  fn setAnimated(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView11setAnimatedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView11setAnimatedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setSortingEnabled<T: QTreeView_setSortingEnabled>(&mut self, value: T) -> i32 {
    value.setSortingEnabled(self);
    return 1;
  }
}

pub trait QTreeView_setSortingEnabled {
  fn setSortingEnabled(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QTreeView_setSortingEnabled for (i8) {
  fn setSortingEnabled(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView17setSortingEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView17setSortingEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn itemsExpandable<T: QTreeView_itemsExpandable>(&mut self, value: T) -> i32 {
    value.itemsExpandable(self);
    return 1;
  }
}

pub trait QTreeView_itemsExpandable {
  fn itemsExpandable(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::itemsExpandable();
impl<'a> /*trait*/ QTreeView_itemsExpandable for () {
  fn itemsExpandable(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView15itemsExpandableEv()};
    unsafe {_ZNK9QTreeView15itemsExpandableEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setRootIsDecorated<T: QTreeView_setRootIsDecorated>(&mut self, value: T) -> i32 {
    value.setRootIsDecorated(self);
    return 1;
  }
}

pub trait QTreeView_setRootIsDecorated {
  fn setRootIsDecorated(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setRootIsDecorated(bool show);
impl<'a> /*trait*/ QTreeView_setRootIsDecorated for (i8) {
  fn setRootIsDecorated(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView18setRootIsDecoratedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView18setRootIsDecoratedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expanded<T: QTreeView_expanded>(&mut self, value: T) -> i32 {
    value.expanded(self);
    return 1;
  }
}

pub trait QTreeView_expanded {
  fn expanded(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::expanded(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_expanded for (&'a  QModelIndex) {
  fn expanded(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView8expandedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTreeView8expandedERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isHeaderHidden<T: QTreeView_isHeaderHidden>(&mut self, value: T) -> i32 {
    value.isHeaderHidden(self);
    return 1;
  }
}

pub trait QTreeView_isHeaderHidden {
  fn isHeaderHidden(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::isHeaderHidden();
impl<'a> /*trait*/ QTreeView_isHeaderHidden for () {
  fn isHeaderHidden(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView14isHeaderHiddenEv()};
    unsafe {_ZNK9QTreeView14isHeaderHiddenEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn columnAt<T: QTreeView_columnAt>(&mut self, value: T) -> i32 {
    value.columnAt(self);
    return 1;
  }
}

pub trait QTreeView_columnAt {
  fn columnAt(self, this: &mut QTreeView) -> i32;
}

// proto: int QTreeView::columnAt(int x);
impl<'a> /*trait*/ QTreeView_columnAt for (i32) {
  fn columnAt(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView8columnAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QTreeView8columnAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn isColumnHidden<T: QTreeView_isColumnHidden>(&mut self, value: T) -> i32 {
    value.isColumnHidden(self);
    return 1;
  }
}

pub trait QTreeView_isColumnHidden {
  fn isColumnHidden(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::isColumnHidden(int column);
impl<'a> /*trait*/ QTreeView_isColumnHidden for (i32) {
  fn isColumnHidden(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView14isColumnHiddenEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QTreeView14isColumnHiddenEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn uniformRowHeights<T: QTreeView_uniformRowHeights>(&mut self, value: T) -> i32 {
    value.uniformRowHeights(self);
    return 1;
  }
}

pub trait QTreeView_uniformRowHeights {
  fn uniformRowHeights(self, this: &mut QTreeView) -> i32;
}

// proto: bool QTreeView::uniformRowHeights();
impl<'a> /*trait*/ QTreeView_uniformRowHeights for () {
  fn uniformRowHeights(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView17uniformRowHeightsEv()};
    unsafe {_ZNK9QTreeView17uniformRowHeightsEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setUniformRowHeights<T: QTreeView_setUniformRowHeights>(&mut self, value: T) -> i32 {
    value.setUniformRowHeights(self);
    return 1;
  }
}

pub trait QTreeView_setUniformRowHeights {
  fn setUniformRowHeights(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setUniformRowHeights(bool uniform);
impl<'a> /*trait*/ QTreeView_setUniformRowHeights for (i8) {
  fn setUniformRowHeights(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView20setUniformRowHeightsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTreeView20setUniformRowHeightsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expandToDepth<T: QTreeView_expandToDepth>(&mut self, value: T) -> i32 {
    value.expandToDepth(self);
    return 1;
  }
}

pub trait QTreeView_expandToDepth {
  fn expandToDepth(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::expandToDepth(int depth);
impl<'a> /*trait*/ QTreeView_expandToDepth for (i32) {
  fn expandToDepth(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView13expandToDepthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTreeView13expandToDepthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn indexBelow<T: QTreeView_indexBelow>(&mut self, value: T) -> i32 {
    value.indexBelow(self);
    return 1;
  }
}

pub trait QTreeView_indexBelow {
  fn indexBelow(self, this: &mut QTreeView) -> i32;
}

// proto: QModelIndex QTreeView::indexBelow(const QModelIndex & index);
impl<'a> /*trait*/ QTreeView_indexBelow for (&'a  QModelIndex) {
  fn indexBelow(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView10indexBelowERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTreeView10indexBelowERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn expandAll<T: QTreeView_expandAll>(&mut self, value: T) -> i32 {
    value.expandAll(self);
    return 1;
  }
}

pub trait QTreeView_expandAll {
  fn expandAll(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::expandAll();
impl<'a> /*trait*/ QTreeView_expandAll for () {
  fn expandAll(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView9expandAllEv()};
    unsafe {_ZN9QTreeView9expandAllEv()};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn indexAt<T: QTreeView_indexAt>(&mut self, value: T) -> i32 {
    value.indexAt(self);
    return 1;
  }
}

pub trait QTreeView_indexAt {
  fn indexAt(self, this: &mut QTreeView) -> i32;
}

// proto: QModelIndex QTreeView::indexAt(const QPoint & p);
impl<'a> /*trait*/ QTreeView_indexAt for (&'a  QPoint) {
  fn indexAt(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTreeView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QTreeView7indexAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QTreeView {
  pub fn setColumnWidth<T: QTreeView_setColumnWidth>(&mut self, value: T) -> i32 {
    value.setColumnWidth(self);
    return 1;
  }
}

pub trait QTreeView_setColumnWidth {
  fn setColumnWidth(self, this: &mut QTreeView) -> i32;
}

// proto: void QTreeView::setColumnWidth(int column, int width);
impl<'a> /*trait*/ QTreeView_setColumnWidth for (i32, i32) {
  fn setColumnWidth(self, this: &mut QTreeView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTreeView14setColumnWidthEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QTreeView14setColumnWidthEii(arg0, arg1)};
    return 1;
  }
}

