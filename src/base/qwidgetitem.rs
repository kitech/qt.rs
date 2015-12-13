// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qwidget::QWidget;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QSize QWidgetItem::sizeHint();
  fn _ZNK11QWidgetItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QWidgetItem::minimumSize();
  fn _ZNK11QWidgetItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidgetItem::hasHeightForWidth();
  fn _ZNK11QWidgetItem17hasHeightForWidthEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidgetItem::FreeQWidgetItem();
  fn _ZN11QWidgetItemD0Ev(qthis: *mut c_void) ;
  // proto:  void QWidgetItem::NewQWidgetItem(QWidget * w);
  fn _ZN11QWidgetItemC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QWidgetItem::widget();
  fn _ZN11QWidgetItem6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidgetItem::setGeometry(const QRect & );
  fn _ZN11QWidgetItem11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QWidgetItem::heightForWidth(int );
  fn _ZNK11QWidgetItem14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QWidgetItem::NewQWidgetItem(const QWidgetItem & );
  fn _ZN11QWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QWidgetItem::maximumSize();
  fn _ZNK11QWidgetItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidgetItem::isEmpty();
  fn _ZNK11QWidgetItem7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRect QWidgetItem::geometry();
  fn _ZNK11QWidgetItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QWidgetItem)=1
pub struct QWidgetItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWidgetItem {
  pub fn sizeHint<T: QWidgetItem_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QWidgetItem_sizeHint {
  fn sizeHint(self, rsthis: &mut QWidgetItem) -> QSize;
}

// proto:  QSize QWidgetItem::sizeHint();
impl<'a> /*trait*/ QWidgetItem_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn minimumSize<T: QWidgetItem_minimumSize>(&mut self, value: T) -> QSize {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QWidgetItem_minimumSize {
  fn minimumSize(self, rsthis: &mut QWidgetItem) -> QSize;
}

// proto:  QSize QWidgetItem::minimumSize();
impl<'a> /*trait*/ QWidgetItem_minimumSize for () {
  fn minimumSize(self, rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn hasHeightForWidth<T: QWidgetItem_hasHeightForWidth>(&mut self, value: T) -> i8 {
    return value.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItem_hasHeightForWidth {
  fn hasHeightForWidth(self, rsthis: &mut QWidgetItem) -> i8;
}

// proto:  bool QWidgetItem::hasHeightForWidth();
impl<'a> /*trait*/ QWidgetItem_hasHeightForWidth for () {
  fn hasHeightForWidth(self, rsthis: &mut QWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn FreeQWidgetItem<T: QWidgetItem_FreeQWidgetItem>(&mut self, value: T)  {
     value.FreeQWidgetItem(self);
    // return 1;
  }
}

pub trait QWidgetItem_FreeQWidgetItem {
  fn FreeQWidgetItem(self, rsthis: &mut QWidgetItem) ;
}

// proto:  void QWidgetItem::FreeQWidgetItem();
impl<'a> /*trait*/ QWidgetItem_FreeQWidgetItem for () {
  fn FreeQWidgetItem(self, rsthis: &mut QWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemD0Ev()};
     unsafe {_ZN11QWidgetItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn NewQWidgetItem<T: QWidgetItem_NewQWidgetItem>(value: T) -> QWidgetItem {
    let rsthis = value.NewQWidgetItem();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItem_NewQWidgetItem {
  fn NewQWidgetItem(self) -> QWidgetItem;
}

// proto: void QWidgetItem::NewQWidgetItem(QWidget * w);
impl<'a> /*trait*/ QWidgetItem_NewQWidgetItem for (&'a mut QWidget) {
  fn NewQWidgetItem(self) -> QWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWidgetItemC1EP7QWidget(qthis, arg0)};
    let rsthis = QWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn widget<T: QWidgetItem_widget>(&mut self, value: T) -> QWidget {
    return value.widget(self);
    // return 1;
  }
}

pub trait QWidgetItem_widget {
  fn widget(self, rsthis: &mut QWidgetItem) -> QWidget;
}

// proto:  QWidget * QWidgetItem::widget();
impl<'a> /*trait*/ QWidgetItem_widget for () {
  fn widget(self, rsthis: &mut QWidgetItem) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem6widgetEv()};
    let mut ret = unsafe {_ZN11QWidgetItem6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn setGeometry<T: QWidgetItem_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QWidgetItem_setGeometry {
  fn setGeometry(self, rsthis: &mut QWidgetItem) ;
}

// proto:  void QWidgetItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QWidgetItem_setGeometry for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QWidgetItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn heightForWidth<T: QWidgetItem_heightForWidth>(&mut self, value: T) -> i32 {
    return value.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItem_heightForWidth {
  fn heightForWidth(self, rsthis: &mut QWidgetItem) -> i32;
}

// proto:  int QWidgetItem::heightForWidth(int );
impl<'a> /*trait*/ QWidgetItem_heightForWidth for (i32) {
  fn heightForWidth(self, rsthis: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QWidgetItem14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QWidgetItem::NewQWidgetItem(const QWidgetItem & );
impl<'a> /*trait*/ QWidgetItem_NewQWidgetItem for (&'a  QWidgetItem) {
  fn NewQWidgetItem(self) -> QWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn maximumSize<T: QWidgetItem_maximumSize>(&mut self, value: T) -> QSize {
    return value.maximumSize(self);
    // return 1;
  }
}

pub trait QWidgetItem_maximumSize {
  fn maximumSize(self, rsthis: &mut QWidgetItem) -> QSize;
}

// proto:  QSize QWidgetItem::maximumSize();
impl<'a> /*trait*/ QWidgetItem_maximumSize for () {
  fn maximumSize(self, rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn isEmpty<T: QWidgetItem_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QWidgetItem_isEmpty {
  fn isEmpty(self, rsthis: &mut QWidgetItem) -> i8;
}

// proto:  bool QWidgetItem::isEmpty();
impl<'a> /*trait*/ QWidgetItem_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn geometry<T: QWidgetItem_geometry>(&mut self, value: T) -> QRect {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QWidgetItem_geometry {
  fn geometry(self, rsthis: &mut QWidgetItem) -> QRect;
}

// proto:  QRect QWidgetItem::geometry();
impl<'a> /*trait*/ QWidgetItem_geometry for () {
  fn geometry(self, rsthis: &mut QWidgetItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

