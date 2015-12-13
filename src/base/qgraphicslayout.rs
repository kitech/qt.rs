// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qevent::QEvent;
use super::qgraphicslayoutitem::QGraphicsLayoutItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsLayout::updateGeometry();
  fn _ZN15QGraphicsLayout14updateGeometryEv() -> i32;
  // proto: bool QGraphicsLayout::isActivated();
  fn _ZNK15QGraphicsLayout11isActivatedEv() -> i32;
  // proto: void QGraphicsLayout::invalidate();
  fn _ZN15QGraphicsLayout10invalidateEv() -> i32;
  // proto: void QGraphicsLayout::removeAt(int index);
  fn _ZN15QGraphicsLayout8removeAtEi(arg0: c_int) -> i32;
  // proto: QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
  fn _ZNK15QGraphicsLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: void QGraphicsLayout::NewQGraphicsLayout(const QGraphicsLayout & );
  fn _ZN15QGraphicsLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsLayout18setContentsMarginsEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  // proto: void QGraphicsLayout::widgetEvent(QEvent * e);
  fn _ZN15QGraphicsLayout11widgetEventEP6QEvent(arg0: *mut c_void) -> i32;
  // proto: bool QGraphicsLayout::instantInvalidatePropagation();
  fn _ZN15QGraphicsLayout28instantInvalidatePropagationEv() -> i32;
  // proto: void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
  fn _ZN15QGraphicsLayout31setInstantInvalidatePropagationEb(arg0: int8_t) -> i32;
  // proto: void QGraphicsLayout::FreeQGraphicsLayout();
  fn _ZN15QGraphicsLayoutD0Ev() -> i32;
  // proto: void QGraphicsLayout::activate();
  fn _ZN15QGraphicsLayout8activateEv() -> i32;
  // proto: int QGraphicsLayout::count();
  fn _ZNK15QGraphicsLayout5countEv() -> i32;
  // proto: void QGraphicsLayout::NewQGraphicsLayout(QGraphicsLayoutItem * parent);
  fn _ZN15QGraphicsLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsLayout)=1
pub struct QGraphicsLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLayout {
  pub fn updateGeometry<T: QGraphicsLayout_updateGeometry>(&mut self, value: T) -> i32 {
    value.updateGeometry(self);
    return 1;
  }
}

