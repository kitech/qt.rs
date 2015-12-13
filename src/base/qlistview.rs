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
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QListView::NewQListView(QWidget * parent);
  fn _ZN9QListViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListView::setWordWrap(bool on);
  fn _ZN9QListView11setWordWrapEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QListView::doItemsLayout();
  fn _ZN9QListView13doItemsLayoutEv(qthis: *mut c_void) ;
  // proto:  int QListView::spacing();
  fn _ZNK9QListView7spacingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QListView::setGridSize(const QSize & size);
  fn _ZN9QListView11setGridSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QListView::indexAt(const QPoint & p);
  fn _ZNK9QListView7indexAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListView::setWrapping(bool enable);
  fn _ZN9QListView11setWrappingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QListView::setSelectionRectVisible(bool show);
  fn _ZN9QListView23setSelectionRectVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QListView::setBatchSize(int batchSize);
  fn _ZN9QListView12setBatchSizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QListView::uniformItemSizes();
  fn _ZNK9QListView16uniformItemSizesEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QListView::setRootIndex(const QModelIndex & index);
  fn _ZN9QListView12setRootIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QListView::isWrapping();
  fn _ZNK9QListView10isWrappingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QListView::reset();
  fn _ZN9QListView5resetEv(qthis: *mut c_void) ;
  // proto:  QSize QListView::gridSize();
  fn _ZNK9QListView8gridSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListView::setModelColumn(int column);
  fn _ZN9QListView14setModelColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QListView::NewQListView(const QListView & );
  fn _ZN9QListViewC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QListView::setSpacing(int space);
  fn _ZN9QListView10setSpacingEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRect QListView::visualRect(const QModelIndex & index);
  fn _ZNK9QListView10visualRectERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QListView::isRowHidden(int row);
  fn _ZNK9QListView11isRowHiddenEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QListView::FreeQListView();
  fn _ZN9QListViewD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QListView::metaObject();
  fn _ZNK9QListView10metaObjectEv(qthis: *mut c_void) ;
  // proto:  int QListView::batchSize();
  fn _ZNK9QListView9batchSizeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QListView::isSelectionRectVisible();
  fn _ZNK9QListView22isSelectionRectVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QListView::wordWrap();
  fn _ZNK9QListView8wordWrapEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QListView::setRowHidden(int row, bool hide);
  fn _ZN9QListView12setRowHiddenEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QListView::clearPropertyFlags();
  fn _ZN9QListView18clearPropertyFlagsEv(qthis: *mut c_void) ;
  // proto:  int QListView::modelColumn();
  fn _ZNK9QListView11modelColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QListView::setUniformItemSizes(bool enable);
  fn _ZN9QListView19setUniformItemSizesEb(qthis: *mut c_void, arg0: int8_t) ;
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
  pub fn setWordWrap<T: QListView_setWordWrap>(&mut self, value: T)  {
     value.setWordWrap(self);
    // return 1;
  }
}

