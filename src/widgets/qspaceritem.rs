// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qsizepolicy::QSizePolicy;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QSize QSpacerItem::minimumSize();
  fn _ZNK11QSpacerItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizePolicy QSpacerItem::sizePolicy();
  fn _ZNK11QSpacerItem10sizePolicyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSpacerItem::~QSpacerItem();
  fn _ZN11QSpacerItemD0Ev(qthis: *mut c_void);
  // proto:  QSize QSpacerItem::sizeHint();
  fn _ZNK11QSpacerItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QSpacerItem::maximumSize();
  fn _ZNK11QSpacerItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSpacerItem::isEmpty();
  fn _ZNK11QSpacerItem7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QRect QSpacerItem::geometry();
  fn _ZNK11QSpacerItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSpacerItem::setGeometry(const QRect & );
  fn _ZN11QSpacerItem11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSpacerItem * QSpacerItem::spacerItem();
  fn _ZN11QSpacerItem10spacerItemEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QSpacerItem)=1
pub struct QSpacerItem {
  pub qclsinst: *mut c_void,
}

  // proto:  QSize QSpacerItem::minimumSize();
impl /*struct*/ QSpacerItem {
  pub fn minimumSize<RetType, T: QSpacerItem_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QSpacerItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::minimumSize();
impl<'a> /*trait*/ QSpacerItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSizePolicy QSpacerItem::sizePolicy();
impl /*struct*/ QSpacerItem {
  pub fn sizePolicy<RetType, T: QSpacerItem_sizePolicy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy(self);
    // return 1;
  }
}

pub trait QSpacerItem_sizePolicy<RetType> {
  fn sizePolicy(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSizePolicy QSpacerItem::sizePolicy();
impl<'a> /*trait*/ QSpacerItem_sizePolicy<QSizePolicy> for () {
  fn sizePolicy(self , rsthis: &mut QSpacerItem) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem10sizePolicyEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSpacerItem::~QSpacerItem();
impl /*struct*/ QSpacerItem {
  pub fn FreeQSpacerItem<RetType, T: QSpacerItem_FreeQSpacerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSpacerItem(self);
    // return 1;
  }
}

pub trait QSpacerItem_FreeQSpacerItem<RetType> {
  fn FreeQSpacerItem(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  void QSpacerItem::~QSpacerItem();
impl<'a> /*trait*/ QSpacerItem_FreeQSpacerItem<()> for () {
  fn FreeQSpacerItem(self , rsthis: &mut QSpacerItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItemD0Ev()};
     unsafe {_ZN11QSpacerItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QSpacerItem::sizeHint();
impl /*struct*/ QSpacerItem {
  pub fn sizeHint<RetType, T: QSpacerItem_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSpacerItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::sizeHint();
impl<'a> /*trait*/ QSpacerItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QSpacerItem::maximumSize();
impl /*struct*/ QSpacerItem {
  pub fn maximumSize<RetType, T: QSpacerItem_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QSpacerItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::maximumSize();
impl<'a> /*trait*/ QSpacerItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSpacerItem::isEmpty();
impl /*struct*/ QSpacerItem {
  pub fn isEmpty<RetType, T: QSpacerItem_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QSpacerItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  bool QSpacerItem::isEmpty();
impl<'a> /*trait*/ QSpacerItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QSpacerItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QSpacerItem::geometry();
impl /*struct*/ QSpacerItem {
  pub fn geometry<RetType, T: QSpacerItem_geometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QSpacerItem_geometry<RetType> {
  fn geometry(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QRect QSpacerItem::geometry();
impl<'a> /*trait*/ QSpacerItem_geometry<QRect> for () {
  fn geometry(self , rsthis: &mut QSpacerItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSpacerItem::setGeometry(const QRect & );
impl /*struct*/ QSpacerItem {
  pub fn setGeometry<RetType, T: QSpacerItem_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QSpacerItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  void QSpacerItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QSpacerItem_setGeometry<()> for (QRect) {
  fn setGeometry(self , rsthis: &mut QSpacerItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QSpacerItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSpacerItem * QSpacerItem::spacerItem();
impl /*struct*/ QSpacerItem {
  pub fn spacerItem<RetType, T: QSpacerItem_spacerItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.spacerItem(self);
    // return 1;
  }
}

pub trait QSpacerItem_spacerItem<RetType> {
  fn spacerItem(self , rsthis: &mut QSpacerItem) -> RetType;
}

  // proto:  QSpacerItem * QSpacerItem::spacerItem();
impl<'a> /*trait*/ QSpacerItem_spacerItem<QSpacerItem> for () {
  fn spacerItem(self , rsthis: &mut QSpacerItem) -> QSpacerItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem10spacerItemEv()};
    let mut ret = unsafe {_ZN11QSpacerItem10spacerItemEv(rsthis.qclsinst)};
    let mut ret1 = QSpacerItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

