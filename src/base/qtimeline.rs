// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qeasingcurve::QEasingCurve;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN9QTimeLine5startEv() -> i32;
  fn _ZN9QTimeLineC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK9QTimeLine8durationEv() -> i32;
  fn _ZNK9QTimeLine12currentFrameEv() -> i32;
  fn _ZNK9QTimeLine10metaObjectEv() -> i32;
  fn _ZN9QTimeLine4stopEv() -> i32;
  fn _ZN9QTimeLineD0Ev() -> i32;
  fn _ZN9QTimeLine17setUpdateIntervalEi(arg0: c_int) -> i32;
  fn _ZNK9QTimeLine11easingCurveEv() -> i32;
  fn _ZNK9QTimeLine9loopCountEv() -> i32;
  fn _ZN9QTimeLine13setStartFrameEi(arg0: c_int) -> i32;
  fn _ZN9QTimeLineC1EiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> i32;
  fn _ZN9QTimeLine6resumeEv() -> i32;
  fn _ZN9QTimeLine14setEasingCurveERK12QEasingCurve(arg0: *const c_void) -> i32;
  fn _ZNK9QTimeLine10startFrameEv() -> i32;
  fn _ZN9QTimeLine11setEndFrameEi(arg0: c_int) -> i32;
  fn _ZNK9QTimeLine14updateIntervalEv() -> i32;
  fn _ZN9QTimeLine12setLoopCountEi(arg0: c_int) -> i32;
  fn _ZN9QTimeLine14setCurrentTimeEi(arg0: c_int) -> i32;
  fn _ZNK9QTimeLine11currentTimeEv() -> i32;
  fn _ZN9QTimeLine11setDurationEi(arg0: c_int) -> i32;
  fn _ZN9QTimeLine15toggleDirectionEv() -> i32;
  fn _ZNK9QTimeLine8endFrameEv() -> i32;
  fn _ZN9QTimeLine9setPausedEb(arg0: int8_t) -> i32;
  fn _ZNK9QTimeLine12frameForTimeEi(arg0: c_int) -> i32;
  fn _ZN9QTimeLine13setFrameRangeEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK9QTimeLine12valueForTimeEi(arg0: c_int) -> i32;
  fn _ZNK9QTimeLine12currentValueEv() -> i32;
}

