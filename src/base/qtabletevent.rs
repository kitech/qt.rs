// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QTabletEvent::x();
  fn _ZNK12QTabletEvent1xEv() -> i32;
  // proto: int QTabletEvent::xTilt();
  fn _ZNK12QTabletEvent5xTiltEv() -> i32;
  // proto: long long QTabletEvent::uniqueId();
  fn _ZNK12QTabletEvent8uniqueIdEv() -> i32;
  // proto: const QPointF & QTabletEvent::globalPosF();
  fn _ZNK12QTabletEvent10globalPosFEv() -> i32;
  // proto: int QTabletEvent::z();
  fn _ZNK12QTabletEvent1zEv() -> i32;
  // proto: int QTabletEvent::y();
  fn _ZNK12QTabletEvent1yEv() -> i32;
  // proto: QPoint QTabletEvent::pos();
  fn _ZNK12QTabletEvent3posEv() -> i32;
  // proto: double QTabletEvent::rotation();
  fn _ZNK12QTabletEvent8rotationEv() -> i32;
  // proto: QPoint QTabletEvent::globalPos();
  fn _ZNK12QTabletEvent9globalPosEv() -> i32;
  // proto: void QTabletEvent::FreeQTabletEvent();
  fn _ZN12QTabletEventD0Ev() -> i32;
  // proto: double QTabletEvent::tangentialPressure();
  fn _ZNK12QTabletEvent18tangentialPressureEv() -> i32;
  // proto: double QTabletEvent::hiResGlobalX();
  fn _ZNK12QTabletEvent12hiResGlobalXEv() -> i32;
  // proto: int QTabletEvent::globalY();
  fn _ZNK12QTabletEvent7globalYEv() -> i32;
  // proto: double QTabletEvent::hiResGlobalY();
  fn _ZNK12QTabletEvent12hiResGlobalYEv() -> i32;
  // proto: int QTabletEvent::globalX();
  fn _ZNK12QTabletEvent7globalXEv() -> i32;
  // proto: const QPointF & QTabletEvent::posF();
  fn _ZNK12QTabletEvent4posFEv() -> i32;
  // proto: double QTabletEvent::pressure();
  fn _ZNK12QTabletEvent8pressureEv() -> i32;
  // proto: int QTabletEvent::yTilt();
  fn _ZNK12QTabletEvent5yTiltEv() -> i32;
}