pub trait QListView_setWordWrap {
  fn setWordWrap(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setWordWrap(bool on);
impl<'a> /*trait*/ QListView_setWordWrap for (i8) {
  fn setWordWrap(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setWordWrapEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QListView11setWordWrapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn doItemsLayout<T: QListView_doItemsLayout>(&mut self, value: T)  {
     value.doItemsLayout(self);
    // return 1;
  }
}

pub trait QListView_doItemsLayout {
  fn doItemsLayout(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::doItemsLayout();
impl<'a> /*trait*/ QListView_doItemsLayout for () {
  fn doItemsLayout(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView13doItemsLayoutEv()};
     unsafe {_ZN9QListView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn spacing<T: QListView_spacing>(&mut self, value: T) -> i32 {
    return value.spacing(self);
    // return 1;
  }
}

pub trait QListView_spacing {
  fn spacing(self, rsthis: &mut QListView) -> i32;
}

// proto:  int QListView::spacing();
impl<'a> /*trait*/ QListView_spacing for () {
  fn spacing(self, rsthis: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView7spacingEv()};
    let mut ret = unsafe {_ZNK9QListView7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setGridSize<T: QListView_setGridSize>(&mut self, value: T)  {
     value.setGridSize(self);
    // return 1;
  }
}

pub trait QListView_setGridSize {
  fn setGridSize(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setGridSize(const QSize & size);
impl<'a> /*trait*/ QListView_setGridSize for (&'a  QSize) {
  fn setGridSize(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setGridSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QListView11setGridSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn indexAt<T: QListView_indexAt>(&mut self, value: T) -> QModelIndex {
    return value.indexAt(self);
    // return 1;
  }
}

pub trait QListView_indexAt {
  fn indexAt(self, rsthis: &mut QListView) -> QModelIndex;
}

// proto:  QModelIndex QListView::indexAt(const QPoint & p);
impl<'a> /*trait*/ QListView_indexAt for (&'a  QPoint) {
  fn indexAt(self, rsthis: &mut QListView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QListView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setWrapping<T: QListView_setWrapping>(&mut self, value: T)  {
     value.setWrapping(self);
    // return 1;
  }
}

pub trait QListView_setWrapping {
  fn setWrapping(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setWrapping(bool enable);
impl<'a> /*trait*/ QListView_setWrapping for (i8) {
  fn setWrapping(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setWrappingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QListView11setWrappingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setSelectionRectVisible<T: QListView_setSelectionRectVisible>(&mut self, value: T)  {
     value.setSelectionRectVisible(self);
    // return 1;
  }
}

pub trait QListView_setSelectionRectVisible {
  fn setSelectionRectVisible(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setSelectionRectVisible(bool show);
impl<'a> /*trait*/ QListView_setSelectionRectVisible for (i8) {
  fn setSelectionRectVisible(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView23setSelectionRectVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QListView23setSelectionRectVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setBatchSize<T: QListView_setBatchSize>(&mut self, value: T)  {
     value.setBatchSize(self);
    // return 1;
  }
}

pub trait QListView_setBatchSize {
  fn setBatchSize(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setBatchSize(int batchSize);
impl<'a> /*trait*/ QListView_setBatchSize for (i32) {
  fn setBatchSize(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setBatchSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QListView12setBatchSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn uniformItemSizes<T: QListView_uniformItemSizes>(&mut self, value: T) -> i8 {
    return value.uniformItemSizes(self);
    // return 1;
  }
}

pub trait QListView_uniformItemSizes {
  fn uniformItemSizes(self, rsthis: &mut QListView) -> i8;
}

// proto:  bool QListView::uniformItemSizes();
impl<'a> /*trait*/ QListView_uniformItemSizes for () {
  fn uniformItemSizes(self, rsthis: &mut QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView16uniformItemSizesEv()};
    let mut ret = unsafe {_ZNK9QListView16uniformItemSizesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setRootIndex<T: QListView_setRootIndex>(&mut self, value: T)  {
     value.setRootIndex(self);
    // return 1;
  }
}

pub trait QListView_setRootIndex {
  fn setRootIndex(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QListView_setRootIndex for (&'a  QModelIndex) {
  fn setRootIndex(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QListView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn isWrapping<T: QListView_isWrapping>(&mut self, value: T) -> i8 {
    return value.isWrapping(self);
    // return 1;
  }
}

pub trait QListView_isWrapping {
  fn isWrapping(self, rsthis: &mut QListView) -> i8;
}

// proto:  bool QListView::isWrapping();
impl<'a> /*trait*/ QListView_isWrapping for () {
  fn isWrapping(self, rsthis: &mut QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10isWrappingEv()};
    let mut ret = unsafe {_ZNK9QListView10isWrappingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn reset<T: QListView_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QListView_reset {
  fn reset(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::reset();
impl<'a> /*trait*/ QListView_reset for () {
  fn reset(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView5resetEv()};
     unsafe {_ZN9QListView5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn gridSize<T: QListView_gridSize>(&mut self, value: T) -> QSize {
    return value.gridSize(self);
    // return 1;
  }
}

pub trait QListView_gridSize {
  fn gridSize(self, rsthis: &mut QListView) -> QSize;
}

// proto:  QSize QListView::gridSize();
impl<'a> /*trait*/ QListView_gridSize for () {
  fn gridSize(self, rsthis: &mut QListView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView8gridSizeEv()};
    let mut ret = unsafe {_ZNK9QListView8gridSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setModelColumn<T: QListView_setModelColumn>(&mut self, value: T)  {
     value.setModelColumn(self);
    // return 1;
  }
}

pub trait QListView_setModelColumn {
  fn setModelColumn(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setModelColumn(int column);
impl<'a> /*trait*/ QListView_setModelColumn for (i32) {
  fn setModelColumn(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView14setModelColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QListView14setModelColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QListView::NewQListView(const QListView & );
impl<'a> /*trait*/ QListView_NewQListView for (&'a  QListView) {
  fn NewQListView(self) -> QListView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListViewC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QListViewC1ERKS_(qthis, arg0)};
    let rsthis = QListView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setSpacing<T: QListView_setSpacing>(&mut self, value: T)  {
     value.setSpacing(self);
    // return 1;
  }
}

pub trait QListView_setSpacing {
  fn setSpacing(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setSpacing(int space);
impl<'a> /*trait*/ QListView_setSpacing for (i32) {
  fn setSpacing(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QListView10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn visualRect<T: QListView_visualRect>(&mut self, value: T) -> QRect {
    return value.visualRect(self);
    // return 1;
  }
}

pub trait QListView_visualRect {
  fn visualRect(self, rsthis: &mut QListView) -> QRect;
}

// proto:  QRect QListView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QListView_visualRect for (&'a  QModelIndex) {
  fn visualRect(self, rsthis: &mut QListView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QListView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn isRowHidden<T: QListView_isRowHidden>(&mut self, value: T) -> i8 {
    return value.isRowHidden(self);
    // return 1;
  }
}

pub trait QListView_isRowHidden {
  fn isRowHidden(self, rsthis: &mut QListView) -> i8;
}

// proto:  bool QListView::isRowHidden(int row);
impl<'a> /*trait*/ QListView_isRowHidden for (i32) {
  fn isRowHidden(self, rsthis: &mut QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView11isRowHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QListView11isRowHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn FreeQListView<T: QListView_FreeQListView>(&mut self, value: T)  {
     value.FreeQListView(self);
    // return 1;
  }
}

pub trait QListView_FreeQListView {
  fn FreeQListView(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::FreeQListView();
impl<'a> /*trait*/ QListView_FreeQListView for () {
  fn FreeQListView(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListViewD0Ev()};
     unsafe {_ZN9QListViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn metaObject<T: QListView_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QListView_metaObject {
  fn metaObject(self, rsthis: &mut QListView) ;
}

// proto:  const QMetaObject * QListView::metaObject();
impl<'a> /*trait*/ QListView_metaObject for () {
  fn metaObject(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10metaObjectEv()};
     unsafe {_ZNK9QListView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn batchSize<T: QListView_batchSize>(&mut self, value: T) -> i32 {
    return value.batchSize(self);
    // return 1;
  }
}

pub trait QListView_batchSize {
  fn batchSize(self, rsthis: &mut QListView) -> i32;
}

// proto:  int QListView::batchSize();
impl<'a> /*trait*/ QListView_batchSize for () {
  fn batchSize(self, rsthis: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView9batchSizeEv()};
    let mut ret = unsafe {_ZNK9QListView9batchSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn isSelectionRectVisible<T: QListView_isSelectionRectVisible>(&mut self, value: T) -> i8 {
    return value.isSelectionRectVisible(self);
    // return 1;
  }
}

pub trait QListView_isSelectionRectVisible {
  fn isSelectionRectVisible(self, rsthis: &mut QListView) -> i8;
}

// proto:  bool QListView::isSelectionRectVisible();
impl<'a> /*trait*/ QListView_isSelectionRectVisible for () {
  fn isSelectionRectVisible(self, rsthis: &mut QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView22isSelectionRectVisibleEv()};
    let mut ret = unsafe {_ZNK9QListView22isSelectionRectVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn wordWrap<T: QListView_wordWrap>(&mut self, value: T) -> i8 {
    return value.wordWrap(self);
    // return 1;
  }
}

pub trait QListView_wordWrap {
  fn wordWrap(self, rsthis: &mut QListView) -> i8;
}

// proto:  bool QListView::wordWrap();
impl<'a> /*trait*/ QListView_wordWrap for () {
  fn wordWrap(self, rsthis: &mut QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView8wordWrapEv()};
    let mut ret = unsafe {_ZNK9QListView8wordWrapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setRowHidden<T: QListView_setRowHidden>(&mut self, value: T)  {
     value.setRowHidden(self);
    // return 1;
  }
}

pub trait QListView_setRowHidden {
  fn setRowHidden(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setRowHidden(int row, bool hide);
impl<'a> /*trait*/ QListView_setRowHidden for (i32, i8) {
  fn setRowHidden(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setRowHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN9QListView12setRowHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn clearPropertyFlags<T: QListView_clearPropertyFlags>(&mut self, value: T)  {
     value.clearPropertyFlags(self);
    // return 1;
  }
}

pub trait QListView_clearPropertyFlags {
  fn clearPropertyFlags(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::clearPropertyFlags();
impl<'a> /*trait*/ QListView_clearPropertyFlags for () {
  fn clearPropertyFlags(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView18clearPropertyFlagsEv()};
     unsafe {_ZN9QListView18clearPropertyFlagsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn modelColumn<T: QListView_modelColumn>(&mut self, value: T) -> i32 {
    return value.modelColumn(self);
    // return 1;
  }
}

pub trait QListView_modelColumn {
  fn modelColumn(self, rsthis: &mut QListView) -> i32;
}

// proto:  int QListView::modelColumn();
impl<'a> /*trait*/ QListView_modelColumn for () {
  fn modelColumn(self, rsthis: &mut QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView11modelColumnEv()};
    let mut ret = unsafe {_ZNK9QListView11modelColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QListView {
  pub fn setUniformItemSizes<T: QListView_setUniformItemSizes>(&mut self, value: T)  {
     value.setUniformItemSizes(self);
    // return 1;
  }
}

pub trait QListView_setUniformItemSizes {
  fn setUniformItemSizes(self, rsthis: &mut QListView) ;
}

// proto:  void QListView::setUniformItemSizes(bool enable);
impl<'a> /*trait*/ QListView_setUniformItemSizes for (i8) {
  fn setUniformItemSizes(self, rsthis: &mut QListView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView19setUniformItemSizesEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QListView19setUniformItemSizesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

