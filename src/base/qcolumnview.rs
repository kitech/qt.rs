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
  pub fn selectAll<RetType, T: QColumnView_selectAll<RetType>>(&mut self, value: T) -> RetType {
    return value.selectAll(self);
    // return 1;
  }
}

pub trait QColumnView_selectAll<RetType> {
  fn selectAll(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  void QColumnView::selectAll();
impl<'a> /*trait*/ QColumnView_selectAll<()> for () {
  fn selectAll(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView9selectAllEv()};
     unsafe {_ZN11QColumnView9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn setPreviewWidget<RetType, T: QColumnView_setPreviewWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.setPreviewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_setPreviewWidget<RetType> {
  fn setPreviewWidget(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  void QColumnView::setPreviewWidget(QWidget * widget);
impl<'a> /*trait*/ QColumnView_setPreviewWidget<()> for (&'a mut QWidget) {
  fn setPreviewWidget(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView16setPreviewWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView16setPreviewWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn indexAt<RetType, T: QColumnView_indexAt<RetType>>(&mut self, value: T) -> RetType {
    return value.indexAt(self);
    // return 1;
  }
}

pub trait QColumnView_indexAt<RetType> {
  fn indexAt(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  QModelIndex QColumnView::indexAt(const QPoint & point);
impl<'a> /*trait*/ QColumnView_indexAt<QModelIndex> for (&'a  QPoint) {
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
  pub fn metaObject<RetType, T: QColumnView_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QColumnView_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  const QMetaObject * QColumnView::metaObject();
impl<'a> /*trait*/ QColumnView_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView10metaObjectEv()};
     unsafe {_ZNK11QColumnView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn sizeHint<RetType, T: QColumnView_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QColumnView_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  QSize QColumnView::sizeHint();
impl<'a> /*trait*/ QColumnView_sizeHint<QSize> for () {
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
  pub fn columnWidths<RetType, T: QColumnView_columnWidths<RetType>>(&mut self, value: T) -> RetType {
    return value.columnWidths(self);
    // return 1;
  }
}

pub trait QColumnView_columnWidths<RetType> {
  fn columnWidths(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  QList<int> QColumnView::columnWidths();
impl<'a> /*trait*/ QColumnView_columnWidths<()> for () {
  fn columnWidths(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QColumnView12columnWidthsEv()};
     unsafe {_ZNK11QColumnView12columnWidthsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn setResizeGripsVisible<RetType, T: QColumnView_setResizeGripsVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setResizeGripsVisible(self);
    // return 1;
  }
}

pub trait QColumnView_setResizeGripsVisible<RetType> {
  fn setResizeGripsVisible(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  void QColumnView::setResizeGripsVisible(bool visible);
impl<'a> /*trait*/ QColumnView_setResizeGripsVisible<()> for (i8) {
  fn setResizeGripsVisible(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView21setResizeGripsVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QColumnView21setResizeGripsVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn resizeGripsVisible<RetType, T: QColumnView_resizeGripsVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.resizeGripsVisible(self);
    // return 1;
  }
}

pub trait QColumnView_resizeGripsVisible<RetType> {
  fn resizeGripsVisible(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  bool QColumnView::resizeGripsVisible();
impl<'a> /*trait*/ QColumnView_resizeGripsVisible<i8> for () {
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
  pub fn updatePreviewWidget<RetType, T: QColumnView_updatePreviewWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.updatePreviewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_updatePreviewWidget<RetType> {
  fn updatePreviewWidget(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  void QColumnView::updatePreviewWidget(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_updatePreviewWidget<()> for (&'a  QModelIndex) {
  fn updatePreviewWidget(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView19updatePreviewWidgetERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView19updatePreviewWidgetERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn setRootIndex<RetType, T: QColumnView_setRootIndex<RetType>>(&mut self, value: T) -> RetType {
    return value.setRootIndex(self);
    // return 1;
  }
}

pub trait QColumnView_setRootIndex<RetType> {
  fn setRootIndex(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  void QColumnView::setRootIndex(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_setRootIndex<()> for (&'a  QModelIndex) {
  fn setRootIndex(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView12setRootIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView12setRootIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn previewWidget<RetType, T: QColumnView_previewWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.previewWidget(self);
    // return 1;
  }
}

pub trait QColumnView_previewWidget<RetType> {
  fn previewWidget(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  QWidget * QColumnView::previewWidget();
impl<'a> /*trait*/ QColumnView_previewWidget<QWidget> for () {
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
  pub fn setSelectionModel<RetType, T: QColumnView_setSelectionModel<RetType>>(&mut self, value: T) -> RetType {
    return value.setSelectionModel(self);
    // return 1;
  }
}

pub trait QColumnView_setSelectionModel<RetType> {
  fn setSelectionModel(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  void QColumnView::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QColumnView_setSelectionModel<()> for (&'a mut QItemSelectionModel) {
  fn setSelectionModel(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnView17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QColumnView17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QColumnView {
  pub fn visualRect<RetType, T: QColumnView_visualRect<RetType>>(&mut self, value: T) -> RetType {
    return value.visualRect(self);
    // return 1;
  }
}

pub trait QColumnView_visualRect<RetType> {
  fn visualRect(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  QRect QColumnView::visualRect(const QModelIndex & index);
impl<'a> /*trait*/ QColumnView_visualRect<QRect> for (&'a  QModelIndex) {
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
  pub fn FreeQColumnView<RetType, T: QColumnView_FreeQColumnView<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQColumnView(self);
    // return 1;
  }
}

pub trait QColumnView_FreeQColumnView<RetType> {
  fn FreeQColumnView(self, rsthis: &mut QColumnView) -> RetType;
}

// proto:  void QColumnView::FreeQColumnView();
impl<'a> /*trait*/ QColumnView_FreeQColumnView<()> for () {
  fn FreeQColumnView(self, rsthis: &mut QColumnView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QColumnViewD0Ev()};
     unsafe {_ZN11QColumnViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

