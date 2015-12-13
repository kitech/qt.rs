// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qsize::QSize;
use super::qpoint::QPoint;
use super::qmodelindex::QModelIndex;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QListView::NewQListView(QWidget * parent);
  fn _ZN9QListViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QListView::setWordWrap(bool on);
  fn _ZN9QListView11setWordWrapEb(arg0: int8_t) -> i32;
  // proto: void QListView::doItemsLayout();
  fn _ZN9QListView13doItemsLayoutEv() -> i32;
  // proto: int QListView::spacing();
  fn _ZNK9QListView7spacingEv() -> i32;
  // proto: void QListView::setGridSize(const QSize & size);
  fn _ZN9QListView11setGridSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: QModelIndex QListView::indexAt(const QPoint & p);
  fn _ZNK9QListView7indexAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QListView::setWrapping(bool enable);
  fn _ZN9QListView11setWrappingEb(arg0: int8_t) -> i32;
  // proto: void QListView::setSelectionRectVisible(bool show);
  fn _ZN9QListView23setSelectionRectVisibleEb(arg0: int8_t) -> i32;
  // proto: void QListView::setBatchSize(int batchSize);
  fn _ZN9QListView12setBatchSizeEi(arg0: c_int) -> i32;
  // proto: bool QListView::uniformItemSizes();
  fn _ZNK9QListView16uniformItemSizesEv() -> i32;
  // proto: void QListView::setRootIndex(const QModelIndex & index);
  fn _ZN9QListView12setRootIndexERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QListView::isWrapping();
  fn _ZNK9QListView10isWrappingEv() -> i32;
  // proto: void QListView::reset();
  fn _ZN9QListView5resetEv() -> i32;
  // proto: QSize QListView::gridSize();
  fn _ZNK9QListView8gridSizeEv() -> i32;
  // proto: void QListView::setModelColumn(int column);
  fn _ZN9QListView14setModelColumnEi(arg0: c_int) -> i32;
  // proto: void QListView::NewQListView(const QListView & );
  fn _ZN9QListViewC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QListView::setSpacing(int space);
  fn _ZN9QListView10setSpacingEi(arg0: c_int) -> i32;
  // proto: QRect QListView::visualRect(const QModelIndex & index);
  fn _ZNK9QListView10visualRectERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QListView::isRowHidden(int row);
  fn _ZNK9QListView11isRowHiddenEi(arg0: c_int) -> i32;
  // proto: void QListView::FreeQListView();
  fn _ZN9QListViewD0Ev() -> i32;
  // proto: const QMetaObject * QListView::metaObject();
  fn _ZNK9QListView10metaObjectEv() -> i32;
  // proto: int QListView::batchSize();
  fn _ZNK9QListView9batchSizeEv() -> i32;
  // proto: bool QListView::isSelectionRectVisible();
  fn _ZNK9QListView22isSelectionRectVisibleEv() -> i32;
  // proto: bool QListView::wordWrap();
  fn _ZNK9QListView8wordWrapEv() -> i32;
  // proto: void QListView::setRowHidden(int row, bool hide);
  fn _ZN9QListView12setRowHiddenEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QListView::clearPropertyFlags();
  fn _ZN9QListView18clearPropertyFlagsEv() -> i32;
  // proto: int QListView::modelColumn();
  fn _ZNK9QListView11modelColumnEv() -> i32;
  // proto: void QListView::setUniformItemSizes(bool enable);
  fn _ZN9QListView19setUniformItemSizesEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QListView)=1
pub struct QListView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QListView {
  pub fn NewQListView<T: QListView_NewQListView>(value: T) -> QListView {
    let rsthis = value.NewQListView();
    return rsthis;
    // return 1;
  }
}

pub trait QListView_NewQListView {
  fn NewQListView(self) -> QListView;
}

