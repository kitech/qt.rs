// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicslayoutitem::QGraphicsLayoutItem;
use super::qrectf::QRectF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsAnchorLayout::NewQGraphicsAnchorLayout(QGraphicsLayoutItem * parent);
  fn _ZN21QGraphicsAnchorLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsAnchorLayout::NewQGraphicsAnchorLayout(const QGraphicsAnchorLayout & );
  fn _ZN21QGraphicsAnchorLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QGraphicsAnchorLayout::verticalSpacing();
  fn _ZNK21QGraphicsAnchorLayout15verticalSpacingEv() -> i32;
  // proto: void QGraphicsAnchorLayout::setSpacing(qreal spacing);
  fn _ZN21QGraphicsAnchorLayout10setSpacingEd(arg0: c_double) -> i32;
  // proto: int QGraphicsAnchorLayout::count();
  fn _ZNK21QGraphicsAnchorLayout5countEv() -> i32;
  // proto: double QGraphicsAnchorLayout::horizontalSpacing();
  fn _ZNK21QGraphicsAnchorLayout17horizontalSpacingEv() -> i32;
  // proto: void QGraphicsAnchorLayout::invalidate();
  fn _ZN21QGraphicsAnchorLayout10invalidateEv() -> i32;
  // proto: QGraphicsLayoutItem * QGraphicsAnchorLayout::itemAt(int index);
  fn _ZNK21QGraphicsAnchorLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: void QGraphicsAnchorLayout::setVerticalSpacing(qreal spacing);
  fn _ZN21QGraphicsAnchorLayout18setVerticalSpacingEd(arg0: c_double) -> i32;
  // proto: void QGraphicsAnchorLayout::setGeometry(const QRectF & rect);
  fn _ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsAnchorLayout::setHorizontalSpacing(qreal spacing);
  fn _ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd(arg0: c_double) -> i32;
  // proto: void QGraphicsAnchorLayout::FreeQGraphicsAnchorLayout();
  fn _ZN21QGraphicsAnchorLayoutD0Ev() -> i32;
  // proto: void QGraphicsAnchorLayout::removeAt(int index);
  fn _ZN21QGraphicsAnchorLayout8removeAtEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QGraphicsAnchorLayout)=1
pub struct QGraphicsAnchorLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn NewQGraphicsAnchorLayout<T: QGraphicsAnchorLayout_NewQGraphicsAnchorLayout>(value: T) -> QGraphicsAnchorLayout {
    let rsthis = value.NewQGraphicsAnchorLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_NewQGraphicsAnchorLayout {
  fn NewQGraphicsAnchorLayout(self) -> QGraphicsAnchorLayout;
}

// proto: void QGraphicsAnchorLayout::NewQGraphicsAnchorLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsAnchorLayout_NewQGraphicsAnchorLayout for (&'a mut QGraphicsLayoutItem) {
  fn NewQGraphicsAnchorLayout(self) -> QGraphicsAnchorLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayoutC1EP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsAnchorLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let rsthis = QGraphicsAnchorLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsAnchorLayout::NewQGraphicsAnchorLayout(const QGraphicsAnchorLayout & );
impl<'a> /*trait*/ QGraphicsAnchorLayout_NewQGraphicsAnchorLayout for (&'a  QGraphicsAnchorLayout) {
  fn NewQGraphicsAnchorLayout(self) -> QGraphicsAnchorLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QGraphicsAnchorLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsAnchorLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn verticalSpacing<T: QGraphicsAnchorLayout_verticalSpacing>(&mut self, value: T) -> i32 {
    value.verticalSpacing(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_verticalSpacing {
  fn verticalSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: double QGraphicsAnchorLayout::verticalSpacing();
impl<'a> /*trait*/ QGraphicsAnchorLayout_verticalSpacing for () {
  fn verticalSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout15verticalSpacingEv()};
    unsafe {_ZNK21QGraphicsAnchorLayout15verticalSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setSpacing<T: QGraphicsAnchorLayout_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_setSpacing {
  fn setSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: void QGraphicsAnchorLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setSpacing for (f64) {
  fn setSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout10setSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN21QGraphicsAnchorLayout10setSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn count<T: QGraphicsAnchorLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_count {
  fn count(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: int QGraphicsAnchorLayout::count();
impl<'a> /*trait*/ QGraphicsAnchorLayout_count for () {
  fn count(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout5countEv()};
    unsafe {_ZNK21QGraphicsAnchorLayout5countEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn horizontalSpacing<T: QGraphicsAnchorLayout_horizontalSpacing>(&mut self, value: T) -> i32 {
    value.horizontalSpacing(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_horizontalSpacing {
  fn horizontalSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: double QGraphicsAnchorLayout::horizontalSpacing();
impl<'a> /*trait*/ QGraphicsAnchorLayout_horizontalSpacing for () {
  fn horizontalSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout17horizontalSpacingEv()};
    unsafe {_ZNK21QGraphicsAnchorLayout17horizontalSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn invalidate<T: QGraphicsAnchorLayout_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_invalidate {
  fn invalidate(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: void QGraphicsAnchorLayout::invalidate();
impl<'a> /*trait*/ QGraphicsAnchorLayout_invalidate for () {
  fn invalidate(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout10invalidateEv()};
    unsafe {_ZN21QGraphicsAnchorLayout10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn itemAt<T: QGraphicsAnchorLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_itemAt {
  fn itemAt(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: QGraphicsLayoutItem * QGraphicsAnchorLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsAnchorLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK21QGraphicsAnchorLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setVerticalSpacing<T: QGraphicsAnchorLayout_setVerticalSpacing>(&mut self, value: T) -> i32 {
    value.setVerticalSpacing(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_setVerticalSpacing {
  fn setVerticalSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: void QGraphicsAnchorLayout::setVerticalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setVerticalSpacing for (f64) {
  fn setVerticalSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout18setVerticalSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN21QGraphicsAnchorLayout18setVerticalSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setGeometry<T: QGraphicsAnchorLayout_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_setGeometry {
  fn setGeometry(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: void QGraphicsAnchorLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setGeometry for (&'a  QRectF) {
  fn setGeometry(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setHorizontalSpacing<T: QGraphicsAnchorLayout_setHorizontalSpacing>(&mut self, value: T) -> i32 {
    value.setHorizontalSpacing(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_setHorizontalSpacing {
  fn setHorizontalSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: void QGraphicsAnchorLayout::setHorizontalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setHorizontalSpacing for (f64) {
  fn setHorizontalSpacing(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn FreeQGraphicsAnchorLayout<T: QGraphicsAnchorLayout_FreeQGraphicsAnchorLayout>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsAnchorLayout(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_FreeQGraphicsAnchorLayout {
  fn FreeQGraphicsAnchorLayout(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: void QGraphicsAnchorLayout::FreeQGraphicsAnchorLayout();
impl<'a> /*trait*/ QGraphicsAnchorLayout_FreeQGraphicsAnchorLayout for () {
  fn FreeQGraphicsAnchorLayout(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayoutD0Ev()};
    unsafe {_ZN21QGraphicsAnchorLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn removeAt<T: QGraphicsAnchorLayout_removeAt>(&mut self, value: T) -> i32 {
    value.removeAt(self);
    return 1;
  }
}

pub trait QGraphicsAnchorLayout_removeAt {
  fn removeAt(self, this: &mut QGraphicsAnchorLayout) -> i32;
}

// proto: void QGraphicsAnchorLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsAnchorLayout_removeAt for (i32) {
  fn removeAt(self, this: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout8removeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QGraphicsAnchorLayout8removeAtEi(arg0)};
    return 1;
  }
}

