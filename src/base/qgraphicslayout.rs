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
  // proto:  void QGraphicsLayout::updateGeometry();
  fn _ZN15QGraphicsLayout14updateGeometryEv(qthis: *mut c_void) ;
  // proto:  bool QGraphicsLayout::isActivated();
  fn _ZNK15QGraphicsLayout11isActivatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsLayout::invalidate();
  fn _ZN15QGraphicsLayout10invalidateEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsLayout::removeAt(int index);
  fn _ZN15QGraphicsLayout8removeAtEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
  fn _ZNK15QGraphicsLayout6itemAtEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGraphicsLayout::NewQGraphicsLayout(const QGraphicsLayout & );
  fn _ZN15QGraphicsLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto:  void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsLayout18setContentsMarginsEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QGraphicsLayout::widgetEvent(QEvent * e);
  fn _ZN15QGraphicsLayout11widgetEventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QGraphicsLayout::instantInvalidatePropagation();
  fn _ZN15QGraphicsLayout28instantInvalidatePropagationEv() -> int8_t;
  // proto: static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
  fn _ZN15QGraphicsLayout31setInstantInvalidatePropagationEb(arg0: int8_t) ;
  // proto:  void QGraphicsLayout::FreeQGraphicsLayout();
  fn _ZN15QGraphicsLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsLayout::activate();
  fn _ZN15QGraphicsLayout8activateEv(qthis: *mut c_void) ;
  // proto:  int QGraphicsLayout::count();
  fn _ZNK15QGraphicsLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsLayout::NewQGraphicsLayout(QGraphicsLayoutItem * parent);
  fn _ZN15QGraphicsLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsLayout)=1
pub struct QGraphicsLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLayout {
  pub fn updateGeometry<T: QGraphicsLayout_updateGeometry>(&mut self, value: T)  {
     value.updateGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_updateGeometry {
  fn updateGeometry(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  void QGraphicsLayout::updateGeometry();
impl<'a> /*trait*/ QGraphicsLayout_updateGeometry for () {
  fn updateGeometry(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout14updateGeometryEv()};
     unsafe {_ZN15QGraphicsLayout14updateGeometryEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn isActivated<T: QGraphicsLayout_isActivated>(&mut self, value: T) -> i8 {
    return value.isActivated(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_isActivated {
  fn isActivated(self, rsthis: &mut QGraphicsLayout) -> i8;
}

// proto:  bool QGraphicsLayout::isActivated();
impl<'a> /*trait*/ QGraphicsLayout_isActivated for () {
  fn isActivated(self, rsthis: &mut QGraphicsLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout11isActivatedEv()};
    let mut ret = unsafe {_ZNK15QGraphicsLayout11isActivatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn invalidate<T: QGraphicsLayout_invalidate>(&mut self, value: T)  {
     value.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_invalidate {
  fn invalidate(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  void QGraphicsLayout::invalidate();
impl<'a> /*trait*/ QGraphicsLayout_invalidate for () {
  fn invalidate(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout10invalidateEv()};
     unsafe {_ZN15QGraphicsLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn removeAt<T: QGraphicsLayout_removeAt>(&mut self, value: T)  {
     value.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_removeAt {
  fn removeAt(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  void QGraphicsLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsLayout_removeAt for (i32) {
  fn removeAt(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QGraphicsLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn itemAt<T: QGraphicsLayout_itemAt>(&mut self, value: T)  {
     value.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_itemAt {
  fn itemAt(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  QGraphicsLayoutItem * QGraphicsLayout::itemAt(int i);
impl<'a> /*trait*/ QGraphicsLayout_itemAt for (i32) {
  fn itemAt(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout6itemAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK15QGraphicsLayout6itemAtEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn getContentsMargins<T: QGraphicsLayout_getContentsMargins>(&mut self, value: T)  {
     value.getContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_getContentsMargins {
  fn getContentsMargins(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  void QGraphicsLayout::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsLayout_getContentsMargins for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getContentsMargins(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK15QGraphicsLayout18getContentsMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn setContentsMargins<T: QGraphicsLayout_setContentsMargins>(&mut self, value: T)  {
     value.setContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_setContentsMargins {
  fn setContentsMargins(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  void QGraphicsLayout::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsLayout_setContentsMargins for (f64, f64, f64, f64) {
  fn setContentsMargins(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout18setContentsMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN15QGraphicsLayout18setContentsMarginsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn widgetEvent<T: QGraphicsLayout_widgetEvent>(&mut self, value: T)  {
     value.widgetEvent(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_widgetEvent {
  fn widgetEvent(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  void QGraphicsLayout::widgetEvent(QEvent * e);
impl<'a> /*trait*/ QGraphicsLayout_widgetEvent for (&'a mut QEvent) {
  fn widgetEvent(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout11widgetEventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsLayout11widgetEventEP6QEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn instantInvalidatePropagation<T: QGraphicsLayout_instantInvalidatePropagation>(&mut self, value: T) -> i8 {
    return value.instantInvalidatePropagation(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_instantInvalidatePropagation {
  fn instantInvalidatePropagation(self, rsthis: &mut QGraphicsLayout) -> i8;
}

// proto: static bool QGraphicsLayout::instantInvalidatePropagation();
impl<'a> /*trait*/ QGraphicsLayout_instantInvalidatePropagation for () {
  fn instantInvalidatePropagation(self, rsthis: &mut QGraphicsLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout28instantInvalidatePropagationEv()};
    let mut ret = unsafe {_ZN15QGraphicsLayout28instantInvalidatePropagationEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn setInstantInvalidatePropagation<T: QGraphicsLayout_setInstantInvalidatePropagation>(&mut self, value: T)  {
     value.setInstantInvalidatePropagation(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_setInstantInvalidatePropagation {
  fn setInstantInvalidatePropagation(self, rsthis: &mut QGraphicsLayout) ;
}

// proto: static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable);
impl<'a> /*trait*/ QGraphicsLayout_setInstantInvalidatePropagation for (i8) {
  fn setInstantInvalidatePropagation(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QGraphicsLayout31setInstantInvalidatePropagationEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn FreeQGraphicsLayout<T: QGraphicsLayout_FreeQGraphicsLayout>(&mut self, value: T)  {
     value.FreeQGraphicsLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_FreeQGraphicsLayout {
  fn FreeQGraphicsLayout(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  void QGraphicsLayout::FreeQGraphicsLayout();
impl<'a> /*trait*/ QGraphicsLayout_FreeQGraphicsLayout for () {
  fn FreeQGraphicsLayout(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayoutD0Ev()};
     unsafe {_ZN15QGraphicsLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn activate<T: QGraphicsLayout_activate>(&mut self, value: T)  {
     value.activate(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_activate {
  fn activate(self, rsthis: &mut QGraphicsLayout) ;
}

// proto:  void QGraphicsLayout::activate();
impl<'a> /*trait*/ QGraphicsLayout_activate for () {
  fn activate(self, rsthis: &mut QGraphicsLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsLayout8activateEv()};
     unsafe {_ZN15QGraphicsLayout8activateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayout {
  pub fn count<T: QGraphicsLayout_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QGraphicsLayout_count {
  fn count(self, rsthis: &mut QGraphicsLayout) -> i32;
}

// proto:  int QGraphicsLayout::count();
impl<'a> /*trait*/ QGraphicsLayout_count for () {
  fn count(self, rsthis: &mut QGraphicsLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsLayout5countEv()};
    let mut ret = unsafe {_ZNK15QGraphicsLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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

