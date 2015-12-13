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
  // proto: void QStyleHints::setMouseDoubleClickInterval(int mouseDoubleClickInterval);
  fn _ZN11QStyleHints27setMouseDoubleClickIntervalEi(arg0: c_int) -> i32;
  // proto: int QStyleHints::mousePressAndHoldInterval();
  fn _ZNK11QStyleHints25mousePressAndHoldIntervalEv() -> i32;
  // proto: int QStyleHints::passwordMaskDelay();
  fn _ZNK11QStyleHints17passwordMaskDelayEv() -> i32;
  // proto: const QMetaObject * QStyleHints::metaObject();
  fn _ZNK11QStyleHints10metaObjectEv() -> i32;
  // proto: void QStyleHints::setKeyboardInputInterval(int keyboardInputInterval);
  fn _ZN11QStyleHints24setKeyboardInputIntervalEi(arg0: c_int) -> i32;
  // proto: void QStyleHints::NewQStyleHints();
  fn _ZN11QStyleHintsC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QStyleHints::startDragDistanceChanged(int startDragDistance);
  fn _ZN11QStyleHints24startDragDistanceChangedEi(arg0: c_int) -> i32;
  // proto: bool QStyleHints::showIsFullScreen();
  fn _ZNK11QStyleHints16showIsFullScreenEv() -> i32;
  // proto: bool QStyleHints::useRtlExtensions();
  fn _ZNK11QStyleHints16useRtlExtensionsEv() -> i32;
  // proto: void QStyleHints::mouseDoubleClickIntervalChanged(int mouseDoubleClickInterval);
  fn _ZN11QStyleHints31mouseDoubleClickIntervalChangedEi(arg0: c_int) -> i32;
  // proto: void QStyleHints::setStartDragDistance(int startDragDistance);
  fn _ZN11QStyleHints20setStartDragDistanceEi(arg0: c_int) -> i32;
  // proto: bool QStyleHints::setFocusOnTouchRelease();
  fn _ZNK11QStyleHints22setFocusOnTouchReleaseEv() -> i32;
  // proto: int QStyleHints::startDragVelocity();
  fn _ZNK11QStyleHints17startDragVelocityEv() -> i32;
  // proto: int QStyleHints::startDragTime();
  fn _ZNK11QStyleHints13startDragTimeEv() -> i32;
  // proto: int QStyleHints::keyboardInputInterval();
  fn _ZNK11QStyleHints21keyboardInputIntervalEv() -> i32;
  // proto: void QStyleHints::keyboardInputIntervalChanged(int keyboardInputInterval);
  fn _ZN11QStyleHints28keyboardInputIntervalChangedEi(arg0: c_int) -> i32;
  // proto: void QStyleHints::setStartDragTime(int startDragTime);
  fn _ZN11QStyleHints16setStartDragTimeEi(arg0: c_int) -> i32;
  // proto: void QStyleHints::setCursorFlashTime(int cursorFlashTime);
  fn _ZN11QStyleHints18setCursorFlashTimeEi(arg0: c_int) -> i32;
  // proto: int QStyleHints::cursorFlashTime();
  fn _ZNK11QStyleHints15cursorFlashTimeEv() -> i32;
  // proto: void QStyleHints::cursorFlashTimeChanged(int cursorFlashTime);
  fn _ZN11QStyleHints22cursorFlashTimeChangedEi(arg0: c_int) -> i32;
  // proto: QChar QStyleHints::passwordMaskCharacter();
  fn _ZNK11QStyleHints21passwordMaskCharacterEv() -> i32;
  // proto: int QStyleHints::keyboardAutoRepeatRate();
  fn _ZNK11QStyleHints22keyboardAutoRepeatRateEv() -> i32;
  // proto: int QStyleHints::startDragDistance();
  fn _ZNK11QStyleHints17startDragDistanceEv() -> i32;
  // proto: double QStyleHints::fontSmoothingGamma();
  fn _ZNK11QStyleHints18fontSmoothingGammaEv() -> i32;
  // proto: bool QStyleHints::singleClickActivation();
  fn _ZNK11QStyleHints21singleClickActivationEv() -> i32;
  // proto: int QStyleHints::mouseDoubleClickInterval();
  fn _ZNK11QStyleHints24mouseDoubleClickIntervalEv() -> i32;
  // proto: void QStyleHints::startDragTimeChanged(int startDragTime);
  fn _ZN11QStyleHints20startDragTimeChangedEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QStyleHints)=1
