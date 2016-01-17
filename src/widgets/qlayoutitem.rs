// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtWidgets/qlayoutitem.h
// dst-file: /src/widgets/qlayoutitem.rs
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
use std::ops::Deref;
// use super::qlayoutitem::QSpacerItem; // 773
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qrect::QRect; // 771
use super::qlayout::QLayout; // 773
// use super::qlayoutitem::QLayoutItem; // 773
use super::qsizepolicy::QSizePolicy; // 773
// use super::qlayoutitem::QWidgetItem; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLayoutItem_Class_Size() -> c_int;
  // proto:  QSpacerItem * QLayoutItem::spacerItem();
  fn _ZN11QLayoutItem10spacerItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QLayoutItem::minimumSize();
  fn _ZNK11QLayoutItem11minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QLayoutItem::widget();
  fn _ZN11QLayoutItem6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLayoutItem::invalidate();
  fn _ZN11QLayoutItem10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QLayoutItem::setGeometry(const QRect & );
  fn _ZN11QLayoutItem11setGeometryERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QLayout * QLayoutItem::layout();
  fn _ZN11QLayoutItem6layoutEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QLayoutItem::isEmpty();
  fn _ZNK11QLayoutItem7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSize QLayoutItem::sizeHint();
  fn _ZNK11QLayoutItem8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLayoutItem::~QLayoutItem();
  fn _ZN11QLayoutItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QLayoutItem::hasHeightForWidth();
  fn _ZNK11QLayoutItem17hasHeightForWidthEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QLayoutItem::heightForWidth(int );
  fn _ZNK11QLayoutItem14heightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  QRect QLayoutItem::geometry();
  fn _ZNK11QLayoutItem8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QLayoutItem::maximumSize();
  fn _ZNK11QLayoutItem11maximumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QLayoutItem::minimumHeightForWidth(int );
  fn _ZNK11QLayoutItem21minimumHeightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  fn QSpacerItem_Class_Size() -> c_int;
  // proto:  QSize QSpacerItem::minimumSize();
  fn _ZNK11QSpacerItem11minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSizePolicy QSpacerItem::sizePolicy();
  fn _ZNK11QSpacerItem10sizePolicyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSpacerItem::~QSpacerItem();
  fn _ZN11QSpacerItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QSpacerItem::sizeHint();
  fn _ZNK11QSpacerItem8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QSpacerItem::maximumSize();
  fn _ZNK11QSpacerItem11maximumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSpacerItem::isEmpty();
  fn _ZNK11QSpacerItem7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRect QSpacerItem::geometry();
  fn _ZNK11QSpacerItem8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSpacerItem::setGeometry(const QRect & );
  fn _ZN11QSpacerItem11setGeometryERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSpacerItem * QSpacerItem::spacerItem();
  fn _ZN11QSpacerItem10spacerItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QWidgetItem_Class_Size() -> c_int;
  // proto:  QSize QWidgetItem::sizeHint();
  fn _ZNK11QWidgetItem8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QWidgetItem::minimumSize();
  fn _ZNK11QWidgetItem11minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidgetItem::hasHeightForWidth();
  fn _ZNK11QWidgetItem17hasHeightForWidthEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidgetItem::~QWidgetItem();
  fn _ZN11QWidgetItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidgetItem::QWidgetItem(QWidget * w);
  fn _ZN11QWidgetItemC2EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QWidgetItem::widget();
  fn _ZN11QWidgetItem6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidgetItem::setGeometry(const QRect & );
  fn _ZN11QWidgetItem11setGeometryERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QWidgetItem::heightForWidth(int );
  fn _ZNK11QWidgetItem14heightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QWidgetItem::QWidgetItem(const QWidgetItem & );
  fn _ZN11QWidgetItemC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QWidgetItem::maximumSize();
  fn _ZNK11QWidgetItem11maximumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidgetItem::isEmpty();
  fn _ZNK11QWidgetItem7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRect QWidgetItem::geometry();
  fn _ZNK11QWidgetItem8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QWidgetItemV2_Class_Size() -> c_int;
  // proto:  QSize QWidgetItemV2::sizeHint();
  fn _ZNK13QWidgetItemV28sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QWidgetItemV2::minimumSize();
  fn _ZNK13QWidgetItemV211minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QWidgetItemV2::heightForWidth(int width);
  fn _ZNK13QWidgetItemV214heightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QWidgetItemV2::~QWidgetItemV2();
  fn _ZN13QWidgetItemV2D2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
  fn _ZN13QWidgetItemV2C2EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QWidgetItemV2::maximumSize();
  fn _ZNK13QWidgetItemV211maximumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidgetItemV2::QWidgetItemV2(const QWidgetItemV2 & );
  fn _ZN13QWidgetItemV2C2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLayoutItem)=1
