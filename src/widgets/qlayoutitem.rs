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
  fn _ZN11QLayoutItem10invalidateEv(qthis: *mut c_void);
  // proto:  void QLayoutItem::setGeometry(const QRect & );
  fn _ZN11QLayoutItem11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QLayout * QLayoutItem::layout();
  fn _ZN11QLayoutItem6layoutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QLayoutItem::isEmpty();
  fn _ZNK11QLayoutItem7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QSize QLayoutItem::sizeHint();
  fn _ZNK11QLayoutItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayoutItem::~QLayoutItem();
  fn _ZN11QLayoutItemD0Ev(qthis: *mut c_void);
  // proto:  bool QLayoutItem::hasHeightForWidth();
  fn _ZNK11QLayoutItem17hasHeightForWidthEv(qthis: *mut c_void) -> c_char;
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

  // proto:  QSpacerItem * QLayoutItem::spacerItem();
impl /*struct*/ QLayoutItem {
  pub fn spacerItem<RetType, T: QLayoutItem_spacerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.spacerItem(self);
    // return 1;
  }
}

pub trait QLayoutItem_spacerItem<RetType> {
  fn spacerItem(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QSpacerItem * QLayoutItem::spacerItem();
impl<'a> /*trait*/ QLayoutItem_spacerItem<QSpacerItem> for () {
  fn spacerItem(self , rsthis: &mut QLayoutItem) -> QSpacerItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10spacerItemEv()};
    let mut ret = unsafe {_ZN11QLayoutItem10spacerItemEv(rsthis.qclsinst)};
    let mut ret1 = QSpacerItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::minimumSize();
impl /*struct*/ QLayoutItem {
  pub fn minimumSize<RetType, T: QLayoutItem_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QLayoutItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::minimumSize();
impl<'a> /*trait*/ QLayoutItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QLayoutItem::widget();
impl /*struct*/ QLayoutItem {
  pub fn widget<RetType, T: QLayoutItem_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QLayoutItem_widget<RetType> {
  fn widget(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QWidget * QLayoutItem::widget();
impl<'a> /*trait*/ QLayoutItem_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QLayoutItem) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6widgetEv()};
    let mut ret = unsafe {_ZN11QLayoutItem6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLayoutItem::invalidate();
impl /*struct*/ QLayoutItem {
  pub fn invalidate<RetType, T: QLayoutItem_invalidate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QLayoutItem_invalidate<RetType> {
  fn invalidate(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::invalidate();
impl<'a> /*trait*/ QLayoutItem_invalidate<()> for () {
  fn invalidate(self , rsthis: &mut QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10invalidateEv()};
     unsafe {_ZN11QLayoutItem10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLayoutItem::setGeometry(const QRect & );
impl /*struct*/ QLayoutItem {
  pub fn setGeometry<RetType, T: QLayoutItem_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QLayoutItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QLayoutItem_setGeometry<()> for (QRect) {
  fn setGeometry(self , rsthis: &mut QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QLayoutItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QLayout * QLayoutItem::layout();
impl /*struct*/ QLayoutItem {
  pub fn layout<RetType, T: QLayoutItem_layout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.layout(self);
    // return 1;
  }
}

pub trait QLayoutItem_layout<RetType> {
  fn layout(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QLayout * QLayoutItem::layout();
impl<'a> /*trait*/ QLayoutItem_layout<QLayout> for () {
  fn layout(self , rsthis: &mut QLayoutItem) -> QLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6layoutEv()};
    let mut ret = unsafe {_ZN11QLayoutItem6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QLayout{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QLayoutItem::isEmpty();
impl /*struct*/ QLayoutItem {
  pub fn isEmpty<RetType, T: QLayoutItem_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QLayoutItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  bool QLayoutItem::isEmpty();
impl<'a> /*trait*/ QLayoutItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::sizeHint();
impl /*struct*/ QLayoutItem {
  pub fn sizeHint<RetType, T: QLayoutItem_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QLayoutItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::sizeHint();
impl<'a> /*trait*/ QLayoutItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLayoutItem::~QLayoutItem();
impl /*struct*/ QLayoutItem {
  pub fn FreeQLayoutItem<RetType, T: QLayoutItem_FreeQLayoutItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQLayoutItem(self);
    // return 1;
  }
}

pub trait QLayoutItem_FreeQLayoutItem<RetType> {
  fn FreeQLayoutItem(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::~QLayoutItem();
impl<'a> /*trait*/ QLayoutItem_FreeQLayoutItem<()> for () {
  fn FreeQLayoutItem(self , rsthis: &mut QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItemD0Ev()};
     unsafe {_ZN11QLayoutItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLayoutItem::hasHeightForWidth();
impl /*struct*/ QLayoutItem {
  pub fn hasHeightForWidth<RetType, T: QLayoutItem_hasHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  bool QLayoutItem::hasHeightForWidth();
impl<'a> /*trait*/ QLayoutItem_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: &mut QLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QLayoutItem::heightForWidth(int );
impl /*struct*/ QLayoutItem {
  pub fn heightForWidth<RetType, T: QLayoutItem_heightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  int QLayoutItem::heightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QLayoutItem14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QLayoutItem::geometry();
impl /*struct*/ QLayoutItem {
  pub fn geometry<RetType, T: QLayoutItem_geometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QLayoutItem_geometry<RetType> {
  fn geometry(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QRect QLayoutItem::geometry();
impl<'a> /*trait*/ QLayoutItem_geometry<QRect> for () {
  fn geometry(self , rsthis: &mut QLayoutItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::maximumSize();
impl /*struct*/ QLayoutItem {
  pub fn maximumSize<RetType, T: QLayoutItem_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QLayoutItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::maximumSize();
impl<'a> /*trait*/ QLayoutItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QLayoutItem::minimumHeightForWidth(int );
impl /*struct*/ QLayoutItem {
  pub fn minimumHeightForWidth<RetType, T: QLayoutItem_minimumHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_minimumHeightForWidth<RetType> {
  fn minimumHeightForWidth(self , rsthis: &mut QLayoutItem) -> RetType;
}

  // proto:  int QLayoutItem::minimumHeightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_minimumHeightForWidth<i32> for (i32) {
  fn minimumHeightForWidth(self , rsthis: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QLayoutItem21minimumHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

