// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qgraphicslayout.h
// dst-file: /src/widgets/qgraphicslayout.rs
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
use super::qgraphicslayoutitem::*; // 773
use std::ops::Deref;
use super::super::core::qcoreevent::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsLayout_Class_Size() -> c_int;
  // proto:  void QGraphicsLayout::updateGeometry();
  fn C_ZN15QGraphicsLayout14updateGeometryEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QGraphicsLayout::isActivated();
  fn C_ZNK15QGraphicsLayout11isActivatedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsLayout::invalidate();
  fn C_ZN15QGraphicsLayout10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsLayout::removeAt(int index);
  fn C_ZN15QGraphicsLayout8removeAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
  fn C_ZNK15QGraphicsLayout6itemAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn C_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn C_ZN15QGraphicsLayout18setContentsMarginsEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsLayout::widgetEvent(QEvent * e);
  fn C_ZN15QGraphicsLayout11widgetEventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static bool QGraphicsLayout::instantInvalidatePropagation();
  fn C_ZN15QGraphicsLayout28instantInvalidatePropagationEv() -> c_char;
  // proto: static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
  fn C_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb(arg0: c_char);
  // proto:  void QGraphicsLayout::~QGraphicsLayout();
  fn C_ZN15QGraphicsLayoutD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsLayout::activate();
  fn C_ZN15QGraphicsLayout8activateEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QGraphicsLayout::count();
  fn C_ZNK15QGraphicsLayout5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGraphicsLayout::QGraphicsLayout(QGraphicsLayoutItem * parent);
  fn C_ZN15QGraphicsLayoutC2EP19QGraphicsLayoutItem(arg0: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsLayout)=1