// body block begin
// class sizeof(QTabletEvent)=1
pub struct QTabletEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTabletEvent {
  pub fn x<T: QTabletEvent_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QTabletEvent_x {
  fn x(self, this: &mut QTabletEvent) -> i32;
}

// proto: int QTabletEvent::x();
impl<'a> /*trait*/ QTabletEvent_x for () {
  fn x(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1xEv()};
    unsafe {_ZNK12QTabletEvent1xEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn xTilt<T: QTabletEvent_xTilt>(&mut self, value: T) -> i32 {
    value.xTilt(self);
    return 1;
  }
}

pub trait QTabletEvent_xTilt {
  fn xTilt(self, this: &mut QTabletEvent) -> i32;
}

// proto: int QTabletEvent::xTilt();
impl<'a> /*trait*/ QTabletEvent_xTilt for () {
  fn xTilt(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5xTiltEv()};
    unsafe {_ZNK12QTabletEvent5xTiltEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn uniqueId<T: QTabletEvent_uniqueId>(&mut self, value: T) -> i32 {
    value.uniqueId(self);
    return 1;
  }
}

pub trait QTabletEvent_uniqueId {
  fn uniqueId(self, this: &mut QTabletEvent) -> i32;
}

// proto: long long QTabletEvent::uniqueId();
impl<'a> /*trait*/ QTabletEvent_uniqueId for () {
  fn uniqueId(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8uniqueIdEv()};
    unsafe {_ZNK12QTabletEvent8uniqueIdEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn globalPosF<T: QTabletEvent_globalPosF>(&mut self, value: T) -> i32 {
    value.globalPosF(self);
    return 1;
  }
}

pub trait QTabletEvent_globalPosF {
  fn globalPosF(self, this: &mut QTabletEvent) -> i32;
}

// proto: const QPointF & QTabletEvent::globalPosF();
impl<'a> /*trait*/ QTabletEvent_globalPosF for () {
  fn globalPosF(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent10globalPosFEv()};
    unsafe {_ZNK12QTabletEvent10globalPosFEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn z<T: QTabletEvent_z>(&mut self, value: T) -> i32 {
    value.z(self);
    return 1;
  }
}

pub trait QTabletEvent_z {
  fn z(self, this: &mut QTabletEvent) -> i32;
}

// proto: int QTabletEvent::z();
impl<'a> /*trait*/ QTabletEvent_z for () {
  fn z(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1zEv()};
    unsafe {_ZNK12QTabletEvent1zEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn y<T: QTabletEvent_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QTabletEvent_y {
  fn y(self, this: &mut QTabletEvent) -> i32;
}

// proto: int QTabletEvent::y();
impl<'a> /*trait*/ QTabletEvent_y for () {
  fn y(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1yEv()};
    unsafe {_ZNK12QTabletEvent1yEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn pos<T: QTabletEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QTabletEvent_pos {
  fn pos(self, this: &mut QTabletEvent) -> i32;
}

// proto: QPoint QTabletEvent::pos();
impl<'a> /*trait*/ QTabletEvent_pos for () {
  fn pos(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent3posEv()};
    unsafe {_ZNK12QTabletEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn rotation<T: QTabletEvent_rotation>(&mut self, value: T) -> i32 {
    value.rotation(self);
    return 1;
  }
}

pub trait QTabletEvent_rotation {
  fn rotation(self, this: &mut QTabletEvent) -> i32;
}

// proto: double QTabletEvent::rotation();
impl<'a> /*trait*/ QTabletEvent_rotation for () {
  fn rotation(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8rotationEv()};
    unsafe {_ZNK12QTabletEvent8rotationEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn globalPos<T: QTabletEvent_globalPos>(&mut self, value: T) -> i32 {
    value.globalPos(self);
    return 1;
  }
}

pub trait QTabletEvent_globalPos {
  fn globalPos(self, this: &mut QTabletEvent) -> i32;
}

// proto: QPoint QTabletEvent::globalPos();
impl<'a> /*trait*/ QTabletEvent_globalPos for () {
  fn globalPos(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent9globalPosEv()};
    unsafe {_ZNK12QTabletEvent9globalPosEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn FreeQTabletEvent<T: QTabletEvent_FreeQTabletEvent>(&mut self, value: T) -> i32 {
    value.FreeQTabletEvent(self);
    return 1;
  }
}

pub trait QTabletEvent_FreeQTabletEvent {
  fn FreeQTabletEvent(self, this: &mut QTabletEvent) -> i32;
}

// proto: void QTabletEvent::FreeQTabletEvent();
impl<'a> /*trait*/ QTabletEvent_FreeQTabletEvent for () {
  fn FreeQTabletEvent(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTabletEventD0Ev()};
    unsafe {_ZN12QTabletEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn tangentialPressure<T: QTabletEvent_tangentialPressure>(&mut self, value: T) -> i32 {
    value.tangentialPressure(self);
    return 1;
  }
}

pub trait QTabletEvent_tangentialPressure {
  fn tangentialPressure(self, this: &mut QTabletEvent) -> i32;
}

// proto: double QTabletEvent::tangentialPressure();
impl<'a> /*trait*/ QTabletEvent_tangentialPressure for () {
  fn tangentialPressure(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent18tangentialPressureEv()};
    unsafe {_ZNK12QTabletEvent18tangentialPressureEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalX<T: QTabletEvent_hiResGlobalX>(&mut self, value: T) -> i32 {
    value.hiResGlobalX(self);
    return 1;
  }
}

pub trait QTabletEvent_hiResGlobalX {
  fn hiResGlobalX(self, this: &mut QTabletEvent) -> i32;
}

// proto: double QTabletEvent::hiResGlobalX();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalX for () {
  fn hiResGlobalX(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalXEv()};
    unsafe {_ZNK12QTabletEvent12hiResGlobalXEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn globalY<T: QTabletEvent_globalY>(&mut self, value: T) -> i32 {
    value.globalY(self);
    return 1;
  }
}

pub trait QTabletEvent_globalY {
  fn globalY(self, this: &mut QTabletEvent) -> i32;
}

// proto: int QTabletEvent::globalY();
impl<'a> /*trait*/ QTabletEvent_globalY for () {
  fn globalY(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalYEv()};
    unsafe {_ZNK12QTabletEvent7globalYEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalY<T: QTabletEvent_hiResGlobalY>(&mut self, value: T) -> i32 {
    value.hiResGlobalY(self);
    return 1;
  }
}

pub trait QTabletEvent_hiResGlobalY {
  fn hiResGlobalY(self, this: &mut QTabletEvent) -> i32;
}

// proto: double QTabletEvent::hiResGlobalY();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalY for () {
  fn hiResGlobalY(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalYEv()};
    unsafe {_ZNK12QTabletEvent12hiResGlobalYEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn globalX<T: QTabletEvent_globalX>(&mut self, value: T) -> i32 {
    value.globalX(self);
    return 1;
  }
}

pub trait QTabletEvent_globalX {
  fn globalX(self, this: &mut QTabletEvent) -> i32;
}

// proto: int QTabletEvent::globalX();
impl<'a> /*trait*/ QTabletEvent_globalX for () {
  fn globalX(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalXEv()};
    unsafe {_ZNK12QTabletEvent7globalXEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn posF<T: QTabletEvent_posF>(&mut self, value: T) -> i32 {
    value.posF(self);
    return 1;
  }
}

pub trait QTabletEvent_posF {
  fn posF(self, this: &mut QTabletEvent) -> i32;
}

// proto: const QPointF & QTabletEvent::posF();
impl<'a> /*trait*/ QTabletEvent_posF for () {
  fn posF(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent4posFEv()};
    unsafe {_ZNK12QTabletEvent4posFEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn pressure<T: QTabletEvent_pressure>(&mut self, value: T) -> i32 {
    value.pressure(self);
    return 1;
  }
}

pub trait QTabletEvent_pressure {
  fn pressure(self, this: &mut QTabletEvent) -> i32;
}

// proto: double QTabletEvent::pressure();
impl<'a> /*trait*/ QTabletEvent_pressure for () {
  fn pressure(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8pressureEv()};
    unsafe {_ZNK12QTabletEvent8pressureEv()};
    return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn yTilt<T: QTabletEvent_yTilt>(&mut self, value: T) -> i32 {
    value.yTilt(self);
    return 1;
  }
}

pub trait QTabletEvent_yTilt {
  fn yTilt(self, this: &mut QTabletEvent) -> i32;
}

// proto: int QTabletEvent::yTilt();
impl<'a> /*trait*/ QTabletEvent_yTilt for () {
  fn yTilt(self, this: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5yTiltEv()};
    unsafe {_ZNK12QTabletEvent5yTiltEv()};
    return 1;
  }
}

