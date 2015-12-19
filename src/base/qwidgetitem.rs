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

// proto:  QSize QWidgetItem::sizeHint();
impl /*struct*/ QWidgetItem {
  pub fn sizeHint<RetType, T: QWidgetItem_sizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWidgetItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  QSize QWidgetItem::sizeHint();
impl<'a> /*trait*/ QWidgetItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QSize QWidgetItem::minimumSize();
impl /*struct*/ QWidgetItem {
  pub fn minimumSize<RetType, T: QWidgetItem_minimumSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWidgetItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  QSize QWidgetItem::minimumSize();
impl<'a> /*trait*/ QWidgetItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QWidgetItem::hasHeightForWidth();
impl /*struct*/ QWidgetItem {
  pub fn hasHeightForWidth<RetType, T: QWidgetItem_hasHeightForWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItem_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  bool QWidgetItem::hasHeightForWidth();
impl<'a> /*trait*/ QWidgetItem_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: &mut QWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QWidgetItem::FreeQWidgetItem();
impl /*struct*/ QWidgetItem {
  pub fn FreeQWidgetItem<RetType, T: QWidgetItem_FreeQWidgetItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQWidgetItem(self);
    // return 1;
  }
}

pub trait QWidgetItem_FreeQWidgetItem<RetType> {
  fn FreeQWidgetItem(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  void QWidgetItem::FreeQWidgetItem();
impl<'a> /*trait*/ QWidgetItem_FreeQWidgetItem<()> for () {
  fn FreeQWidgetItem(self , rsthis: &mut QWidgetItem) -> () {
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

// proto:  QWidget * QWidgetItem::widget();
impl /*struct*/ QWidgetItem {
  pub fn widget<RetType, T: QWidgetItem_widget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QWidgetItem_widget<RetType> {
  fn widget(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  QWidget * QWidgetItem::widget();
impl<'a> /*trait*/ QWidgetItem_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QWidgetItem) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem6widgetEv()};
    let mut ret = unsafe {_ZN11QWidgetItem6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QWidgetItem::setGeometry(const QRect & );
impl /*struct*/ QWidgetItem {
  pub fn setGeometry<RetType, T: QWidgetItem_setGeometry<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QWidgetItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  void QWidgetItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QWidgetItem_setGeometry<()> for (&'a  QRect) {
  fn setGeometry(self , rsthis: &mut QWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QWidgetItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QWidgetItem::heightForWidth(int );
impl /*struct*/ QWidgetItem {
  pub fn heightForWidth<RetType, T: QWidgetItem_heightForWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItem_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  int QWidgetItem::heightForWidth(int );
impl<'a> /*trait*/ QWidgetItem_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QWidgetItem) -> i32 {
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

// proto:  QSize QWidgetItem::maximumSize();
impl /*struct*/ QWidgetItem {
  pub fn maximumSize<RetType, T: QWidgetItem_maximumSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWidgetItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  QSize QWidgetItem::maximumSize();
impl<'a> /*trait*/ QWidgetItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QWidgetItem::isEmpty();
impl /*struct*/ QWidgetItem {
  pub fn isEmpty<RetType, T: QWidgetItem_isEmpty<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QWidgetItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  bool QWidgetItem::isEmpty();
impl<'a> /*trait*/ QWidgetItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QRect QWidgetItem::geometry();
impl /*struct*/ QWidgetItem {
  pub fn geometry<RetType, T: QWidgetItem_geometry<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QWidgetItem_geometry<RetType> {
  fn geometry(self , rsthis: &mut QWidgetItem) -> RetType;
}

// proto:  QRect QWidgetItem::geometry();
impl<'a> /*trait*/ QWidgetItem_geometry<QRect> for () {
  fn geometry(self , rsthis: &mut QWidgetItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

