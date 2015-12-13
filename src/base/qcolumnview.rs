// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qpoint::QPoint;
use super::qmodelindex::QModelIndex;
use super::qsize::QSize;
use super::qitemselectionmodel::QItemSelectionModel;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QColumnView::NewQColumnView(QWidget * parent);
  fn _ZN11QColumnViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QColumnView::selectAll();
  fn _ZN11QColumnView9selectAllEv(qthis: *mut c_void) ;
  // proto:  void QColumnView::setPreviewWidget(QWidget * widget);
  fn _ZN11QColumnView16setPreviewWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QColumnView::indexAt(const QPoint & point);
  fn _ZNK11QColumnView7indexAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QColumnView::metaObject();
  fn _ZNK11QColumnView10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QSize QColumnView::sizeHint();
  fn _ZNK11QColumnView8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<int> QColumnView::columnWidths();
  fn _ZNK11QColumnView12columnWidthsEv(qthis: *mut c_void) ;
  // proto:  void QColumnView::setResizeGripsVisible(bool visible);
  fn _ZN11QColumnView21setResizeGripsVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QColumnView::resizeGripsVisible();
  fn _ZNK11QColumnView18resizeGripsVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QColumnView::NewQColumnView(const QColumnView & );
  fn _ZN11QColumnViewC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QColumnView::updatePreviewWidget(const QModelIndex & index);
  fn _ZN11QColumnView19updatePreviewWidgetERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QColumnView::setRootIndex(const QModelIndex & index);
  fn _ZN11QColumnView12setRootIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QColumnView::previewWidget();
  fn _ZNK11QColumnView13previewWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QColumnView::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN11QColumnView17setSelectionModelEP19QItemSelectionModel(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QColumnView::visualRect(const QModelIndex & index);
  fn _ZNK11QColumnView10visualRectERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QColumnView::FreeQColumnView();
  fn _ZN11QColumnViewD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QColumnView)=1
pub struct QColumnView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QColumnView {
  pub fn NewQColumnView<T: QColumnView_NewQColumnView>(value: T) -> QColumnView {
    let rsthis = value.NewQColumnView();
    return rsthis;
    // return 1;
  }
}

pub trait QColumnView_NewQColumnView {
  fn NewQColumnView(self) -> QColumnView;
}

// proto: void QColumnView::NewQColumnView(QWidget * parent);
impl<'a> /*trait*/ QColumnView_NewQColumnView for (&'a mut QWidget) {
  fn NewQColumnView(self) -> QColumnView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnViewC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QColumnViewC1EP7QWidget(qthis, arg0)};
    let rsthis = QColumnView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn selectAll<T: QColumnView_selectAll>(&mut self, value: T)  {
     value.selectAll(self);
    // return 1;
  }
}

pub trait QColumnView_selectAll {
  fn selectAll(self, rsthis: &mut QColumnView) ;
}