// proto: void QListView::NewQListView(QWidget * parent);
impl<'a> /*trait*/ QListView_NewQListView for (&'a mut QWidget) {
  fn NewQListView(self) -> QListView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListViewC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QListViewC1EP7QWidget(qthis, arg0)};
    let rsthis = QListView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setWordWrap<T: QListView_setWordWrap>(&mut self, value: T) -> i32 {
    value.setWordWrap(self);
    return 1;
  }
}

pub trait QListView_setWordWrap {
  fn setWordWrap(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setWordWrap(bool on);
impl<'a> /*trait*/ QListView_setWordWrap for (i8) {
  fn setWordWrap(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setWordWrapEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QListView11setWordWrapEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn doItemsLayout<T: QListView_doItemsLayout>(&mut self, value: T) -> i32 {
    value.doItemsLayout(self);
    return 1;
  }
}

pub trait QListView_doItemsLayout {
  fn doItemsLayout(self, this: &mut QListView) -> i32;
}

// proto: void QListView::doItemsLayout();
impl<'a> /*trait*/ QListView_doItemsLayout for () {
  fn doItemsLayout(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView13doItemsLayoutEv()};
    unsafe {_ZN9QListView13doItemsLayoutEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn spacing<T: QListView_spacing>(&mut self, value: T) -> i32 {
    value.spacing(self);
    return 1;
  }
}

pub trait QListView_spacing {
  fn spacing(self, this: &mut QListView) -> i32;
}

// proto: int QListView::spacing();
impl<'a> /*trait*/ QListView_spacing for () {
  fn spacing(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView7spacingEv()};
    unsafe {_ZNK9QListView7spacingEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setGridSize<T: QListView_setGridSize>(&mut self, value: T) -> i32 {
    value.setGridSize(self);
    return 1;
  }
}

pub trait QListView_setGridSize {
  fn setGridSize(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setGridSize(const QSize & size);
impl<'a> /*trait*/ QListView_setGridSize for (&'a  QSize) {
  fn setGridSize(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setGridSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QListView11setGridSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn indexAt<T: QListView_indexAt>(&mut self, value: T) -> i32 {
    value.indexAt(self);
    return 1;
  }
}

pub trait QListView_indexAt {
  fn indexAt(self, this: &mut QListView) -> i32;
}

// proto: QModelIndex QListView::indexAt(const QPoint & p);
impl<'a> /*trait*/ QListView_indexAt for (&'a  QPoint) {
  fn indexAt(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QListView7indexAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setWrapping<T: QListView_setWrapping>(&mut self, value: T) -> i32 {
    value.setWrapping(self);
    return 1;
  }
}

pub trait QListView_setWrapping {
  fn setWrapping(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setWrapping(bool enable);
impl<'a> /*trait*/ QListView_setWrapping for (i8) {
  fn setWrapping(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setWrappingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QListView11setWrappingEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setSelectionRectVisible<T: QListView_setSelectionRectVisible>(&mut self, value: T) -> i32 {
    value.setSelectionRectVisible(self);
    return 1;
  }
}

pub trait QListView_setSelectionRectVisible {
  fn setSelectionRectVisible(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setSelectionRectVisible(bool show);
impl<'a> /*trait*/ QListView_setSelectionRectVisible for (i8) {
  fn setSelectionRectVisible(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView23setSelectionRectVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QListView23setSelectionRectVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setBatchSize<T: QListView_setBatchSize>(&mut self, value: T) -> i32 {
    value.setBatchSize(self);
    return 1;
  }
}

pub trait QListView_setBatchSize {
  fn setBatchSize(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setBatchSize(int batchSize);
impl<'a> /*trait*/ QListView_setBatchSize for (i32) {
  fn setBatchSize(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setBatchSizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QListView12setBatchSizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn uniformItemSizes<T: QListView_uniformItemSizes>(&mut self, value: T) -> i32 {
    value.uniformItemSizes(self);
    return 1;
  }
}

pub trait QListView_uniformItemSizes {
  fn uniformItemSizes(self, this: &mut QListView) -> i32;
}

// proto: bool QListView::uniformItemSizes();
impl<'a> /*trait*/ QListView_uniformItemSizes for () {
  fn uniformItemSizes(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView16uniformItemSizesEv()};
    unsafe {_ZNK9QListView16uniformItemSizesEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setRootIndex<T: QListView_setRootIndex>(&mut self, value: T) -> i32 {
    value.setRootIndex(self);
    return 1;
  }
}

pub trait QListView_setRootIndex {
  fn setRootIndex(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QListView_setRootIndex for (&'a  QModelIndex) {
  fn setRootIndex(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QListView12setRootIndexERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn isWrapping<T: QListView_isWrapping>(&mut self, value: T) -> i32 {
    value.isWrapping(self);
    return 1;
  }
}

pub trait QListView_isWrapping {
  fn isWrapping(self, this: &mut QListView) -> i32;
}

// proto: bool QListView::isWrapping();
impl<'a> /*trait*/ QListView_isWrapping for () {
  fn isWrapping(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10isWrappingEv()};
    unsafe {_ZNK9QListView10isWrappingEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn reset<T: QListView_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QListView_reset {
  fn reset(self, this: &mut QListView) -> i32;
}

// proto: void QListView::reset();
impl<'a> /*trait*/ QListView_reset for () {
  fn reset(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView5resetEv()};
    unsafe {_ZN9QListView5resetEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn gridSize<T: QListView_gridSize>(&mut self, value: T) -> i32 {
    value.gridSize(self);
    return 1;
  }
}

pub trait QListView_gridSize {
  fn gridSize(self, this: &mut QListView) -> i32;
}

// proto: QSize QListView::gridSize();
impl<'a> /*trait*/ QListView_gridSize for () {
  fn gridSize(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView8gridSizeEv()};
    unsafe {_ZNK9QListView8gridSizeEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setModelColumn<T: QListView_setModelColumn>(&mut self, value: T) -> i32 {
    value.setModelColumn(self);
    return 1;
  }
}

pub trait QListView_setModelColumn {
  fn setModelColumn(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setModelColumn(int column);
impl<'a> /*trait*/ QListView_setModelColumn for (i32) {
  fn setModelColumn(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView14setModelColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QListView14setModelColumnEi(arg0)};
    return 1;
  }
}

// proto: void QListView::NewQListView(const QListView & );
impl<'a> /*trait*/ QListView_NewQListView for (&'a  QListView) {
  fn NewQListView(self) -> QListView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListViewC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QListViewC1ERKS_(qthis, arg0)};
    let rsthis = QListView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setSpacing<T: QListView_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QListView_setSpacing {
  fn setSpacing(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setSpacing(int space);
impl<'a> /*trait*/ QListView_setSpacing for (i32) {
  fn setSpacing(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView10setSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QListView10setSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn visualRect<T: QListView_visualRect>(&mut self, value: T) -> i32 {
    value.visualRect(self);
    return 1;
  }
}

pub trait QListView_visualRect {
  fn visualRect(self, this: &mut QListView) -> i32;
}

// proto: QRect QListView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QListView_visualRect for (&'a  QModelIndex) {
  fn visualRect(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QListView10visualRectERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn isRowHidden<T: QListView_isRowHidden>(&mut self, value: T) -> i32 {
    value.isRowHidden(self);
    return 1;
  }
}

pub trait QListView_isRowHidden {
  fn isRowHidden(self, this: &mut QListView) -> i32;
}

// proto: bool QListView::isRowHidden(int row);
impl<'a> /*trait*/ QListView_isRowHidden for (i32) {
  fn isRowHidden(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView11isRowHiddenEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QListView11isRowHiddenEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn FreeQListView<T: QListView_FreeQListView>(&mut self, value: T) -> i32 {
    value.FreeQListView(self);
    return 1;
  }
}

pub trait QListView_FreeQListView {
  fn FreeQListView(self, this: &mut QListView) -> i32;
}

// proto: void QListView::FreeQListView();
impl<'a> /*trait*/ QListView_FreeQListView for () {
  fn FreeQListView(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListViewD0Ev()};
    unsafe {_ZN9QListViewD0Ev()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn metaObject<T: QListView_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QListView_metaObject {
  fn metaObject(self, this: &mut QListView) -> i32;
}

// proto: const QMetaObject * QListView::metaObject();
impl<'a> /*trait*/ QListView_metaObject for () {
  fn metaObject(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10metaObjectEv()};
    unsafe {_ZNK9QListView10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn batchSize<T: QListView_batchSize>(&mut self, value: T) -> i32 {
    value.batchSize(self);
    return 1;
  }
}

pub trait QListView_batchSize {
  fn batchSize(self, this: &mut QListView) -> i32;
}

// proto: int QListView::batchSize();
impl<'a> /*trait*/ QListView_batchSize for () {
  fn batchSize(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView9batchSizeEv()};
    unsafe {_ZNK9QListView9batchSizeEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn isSelectionRectVisible<T: QListView_isSelectionRectVisible>(&mut self, value: T) -> i32 {
    value.isSelectionRectVisible(self);
    return 1;
  }
}

pub trait QListView_isSelectionRectVisible {
  fn isSelectionRectVisible(self, this: &mut QListView) -> i32;
}

// proto: bool QListView::isSelectionRectVisible();
impl<'a> /*trait*/ QListView_isSelectionRectVisible for () {
  fn isSelectionRectVisible(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView22isSelectionRectVisibleEv()};
    unsafe {_ZNK9QListView22isSelectionRectVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn wordWrap<T: QListView_wordWrap>(&mut self, value: T) -> i32 {
    value.wordWrap(self);
    return 1;
  }
}

pub trait QListView_wordWrap {
  fn wordWrap(self, this: &mut QListView) -> i32;
}

// proto: bool QListView::wordWrap();
impl<'a> /*trait*/ QListView_wordWrap for () {
  fn wordWrap(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView8wordWrapEv()};
    unsafe {_ZNK9QListView8wordWrapEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setRowHidden<T: QListView_setRowHidden>(&mut self, value: T) -> i32 {
    value.setRowHidden(self);
    return 1;
  }
}

pub trait QListView_setRowHidden {
  fn setRowHidden(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setRowHidden(int row, bool hide);
impl<'a> /*trait*/ QListView_setRowHidden for (i32, i8) {
  fn setRowHidden(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setRowHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN9QListView12setRowHiddenEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn clearPropertyFlags<T: QListView_clearPropertyFlags>(&mut self, value: T) -> i32 {
    value.clearPropertyFlags(self);
    return 1;
  }
}

pub trait QListView_clearPropertyFlags {
  fn clearPropertyFlags(self, this: &mut QListView) -> i32;
}

// proto: void QListView::clearPropertyFlags();
impl<'a> /*trait*/ QListView_clearPropertyFlags for () {
  fn clearPropertyFlags(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView18clearPropertyFlagsEv()};
    unsafe {_ZN9QListView18clearPropertyFlagsEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn modelColumn<T: QListView_modelColumn>(&mut self, value: T) -> i32 {
    value.modelColumn(self);
    return 1;
  }
}

pub trait QListView_modelColumn {
  fn modelColumn(self, this: &mut QListView) -> i32;
}

// proto: int QListView::modelColumn();
impl<'a> /*trait*/ QListView_modelColumn for () {
  fn modelColumn(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView11modelColumnEv()};
    unsafe {_ZNK9QListView11modelColumnEv()};
    return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setUniformItemSizes<T: QListView_setUniformItemSizes>(&mut self, value: T) -> i32 {
    value.setUniformItemSizes(self);
    return 1;
  }
}

pub trait QListView_setUniformItemSizes {
  fn setUniformItemSizes(self, this: &mut QListView) -> i32;
}

// proto: void QListView::setUniformItemSizes(bool enable);
impl<'a> /*trait*/ QListView_setUniformItemSizes for (i8) {
  fn setUniformItemSizes(self, this: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView19setUniformItemSizesEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QListView19setUniformItemSizesEb(arg0)};
    return 1;
  }
}

