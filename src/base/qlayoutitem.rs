// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qspaceritem::QSpacerItem;
use super::qsize::QSize;
use super::qwidget::QWidget;
use super::qrect::QRect;
use super::qlayout::QLayout;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QSpacerItem * QLayoutItem::spacerItem();
  fn _ZN11QLayoutItem10spacerItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QLayoutItem::minimumSize();
  fn _ZNK11QLayoutItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QLayoutItem::widget();
  fn _ZN11QLayoutItem6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayoutItem::invalidate();
  fn _ZN11QLayoutItem10invalidateEv(qthis: *mut c_void) ;
  // proto:  void QLayoutItem::setGeometry(const QRect & );
  fn _ZN11QLayoutItem11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QLayout * QLayoutItem::layout();
  fn _ZN11QLayoutItem6layoutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QLayoutItem::isEmpty();
  fn _ZNK11QLayoutItem7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QLayoutItem::sizeHint();
  fn _ZNK11QLayoutItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayoutItem::FreeQLayoutItem();
  fn _ZN11QLayoutItemD0Ev(qthis: *mut c_void) ;
  // proto:  bool QLayoutItem::hasHeightForWidth();
  fn _ZNK11QLayoutItem17hasHeightForWidthEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QLayoutItem::heightForWidth(int );
  fn _ZNK11QLayoutItem14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QRect QLayoutItem::geometry();
  fn _ZNK11QLayoutItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QLayoutItem::maximumSize();
  fn _ZNK11QLayoutItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLayoutItem::minimumHeightForWidth(int );
  fn _ZNK11QLayoutItem21minimumHeightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
}

// body block begin
// class sizeof(QLayoutItem)=1
pub struct QLayoutItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLayoutItem {
  pub fn spacerItem<T: QLayoutItem_spacerItem>(&mut self, value: T) -> QSpacerItem {
    return value.spacerItem(self);
    // return 1;
  }
}

pub trait QLayoutItem_spacerItem {
  fn spacerItem(self, rsthis: &mut QLayoutItem) -> QSpacerItem;
}

// proto:  QSpacerItem * QLayoutItem::spacerItem();
impl<'a> /*trait*/ QLayoutItem_spacerItem for () {
  fn spacerItem(self, rsthis: &mut QLayoutItem) -> QSpacerItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10spacerItemEv()};
    let mut ret = unsafe {_ZN11QLayoutItem10spacerItemEv(rsthis.qclsinst)};
    let mut ret1 = QSpacerItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn minimumSize<T: QLayoutItem_minimumSize>(&mut self, value: T) -> QSize {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QLayoutItem_minimumSize {
  fn minimumSize(self, rsthis: &mut QLayoutItem) -> QSize;
}

// proto:  QSize QLayoutItem::minimumSize();
impl<'a> /*trait*/ QLayoutItem_minimumSize for () {
  fn minimumSize(self, rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn widget<T: QLayoutItem_widget>(&mut self, value: T) -> QWidget {
    return value.widget(self);
    // return 1;
  }
}

pub trait QLayoutItem_widget {
  fn widget(self, rsthis: &mut QLayoutItem) -> QWidget;
}

// proto:  QWidget * QLayoutItem::widget();
impl<'a> /*trait*/ QLayoutItem_widget for () {
  fn widget(self, rsthis: &mut QLayoutItem) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6widgetEv()};
    let mut ret = unsafe {_ZN11QLayoutItem6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn invalidate<T: QLayoutItem_invalidate>(&mut self, value: T)  {
     value.invalidate(self);
    // return 1;
  }
}

pub trait QLayoutItem_invalidate {
  fn invalidate(self, rsthis: &mut QLayoutItem) ;
}

// proto:  void QLayoutItem::invalidate();
impl<'a> /*trait*/ QLayoutItem_invalidate for () {
  fn invalidate(self, rsthis: &mut QLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10invalidateEv()};
     unsafe {_ZN11QLayoutItem10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn setGeometry<T: QLayoutItem_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QLayoutItem_setGeometry {
  fn setGeometry(self, rsthis: &mut QLayoutItem) ;
}

// proto:  void QLayoutItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QLayoutItem_setGeometry for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QLayoutItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn layout<T: QLayoutItem_layout>(&mut self, value: T) -> QLayout {
    return value.layout(self);
    // return 1;
  }
}

