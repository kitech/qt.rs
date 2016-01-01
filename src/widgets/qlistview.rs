// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtWidgets/qlistview.h
// dst-file: /src/widgets/qlistview.rs
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
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QListView_Class_Size() -> c_int;
  // proto:  void QListView::QListView(QWidget * parent);
  fn dector_ZN9QListViewC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QListViewC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListView::setWordWrap(bool on);
  fn _ZN9QListView11setWordWrapEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QListView::doItemsLayout();
  fn _ZN9QListView13doItemsLayoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QListView::spacing();
  fn _ZNK9QListView7spacingEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QListView::setGridSize(const QSize & size);
  fn _ZN9QListView11setGridSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QListView::indexAt(const QPoint & p);
  fn _ZNK9QListView7indexAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListView::setWrapping(bool enable);
  fn _ZN9QListView11setWrappingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QListView::setSelectionRectVisible(bool show);
  fn _ZN9QListView23setSelectionRectVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QListView::setBatchSize(int batchSize);
  fn _ZN9QListView12setBatchSizeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QListView::uniformItemSizes();
  fn _ZNK9QListView16uniformItemSizesEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QListView::setRootIndex(const QModelIndex & index);
  fn _ZN9QListView12setRootIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QListView::isWrapping();
  fn _ZNK9QListView10isWrappingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QListView::reset();
  fn _ZN9QListView5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QListView::gridSize();
  fn _ZNK9QListView8gridSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListView::setModelColumn(int column);
  fn _ZN9QListView14setModelColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QListView::QListView(const QListView & );
  fn dector_ZN9QListViewC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QListViewC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListView::setSpacing(int space);
  fn _ZN9QListView10setSpacingEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRect QListView::visualRect(const QModelIndex & index);
  fn _ZNK9QListView10visualRectERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QListView::isRowHidden(int row);
  fn _ZNK9QListView11isRowHiddenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QListView::~QListView();
  fn _ZN9QListViewD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QListView::metaObject();
  fn _ZNK9QListView10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QListView::batchSize();
  fn _ZNK9QListView9batchSizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QListView::isSelectionRectVisible();
  fn _ZNK9QListView22isSelectionRectVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QListView::wordWrap();
  fn _ZNK9QListView8wordWrapEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QListView::setRowHidden(int row, bool hide);
  fn _ZN9QListView12setRowHiddenEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  void QListView::clearPropertyFlags();
  fn _ZN9QListView18clearPropertyFlagsEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QListView::modelColumn();
  fn _ZNK9QListView11modelColumnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QListView::setUniformItemSizes(bool enable);
  fn _ZN9QListView19setUniformItemSizesEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QListView)=1
#[derive(Default)]
pub struct QListView {
  qbase: QAbstractItemView,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _indexesMoved_1: QListView_indexesMoved_signal,
}