#[derive(Default)]
pub struct QLayoutItem {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSpacerItem)=1
#[derive(Default)]
pub struct QSpacerItem {
  qbase: QLayoutItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QWidgetItem)=1
#[derive(Default)]
pub struct QWidgetItem {
  qbase: QLayoutItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QWidgetItemV2)=1
#[derive(Default)]
pub struct QWidgetItemV2 {
  qbase: QWidgetItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QLayoutItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLayoutItem {
    return QLayoutItem{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QSpacerItem * QLayoutItem::spacerItem();
impl /*struct*/ QLayoutItem {
  pub fn spacerItem<RetType, T: QLayoutItem_spacerItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spacerItem(self);
    // return 1;
  }
}

pub trait QLayoutItem_spacerItem<RetType> {
  fn spacerItem(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  QSpacerItem * QLayoutItem::spacerItem();
impl<'a> /*trait*/ QLayoutItem_spacerItem<QSpacerItem> for () {
  fn spacerItem(self , rsthis: & QLayoutItem) -> QSpacerItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10spacerItemEv()};
    let mut ret = unsafe {_ZN11QLayoutItem10spacerItemEv(rsthis.qclsinst)};
    let mut ret1 = QSpacerItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::minimumSize();
impl /*struct*/ QLayoutItem {
  pub fn minimumSize<RetType, T: QLayoutItem_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QLayoutItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::minimumSize();
impl<'a> /*trait*/ QLayoutItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: & QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QLayoutItem::widget();
impl /*struct*/ QLayoutItem {
  pub fn widget<RetType, T: QLayoutItem_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QLayoutItem_widget<RetType> {
  fn widget(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  QWidget * QLayoutItem::widget();
impl<'a> /*trait*/ QLayoutItem_widget<QWidget> for () {
  fn widget(self , rsthis: & QLayoutItem) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6widgetEv()};
    let mut ret = unsafe {_ZN11QLayoutItem6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLayoutItem::invalidate();
impl /*struct*/ QLayoutItem {
  pub fn invalidate<RetType, T: QLayoutItem_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QLayoutItem_invalidate<RetType> {
  fn invalidate(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::invalidate();
impl<'a> /*trait*/ QLayoutItem_invalidate<()> for () {
  fn invalidate(self , rsthis: & QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10invalidateEv()};
     unsafe {_ZN11QLayoutItem10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLayoutItem::setGeometry(const QRect & );
impl /*struct*/ QLayoutItem {
  pub fn setGeometry<RetType, T: QLayoutItem_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QLayoutItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QLayoutItem_setGeometry<()> for (&'a QRect) {
  fn setGeometry(self , rsthis: & QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QLayoutItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QLayout * QLayoutItem::layout();
impl /*struct*/ QLayoutItem {
  pub fn layout<RetType, T: QLayoutItem_layout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.layout(self);
    // return 1;
  }
}

pub trait QLayoutItem_layout<RetType> {
  fn layout(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  QLayout * QLayoutItem::layout();
impl<'a> /*trait*/ QLayoutItem_layout<QLayout> for () {
  fn layout(self , rsthis: & QLayoutItem) -> QLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6layoutEv()};
    let mut ret = unsafe {_ZN11QLayoutItem6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QLayout::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QLayoutItem::isEmpty();
impl /*struct*/ QLayoutItem {
  pub fn isEmpty<RetType, T: QLayoutItem_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QLayoutItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  bool QLayoutItem::isEmpty();
impl<'a> /*trait*/ QLayoutItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::sizeHint();
impl /*struct*/ QLayoutItem {
  pub fn sizeHint<RetType, T: QLayoutItem_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QLayoutItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::sizeHint();
impl<'a> /*trait*/ QLayoutItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLayoutItem::~QLayoutItem();
impl /*struct*/ QLayoutItem {
  pub fn free<RetType, T: QLayoutItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QLayoutItem_free<RetType> {
  fn free(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  void QLayoutItem::~QLayoutItem();
impl<'a> /*trait*/ QLayoutItem_free<()> for () {
  fn free(self , rsthis: & QLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItemD2Ev()};
     unsafe {_ZN11QLayoutItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLayoutItem::hasHeightForWidth();
impl /*struct*/ QLayoutItem {
  pub fn hasHeightForWidth<RetType, T: QLayoutItem_hasHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  bool QLayoutItem::hasHeightForWidth();
impl<'a> /*trait*/ QLayoutItem_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: & QLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QLayoutItem::heightForWidth(int );
impl /*struct*/ QLayoutItem {
  pub fn heightForWidth<RetType, T: QLayoutItem_heightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  int QLayoutItem::heightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: & QLayoutItem) -> i32 {
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
  pub fn geometry<RetType, T: QLayoutItem_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QLayoutItem_geometry<RetType> {
  fn geometry(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  QRect QLayoutItem::geometry();
impl<'a> /*trait*/ QLayoutItem_geometry<QRect> for () {
  fn geometry(self , rsthis: & QLayoutItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QLayoutItem::maximumSize();
impl /*struct*/ QLayoutItem {
  pub fn maximumSize<RetType, T: QLayoutItem_maximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QLayoutItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  QSize QLayoutItem::maximumSize();
impl<'a> /*trait*/ QLayoutItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: & QLayoutItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QLayoutItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QLayoutItem::minimumHeightForWidth(int );
impl /*struct*/ QLayoutItem {
  pub fn minimumHeightForWidth<RetType, T: QLayoutItem_minimumHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayoutItem_minimumHeightForWidth<RetType> {
  fn minimumHeightForWidth(self , rsthis: & QLayoutItem) -> RetType;
}

  // proto:  int QLayoutItem::minimumHeightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_minimumHeightForWidth<i32> for (i32) {
  fn minimumHeightForWidth(self , rsthis: & QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QLayoutItem21minimumHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSpacerItem {
    return QSpacerItem{qbase: QLayoutItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSpacerItem {
  type Target = QLayoutItem;

  fn deref(&self) -> &QLayoutItem {
    return & self.qbase;
  }
}
impl AsRef<QLayoutItem> for QSpacerItem {
  fn as_ref(& self) -> & QLayoutItem {
    return & self.qbase;
  }
}
  // proto:  QSize QSpacerItem::minimumSize();
impl /*struct*/ QSpacerItem {
  pub fn minimumSize<RetType, T: QSpacerItem_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QSpacerItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::minimumSize();
impl<'a> /*trait*/ QSpacerItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: & QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizePolicy QSpacerItem::sizePolicy();
impl /*struct*/ QSpacerItem {
  pub fn sizePolicy<RetType, T: QSpacerItem_sizePolicy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy(self);
    // return 1;
  }
}

pub trait QSpacerItem_sizePolicy<RetType> {
  fn sizePolicy(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  QSizePolicy QSpacerItem::sizePolicy();
impl<'a> /*trait*/ QSpacerItem_sizePolicy<QSizePolicy> for () {
  fn sizePolicy(self , rsthis: & QSpacerItem) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem10sizePolicyEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSpacerItem::~QSpacerItem();
impl /*struct*/ QSpacerItem {
  pub fn free<RetType, T: QSpacerItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSpacerItem_free<RetType> {
  fn free(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  void QSpacerItem::~QSpacerItem();
impl<'a> /*trait*/ QSpacerItem_free<()> for () {
  fn free(self , rsthis: & QSpacerItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItemD2Ev()};
     unsafe {_ZN11QSpacerItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QSpacerItem::sizeHint();
impl /*struct*/ QSpacerItem {
  pub fn sizeHint<RetType, T: QSpacerItem_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSpacerItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::sizeHint();
impl<'a> /*trait*/ QSpacerItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QSpacerItem::maximumSize();
impl /*struct*/ QSpacerItem {
  pub fn maximumSize<RetType, T: QSpacerItem_maximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QSpacerItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  QSize QSpacerItem::maximumSize();
impl<'a> /*trait*/ QSpacerItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: & QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSpacerItem::isEmpty();
impl /*struct*/ QSpacerItem {
  pub fn isEmpty<RetType, T: QSpacerItem_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QSpacerItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  bool QSpacerItem::isEmpty();
impl<'a> /*trait*/ QSpacerItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QSpacerItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QSpacerItem::geometry();
impl /*struct*/ QSpacerItem {
  pub fn geometry<RetType, T: QSpacerItem_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QSpacerItem_geometry<RetType> {
  fn geometry(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  QRect QSpacerItem::geometry();
impl<'a> /*trait*/ QSpacerItem_geometry<QRect> for () {
  fn geometry(self , rsthis: & QSpacerItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSpacerItem::setGeometry(const QRect & );
impl /*struct*/ QSpacerItem {
  pub fn setGeometry<RetType, T: QSpacerItem_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QSpacerItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  void QSpacerItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QSpacerItem_setGeometry<()> for (&'a QRect) {
  fn setGeometry(self , rsthis: & QSpacerItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QSpacerItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSpacerItem * QSpacerItem::spacerItem();
impl /*struct*/ QSpacerItem {
  pub fn spacerItem<RetType, T: QSpacerItem_spacerItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spacerItem(self);
    // return 1;
  }
}

pub trait QSpacerItem_spacerItem<RetType> {
  fn spacerItem(self , rsthis: & QSpacerItem) -> RetType;
}

  // proto:  QSpacerItem * QSpacerItem::spacerItem();
impl<'a> /*trait*/ QSpacerItem_spacerItem<QSpacerItem> for () {
  fn spacerItem(self , rsthis: & QSpacerItem) -> QSpacerItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem10spacerItemEv()};
    let mut ret = unsafe {_ZN11QSpacerItem10spacerItemEv(rsthis.qclsinst)};
    let mut ret1 = QSpacerItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWidgetItem {
    return QWidgetItem{qbase: QLayoutItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWidgetItem {
  type Target = QLayoutItem;

  fn deref(&self) -> &QLayoutItem {
    return & self.qbase;
  }
}
impl AsRef<QLayoutItem> for QWidgetItem {
  fn as_ref(& self) -> & QLayoutItem {
    return & self.qbase;
  }
}
  // proto:  QSize QWidgetItem::sizeHint();
impl /*struct*/ QWidgetItem {
  pub fn sizeHint<RetType, T: QWidgetItem_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWidgetItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  QSize QWidgetItem::sizeHint();
impl<'a> /*trait*/ QWidgetItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QWidgetItem::minimumSize();
impl /*struct*/ QWidgetItem {
  pub fn minimumSize<RetType, T: QWidgetItem_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWidgetItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  QSize QWidgetItem::minimumSize();
impl<'a> /*trait*/ QWidgetItem_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: & QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidgetItem::hasHeightForWidth();
impl /*struct*/ QWidgetItem {
  pub fn hasHeightForWidth<RetType, T: QWidgetItem_hasHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItem_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  bool QWidgetItem::hasHeightForWidth();
impl<'a> /*trait*/ QWidgetItem_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: & QWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWidgetItem::~QWidgetItem();
impl /*struct*/ QWidgetItem {
  pub fn free<RetType, T: QWidgetItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWidgetItem_free<RetType> {
  fn free(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  void QWidgetItem::~QWidgetItem();
impl<'a> /*trait*/ QWidgetItem_free<()> for () {
  fn free(self , rsthis: & QWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemD2Ev()};
     unsafe {_ZN11QWidgetItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidgetItem::QWidgetItem(QWidget * w);
impl /*struct*/ QWidgetItem {
  pub fn new<T: QWidgetItem_new>(value: T) -> QWidgetItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItem_new {
  fn new(self) -> QWidgetItem;
}

  // proto:  void QWidgetItem::QWidgetItem(QWidget * w);
impl<'a> /*trait*/ QWidgetItem_new for (&'a QWidget) {
  fn new(self) -> QWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemC2EP7QWidget()};
    let ctysz: c_int = unsafe{QWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWidgetItemC2EP7QWidget(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QWidgetItem{qbase: QLayoutItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWidget * QWidgetItem::widget();
impl /*struct*/ QWidgetItem {
  pub fn widget<RetType, T: QWidgetItem_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QWidgetItem_widget<RetType> {
  fn widget(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  QWidget * QWidgetItem::widget();
impl<'a> /*trait*/ QWidgetItem_widget<QWidget> for () {
  fn widget(self , rsthis: & QWidgetItem) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem6widgetEv()};
    let mut ret = unsafe {_ZN11QWidgetItem6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidgetItem::setGeometry(const QRect & );
impl /*struct*/ QWidgetItem {
  pub fn setGeometry<RetType, T: QWidgetItem_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QWidgetItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  void QWidgetItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QWidgetItem_setGeometry<()> for (&'a QRect) {
  fn setGeometry(self , rsthis: & QWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QWidgetItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWidgetItem::heightForWidth(int );
impl /*struct*/ QWidgetItem {
  pub fn heightForWidth<RetType, T: QWidgetItem_heightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItem_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  int QWidgetItem::heightForWidth(int );
impl<'a> /*trait*/ QWidgetItem_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: & QWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QWidgetItem14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWidgetItem::QWidgetItem(const QWidgetItem & );
impl<'a> /*trait*/ QWidgetItem_new for (&'a QWidgetItem) {
  fn new(self) -> QWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWidgetItemC2ERKS_()};
    let ctysz: c_int = unsafe{QWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWidgetItemC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QWidgetItem{qbase: QLayoutItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QWidgetItem::maximumSize();
impl /*struct*/ QWidgetItem {
  pub fn maximumSize<RetType, T: QWidgetItem_maximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWidgetItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  QSize QWidgetItem::maximumSize();
impl<'a> /*trait*/ QWidgetItem_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: & QWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidgetItem::isEmpty();
impl /*struct*/ QWidgetItem {
  pub fn isEmpty<RetType, T: QWidgetItem_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QWidgetItem_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  bool QWidgetItem::isEmpty();
impl<'a> /*trait*/ QWidgetItem_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QWidgetItem::geometry();
impl /*struct*/ QWidgetItem {
  pub fn geometry<RetType, T: QWidgetItem_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QWidgetItem_geometry<RetType> {
  fn geometry(self , rsthis: & QWidgetItem) -> RetType;
}

  // proto:  QRect QWidgetItem::geometry();
impl<'a> /*trait*/ QWidgetItem_geometry<QRect> for () {
  fn geometry(self , rsthis: & QWidgetItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWidgetItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QWidgetItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidgetItemV2 {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWidgetItemV2 {
    return QWidgetItemV2{qbase: QWidgetItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWidgetItemV2 {
  type Target = QWidgetItem;

  fn deref(&self) -> &QWidgetItem {
    return & self.qbase;
  }
}
impl AsRef<QWidgetItem> for QWidgetItemV2 {
  fn as_ref(& self) -> & QWidgetItem {
    return & self.qbase;
  }
}
  // proto:  QSize QWidgetItemV2::sizeHint();
impl /*struct*/ QWidgetItemV2 {
  pub fn sizeHint<RetType, T: QWidgetItemV2_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::sizeHint();
impl<'a> /*trait*/ QWidgetItemV2_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV28sizeHintEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV28sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QWidgetItemV2::minimumSize();
impl /*struct*/ QWidgetItemV2 {
  pub fn minimumSize<RetType, T: QWidgetItemV2_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::minimumSize();
impl<'a> /*trait*/ QWidgetItemV2_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: & QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV211minimumSizeEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV211minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWidgetItemV2::heightForWidth(int width);
impl /*struct*/ QWidgetItemV2 {
  pub fn heightForWidth<RetType, T: QWidgetItemV2_heightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: & QWidgetItemV2) -> RetType;
}

  // proto:  int QWidgetItemV2::heightForWidth(int width);
impl<'a> /*trait*/ QWidgetItemV2_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: & QWidgetItemV2) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV214heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QWidgetItemV214heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::~QWidgetItemV2();
impl /*struct*/ QWidgetItemV2 {
  pub fn free<RetType, T: QWidgetItemV2_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_free<RetType> {
  fn free(self , rsthis: & QWidgetItemV2) -> RetType;
}

  // proto:  void QWidgetItemV2::~QWidgetItemV2();
impl<'a> /*trait*/ QWidgetItemV2_free<()> for () {
  fn free(self , rsthis: & QWidgetItemV2) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2D2Ev()};
     unsafe {_ZN13QWidgetItemV2D2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
impl /*struct*/ QWidgetItemV2 {
  pub fn new<T: QWidgetItemV2_new>(value: T) -> QWidgetItemV2 {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QWidgetItemV2_new {
  fn new(self) -> QWidgetItemV2;
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(QWidget * widget);
impl<'a> /*trait*/ QWidgetItemV2_new for (&'a QWidget) {
  fn new(self) -> QWidgetItemV2 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2C2EP7QWidget()};
    let ctysz: c_int = unsafe{QWidgetItemV2_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetItemV2C2EP7QWidget(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QWidgetItemV2{qbase: QWidgetItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QWidgetItemV2::maximumSize();
impl /*struct*/ QWidgetItemV2 {
  pub fn maximumSize<RetType, T: QWidgetItemV2_maximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWidgetItemV2_maximumSize<RetType> {
  fn maximumSize(self , rsthis: & QWidgetItemV2) -> RetType;
}

  // proto:  QSize QWidgetItemV2::maximumSize();
impl<'a> /*trait*/ QWidgetItemV2_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: & QWidgetItemV2) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QWidgetItemV211maximumSizeEv()};
    let mut ret = unsafe {_ZNK13QWidgetItemV211maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidgetItemV2::QWidgetItemV2(const QWidgetItemV2 & );
impl<'a> /*trait*/ QWidgetItemV2_new for (&'a QWidgetItemV2) {
  fn new(self) -> QWidgetItemV2 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QWidgetItemV2C2ERKS_()};
    let ctysz: c_int = unsafe{QWidgetItemV2_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QWidgetItemV2C2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QWidgetItemV2{qbase: QWidgetItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