pub trait QLayoutItem_layout {
  fn layout(self, rsthis: &mut QLayoutItem) -> QLayout;
}

// proto:  QLayout * QLayoutItem::layout();
impl<'a> /*trait*/ QLayoutItem_layout for () {
  fn layout(self, rsthis: &mut QLayoutItem) -> QLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6layoutEv()};
    let mut ret = unsafe {_ZN11QLayoutItem6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QLayout{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn isEmpty<T: QLayoutItem_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QLayoutItem_isEmpty {
  fn isEmpty(self, rsthis: &mut QLayoutItem) -> i8;
}

// proto:  bool QLayoutItem::isEmpty();
impl<'a> /*trait*/ QLayoutItem_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn sizeHint<T: QLayoutItem_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QLayoutItem_sizeHint {
  fn sizeHint(self, rsthis: &mut QLayoutItem) -> QSize;
}

// proto:  QSize QLayoutItem::sizeHint();
impl<'a> /*trait*/ QLayoutItem_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn FreeQLayoutItem<T: QLayoutItem_FreeQLayoutItem>(&mut self, value: T)  {
     value.FreeQLayoutItem(self);
    // return 1;
  }
}

pub trait QLayoutItem_FreeQLayoutItem {
  fn FreeQLayoutItem(self, rsthis: &mut QLayoutItem) ;
}

// proto:  void QLayoutItem::FreeQLayoutItem();
impl<'a> /*trait*/ QLayoutItem_FreeQLayoutItem for () {
  fn FreeQLayoutItem(self, rsthis: &mut QLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItemD0Ev()};
     unsafe {_ZN11QLayoutItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn hasHeightForWidth<T: QLayoutItem_hasHeightForWidth>(&mut self, value: T) -> i8 {
    return value.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_hasHeightForWidth {
  fn hasHeightForWidth(self, rsthis: &mut QLayoutItem) -> i8;
}

// proto:  bool QLayoutItem::hasHeightForWidth();
impl<'a> /*trait*/ QLayoutItem_hasHeightForWidth for () {
  fn hasHeightForWidth(self, rsthis: &mut QLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn heightForWidth<T: QLayoutItem_heightForWidth>(&mut self, value: T) -> i32 {
    return value.heightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_heightForWidth {
  fn heightForWidth(self, rsthis: &mut QLayoutItem) -> i32;
}

// proto:  int QLayoutItem::heightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_heightForWidth for (i32) {
  fn heightForWidth(self, rsthis: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QLayoutItem14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn geometry<T: QLayoutItem_geometry>(&mut self, value: T) -> QRect {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QLayoutItem_geometry {
  fn geometry(self, rsthis: &mut QLayoutItem) -> QRect;
}

// proto:  QRect QLayoutItem::geometry();
impl<'a> /*trait*/ QLayoutItem_geometry for () {
  fn geometry(self, rsthis: &mut QLayoutItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn maximumSize<T: QLayoutItem_maximumSize>(&mut self, value: T) -> QSize {
    return value.maximumSize(self);
    // return 1;
  }
}

pub trait QLayoutItem_maximumSize {
  fn maximumSize(self, rsthis: &mut QLayoutItem) -> QSize;
}

// proto:  QSize QLayoutItem::maximumSize();
impl<'a> /*trait*/ QLayoutItem_maximumSize for () {
  fn maximumSize(self, rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn minimumHeightForWidth<T: QLayoutItem_minimumHeightForWidth>(&mut self, value: T) -> i32 {
    return value.minimumHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_minimumHeightForWidth {
  fn minimumHeightForWidth(self, rsthis: &mut QLayoutItem) -> i32;
}

// proto:  int QLayoutItem::minimumHeightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_minimumHeightForWidth for (i32) {
  fn minimumHeightForWidth(self, rsthis: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QLayoutItem21minimumHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