// body block begin
// class sizeof(QTimeLine)=1
pub struct QTimeLine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimeLine {
  pub fn start<T: QTimeLine_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QTimeLine_start {
  fn start(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::start();
impl<'a> /*trait*/ QTimeLine_start for () {
  fn start(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine5startEv()};
    unsafe {_ZN9QTimeLine5startEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn NewQTimeLine<T: QTimeLine_NewQTimeLine>(value: T) -> QTimeLine {
    let rsthis = value.NewQTimeLine();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeLine_NewQTimeLine {
  fn NewQTimeLine(self) -> QTimeLine;
}

// proto: void QTimeLine::NewQTimeLine(const QTimeLine & );
impl<'a> /*trait*/ QTimeLine_NewQTimeLine for (&'a  QTimeLine) {
  fn NewQTimeLine(self) -> QTimeLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLineC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTimeLineC1ERKS_(qthis, arg0)};
    let rsthis = QTimeLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn duration<T: QTimeLine_duration>(&mut self, value: T) -> i32 {
    value.duration(self);
    return 1;
  }
}

pub trait QTimeLine_duration {
  fn duration(self, this: &mut QTimeLine) -> i32;
}

// proto: int QTimeLine::duration();
impl<'a> /*trait*/ QTimeLine_duration for () {
  fn duration(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine8durationEv()};
    unsafe {_ZNK9QTimeLine8durationEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn currentFrame<T: QTimeLine_currentFrame>(&mut self, value: T) -> i32 {
    value.currentFrame(self);
    return 1;
  }
}

pub trait QTimeLine_currentFrame {
  fn currentFrame(self, this: &mut QTimeLine) -> i32;
}

// proto: int QTimeLine::currentFrame();
impl<'a> /*trait*/ QTimeLine_currentFrame for () {
  fn currentFrame(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12currentFrameEv()};
    unsafe {_ZNK9QTimeLine12currentFrameEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn metaObject<T: QTimeLine_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTimeLine_metaObject {
  fn metaObject(self, this: &mut QTimeLine) -> i32;
}

// proto: const QMetaObject * QTimeLine::metaObject();
impl<'a> /*trait*/ QTimeLine_metaObject for () {
  fn metaObject(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine10metaObjectEv()};
    unsafe {_ZNK9QTimeLine10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn stop<T: QTimeLine_stop>(&mut self, value: T) -> i32 {
    value.stop(self);
    return 1;
  }
}

pub trait QTimeLine_stop {
  fn stop(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::stop();
impl<'a> /*trait*/ QTimeLine_stop for () {
  fn stop(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine4stopEv()};
    unsafe {_ZN9QTimeLine4stopEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn FreeQTimeLine<T: QTimeLine_FreeQTimeLine>(&mut self, value: T) -> i32 {
    value.FreeQTimeLine(self);
    return 1;
  }
}

pub trait QTimeLine_FreeQTimeLine {
  fn FreeQTimeLine(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::FreeQTimeLine();
impl<'a> /*trait*/ QTimeLine_FreeQTimeLine for () {
  fn FreeQTimeLine(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLineD0Ev()};
    unsafe {_ZN9QTimeLineD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setUpdateInterval<T: QTimeLine_setUpdateInterval>(&mut self, value: T) -> i32 {
    value.setUpdateInterval(self);
    return 1;
  }
}

pub trait QTimeLine_setUpdateInterval {
  fn setUpdateInterval(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setUpdateInterval(int interval);
impl<'a> /*trait*/ QTimeLine_setUpdateInterval for (i32) {
  fn setUpdateInterval(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine17setUpdateIntervalEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTimeLine17setUpdateIntervalEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn easingCurve<T: QTimeLine_easingCurve>(&mut self, value: T) -> i32 {
    value.easingCurve(self);
    return 1;
  }
}

pub trait QTimeLine_easingCurve {
  fn easingCurve(self, this: &mut QTimeLine) -> i32;
}

// proto: QEasingCurve QTimeLine::easingCurve();
impl<'a> /*trait*/ QTimeLine_easingCurve for () {
  fn easingCurve(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine11easingCurveEv()};
    unsafe {_ZNK9QTimeLine11easingCurveEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn loopCount<T: QTimeLine_loopCount>(&mut self, value: T) -> i32 {
    value.loopCount(self);
    return 1;
  }
}

pub trait QTimeLine_loopCount {
  fn loopCount(self, this: &mut QTimeLine) -> i32;
}

// proto: int QTimeLine::loopCount();
impl<'a> /*trait*/ QTimeLine_loopCount for () {
  fn loopCount(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine9loopCountEv()};
    unsafe {_ZNK9QTimeLine9loopCountEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setStartFrame<T: QTimeLine_setStartFrame>(&mut self, value: T) -> i32 {
    value.setStartFrame(self);
    return 1;
  }
}

pub trait QTimeLine_setStartFrame {
  fn setStartFrame(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setStartFrame(int frame);
impl<'a> /*trait*/ QTimeLine_setStartFrame for (i32) {
  fn setStartFrame(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine13setStartFrameEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTimeLine13setStartFrameEi(arg0)};
    return 1;
  }
}

// proto: void QTimeLine::NewQTimeLine(int duration, QObject * parent);
impl<'a> /*trait*/ QTimeLine_NewQTimeLine for (i32, &'a mut QObject) {
  fn NewQTimeLine(self) -> QTimeLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLineC1EiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeLineC1EiP7QObject(qthis, arg0, arg1)};
    let rsthis = QTimeLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn resume<T: QTimeLine_resume>(&mut self, value: T) -> i32 {
    value.resume(self);
    return 1;
  }
}

pub trait QTimeLine_resume {
  fn resume(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::resume();
impl<'a> /*trait*/ QTimeLine_resume for () {
  fn resume(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine6resumeEv()};
    unsafe {_ZN9QTimeLine6resumeEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setEasingCurve<T: QTimeLine_setEasingCurve>(&mut self, value: T) -> i32 {
    value.setEasingCurve(self);
    return 1;
  }
}

pub trait QTimeLine_setEasingCurve {
  fn setEasingCurve(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setEasingCurve(const QEasingCurve & curve);
impl<'a> /*trait*/ QTimeLine_setEasingCurve for (&'a  QEasingCurve) {
  fn setEasingCurve(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine14setEasingCurveERK12QEasingCurve()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTimeLine14setEasingCurveERK12QEasingCurve(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn startFrame<T: QTimeLine_startFrame>(&mut self, value: T) -> i32 {
    value.startFrame(self);
    return 1;
  }
}

pub trait QTimeLine_startFrame {
  fn startFrame(self, this: &mut QTimeLine) -> i32;
}

// proto: int QTimeLine::startFrame();
impl<'a> /*trait*/ QTimeLine_startFrame for () {
  fn startFrame(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine10startFrameEv()};
    unsafe {_ZNK9QTimeLine10startFrameEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setEndFrame<T: QTimeLine_setEndFrame>(&mut self, value: T) -> i32 {
    value.setEndFrame(self);
    return 1;
  }
}

pub trait QTimeLine_setEndFrame {
  fn setEndFrame(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setEndFrame(int frame);
impl<'a> /*trait*/ QTimeLine_setEndFrame for (i32) {
  fn setEndFrame(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine11setEndFrameEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTimeLine11setEndFrameEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn updateInterval<T: QTimeLine_updateInterval>(&mut self, value: T) -> i32 {
    value.updateInterval(self);
    return 1;
  }
}

pub trait QTimeLine_updateInterval {
  fn updateInterval(self, this: &mut QTimeLine) -> i32;
}

// proto: int QTimeLine::updateInterval();
impl<'a> /*trait*/ QTimeLine_updateInterval for () {
  fn updateInterval(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine14updateIntervalEv()};
    unsafe {_ZNK9QTimeLine14updateIntervalEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setLoopCount<T: QTimeLine_setLoopCount>(&mut self, value: T) -> i32 {
    value.setLoopCount(self);
    return 1;
  }
}

pub trait QTimeLine_setLoopCount {
  fn setLoopCount(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setLoopCount(int count);
impl<'a> /*trait*/ QTimeLine_setLoopCount for (i32) {
  fn setLoopCount(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine12setLoopCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTimeLine12setLoopCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setCurrentTime<T: QTimeLine_setCurrentTime>(&mut self, value: T) -> i32 {
    value.setCurrentTime(self);
    return 1;
  }
}

pub trait QTimeLine_setCurrentTime {
  fn setCurrentTime(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setCurrentTime(int msec);
impl<'a> /*trait*/ QTimeLine_setCurrentTime for (i32) {
  fn setCurrentTime(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine14setCurrentTimeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTimeLine14setCurrentTimeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn currentTime<T: QTimeLine_currentTime>(&mut self, value: T) -> i32 {
    value.currentTime(self);
    return 1;
  }
}

pub trait QTimeLine_currentTime {
  fn currentTime(self, this: &mut QTimeLine) -> i32;
}

// proto: int QTimeLine::currentTime();
impl<'a> /*trait*/ QTimeLine_currentTime for () {
  fn currentTime(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine11currentTimeEv()};
    unsafe {_ZNK9QTimeLine11currentTimeEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setDuration<T: QTimeLine_setDuration>(&mut self, value: T) -> i32 {
    value.setDuration(self);
    return 1;
  }
}

pub trait QTimeLine_setDuration {
  fn setDuration(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setDuration(int duration);
impl<'a> /*trait*/ QTimeLine_setDuration for (i32) {
  fn setDuration(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine11setDurationEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTimeLine11setDurationEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn toggleDirection<T: QTimeLine_toggleDirection>(&mut self, value: T) -> i32 {
    value.toggleDirection(self);
    return 1;
  }
}

pub trait QTimeLine_toggleDirection {
  fn toggleDirection(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::toggleDirection();
impl<'a> /*trait*/ QTimeLine_toggleDirection for () {
  fn toggleDirection(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine15toggleDirectionEv()};
    unsafe {_ZN9QTimeLine15toggleDirectionEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn endFrame<T: QTimeLine_endFrame>(&mut self, value: T) -> i32 {
    value.endFrame(self);
    return 1;
  }
}

pub trait QTimeLine_endFrame {
  fn endFrame(self, this: &mut QTimeLine) -> i32;
}

// proto: int QTimeLine::endFrame();
impl<'a> /*trait*/ QTimeLine_endFrame for () {
  fn endFrame(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine8endFrameEv()};
    unsafe {_ZNK9QTimeLine8endFrameEv()};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setPaused<T: QTimeLine_setPaused>(&mut self, value: T) -> i32 {
    value.setPaused(self);
    return 1;
  }
}

pub trait QTimeLine_setPaused {
  fn setPaused(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setPaused(bool paused);
impl<'a> /*trait*/ QTimeLine_setPaused for (i8) {
  fn setPaused(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine9setPausedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTimeLine9setPausedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn frameForTime<T: QTimeLine_frameForTime>(&mut self, value: T) -> i32 {
    value.frameForTime(self);
    return 1;
  }
}

pub trait QTimeLine_frameForTime {
  fn frameForTime(self, this: &mut QTimeLine) -> i32;
}

// proto: int QTimeLine::frameForTime(int msec);
impl<'a> /*trait*/ QTimeLine_frameForTime for (i32) {
  fn frameForTime(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12frameForTimeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QTimeLine12frameForTimeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setFrameRange<T: QTimeLine_setFrameRange>(&mut self, value: T) -> i32 {
    value.setFrameRange(self);
    return 1;
  }
}

pub trait QTimeLine_setFrameRange {
  fn setFrameRange(self, this: &mut QTimeLine) -> i32;
}

// proto: void QTimeLine::setFrameRange(int startFrame, int endFrame);
impl<'a> /*trait*/ QTimeLine_setFrameRange for (i32, i32) {
  fn setFrameRange(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine13setFrameRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN9QTimeLine13setFrameRangeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn valueForTime<T: QTimeLine_valueForTime>(&mut self, value: T) -> i32 {
    value.valueForTime(self);
    return 1;
  }
}

pub trait QTimeLine_valueForTime {
  fn valueForTime(self, this: &mut QTimeLine) -> i32;
}

// proto: double QTimeLine::valueForTime(int msec);
impl<'a> /*trait*/ QTimeLine_valueForTime for (i32) {
  fn valueForTime(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12valueForTimeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QTimeLine12valueForTimeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn currentValue<T: QTimeLine_currentValue>(&mut self, value: T) -> i32 {
    value.currentValue(self);
    return 1;
  }
}

pub trait QTimeLine_currentValue {
  fn currentValue(self, this: &mut QTimeLine) -> i32;
}

// proto: double QTimeLine::currentValue();
impl<'a> /*trait*/ QTimeLine_currentValue for () {
  fn currentValue(self, this: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12currentValueEv()};
    unsafe {_ZNK9QTimeLine12currentValueEv()};
    return 1;
  }
}

