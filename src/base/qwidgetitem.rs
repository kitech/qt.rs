// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QSize QWidgetItem::sizeHint();
  fn _ZNK11QWidgetItem8sizeHintEv() -> i32;
  // proto: QSize QWidgetItem::minimumSize();
  fn _ZNK11QWidgetItem11minimumSizeEv() -> i32;
  // proto: bool QWidgetItem::hasHeightForWidth();
  fn _ZNK11QWidgetItem17hasHeightForWidthEv() -> i32;
  // proto: void QWidgetItem::FreeQWidgetItem();
  fn _ZN11QWidgetItemD0Ev() -> i32;
  // proto: void QWidgetItem::NewQWidgetItem(QWidget * w);
  fn _ZN11QWidgetItemC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QWidget * QWidgetItem::widget();
  fn _ZN11QWidgetItem6widgetEv() -> i32;
  // proto: void QWidgetItem::setGeometry(const QRect & );
  fn _ZN11QWidgetItem11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: int QWidgetItem::heightForWidth(int );
  fn _ZNK11QWidgetItem14heightForWidthEi(arg0: c_int) -> i32;
  // proto: void QWidgetItem::NewQWidgetItem(const QWidgetItem & );
  fn _ZN11QWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QSize QWidgetItem::maximumSize();
  fn _ZNK11QWidgetItem11maximumSizeEv() -> i32;
  // proto: bool QWidgetItem::isEmpty();
  fn _ZNK11QWidgetItem7isEmptyEv() -> i32;
  // proto: QRect QWidgetItem::geometry();
  fn _ZNK11QWidgetItem8geometryEv() -> i32;
}

// body block begin
// class sizeof(QWidgetItem)=1
pub struct QWidgetItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWidgetItem {
  pub fn sizeHint<T: QWidgetItem_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QWidgetItem_sizeHint {
  fn sizeHint(self, this: &mut QWidgetItem) -> i32;
}

// proto: QSize QWidgetItem::sizeHint();
impl<'a> /*trait*/ QWidgetItem_sizeHint for () {
  fn sizeHint(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8sizeHintEv()};
    unsafe {_ZNK11QWidgetItem8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn minimumSize<T: QWidgetItem_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QWidgetItem_minimumSize {
  fn minimumSize(self, this: &mut QWidgetItem) -> i32;
}

// proto: QSize QWidgetItem::minimumSize();
impl<'a> /*trait*/ QWidgetItem_minimumSize for () {
  fn minimumSize(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11minimumSizeEv()};
    unsafe {_ZNK11QWidgetItem11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn hasHeightForWidth<T: QWidgetItem_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QWidgetItem_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QWidgetItem) -> i32;
}

// proto: bool QWidgetItem::hasHeightForWidth();
impl<'a> /*trait*/ QWidgetItem_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem17hasHeightForWidthEv()};
    unsafe {_ZNK11QWidgetItem17hasHeightForWidthEv()};
    return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn FreeQWidgetItem<T: QWidgetItem_FreeQWidgetItem>(&mut self, value: T) -> i32 {
    value.FreeQWidgetItem(self);
    return 1;
  }
}

pub trait QWidgetItem_FreeQWidgetItem {
  fn FreeQWidgetItem(self, this: &mut QWidgetItem) -> i32;
}

// proto: void QWidgetItem::FreeQWidgetItem();
impl<'a> /*trait*/ QWidgetItem_FreeQWidgetItem for () {
  fn FreeQWidgetItem(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemD0Ev()};
    unsafe {_ZN11QWidgetItemD0Ev()};
    return 1;
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
  pub fn widget<T: QWidgetItem_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QWidgetItem_widget {
  fn widget(self, this: &mut QWidgetItem) -> i32;
}

// proto: QWidget * QWidgetItem::widget();
impl<'a> /*trait*/ QWidgetItem_widget for () {
  fn widget(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem6widgetEv()};
    unsafe {_ZN11QWidgetItem6widgetEv()};
    return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn setGeometry<T: QWidgetItem_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QWidgetItem_setGeometry {
  fn setGeometry(self, this: &mut QWidgetItem) -> i32;
}

// proto: void QWidgetItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QWidgetItem_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QWidgetItem11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn heightForWidth<T: QWidgetItem_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QWidgetItem_heightForWidth {
  fn heightForWidth(self, this: &mut QWidgetItem) -> i32;
}

// proto: int QWidgetItem::heightForWidth(int );
impl<'a> /*trait*/ QWidgetItem_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QWidgetItem14heightForWidthEi(arg0)};
    return 1;
  }
}

// proto: void QWidgetItem::NewQWidgetItem(const QWidgetItem & );
impl<'a> /*trait*/ QWidgetItem_NewQWidgetItem for (&'a  QWidgetItem) {
  fn NewQWidgetItem(self) -> QWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn maximumSize<T: QWidgetItem_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QWidgetItem_maximumSize {
  fn maximumSize(self, this: &mut QWidgetItem) -> i32;
}

// proto: QSize QWidgetItem::maximumSize();
impl<'a> /*trait*/ QWidgetItem_maximumSize for () {
  fn maximumSize(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11maximumSizeEv()};
    unsafe {_ZNK11QWidgetItem11maximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn isEmpty<T: QWidgetItem_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QWidgetItem_isEmpty {
  fn isEmpty(self, this: &mut QWidgetItem) -> i32;
}

// proto: bool QWidgetItem::isEmpty();
impl<'a> /*trait*/ QWidgetItem_isEmpty for () {
  fn isEmpty(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem7isEmptyEv()};
    unsafe {_ZNK11QWidgetItem7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn geometry<T: QWidgetItem_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QWidgetItem_geometry {
  fn geometry(self, this: &mut QWidgetItem) -> i32;
}

// proto: QRect QWidgetItem::geometry();
impl<'a> /*trait*/ QWidgetItem_geometry for () {
  fn geometry(self, this: &mut QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8geometryEv()};
    unsafe {_ZNK11QWidgetItem8geometryEv()};
    return 1;
  }
}