impl /*struct*/ QListView {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QListView {
    return QListView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QListView {
  type Target = QAbstractItemView;

  fn deref(&self) -> &QAbstractItemView {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemView> for QListView {
  fn as_ref(& self) -> & QAbstractItemView {
    return & self.qbase;
  }
}
  // proto:  void QListView::QListView(QWidget * parent);
impl /*struct*/ QListView {
  pub fn new<T: QListView_new>(value: T) -> QListView {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QListView_new {
  fn new(self) -> QListView;
}

  // proto:  void QListView::QListView(QWidget * parent);
impl<'a> /*trait*/ QListView_new for (&'a QWidget) {
  fn new(self) -> QListView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListViewC1EP7QWidget()};
    let ctysz: c_int = unsafe{QListView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QListViewC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QListViewC1EP7QWidget(arg0)} as u64;
    let rsthis = QListView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QListView::setWordWrap(bool on);
impl /*struct*/ QListView {
  pub fn setWordWrap<RetType, T: QListView_setWordWrap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWordWrap(self);
    // return 1;
  }
}

pub trait QListView_setWordWrap<RetType> {
  fn setWordWrap(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setWordWrap(bool on);
impl<'a> /*trait*/ QListView_setWordWrap<()> for (i8) {
  fn setWordWrap(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setWordWrapEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QListView11setWordWrapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListView::doItemsLayout();
impl /*struct*/ QListView {
  pub fn doItemsLayout<RetType, T: QListView_doItemsLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doItemsLayout(self);
    // return 1;
  }
}

pub trait QListView_doItemsLayout<RetType> {
  fn doItemsLayout(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::doItemsLayout();
impl<'a> /*trait*/ QListView_doItemsLayout<()> for () {
  fn doItemsLayout(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView13doItemsLayoutEv()};
     unsafe {_ZN9QListView13doItemsLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QListView::spacing();
impl /*struct*/ QListView {
  pub fn spacing<RetType, T: QListView_spacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QListView_spacing<RetType> {
  fn spacing(self , rsthis: & QListView) -> RetType;
}

  // proto:  int QListView::spacing();
impl<'a> /*trait*/ QListView_spacing<i32> for () {
  fn spacing(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView7spacingEv()};
    let mut ret = unsafe {_ZNK9QListView7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QListView::setGridSize(const QSize & size);
impl /*struct*/ QListView {
  pub fn setGridSize<RetType, T: QListView_setGridSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGridSize(self);
    // return 1;
  }
}

pub trait QListView_setGridSize<RetType> {
  fn setGridSize(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setGridSize(const QSize & size);
impl<'a> /*trait*/ QListView_setGridSize<()> for (&'a QSize) {
  fn setGridSize(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setGridSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QListView11setGridSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QListView::indexAt(const QPoint & p);
impl /*struct*/ QListView {
  pub fn indexAt<RetType, T: QListView_indexAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexAt(self);
    // return 1;
  }
}

pub trait QListView_indexAt<RetType> {
  fn indexAt(self , rsthis: & QListView) -> RetType;
}

  // proto:  QModelIndex QListView::indexAt(const QPoint & p);
impl<'a> /*trait*/ QListView_indexAt<QModelIndex> for (&'a QPoint) {
  fn indexAt(self , rsthis: & QListView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QListView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListView::setWrapping(bool enable);
impl /*struct*/ QListView {
  pub fn setWrapping<RetType, T: QListView_setWrapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWrapping(self);
    // return 1;
  }
}

pub trait QListView_setWrapping<RetType> {
  fn setWrapping(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setWrapping(bool enable);
impl<'a> /*trait*/ QListView_setWrapping<()> for (i8) {
  fn setWrapping(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView11setWrappingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QListView11setWrappingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListView::setSelectionRectVisible(bool show);
impl /*struct*/ QListView {
  pub fn setSelectionRectVisible<RetType, T: QListView_setSelectionRectVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelectionRectVisible(self);
    // return 1;
  }
}

pub trait QListView_setSelectionRectVisible<RetType> {
  fn setSelectionRectVisible(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setSelectionRectVisible(bool show);
impl<'a> /*trait*/ QListView_setSelectionRectVisible<()> for (i8) {
  fn setSelectionRectVisible(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView23setSelectionRectVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QListView23setSelectionRectVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListView::setBatchSize(int batchSize);
impl /*struct*/ QListView {
  pub fn setBatchSize<RetType, T: QListView_setBatchSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBatchSize(self);
    // return 1;
  }
}

pub trait QListView_setBatchSize<RetType> {
  fn setBatchSize(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setBatchSize(int batchSize);
impl<'a> /*trait*/ QListView_setBatchSize<()> for (i32) {
  fn setBatchSize(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setBatchSizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QListView12setBatchSizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QListView::uniformItemSizes();
impl /*struct*/ QListView {
  pub fn uniformItemSizes<RetType, T: QListView_uniformItemSizes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.uniformItemSizes(self);
    // return 1;
  }
}

pub trait QListView_uniformItemSizes<RetType> {
  fn uniformItemSizes(self , rsthis: & QListView) -> RetType;
}

  // proto:  bool QListView::uniformItemSizes();
impl<'a> /*trait*/ QListView_uniformItemSizes<i8> for () {
  fn uniformItemSizes(self , rsthis: & QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView16uniformItemSizesEv()};
    let mut ret = unsafe {_ZNK9QListView16uniformItemSizesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListView::setRootIndex(const QModelIndex & index);
impl /*struct*/ QListView {
  pub fn setRootIndex<RetType, T: QListView_setRootIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex(self);
    // return 1;
  }
}

pub trait QListView_setRootIndex<RetType> {
  fn setRootIndex(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QListView_setRootIndex<()> for (&'a QModelIndex) {
  fn setRootIndex(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QListView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QListView::isWrapping();
impl /*struct*/ QListView {
  pub fn isWrapping<RetType, T: QListView_isWrapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWrapping(self);
    // return 1;
  }
}

pub trait QListView_isWrapping<RetType> {
  fn isWrapping(self , rsthis: & QListView) -> RetType;
}

  // proto:  bool QListView::isWrapping();
impl<'a> /*trait*/ QListView_isWrapping<i8> for () {
  fn isWrapping(self , rsthis: & QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10isWrappingEv()};
    let mut ret = unsafe {_ZNK9QListView10isWrappingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListView::reset();
impl /*struct*/ QListView {
  pub fn reset<RetType, T: QListView_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QListView_reset<RetType> {
  fn reset(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::reset();
impl<'a> /*trait*/ QListView_reset<()> for () {
  fn reset(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView5resetEv()};
     unsafe {_ZN9QListView5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QListView::gridSize();
impl /*struct*/ QListView {
  pub fn gridSize<RetType, T: QListView_gridSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.gridSize(self);
    // return 1;
  }
}

pub trait QListView_gridSize<RetType> {
  fn gridSize(self , rsthis: & QListView) -> RetType;
}

  // proto:  QSize QListView::gridSize();
impl<'a> /*trait*/ QListView_gridSize<QSize> for () {
  fn gridSize(self , rsthis: & QListView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView8gridSizeEv()};
    let mut ret = unsafe {_ZNK9QListView8gridSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListView::setModelColumn(int column);
impl /*struct*/ QListView {
  pub fn setModelColumn<RetType, T: QListView_setModelColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModelColumn(self);
    // return 1;
  }
}

pub trait QListView_setModelColumn<RetType> {
  fn setModelColumn(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setModelColumn(int column);
impl<'a> /*trait*/ QListView_setModelColumn<()> for (i32) {
  fn setModelColumn(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView14setModelColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QListView14setModelColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListView::QListView(const QListView & );
impl<'a> /*trait*/ QListView_new for (&'a QListView) {
  fn new(self) -> QListView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListViewC1ERKS_()};
    let ctysz: c_int = unsafe{QListView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QListViewC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN9QListViewC1ERKS_(arg0)} as u64;
    let rsthis = QListView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QListView::setSpacing(int space);
impl /*struct*/ QListView {
  pub fn setSpacing<RetType, T: QListView_setSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QListView_setSpacing<RetType> {
  fn setSpacing(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setSpacing(int space);
impl<'a> /*trait*/ QListView_setSpacing<()> for (i32) {
  fn setSpacing(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QListView10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QListView::visualRect(const QModelIndex & index);
impl /*struct*/ QListView {
  pub fn visualRect<RetType, T: QListView_visualRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualRect(self);
    // return 1;
  }
}

pub trait QListView_visualRect<RetType> {
  fn visualRect(self , rsthis: & QListView) -> RetType;
}

  // proto:  QRect QListView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QListView_visualRect<QRect> for (&'a QModelIndex) {
  fn visualRect(self , rsthis: & QListView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QListView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QListView::isRowHidden(int row);
impl /*struct*/ QListView {
  pub fn isRowHidden<RetType, T: QListView_isRowHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRowHidden(self);
    // return 1;
  }
}

pub trait QListView_isRowHidden<RetType> {
  fn isRowHidden(self , rsthis: & QListView) -> RetType;
}

  // proto:  bool QListView::isRowHidden(int row);
impl<'a> /*trait*/ QListView_isRowHidden<i8> for (i32) {
  fn isRowHidden(self , rsthis: & QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView11isRowHiddenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QListView11isRowHiddenEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListView::~QListView();
impl /*struct*/ QListView {
  pub fn free<RetType, T: QListView_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QListView_free<RetType> {
  fn free(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::~QListView();
impl<'a> /*trait*/ QListView_free<()> for () {
  fn free(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListViewD0Ev()};
     unsafe {_ZN9QListViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QListView::metaObject();
impl /*struct*/ QListView {
  pub fn metaObject<RetType, T: QListView_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QListView_metaObject<RetType> {
  fn metaObject(self , rsthis: & QListView) -> RetType;
}

  // proto:  const QMetaObject * QListView::metaObject();
impl<'a> /*trait*/ QListView_metaObject<()> for () {
  fn metaObject(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView10metaObjectEv()};
     unsafe {_ZNK9QListView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QListView::batchSize();
impl /*struct*/ QListView {
  pub fn batchSize<RetType, T: QListView_batchSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.batchSize(self);
    // return 1;
  }
}

pub trait QListView_batchSize<RetType> {
  fn batchSize(self , rsthis: & QListView) -> RetType;
}

  // proto:  int QListView::batchSize();
impl<'a> /*trait*/ QListView_batchSize<i32> for () {
  fn batchSize(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView9batchSizeEv()};
    let mut ret = unsafe {_ZNK9QListView9batchSizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QListView::isSelectionRectVisible();
impl /*struct*/ QListView {
  pub fn isSelectionRectVisible<RetType, T: QListView_isSelectionRectVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSelectionRectVisible(self);
    // return 1;
  }
}

pub trait QListView_isSelectionRectVisible<RetType> {
  fn isSelectionRectVisible(self , rsthis: & QListView) -> RetType;
}

  // proto:  bool QListView::isSelectionRectVisible();
impl<'a> /*trait*/ QListView_isSelectionRectVisible<i8> for () {
  fn isSelectionRectVisible(self , rsthis: & QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView22isSelectionRectVisibleEv()};
    let mut ret = unsafe {_ZNK9QListView22isSelectionRectVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QListView::wordWrap();
impl /*struct*/ QListView {
  pub fn wordWrap<RetType, T: QListView_wordWrap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wordWrap(self);
    // return 1;
  }
}

pub trait QListView_wordWrap<RetType> {
  fn wordWrap(self , rsthis: & QListView) -> RetType;
}

  // proto:  bool QListView::wordWrap();
impl<'a> /*trait*/ QListView_wordWrap<i8> for () {
  fn wordWrap(self , rsthis: & QListView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView8wordWrapEv()};
    let mut ret = unsafe {_ZNK9QListView8wordWrapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListView::setRowHidden(int row, bool hide);
impl /*struct*/ QListView {
  pub fn setRowHidden<RetType, T: QListView_setRowHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowHidden(self);
    // return 1;
  }
}

pub trait QListView_setRowHidden<RetType> {
  fn setRowHidden(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setRowHidden(int row, bool hide);
impl<'a> /*trait*/ QListView_setRowHidden<()> for (i32, i8) {
  fn setRowHidden(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView12setRowHiddenEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN9QListView12setRowHiddenEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QListView::clearPropertyFlags();
impl /*struct*/ QListView {
  pub fn clearPropertyFlags<RetType, T: QListView_clearPropertyFlags<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearPropertyFlags(self);
    // return 1;
  }
}

pub trait QListView_clearPropertyFlags<RetType> {
  fn clearPropertyFlags(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::clearPropertyFlags();
impl<'a> /*trait*/ QListView_clearPropertyFlags<()> for () {
  fn clearPropertyFlags(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView18clearPropertyFlagsEv()};
     unsafe {_ZN9QListView18clearPropertyFlagsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QListView::modelColumn();
impl /*struct*/ QListView {
  pub fn modelColumn<RetType, T: QListView_modelColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.modelColumn(self);
    // return 1;
  }
}

pub trait QListView_modelColumn<RetType> {
  fn modelColumn(self , rsthis: & QListView) -> RetType;
}

  // proto:  int QListView::modelColumn();
impl<'a> /*trait*/ QListView_modelColumn<i32> for () {
  fn modelColumn(self , rsthis: & QListView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QListView11modelColumnEv()};
    let mut ret = unsafe {_ZNK9QListView11modelColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QListView::setUniformItemSizes(bool enable);
impl /*struct*/ QListView {
  pub fn setUniformItemSizes<RetType, T: QListView_setUniformItemSizes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUniformItemSizes(self);
    // return 1;
  }
}

pub trait QListView_setUniformItemSizes<RetType> {
  fn setUniformItemSizes(self , rsthis: & QListView) -> RetType;
}

  // proto:  void QListView::setUniformItemSizes(bool enable);
impl<'a> /*trait*/ QListView_setUniformItemSizes<()> for (i8) {
  fn setUniformItemSizes(self , rsthis: & QListView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QListView19setUniformItemSizesEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QListView19setUniformItemSizesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QListView_indexesMoved
pub struct QListView_indexesMoved_signal{poi:u64}
impl /* struct */ QListView {
  pub fn indexesMoved_1(&self) -> QListView_indexesMoved_signal {
     return QListView_indexesMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListView_indexesMoved_signal {
  pub fn connect<T: QListView_indexesMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListView_indexesMoved_signal_connect {
  fn connect(self, sigthis: QListView_indexesMoved_signal);
}

// <= body block end