pub trait QGraphicsLayout_updateGeometry {
  fn updateGeometry(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::updateGeometry();
impl<'a> /*trait*/ QGraphicsLayout_updateGeometry for () {
  fn updateGeometry(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout14updateGeometryEv()};
    unsafe {_ZN15QGraphicsLayout14updateGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn isActivated<T: QGraphicsLayout_isActivated>(&mut self, value: T) -> i32 {
    value.isActivated(self);
    return 1;
  }
}

pub trait QGraphicsLayout_isActivated {
  fn isActivated(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: bool QGraphicsLayout::isActivated();
impl<'a> /*trait*/ QGraphicsLayout_isActivated for () {
  fn isActivated(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout11isActivatedEv()};
    unsafe {_ZNK15QGraphicsLayout11isActivatedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn invalidate<T: QGraphicsLayout_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QGraphicsLayout_invalidate {
  fn invalidate(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::invalidate();
impl<'a> /*trait*/ QGraphicsLayout_invalidate for () {
  fn invalidate(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout10invalidateEv()};
    unsafe {_ZN15QGraphicsLayout10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn removeAt<T: QGraphicsLayout_removeAt>(&mut self, value: T) -> i32 {
    value.removeAt(self);
    return 1;
  }
}

pub trait QGraphicsLayout_removeAt {
  fn removeAt(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsLayout_removeAt for (i32) {
  fn removeAt(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout8removeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QGraphicsLayout8removeAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn itemAt<T: QGraphicsLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QGraphicsLayout_itemAt {
  fn itemAt(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
impl<'a> /*trait*/ QGraphicsLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QGraphicsLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn NewQGraphicsLayout<T: QGraphicsLayout_NewQGraphicsLayout>(value: T) -> QGraphicsLayout {
    let rsthis = value.NewQGraphicsLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLayout_NewQGraphicsLayout {
  fn NewQGraphicsLayout(self) -> QGraphicsLayout;
}

// proto: void QGraphicsLayout::NewQGraphicsLayout(const QGraphicsLayout & );
impl<'a> /*trait*/ QGraphicsLayout_NewQGraphicsLayout for (&'a  QGraphicsLayout) {
  fn NewQGraphicsLayout(self) -> QGraphicsLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGraphicsLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn getContentsMargins<T: QGraphicsLayout_getContentsMargins>(&mut self, value: T) -> i32 {
    value.getContentsMargins(self);
    return 1;
  }
}

pub trait QGraphicsLayout_getContentsMargins {
  fn getContentsMargins(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsLayout_getContentsMargins for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getContentsMargins(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn setContentsMargins<T: QGraphicsLayout_setContentsMargins>(&mut self, value: T) -> i32 {
    value.setContentsMargins(self);
    return 1;
  }
}

pub trait QGraphicsLayout_setContentsMargins {
  fn setContentsMargins(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsLayout_setContentsMargins for (f64, f64, f64, f64) {
  fn setContentsMargins(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout18setContentsMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QGraphicsLayout18setContentsMarginsEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn widgetEvent<T: QGraphicsLayout_widgetEvent>(&mut self, value: T) -> i32 {
    value.widgetEvent(self);
    return 1;
  }
}

pub trait QGraphicsLayout_widgetEvent {
  fn widgetEvent(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::widgetEvent(QEvent * e);
impl<'a> /*trait*/ QGraphicsLayout_widgetEvent for (&'a mut QEvent) {
  fn widgetEvent(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout11widgetEventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsLayout11widgetEventEP6QEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn instantInvalidatePropagation<T: QGraphicsLayout_instantInvalidatePropagation>(&mut self, value: T) -> i32 {
    value.instantInvalidatePropagation(self);
    return 1;
  }
}

pub trait QGraphicsLayout_instantInvalidatePropagation {
  fn instantInvalidatePropagation(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: bool QGraphicsLayout::instantInvalidatePropagation();
impl<'a> /*trait*/ QGraphicsLayout_instantInvalidatePropagation for () {
  fn instantInvalidatePropagation(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout28instantInvalidatePropagationEv()};
    unsafe {_ZN15QGraphicsLayout28instantInvalidatePropagationEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn setInstantInvalidatePropagation<T: QGraphicsLayout_setInstantInvalidatePropagation>(&mut self, value: T) -> i32 {
    value.setInstantInvalidatePropagation(self);
    return 1;
  }
}

pub trait QGraphicsLayout_setInstantInvalidatePropagation {
  fn setInstantInvalidatePropagation(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
impl<'a> /*trait*/ QGraphicsLayout_setInstantInvalidatePropagation for (i8) {
  fn setInstantInvalidatePropagation(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn FreeQGraphicsLayout<T: QGraphicsLayout_FreeQGraphicsLayout>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsLayout(self);
    return 1;
  }
}

pub trait QGraphicsLayout_FreeQGraphicsLayout {
  fn FreeQGraphicsLayout(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::FreeQGraphicsLayout();
impl<'a> /*trait*/ QGraphicsLayout_FreeQGraphicsLayout for () {
  fn FreeQGraphicsLayout(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutD0Ev()};
    unsafe {_ZN15QGraphicsLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn activate<T: QGraphicsLayout_activate>(&mut self, value: T) -> i32 {
    value.activate(self);
    return 1;
  }
}

pub trait QGraphicsLayout_activate {
  fn activate(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: void QGraphicsLayout::activate();
impl<'a> /*trait*/ QGraphicsLayout_activate for () {
  fn activate(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout8activateEv()};
    unsafe {_ZN15QGraphicsLayout8activateEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn count<T: QGraphicsLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QGraphicsLayout_count {
  fn count(self, this: &mut QGraphicsLayout) -> i32;
}

// proto: int QGraphicsLayout::count();
impl<'a> /*trait*/ QGraphicsLayout_count for () {
  fn count(self, this: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout5countEv()};
    unsafe {_ZNK15QGraphicsLayout5countEv()};
    return 1;
  }
}

// proto: void QGraphicsLayout::NewQGraphicsLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLayout_NewQGraphicsLayout for (&'a mut QGraphicsLayoutItem) {
  fn NewQGraphicsLayout(self) -> QGraphicsLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutC1EP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let rsthis = QGraphicsLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

