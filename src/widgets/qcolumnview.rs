// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtWidgets/qcolumnview.h
// dst-file: /src/widgets/qcolumnview.rs
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
use super::qwidget::QWidget; // 773
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
use super::super::core::qitemselectionmodel::QItemSelectionModel; // 771
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QColumnView_Class_Size() -> c_int;
  // proto:  void QColumnView::QColumnView(QWidget * parent);
  fn dector_ZN11QColumnViewC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QColumnViewC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QColumnView::selectAll();
  fn _ZN11QColumnView9selectAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QColumnView::setPreviewWidget(QWidget * widget);
  fn _ZN11QColumnView16setPreviewWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QColumnView::indexAt(const QPoint & point);
  fn _ZNK11QColumnView7indexAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QColumnView::metaObject();
  fn _ZNK11QColumnView10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QColumnView::sizeHint();
  fn _ZNK11QColumnView8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<int> QColumnView::columnWidths();
  fn _ZNK11QColumnView12columnWidthsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QColumnView::setResizeGripsVisible(bool visible);
  fn _ZN11QColumnView21setResizeGripsVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QColumnView::resizeGripsVisible();
  fn _ZNK11QColumnView18resizeGripsVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QColumnView::QColumnView(const QColumnView & );
  fn dector_ZN11QColumnViewC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QColumnViewC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QColumnView::setModel(QAbstractItemModel * model);
  fn _ZN11QColumnView8setModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QColumnView::updatePreviewWidget(const QModelIndex & index);
  fn _ZN11QColumnView19updatePreviewWidgetERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QColumnView::setRootIndex(const QModelIndex & index);
  fn _ZN11QColumnView12setRootIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QColumnView::previewWidget();
  fn _ZNK11QColumnView13previewWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QColumnView::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN11QColumnView17setSelectionModelEP19QItemSelectionModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRect QColumnView::visualRect(const QModelIndex & index);
  fn _ZNK11QColumnView10visualRectERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QColumnView::~QColumnView();
  fn _ZN11QColumnViewD0Ev(qthis: u64 /* *mut c_void*/);
  fn QColumnView_SlotProxy_connect__ZN11QColumnView19updatePreviewWidgetERK11QModelIndex(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QColumnView)=1
#[derive(Default)]
pub struct QColumnView {
  qbase: QAbstractItemView,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _updatePreviewWidget_1: QColumnView_updatePreviewWidget_signal,
}