// proto:  void QColumnView::selectAll();
impl<'a> /*trait*/ QColumnView_selectAll for () {
  fn selectAll(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView9selectAllEv()};
     unsafe {_ZN11QColumnView9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn setPreviewWidget<T: QColumnView_setPreviewWidget>(&mut self, value: T)  {
     value.setPreviewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_setPreviewWidget {
  fn setPreviewWidget(self, rsthis: &mut QColumnView) ;
}

// proto:  void QColumnView::setPreviewWidget(QWidget * widget);
impl<'a> /*trait*/ QColumnView_setPreviewWidget for (&'a mut QWidget) {
  fn setPreviewWidget(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView16setPreviewWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView16setPreviewWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn indexAt<T: QColumnView_indexAt>(&mut self, value: T) -> QModelIndex {
    return value.indexAt(self);
    // return 1;
  }
}

pub trait QColumnView_indexAt {
  fn indexAt(self, rsthis: &mut QColumnView) -> QModelIndex;
}

// proto:  QModelIndex QColumnView::indexAt(const QPoint & point);
impl<'a> /*trait*/ QColumnView_indexAt for (&'a  QPoint) {
  fn indexAt(self, rsthis: &mut QColumnView) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView7indexAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QColumnView7indexAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn metaObject<T: QColumnView_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QColumnView_metaObject {
  fn metaObject(self, rsthis: &mut QColumnView) ;
}

// proto:  const QMetaObject * QColumnView::metaObject();
impl<'a> /*trait*/ QColumnView_metaObject for () {
  fn metaObject(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView10metaObjectEv()};
     unsafe {_ZNK11QColumnView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn sizeHint<T: QColumnView_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QColumnView_sizeHint {
  fn sizeHint(self, rsthis: &mut QColumnView) -> QSize;
}

// proto:  QSize QColumnView::sizeHint();
impl<'a> /*trait*/ QColumnView_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QColumnView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QColumnView8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn columnWidths<T: QColumnView_columnWidths>(&mut self, value: T)  {
     value.columnWidths(self);
    // return 1;
  }
}

pub trait QColumnView_columnWidths {
  fn columnWidths(self, rsthis: &mut QColumnView) ;
}

// proto:  QList<int> QColumnView::columnWidths();
impl<'a> /*trait*/ QColumnView_columnWidths for () {
  fn columnWidths(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView12columnWidthsEv()};
     unsafe {_ZNK11QColumnView12columnWidthsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn setResizeGripsVisible<T: QColumnView_setResizeGripsVisible>(&mut self, value: T)  {
     value.setResizeGripsVisible(self);
    // return 1;
  }
}

pub trait QColumnView_setResizeGripsVisible {
  fn setResizeGripsVisible(self, rsthis: &mut QColumnView) ;
}

// proto:  void QColumnView::setResizeGripsVisible(bool visible);
impl<'a> /*trait*/ QColumnView_setResizeGripsVisible for (i8) {
  fn setResizeGripsVisible(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView21setResizeGripsVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QColumnView21setResizeGripsVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn resizeGripsVisible<T: QColumnView_resizeGripsVisible>(&mut self, value: T) -> i8 {
    return value.resizeGripsVisible(self);
    // return 1;
  }
}

pub trait QColumnView_resizeGripsVisible {
  fn resizeGripsVisible(self, rsthis: &mut QColumnView) -> i8;
}

// proto:  bool QColumnView::resizeGripsVisible();
impl<'a> /*trait*/ QColumnView_resizeGripsVisible for () {
  fn resizeGripsVisible(self, rsthis: &mut QColumnView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView18resizeGripsVisibleEv()};
    let mut ret = unsafe {_ZNK11QColumnView18resizeGripsVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QColumnView::NewQColumnView(const QColumnView & );
impl<'a> /*trait*/ QColumnView_NewQColumnView for (&'a  QColumnView) {
  fn NewQColumnView(self) -> QColumnView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnViewC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QColumnViewC1ERKS_(qthis, arg0)};
    let rsthis = QColumnView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn updatePreviewWidget<T: QColumnView_updatePreviewWidget>(&mut self, value: T)  {
     value.updatePreviewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_updatePreviewWidget {
  fn updatePreviewWidget(self, rsthis: &mut QColumnView) ;
}

// proto:  void QColumnView::updatePreviewWidget(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_updatePreviewWidget for (&'a  QModelIndex) {
  fn updatePreviewWidget(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView19updatePreviewWidgetERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView19updatePreviewWidgetERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn setRootIndex<T: QColumnView_setRootIndex>(&mut self, value: T)  {
     value.setRootIndex(self);
    // return 1;
  }
}

pub trait QColumnView_setRootIndex {
  fn setRootIndex(self, rsthis: &mut QColumnView) ;
}

// proto:  void QColumnView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_setRootIndex for (&'a  QModelIndex) {
  fn setRootIndex(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn previewWidget<T: QColumnView_previewWidget>(&mut self, value: T) -> QWidget {
    return value.previewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_previewWidget {
  fn previewWidget(self, rsthis: &mut QColumnView) -> QWidget;
}

// proto:  QWidget * QColumnView::previewWidget();
impl<'a> /*trait*/ QColumnView_previewWidget for () {
  fn previewWidget(self, rsthis: &mut QColumnView) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView13previewWidgetEv()};
    let mut ret = unsafe {_ZNK11QColumnView13previewWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn setSelectionModel<T: QColumnView_setSelectionModel>(&mut self, value: T)  {
     value.setSelectionModel(self);
    // return 1;
  }
}

pub trait QColumnView_setSelectionModel {
  fn setSelectionModel(self, rsthis: &mut QColumnView) ;
}

// proto:  void QColumnView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QColumnView_setSelectionModel for (&'a mut QItemSelectionModel) {
  fn setSelectionModel(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn visualRect<T: QColumnView_visualRect>(&mut self, value: T) -> QRect {
    return value.visualRect(self);
    // return 1;
  }
}

pub trait QColumnView_visualRect {
  fn visualRect(self, rsthis: &mut QColumnView) -> QRect;
}

// proto:  QRect QColumnView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_visualRect for (&'a  QModelIndex) {
  fn visualRect(self, rsthis: &mut QColumnView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView10visualRectERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QColumnView10visualRectERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn FreeQColumnView<T: QColumnView_FreeQColumnView>(&mut self, value: T)  {
     value.FreeQColumnView(self);
    // return 1;
  }
}

pub trait QColumnView_FreeQColumnView {
  fn FreeQColumnView(self, rsthis: &mut QColumnView) ;
}

// proto:  void QColumnView::FreeQColumnView();
impl<'a> /*trait*/ QColumnView_FreeQColumnView for () {
  fn FreeQColumnView(self, rsthis: &mut QColumnView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnViewD0Ev()};
     unsafe {_ZN11QColumnViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

