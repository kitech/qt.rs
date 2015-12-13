// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicsanchorlayout::QGraphicsAnchorLayout;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsAnchor::FreeQGraphicsAnchor();
  fn _ZN15QGraphicsAnchorD0Ev() -> i32;
  // proto: void QGraphicsAnchor::unsetSpacing();
  fn _ZN15QGraphicsAnchor12unsetSpacingEv() -> i32;
  // proto: void QGraphicsAnchor::setSpacing(qreal spacing);
  fn _ZN15QGraphicsAnchor10setSpacingEd(arg0: c_double) -> i32;
  // proto: void QGraphicsAnchor::NewQGraphicsAnchor(QGraphicsAnchorLayout * parent);
  fn _ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QGraphicsAnchor::metaObject();
  fn _ZNK15QGraphicsAnchor10metaObjectEv() -> i32;
  // proto: double QGraphicsAnchor::spacing();
  fn _ZNK15QGraphicsAnchor7spacingEv() -> i32;
}

// body block begin
// class sizeof(QGraphicsAnchor)=1
pub struct QGraphicsAnchor {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsAnchor {
  pub fn FreeQGraphicsAnchor<T: QGraphicsAnchor_FreeQGraphicsAnchor>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsAnchor(self);
    return 1;
  }
}

pub trait QGraphicsAnchor_FreeQGraphicsAnchor {
  fn FreeQGraphicsAnchor(self, this: &mut QGraphicsAnchor) -> i32;
}

// proto: void QGraphicsAnchor::FreeQGraphicsAnchor();
impl<'a> /*trait*/ QGraphicsAnchor_FreeQGraphicsAnchor for () {
  fn FreeQGraphicsAnchor(self, this: &mut QGraphicsAnchor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchorD0Ev()};
    unsafe {_ZN15QGraphicsAnchorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn unsetSpacing<T: QGraphicsAnchor_unsetSpacing>(&mut self, value: T) -> i32 {
    value.unsetSpacing(self);
    return 1;
  }
}

pub trait QGraphicsAnchor_unsetSpacing {
  fn unsetSpacing(self, this: &mut QGraphicsAnchor) -> i32;
}

// proto: void QGraphicsAnchor::unsetSpacing();
impl<'a> /*trait*/ QGraphicsAnchor_unsetSpacing for () {
  fn unsetSpacing(self, this: &mut QGraphicsAnchor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor12unsetSpacingEv()};
    unsafe {_ZN15QGraphicsAnchor12unsetSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn setSpacing<T: QGraphicsAnchor_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QGraphicsAnchor_setSpacing {
  fn setSpacing(self, this: &mut QGraphicsAnchor) -> i32;
}

// proto: void QGraphicsAnchor::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchor_setSpacing for (f64) {
  fn setSpacing(self, this: &mut QGraphicsAnchor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor10setSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN15QGraphicsAnchor10setSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn NewQGraphicsAnchor<T: QGraphicsAnchor_NewQGraphicsAnchor>(value: T) -> QGraphicsAnchor {
    let rsthis = value.NewQGraphicsAnchor();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsAnchor_NewQGraphicsAnchor {
  fn NewQGraphicsAnchor(self) -> QGraphicsAnchor;
}

// proto: void QGraphicsAnchor::NewQGraphicsAnchor(QGraphicsAnchorLayout * parent);
impl<'a> /*trait*/ QGraphicsAnchor_NewQGraphicsAnchor for (&'a mut QGraphicsAnchorLayout) {
  fn NewQGraphicsAnchor(self) -> QGraphicsAnchor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout(qthis, arg0)};
    let rsthis = QGraphicsAnchor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn metaObject<T: QGraphicsAnchor_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsAnchor_metaObject {
  fn metaObject(self, this: &mut QGraphicsAnchor) -> i32;
}

// proto: const QMetaObject * QGraphicsAnchor::metaObject();
impl<'a> /*trait*/ QGraphicsAnchor_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsAnchor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor10metaObjectEv()};
    unsafe {_ZNK15QGraphicsAnchor10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn spacing<T: QGraphicsAnchor_spacing>(&mut self, value: T) -> i32 {
    value.spacing(self);
    return 1;
  }
}

pub trait QGraphicsAnchor_spacing {
  fn spacing(self, this: &mut QGraphicsAnchor) -> i32;
}

// proto: double QGraphicsAnchor::spacing();
impl<'a> /*trait*/ QGraphicsAnchor_spacing for () {
  fn spacing(self, this: &mut QGraphicsAnchor) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor7spacingEv()};
    unsafe {_ZNK15QGraphicsAnchor7spacingEv()};
    return 1;
  }
}