#[derive(Default)]
pub struct QGraphicsLayout {
  qbase: QGraphicsLayoutItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsLayout {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsLayout {
    return QGraphicsLayout{qbase: QGraphicsLayoutItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsLayout {
  type Target = QGraphicsLayoutItem;

  fn deref(&self) -> &QGraphicsLayoutItem {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsLayoutItem> for QGraphicsLayout {
  fn as_ref(& self) -> & QGraphicsLayoutItem {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsLayout::updateGeometry();
impl /*struct*/ QGraphicsLayout {
  pub fn updateGeometry<RetType, T: QGraphicsLayout_updateGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_updateGeometry<RetType> {
  fn updateGeometry(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::updateGeometry();
impl<'a> /*trait*/ QGraphicsLayout_updateGeometry<()> for () {
  fn updateGeometry(self , rsthis: & QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout14updateGeometryEv()};
     unsafe {C_ZN15QGraphicsLayout14updateGeometryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsLayout::isActivated();
impl /*struct*/ QGraphicsLayout {
  pub fn isActivated<RetType, T: QGraphicsLayout_isActivated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActivated(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_isActivated<RetType> {
  fn isActivated(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  bool QGraphicsLayout::isActivated();
impl<'a> /*trait*/ QGraphicsLayout_isActivated<i8> for () {
  fn isActivated(self , rsthis: & QGraphicsLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout11isActivatedEv()};
    let mut ret = unsafe {C_ZNK15QGraphicsLayout11isActivatedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::invalidate();
impl /*struct*/ QGraphicsLayout {
  pub fn invalidate<RetType, T: QGraphicsLayout_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::invalidate();
impl<'a> /*trait*/ QGraphicsLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: & QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout10invalidateEv()};
     unsafe {C_ZN15QGraphicsLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::removeAt(int index);
impl /*struct*/ QGraphicsLayout {
  pub fn removeAt<RetType, T: QGraphicsLayout_removeAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_removeAt<RetType> {
  fn removeAt(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsLayout_removeAt<()> for (i32) {
  fn removeAt(self , rsthis: & QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN15QGraphicsLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
impl /*struct*/ QGraphicsLayout {
  pub fn itemAt<RetType, T: QGraphicsLayout_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
impl<'a> /*trait*/ QGraphicsLayout_itemAt<QGraphicsLayoutItem> for (i32) {
  fn itemAt(self , rsthis: & QGraphicsLayout) -> QGraphicsLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK15QGraphicsLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QGraphicsLayoutItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl /*struct*/ QGraphicsLayout {
  pub fn getContentsMargins<RetType, T: QGraphicsLayout_getContentsMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_getContentsMargins<RetType> {
  fn getContentsMargins(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsLayout_getContentsMargins<()> for (&'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn getContentsMargins(self , rsthis: & QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_double;
    let arg1 = self.1.as_ptr()  as *mut c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {C_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl /*struct*/ QGraphicsLayout {
  pub fn setContentsMargins<RetType, T: QGraphicsLayout_setContentsMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_setContentsMargins<RetType> {
  fn setContentsMargins(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsLayout_setContentsMargins<()> for (f64, f64, f64, f64) {
  fn setContentsMargins(self , rsthis: & QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout18setContentsMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {C_ZN15QGraphicsLayout18setContentsMarginsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::widgetEvent(QEvent * e);
impl /*struct*/ QGraphicsLayout {
  pub fn widgetEvent<RetType, T: QGraphicsLayout_widgetEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widgetEvent(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_widgetEvent<RetType> {
  fn widgetEvent(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::widgetEvent(QEvent * e);
impl<'a> /*trait*/ QGraphicsLayout_widgetEvent<()> for (&'a QEvent) {
  fn widgetEvent(self , rsthis: & QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout11widgetEventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QGraphicsLayout11widgetEventEP6QEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static bool QGraphicsLayout::instantInvalidatePropagation();
impl /*struct*/ QGraphicsLayout {
  pub fn instantInvalidatePropagation_s<RetType, T: QGraphicsLayout_instantInvalidatePropagation_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.instantInvalidatePropagation_s();
    // return 1;
  }
}

pub trait QGraphicsLayout_instantInvalidatePropagation_s<RetType> {
  fn instantInvalidatePropagation_s(self ) -> RetType;
}

  // proto: static bool QGraphicsLayout::instantInvalidatePropagation();
impl<'a> /*trait*/ QGraphicsLayout_instantInvalidatePropagation_s<i8> for () {
  fn instantInvalidatePropagation_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout28instantInvalidatePropagationEv()};
    let mut ret = unsafe {C_ZN15QGraphicsLayout28instantInvalidatePropagationEv()};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
impl /*struct*/ QGraphicsLayout {
  pub fn setInstantInvalidatePropagation_s<RetType, T: QGraphicsLayout_setInstantInvalidatePropagation_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setInstantInvalidatePropagation_s();
    // return 1;
  }
}

pub trait QGraphicsLayout_setInstantInvalidatePropagation_s<RetType> {
  fn setInstantInvalidatePropagation_s(self ) -> RetType;
}

  // proto: static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
impl<'a> /*trait*/ QGraphicsLayout_setInstantInvalidatePropagation_s<()> for (i8) {
  fn setInstantInvalidatePropagation_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb(arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::~QGraphicsLayout();
impl /*struct*/ QGraphicsLayout {
  pub fn free<RetType, T: QGraphicsLayout_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_free<RetType> {
  fn free(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::~QGraphicsLayout();
impl<'a> /*trait*/ QGraphicsLayout_free<()> for () {
  fn free(self , rsthis: & QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutD2Ev()};
     unsafe {C_ZN15QGraphicsLayoutD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::activate();
impl /*struct*/ QGraphicsLayout {
  pub fn activate<RetType, T: QGraphicsLayout_activate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activate(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_activate<RetType> {
  fn activate(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  void QGraphicsLayout::activate();
impl<'a> /*trait*/ QGraphicsLayout_activate<()> for () {
  fn activate(self , rsthis: & QGraphicsLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout8activateEv()};
     unsafe {C_ZN15QGraphicsLayout8activateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QGraphicsLayout::count();
impl /*struct*/ QGraphicsLayout {
  pub fn count<RetType, T: QGraphicsLayout_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_count<RetType> {
  fn count(self , rsthis: & QGraphicsLayout) -> RetType;
}

  // proto:  int QGraphicsLayout::count();
impl<'a> /*trait*/ QGraphicsLayout_count<i32> for () {
  fn count(self , rsthis: & QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout5countEv()};
    let mut ret = unsafe {C_ZNK15QGraphicsLayout5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QGraphicsLayout::QGraphicsLayout(QGraphicsLayoutItem * parent);
impl /*struct*/ QGraphicsLayout {
  pub fn new<T: QGraphicsLayout_new>(value: T) -> QGraphicsLayout {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLayout_new {
  fn new(self) -> QGraphicsLayout;
}

  // proto:  void QGraphicsLayout::QGraphicsLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLayout_new for (Option<&'a QGraphicsLayoutItem>) {
  fn new(self) -> QGraphicsLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutC2EP19QGraphicsLayoutItem()};
    let ctysz: c_int = unsafe{QGraphicsLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QGraphicsLayoutC2EP19QGraphicsLayoutItem(arg0)};
    let rsthis = QGraphicsLayout{qbase: QGraphicsLayoutItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