impl /*struct*/ QColumnView {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QColumnView {
    return QColumnView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QColumnView {
  type Target = QAbstractItemView;

  fn deref(&self) -> &QAbstractItemView {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemView> for QColumnView {
  fn as_ref(& self) -> & QAbstractItemView {
    return & self.qbase;
  }
}
  // proto:  void QColumnView::QColumnView(QWidget * parent);
impl /*struct*/ QColumnView {
  pub fn new<T: QColumnView_new>(value: T) -> QColumnView {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QColumnView_new {
  fn new(self) -> QColumnView;
}

  // proto:  void QColumnView::QColumnView(QWidget * parent);
impl<'a> /*trait*/ QColumnView_new for (&'a QWidget) {
  fn new(self) -> QColumnView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnViewC1EP7QWidget()};
    let ctysz: c_int = unsafe{QColumnView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QColumnViewC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QColumnViewC1EP7QWidget(arg0)} as u64;
    let rsthis = QColumnView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QColumnView::selectAll();
impl /*struct*/ QColumnView {
  pub fn selectAll<RetType, T: QColumnView_selectAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectAll(self);
    // return 1;
  }
}

pub trait QColumnView_selectAll<RetType> {
  fn selectAll(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  void QColumnView::selectAll();
impl<'a> /*trait*/ QColumnView_selectAll<()> for () {
  fn selectAll(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView9selectAllEv()};
     unsafe {_ZN11QColumnView9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QColumnView::setPreviewWidget(QWidget * widget);
impl /*struct*/ QColumnView {
  pub fn setPreviewWidget<RetType, T: QColumnView_setPreviewWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPreviewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_setPreviewWidget<RetType> {
  fn setPreviewWidget(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  void QColumnView::setPreviewWidget(QWidget * widget);
impl<'a> /*trait*/ QColumnView_setPreviewWidget<()> for (&'a QWidget) {
  fn setPreviewWidget(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView16setPreviewWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView16setPreviewWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QColumnView::indexAt(const QPoint & point);
impl /*struct*/ QColumnView {
  pub fn indexAt<RetType, T: QColumnView_indexAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexAt(self);
    // return 1;
  }
}

pub trait QColumnView_indexAt<RetType> {
  fn indexAt(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  QModelIndex QColumnView::indexAt(const QPoint & point);
impl<'a> /*trait*/ QColumnView_indexAt<QModelIndex> for (&'a QPoint) {
  fn indexAt(self , rsthis: & QColumnView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QColumnView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QColumnView::metaObject();
impl /*struct*/ QColumnView {
  pub fn metaObject<RetType, T: QColumnView_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QColumnView_metaObject<RetType> {
  fn metaObject(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  const QMetaObject * QColumnView::metaObject();
impl<'a> /*trait*/ QColumnView_metaObject<()> for () {
  fn metaObject(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView10metaObjectEv()};
     unsafe {_ZNK11QColumnView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QColumnView::sizeHint();
impl /*struct*/ QColumnView {
  pub fn sizeHint<RetType, T: QColumnView_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QColumnView_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  QSize QColumnView::sizeHint();
impl<'a> /*trait*/ QColumnView_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QColumnView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QColumnView8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<int> QColumnView::columnWidths();
impl /*struct*/ QColumnView {
  pub fn columnWidths<RetType, T: QColumnView_columnWidths<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnWidths(self);
    // return 1;
  }
}

pub trait QColumnView_columnWidths<RetType> {
  fn columnWidths(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  QList<int> QColumnView::columnWidths();
impl<'a> /*trait*/ QColumnView_columnWidths<()> for () {
  fn columnWidths(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView12columnWidthsEv()};
     unsafe {_ZNK11QColumnView12columnWidthsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QColumnView::setResizeGripsVisible(bool visible);
impl /*struct*/ QColumnView {
  pub fn setResizeGripsVisible<RetType, T: QColumnView_setResizeGripsVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setResizeGripsVisible(self);
    // return 1;
  }
}

pub trait QColumnView_setResizeGripsVisible<RetType> {
  fn setResizeGripsVisible(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  void QColumnView::setResizeGripsVisible(bool visible);
impl<'a> /*trait*/ QColumnView_setResizeGripsVisible<()> for (i8) {
  fn setResizeGripsVisible(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView21setResizeGripsVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QColumnView21setResizeGripsVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QColumnView::resizeGripsVisible();
impl /*struct*/ QColumnView {
  pub fn resizeGripsVisible<RetType, T: QColumnView_resizeGripsVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resizeGripsVisible(self);
    // return 1;
  }
}

pub trait QColumnView_resizeGripsVisible<RetType> {
  fn resizeGripsVisible(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  bool QColumnView::resizeGripsVisible();
impl<'a> /*trait*/ QColumnView_resizeGripsVisible<i8> for () {
  fn resizeGripsVisible(self , rsthis: & QColumnView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView18resizeGripsVisibleEv()};
    let mut ret = unsafe {_ZNK11QColumnView18resizeGripsVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QColumnView::QColumnView(const QColumnView & );
impl<'a> /*trait*/ QColumnView_new for (&'a QColumnView) {
  fn new(self) -> QColumnView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnViewC1ERKS_()};
    let ctysz: c_int = unsafe{QColumnView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QColumnViewC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QColumnViewC1ERKS_(arg0)} as u64;
    let rsthis = QColumnView{qbase: QAbstractItemView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QColumnView::setModel(QAbstractItemModel * model);
impl /*struct*/ QColumnView {
  pub fn setModel<RetType, T: QColumnView_setModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModel(self);
    // return 1;
  }
}

pub trait QColumnView_setModel<RetType> {
  fn setModel(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  void QColumnView::setModel(QAbstractItemModel * model);
impl<'a> /*trait*/ QColumnView_setModel<()> for (&'a QAbstractItemModel) {
  fn setModel(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView8setModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView8setModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QColumnView::updatePreviewWidget(const QModelIndex & index);
impl /*struct*/ QColumnView {
  pub fn updatePreviewWidget<RetType, T: QColumnView_updatePreviewWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updatePreviewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_updatePreviewWidget<RetType> {
  fn updatePreviewWidget(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  void QColumnView::updatePreviewWidget(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_updatePreviewWidget<()> for (&'a QModelIndex) {
  fn updatePreviewWidget(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView19updatePreviewWidgetERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView19updatePreviewWidgetERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QColumnView::setRootIndex(const QModelIndex & index);
impl /*struct*/ QColumnView {
  pub fn setRootIndex<RetType, T: QColumnView_setRootIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootIndex(self);
    // return 1;
  }
}

pub trait QColumnView_setRootIndex<RetType> {
  fn setRootIndex(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  void QColumnView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_setRootIndex<()> for (&'a QModelIndex) {
  fn setRootIndex(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QColumnView::previewWidget();
impl /*struct*/ QColumnView {
  pub fn previewWidget<RetType, T: QColumnView_previewWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.previewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_previewWidget<RetType> {
  fn previewWidget(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  QWidget * QColumnView::previewWidget();
impl<'a> /*trait*/ QColumnView_previewWidget<QWidget> for () {
  fn previewWidget(self , rsthis: & QColumnView) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView13previewWidgetEv()};
    let mut ret = unsafe {_ZNK11QColumnView13previewWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QColumnView::setSelectionModel(QItemSelectionModel * selectionModel);
impl /*struct*/ QColumnView {
  pub fn setSelectionModel<RetType, T: QColumnView_setSelectionModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel(self);
    // return 1;
  }
}

pub trait QColumnView_setSelectionModel<RetType> {
  fn setSelectionModel(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  void QColumnView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QColumnView_setSelectionModel<()> for (&'a QItemSelectionModel) {
  fn setSelectionModel(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QColumnView::visualRect(const QModelIndex & index);
impl /*struct*/ QColumnView {
  pub fn visualRect<RetType, T: QColumnView_visualRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualRect(self);
    // return 1;
  }
}

pub trait QColumnView_visualRect<RetType> {
  fn visualRect(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  QRect QColumnView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_visualRect<QRect> for (&'a QModelIndex) {
  fn visualRect(self , rsthis: & QColumnView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QColumnView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QColumnView::~QColumnView();
impl /*struct*/ QColumnView {
  pub fn free<RetType, T: QColumnView_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QColumnView_free<RetType> {
  fn free(self , rsthis: & QColumnView) -> RetType;
}

  // proto:  void QColumnView::~QColumnView();
impl<'a> /*trait*/ QColumnView_free<()> for () {
  fn free(self , rsthis: & QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnViewD0Ev()};
     unsafe {_ZN11QColumnViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QColumnView_updatePreviewWidget
pub struct QColumnView_updatePreviewWidget_signal{poi:u64}
impl /* struct */ QColumnView {
  pub fn updatePreviewWidget_1(&self) -> QColumnView_updatePreviewWidget_signal {
     return QColumnView_updatePreviewWidget_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QColumnView_updatePreviewWidget_signal {
  pub fn connect<T: QColumnView_updatePreviewWidget_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QColumnView_updatePreviewWidget_signal_connect {
  fn connect(self, sigthis: QColumnView_updatePreviewWidget_signal);
}

// updatePreviewWidget(const class QModelIndex &)
extern fn QColumnView_updatePreviewWidget_signal_connect_cb_0(rsfptr:fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QColumnView_updatePreviewWidget_signal_connect_cb_box_0(rsfptr_raw:*mut Fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QColumnView_updatePreviewWidget_signal_connect for fn(QModelIndex) {
  fn connect(self, sigthis: QColumnView_updatePreviewWidget_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QColumnView_updatePreviewWidget_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QColumnView_SlotProxy_connect__ZN11QColumnView19updatePreviewWidgetERK11QModelIndex(arg0, arg1, arg2)};
  }
}
impl /* trait */ QColumnView_updatePreviewWidget_signal_connect for Box<Fn(QModelIndex)> {
  fn connect(self, sigthis: QColumnView_updatePreviewWidget_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QColumnView_updatePreviewWidget_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QColumnView_SlotProxy_connect__ZN11QColumnView19updatePreviewWidgetERK11QModelIndex(arg0, arg1, arg2)};
  }
}
// <= body block end