pub struct QStyleHints {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleHints {
  pub fn setMouseDoubleClickInterval<T: QStyleHints_setMouseDoubleClickInterval>(&mut self, value: T) -> i32 {
    value.setMouseDoubleClickInterval(self);
    return 1;
  }
}

pub trait QStyleHints_setMouseDoubleClickInterval {
  fn setMouseDoubleClickInterval(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::setMouseDoubleClickInterval(int mouseDoubleClickInterval);
impl<'a> /*trait*/ QStyleHints_setMouseDoubleClickInterval for (i32) {
  fn setMouseDoubleClickInterval(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints27setMouseDoubleClickIntervalEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints27setMouseDoubleClickIntervalEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn mousePressAndHoldInterval<T: QStyleHints_mousePressAndHoldInterval>(&mut self, value: T) -> i32 {
    value.mousePressAndHoldInterval(self);
    return 1;
  }
}

pub trait QStyleHints_mousePressAndHoldInterval {
  fn mousePressAndHoldInterval(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::mousePressAndHoldInterval();
impl<'a> /*trait*/ QStyleHints_mousePressAndHoldInterval for () {
  fn mousePressAndHoldInterval(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints25mousePressAndHoldIntervalEv()};
    unsafe {_ZNK11QStyleHints25mousePressAndHoldIntervalEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn passwordMaskDelay<T: QStyleHints_passwordMaskDelay>(&mut self, value: T) -> i32 {
    value.passwordMaskDelay(self);
    return 1;
  }
}

pub trait QStyleHints_passwordMaskDelay {
  fn passwordMaskDelay(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::passwordMaskDelay();
impl<'a> /*trait*/ QStyleHints_passwordMaskDelay for () {
  fn passwordMaskDelay(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints17passwordMaskDelayEv()};
    unsafe {_ZNK11QStyleHints17passwordMaskDelayEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn metaObject<T: QStyleHints_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QStyleHints_metaObject {
  fn metaObject(self, this: &mut QStyleHints) -> i32;
}

// proto: const QMetaObject * QStyleHints::metaObject();
impl<'a> /*trait*/ QStyleHints_metaObject for () {
  fn metaObject(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints10metaObjectEv()};
    unsafe {_ZNK11QStyleHints10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn setKeyboardInputInterval<T: QStyleHints_setKeyboardInputInterval>(&mut self, value: T) -> i32 {
    value.setKeyboardInputInterval(self);
    return 1;
  }
}

pub trait QStyleHints_setKeyboardInputInterval {
  fn setKeyboardInputInterval(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::setKeyboardInputInterval(int keyboardInputInterval);
impl<'a> /*trait*/ QStyleHints_setKeyboardInputInterval for (i32) {
  fn setKeyboardInputInterval(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints24setKeyboardInputIntervalEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints24setKeyboardInputIntervalEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn NewQStyleHints<T: QStyleHints_NewQStyleHints>(value: T) -> QStyleHints {
    let rsthis = value.NewQStyleHints();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHints_NewQStyleHints {
  fn NewQStyleHints(self) -> QStyleHints;
}

// proto: void QStyleHints::NewQStyleHints();
impl<'a> /*trait*/ QStyleHints_NewQStyleHints for () {
  fn NewQStyleHints(self) -> QStyleHints {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHintsC1Ev()};
    unsafe {_ZN11QStyleHintsC1Ev(qthis)};
    let rsthis = QStyleHints{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn startDragDistanceChanged<T: QStyleHints_startDragDistanceChanged>(&mut self, value: T) -> i32 {
    value.startDragDistanceChanged(self);
    return 1;
  }
}

pub trait QStyleHints_startDragDistanceChanged {
  fn startDragDistanceChanged(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::startDragDistanceChanged(int startDragDistance);
impl<'a> /*trait*/ QStyleHints_startDragDistanceChanged for (i32) {
  fn startDragDistanceChanged(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints24startDragDistanceChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints24startDragDistanceChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn showIsFullScreen<T: QStyleHints_showIsFullScreen>(&mut self, value: T) -> i32 {
    value.showIsFullScreen(self);
    return 1;
  }
}

pub trait QStyleHints_showIsFullScreen {
  fn showIsFullScreen(self, this: &mut QStyleHints) -> i32;
}

// proto: bool QStyleHints::showIsFullScreen();
impl<'a> /*trait*/ QStyleHints_showIsFullScreen for () {
  fn showIsFullScreen(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints16showIsFullScreenEv()};
    unsafe {_ZNK11QStyleHints16showIsFullScreenEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn useRtlExtensions<T: QStyleHints_useRtlExtensions>(&mut self, value: T) -> i32 {
    value.useRtlExtensions(self);
    return 1;
  }
}

pub trait QStyleHints_useRtlExtensions {
  fn useRtlExtensions(self, this: &mut QStyleHints) -> i32;
}

// proto: bool QStyleHints::useRtlExtensions();
impl<'a> /*trait*/ QStyleHints_useRtlExtensions for () {
  fn useRtlExtensions(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints16useRtlExtensionsEv()};
    unsafe {_ZNK11QStyleHints16useRtlExtensionsEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn mouseDoubleClickIntervalChanged<T: QStyleHints_mouseDoubleClickIntervalChanged>(&mut self, value: T) -> i32 {
    value.mouseDoubleClickIntervalChanged(self);
    return 1;
  }
}

pub trait QStyleHints_mouseDoubleClickIntervalChanged {
  fn mouseDoubleClickIntervalChanged(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::mouseDoubleClickIntervalChanged(int mouseDoubleClickInterval);
impl<'a> /*trait*/ QStyleHints_mouseDoubleClickIntervalChanged for (i32) {
  fn mouseDoubleClickIntervalChanged(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints31mouseDoubleClickIntervalChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints31mouseDoubleClickIntervalChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn setStartDragDistance<T: QStyleHints_setStartDragDistance>(&mut self, value: T) -> i32 {
    value.setStartDragDistance(self);
    return 1;
  }
}

pub trait QStyleHints_setStartDragDistance {
  fn setStartDragDistance(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::setStartDragDistance(int startDragDistance);
impl<'a> /*trait*/ QStyleHints_setStartDragDistance for (i32) {
  fn setStartDragDistance(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints20setStartDragDistanceEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints20setStartDragDistanceEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn setFocusOnTouchRelease<T: QStyleHints_setFocusOnTouchRelease>(&mut self, value: T) -> i32 {
    value.setFocusOnTouchRelease(self);
    return 1;
  }
}

pub trait QStyleHints_setFocusOnTouchRelease {
  fn setFocusOnTouchRelease(self, this: &mut QStyleHints) -> i32;
}

// proto: bool QStyleHints::setFocusOnTouchRelease();
impl<'a> /*trait*/ QStyleHints_setFocusOnTouchRelease for () {
  fn setFocusOnTouchRelease(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints22setFocusOnTouchReleaseEv()};
    unsafe {_ZNK11QStyleHints22setFocusOnTouchReleaseEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn startDragVelocity<T: QStyleHints_startDragVelocity>(&mut self, value: T) -> i32 {
    value.startDragVelocity(self);
    return 1;
  }
}

pub trait QStyleHints_startDragVelocity {
  fn startDragVelocity(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::startDragVelocity();
impl<'a> /*trait*/ QStyleHints_startDragVelocity for () {
  fn startDragVelocity(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints17startDragVelocityEv()};
    unsafe {_ZNK11QStyleHints17startDragVelocityEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn startDragTime<T: QStyleHints_startDragTime>(&mut self, value: T) -> i32 {
    value.startDragTime(self);
    return 1;
  }
}

pub trait QStyleHints_startDragTime {
  fn startDragTime(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::startDragTime();
impl<'a> /*trait*/ QStyleHints_startDragTime for () {
  fn startDragTime(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints13startDragTimeEv()};
    unsafe {_ZNK11QStyleHints13startDragTimeEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn keyboardInputInterval<T: QStyleHints_keyboardInputInterval>(&mut self, value: T) -> i32 {
    value.keyboardInputInterval(self);
    return 1;
  }
}

pub trait QStyleHints_keyboardInputInterval {
  fn keyboardInputInterval(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::keyboardInputInterval();
impl<'a> /*trait*/ QStyleHints_keyboardInputInterval for () {
  fn keyboardInputInterval(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints21keyboardInputIntervalEv()};
    unsafe {_ZNK11QStyleHints21keyboardInputIntervalEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn keyboardInputIntervalChanged<T: QStyleHints_keyboardInputIntervalChanged>(&mut self, value: T) -> i32 {
    value.keyboardInputIntervalChanged(self);
    return 1;
  }
}

pub trait QStyleHints_keyboardInputIntervalChanged {
  fn keyboardInputIntervalChanged(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::keyboardInputIntervalChanged(int keyboardInputInterval);
impl<'a> /*trait*/ QStyleHints_keyboardInputIntervalChanged for (i32) {
  fn keyboardInputIntervalChanged(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints28keyboardInputIntervalChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints28keyboardInputIntervalChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn setStartDragTime<T: QStyleHints_setStartDragTime>(&mut self, value: T) -> i32 {
    value.setStartDragTime(self);
    return 1;
  }
}

pub trait QStyleHints_setStartDragTime {
  fn setStartDragTime(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::setStartDragTime(int startDragTime);
impl<'a> /*trait*/ QStyleHints_setStartDragTime for (i32) {
  fn setStartDragTime(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints16setStartDragTimeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints16setStartDragTimeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn setCursorFlashTime<T: QStyleHints_setCursorFlashTime>(&mut self, value: T) -> i32 {
    value.setCursorFlashTime(self);
    return 1;
  }
}

pub trait QStyleHints_setCursorFlashTime {
  fn setCursorFlashTime(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::setCursorFlashTime(int cursorFlashTime);
impl<'a> /*trait*/ QStyleHints_setCursorFlashTime for (i32) {
  fn setCursorFlashTime(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints18setCursorFlashTimeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints18setCursorFlashTimeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn cursorFlashTime<T: QStyleHints_cursorFlashTime>(&mut self, value: T) -> i32 {
    value.cursorFlashTime(self);
    return 1;
  }
}

pub trait QStyleHints_cursorFlashTime {
  fn cursorFlashTime(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::cursorFlashTime();
impl<'a> /*trait*/ QStyleHints_cursorFlashTime for () {
  fn cursorFlashTime(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints15cursorFlashTimeEv()};
    unsafe {_ZNK11QStyleHints15cursorFlashTimeEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn cursorFlashTimeChanged<T: QStyleHints_cursorFlashTimeChanged>(&mut self, value: T) -> i32 {
    value.cursorFlashTimeChanged(self);
    return 1;
  }
}

pub trait QStyleHints_cursorFlashTimeChanged {
  fn cursorFlashTimeChanged(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::cursorFlashTimeChanged(int cursorFlashTime);
impl<'a> /*trait*/ QStyleHints_cursorFlashTimeChanged for (i32) {
  fn cursorFlashTimeChanged(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints22cursorFlashTimeChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints22cursorFlashTimeChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn passwordMaskCharacter<T: QStyleHints_passwordMaskCharacter>(&mut self, value: T) -> i32 {
    value.passwordMaskCharacter(self);
    return 1;
  }
}

pub trait QStyleHints_passwordMaskCharacter {
  fn passwordMaskCharacter(self, this: &mut QStyleHints) -> i32;
}

// proto: QChar QStyleHints::passwordMaskCharacter();
impl<'a> /*trait*/ QStyleHints_passwordMaskCharacter for () {
  fn passwordMaskCharacter(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints21passwordMaskCharacterEv()};
    unsafe {_ZNK11QStyleHints21passwordMaskCharacterEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn keyboardAutoRepeatRate<T: QStyleHints_keyboardAutoRepeatRate>(&mut self, value: T) -> i32 {
    value.keyboardAutoRepeatRate(self);
    return 1;
  }
}

pub trait QStyleHints_keyboardAutoRepeatRate {
  fn keyboardAutoRepeatRate(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::keyboardAutoRepeatRate();
impl<'a> /*trait*/ QStyleHints_keyboardAutoRepeatRate for () {
  fn keyboardAutoRepeatRate(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints22keyboardAutoRepeatRateEv()};
    unsafe {_ZNK11QStyleHints22keyboardAutoRepeatRateEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn startDragDistance<T: QStyleHints_startDragDistance>(&mut self, value: T) -> i32 {
    value.startDragDistance(self);
    return 1;
  }
}

pub trait QStyleHints_startDragDistance {
  fn startDragDistance(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::startDragDistance();
impl<'a> /*trait*/ QStyleHints_startDragDistance for () {
  fn startDragDistance(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints17startDragDistanceEv()};
    unsafe {_ZNK11QStyleHints17startDragDistanceEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn fontSmoothingGamma<T: QStyleHints_fontSmoothingGamma>(&mut self, value: T) -> i32 {
    value.fontSmoothingGamma(self);
    return 1;
  }
}

pub trait QStyleHints_fontSmoothingGamma {
  fn fontSmoothingGamma(self, this: &mut QStyleHints) -> i32;
}

// proto: double QStyleHints::fontSmoothingGamma();
impl<'a> /*trait*/ QStyleHints_fontSmoothingGamma for () {
  fn fontSmoothingGamma(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints18fontSmoothingGammaEv()};
    unsafe {_ZNK11QStyleHints18fontSmoothingGammaEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn singleClickActivation<T: QStyleHints_singleClickActivation>(&mut self, value: T) -> i32 {
    value.singleClickActivation(self);
    return 1;
  }
}

pub trait QStyleHints_singleClickActivation {
  fn singleClickActivation(self, this: &mut QStyleHints) -> i32;
}

// proto: bool QStyleHints::singleClickActivation();
impl<'a> /*trait*/ QStyleHints_singleClickActivation for () {
  fn singleClickActivation(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints21singleClickActivationEv()};
    unsafe {_ZNK11QStyleHints21singleClickActivationEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn mouseDoubleClickInterval<T: QStyleHints_mouseDoubleClickInterval>(&mut self, value: T) -> i32 {
    value.mouseDoubleClickInterval(self);
    return 1;
  }
}

pub trait QStyleHints_mouseDoubleClickInterval {
  fn mouseDoubleClickInterval(self, this: &mut QStyleHints) -> i32;
}

// proto: int QStyleHints::mouseDoubleClickInterval();
impl<'a> /*trait*/ QStyleHints_mouseDoubleClickInterval for () {
  fn mouseDoubleClickInterval(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStyleHints24mouseDoubleClickIntervalEv()};
    unsafe {_ZNK11QStyleHints24mouseDoubleClickIntervalEv()};
    return 1;
  }
}

impl /*struct*/ QStyleHints {
  pub fn startDragTimeChanged<T: QStyleHints_startDragTimeChanged>(&mut self, value: T) -> i32 {
    value.startDragTimeChanged(self);
    return 1;
  }
}

pub trait QStyleHints_startDragTimeChanged {
  fn startDragTimeChanged(self, this: &mut QStyleHints) -> i32;
}

// proto: void QStyleHints::startDragTimeChanged(int startDragTime);
impl<'a> /*trait*/ QStyleHints_startDragTimeChanged for (i32) {
  fn startDragTimeChanged(self, this: &mut QStyleHints) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStyleHints20startDragTimeChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QStyleHints20startDragTimeChangedEi(arg0)};
    return 1;
  }
}

