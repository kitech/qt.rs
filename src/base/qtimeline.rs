// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qeasingcurve::QEasingCurve;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTimeLine::start();
  fn _ZN9QTimeLine5startEv(qthis: *mut c_void) ;
  // proto:  void QTimeLine::NewQTimeLine(const QTimeLine & );
  fn _ZN9QTimeLineC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTimeLine::duration();
  fn _ZNK9QTimeLine8durationEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTimeLine::currentFrame();
  fn _ZNK9QTimeLine12currentFrameEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QTimeLine::metaObject();
  fn _ZNK9QTimeLine10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTimeLine::stop();
  fn _ZN9QTimeLine4stopEv(qthis: *mut c_void) ;
  // proto:  void QTimeLine::FreeQTimeLine();
  fn _ZN9QTimeLineD0Ev(qthis: *mut c_void) ;
  // proto:  void QTimeLine::setUpdateInterval(int interval);
  fn _ZN9QTimeLine17setUpdateIntervalEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QEasingCurve QTimeLine::easingCurve();
  fn _ZNK9QTimeLine11easingCurveEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTimeLine::loopCount();
  fn _ZNK9QTimeLine9loopCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setStartFrame(int frame);
  fn _ZN9QTimeLine13setStartFrameEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTimeLine::NewQTimeLine(int duration, QObject * parent);
  fn _ZN9QTimeLineC1EiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTimeLine::resume();
  fn _ZN9QTimeLine6resumeEv(qthis: *mut c_void) ;
  // proto:  void QTimeLine::setEasingCurve(const QEasingCurve & curve);
  fn _ZN9QTimeLine14setEasingCurveERK12QEasingCurve(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTimeLine::startFrame();
  fn _ZNK9QTimeLine10startFrameEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setEndFrame(int frame);
  fn _ZN9QTimeLine11setEndFrameEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTimeLine::updateInterval();
  fn _ZNK9QTimeLine14updateIntervalEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setLoopCount(int count);
  fn _ZN9QTimeLine12setLoopCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTimeLine::setCurrentTime(int msec);
  fn _ZN9QTimeLine14setCurrentTimeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTimeLine::currentTime();
  fn _ZNK9QTimeLine11currentTimeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setDuration(int duration);
  fn _ZN9QTimeLine11setDurationEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTimeLine::toggleDirection();
  fn _ZN9QTimeLine15toggleDirectionEv(qthis: *mut c_void) ;
  // proto:  int QTimeLine::endFrame();
  fn _ZNK9QTimeLine8endFrameEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setPaused(bool paused);
  fn _ZN9QTimeLine9setPausedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QTimeLine::frameForTime(int msec);
  fn _ZNK9QTimeLine12frameForTimeEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTimeLine::setFrameRange(int startFrame, int endFrame);
  fn _ZN9QTimeLine13setFrameRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  double QTimeLine::valueForTime(int msec);
  fn _ZNK9QTimeLine12valueForTimeEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  double QTimeLine::currentValue();
  fn _ZNK9QTimeLine12currentValueEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QTimeLine)=1
pub struct QTimeLine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimeLine {
  pub fn start<T: QTimeLine_start>(&mut self, value: T)  {
     value.start(self);
    // return 1;
  }
}

pub trait QTimeLine_start {
  fn start(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::start();
impl<'a> /*trait*/ QTimeLine_start for () {
  fn start(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine5startEv()};
     unsafe {_ZN9QTimeLine5startEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeLineC1ERKS_(qthis, arg0)};
    let rsthis = QTimeLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn duration<T: QTimeLine_duration>(&mut self, value: T) -> i32 {
    return value.duration(self);
    // return 1;
  }
}

pub trait QTimeLine_duration {
  fn duration(self, rsthis: &mut QTimeLine) -> i32;
}

// proto:  int QTimeLine::duration();
impl<'a> /*trait*/ QTimeLine_duration for () {
  fn duration(self, rsthis: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine8durationEv()};
    let mut ret = unsafe {_ZNK9QTimeLine8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn currentFrame<T: QTimeLine_currentFrame>(&mut self, value: T) -> i32 {
    return value.currentFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_currentFrame {
  fn currentFrame(self, rsthis: &mut QTimeLine) -> i32;
}

// proto:  int QTimeLine::currentFrame();
impl<'a> /*trait*/ QTimeLine_currentFrame for () {
  fn currentFrame(self, rsthis: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12currentFrameEv()};
    let mut ret = unsafe {_ZNK9QTimeLine12currentFrameEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn metaObject<T: QTimeLine_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTimeLine_metaObject {
  fn metaObject(self, rsthis: &mut QTimeLine) ;
}

// proto:  const QMetaObject * QTimeLine::metaObject();
impl<'a> /*trait*/ QTimeLine_metaObject for () {
  fn metaObject(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine10metaObjectEv()};
     unsafe {_ZNK9QTimeLine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn stop<T: QTimeLine_stop>(&mut self, value: T)  {
     value.stop(self);
    // return 1;
  }
}

pub trait QTimeLine_stop {
  fn stop(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::stop();
impl<'a> /*trait*/ QTimeLine_stop for () {
  fn stop(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine4stopEv()};
     unsafe {_ZN9QTimeLine4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn FreeQTimeLine<T: QTimeLine_FreeQTimeLine>(&mut self, value: T)  {
     value.FreeQTimeLine(self);
    // return 1;
  }
}

pub trait QTimeLine_FreeQTimeLine {
  fn FreeQTimeLine(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::FreeQTimeLine();
impl<'a> /*trait*/ QTimeLine_FreeQTimeLine for () {
  fn FreeQTimeLine(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLineD0Ev()};
     unsafe {_ZN9QTimeLineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setUpdateInterval<T: QTimeLine_setUpdateInterval>(&mut self, value: T)  {
     value.setUpdateInterval(self);
    // return 1;
  }
}

pub trait QTimeLine_setUpdateInterval {
  fn setUpdateInterval(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setUpdateInterval(int interval);
impl<'a> /*trait*/ QTimeLine_setUpdateInterval for (i32) {
  fn setUpdateInterval(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine17setUpdateIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine17setUpdateIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn easingCurve<T: QTimeLine_easingCurve>(&mut self, value: T) -> QEasingCurve {
    return value.easingCurve(self);
    // return 1;
  }
}

pub trait QTimeLine_easingCurve {
  fn easingCurve(self, rsthis: &mut QTimeLine) -> QEasingCurve;
}

// proto:  QEasingCurve QTimeLine::easingCurve();
impl<'a> /*trait*/ QTimeLine_easingCurve for () {
  fn easingCurve(self, rsthis: &mut QTimeLine) -> QEasingCurve {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine11easingCurveEv()};
    let mut ret = unsafe {_ZNK9QTimeLine11easingCurveEv(rsthis.qclsinst)};
    let mut ret1 = QEasingCurve{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn loopCount<T: QTimeLine_loopCount>(&mut self, value: T) -> i32 {
    return value.loopCount(self);
    // return 1;
  }
}

pub trait QTimeLine_loopCount {
  fn loopCount(self, rsthis: &mut QTimeLine) -> i32;
}

// proto:  int QTimeLine::loopCount();
impl<'a> /*trait*/ QTimeLine_loopCount for () {
  fn loopCount(self, rsthis: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine9loopCountEv()};
    let mut ret = unsafe {_ZNK9QTimeLine9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setStartFrame<T: QTimeLine_setStartFrame>(&mut self, value: T)  {
     value.setStartFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_setStartFrame {
  fn setStartFrame(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setStartFrame(int frame);
impl<'a> /*trait*/ QTimeLine_setStartFrame for (i32) {
  fn setStartFrame(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine13setStartFrameEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine13setStartFrameEi(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn resume<T: QTimeLine_resume>(&mut self, value: T)  {
     value.resume(self);
    // return 1;
  }
}

pub trait QTimeLine_resume {
  fn resume(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::resume();
impl<'a> /*trait*/ QTimeLine_resume for () {
  fn resume(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine6resumeEv()};
     unsafe {_ZN9QTimeLine6resumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setEasingCurve<T: QTimeLine_setEasingCurve>(&mut self, value: T)  {
     value.setEasingCurve(self);
    // return 1;
  }
}

pub trait QTimeLine_setEasingCurve {
  fn setEasingCurve(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setEasingCurve(const QEasingCurve & curve);
impl<'a> /*trait*/ QTimeLine_setEasingCurve for (&'a  QEasingCurve) {
  fn setEasingCurve(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine14setEasingCurveERK12QEasingCurve()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTimeLine14setEasingCurveERK12QEasingCurve(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn startFrame<T: QTimeLine_startFrame>(&mut self, value: T) -> i32 {
    return value.startFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_startFrame {
  fn startFrame(self, rsthis: &mut QTimeLine) -> i32;
}

// proto:  int QTimeLine::startFrame();
impl<'a> /*trait*/ QTimeLine_startFrame for () {
  fn startFrame(self, rsthis: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine10startFrameEv()};
    let mut ret = unsafe {_ZNK9QTimeLine10startFrameEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setEndFrame<T: QTimeLine_setEndFrame>(&mut self, value: T)  {
     value.setEndFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_setEndFrame {
  fn setEndFrame(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setEndFrame(int frame);
impl<'a> /*trait*/ QTimeLine_setEndFrame for (i32) {
  fn setEndFrame(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine11setEndFrameEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine11setEndFrameEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn updateInterval<T: QTimeLine_updateInterval>(&mut self, value: T) -> i32 {
    return value.updateInterval(self);
    // return 1;
  }
}

pub trait QTimeLine_updateInterval {
  fn updateInterval(self, rsthis: &mut QTimeLine) -> i32;
}

// proto:  int QTimeLine::updateInterval();
impl<'a> /*trait*/ QTimeLine_updateInterval for () {
  fn updateInterval(self, rsthis: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine14updateIntervalEv()};
    let mut ret = unsafe {_ZNK9QTimeLine14updateIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setLoopCount<T: QTimeLine_setLoopCount>(&mut self, value: T)  {
     value.setLoopCount(self);
    // return 1;
  }
}

pub trait QTimeLine_setLoopCount {
  fn setLoopCount(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setLoopCount(int count);
impl<'a> /*trait*/ QTimeLine_setLoopCount for (i32) {
  fn setLoopCount(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine12setLoopCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine12setLoopCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setCurrentTime<T: QTimeLine_setCurrentTime>(&mut self, value: T)  {
     value.setCurrentTime(self);
    // return 1;
  }
}

pub trait QTimeLine_setCurrentTime {
  fn setCurrentTime(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setCurrentTime(int msec);
impl<'a> /*trait*/ QTimeLine_setCurrentTime for (i32) {
  fn setCurrentTime(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine14setCurrentTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine14setCurrentTimeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn currentTime<T: QTimeLine_currentTime>(&mut self, value: T) -> i32 {
    return value.currentTime(self);
    // return 1;
  }
}

pub trait QTimeLine_currentTime {
  fn currentTime(self, rsthis: &mut QTimeLine) -> i32;
}

// proto:  int QTimeLine::currentTime();
impl<'a> /*trait*/ QTimeLine_currentTime for () {
  fn currentTime(self, rsthis: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine11currentTimeEv()};
    let mut ret = unsafe {_ZNK9QTimeLine11currentTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setDuration<T: QTimeLine_setDuration>(&mut self, value: T)  {
     value.setDuration(self);
    // return 1;
  }
}

pub trait QTimeLine_setDuration {
  fn setDuration(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setDuration(int duration);
impl<'a> /*trait*/ QTimeLine_setDuration for (i32) {
  fn setDuration(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine11setDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine11setDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn toggleDirection<T: QTimeLine_toggleDirection>(&mut self, value: T)  {
     value.toggleDirection(self);
    // return 1;
  }
}

pub trait QTimeLine_toggleDirection {
  fn toggleDirection(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::toggleDirection();
impl<'a> /*trait*/ QTimeLine_toggleDirection for () {
  fn toggleDirection(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine15toggleDirectionEv()};
     unsafe {_ZN9QTimeLine15toggleDirectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn endFrame<T: QTimeLine_endFrame>(&mut self, value: T) -> i32 {
    return value.endFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_endFrame {
  fn endFrame(self, rsthis: &mut QTimeLine) -> i32;
}

// proto:  int QTimeLine::endFrame();
impl<'a> /*trait*/ QTimeLine_endFrame for () {
  fn endFrame(self, rsthis: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine8endFrameEv()};
    let mut ret = unsafe {_ZNK9QTimeLine8endFrameEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setPaused<T: QTimeLine_setPaused>(&mut self, value: T)  {
     value.setPaused(self);
    // return 1;
  }
}

pub trait QTimeLine_setPaused {
  fn setPaused(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setPaused(bool paused);
impl<'a> /*trait*/ QTimeLine_setPaused for (i8) {
  fn setPaused(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine9setPausedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTimeLine9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn frameForTime<T: QTimeLine_frameForTime>(&mut self, value: T) -> i32 {
    return value.frameForTime(self);
    // return 1;
  }
}

pub trait QTimeLine_frameForTime {
  fn frameForTime(self, rsthis: &mut QTimeLine) -> i32;
}

// proto:  int QTimeLine::frameForTime(int msec);
impl<'a> /*trait*/ QTimeLine_frameForTime for (i32) {
  fn frameForTime(self, rsthis: &mut QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12frameForTimeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTimeLine12frameForTimeEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn setFrameRange<T: QTimeLine_setFrameRange>(&mut self, value: T)  {
     value.setFrameRange(self);
    // return 1;
  }
}

pub trait QTimeLine_setFrameRange {
  fn setFrameRange(self, rsthis: &mut QTimeLine) ;
}

// proto:  void QTimeLine::setFrameRange(int startFrame, int endFrame);
impl<'a> /*trait*/ QTimeLine_setFrameRange for (i32, i32) {
  fn setFrameRange(self, rsthis: &mut QTimeLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine13setFrameRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QTimeLine13setFrameRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn valueForTime<T: QTimeLine_valueForTime>(&mut self, value: T) -> f64 {
    return value.valueForTime(self);
    // return 1;
  }
}

pub trait QTimeLine_valueForTime {
  fn valueForTime(self, rsthis: &mut QTimeLine) -> f64;
}

// proto:  double QTimeLine::valueForTime(int msec);
impl<'a> /*trait*/ QTimeLine_valueForTime for (i32) {
  fn valueForTime(self, rsthis: &mut QTimeLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12valueForTimeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTimeLine12valueForTimeEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTimeLine {
  pub fn currentValue<T: QTimeLine_currentValue>(&mut self, value: T) -> f64 {
    return value.currentValue(self);
    // return 1;
  }
}

pub trait QTimeLine_currentValue {
  fn currentValue(self, rsthis: &mut QTimeLine) -> f64;
}

// proto:  double QTimeLine::currentValue();
impl<'a> /*trait*/ QTimeLine_currentValue for () {
  fn currentValue(self, rsthis: &mut QTimeLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12currentValueEv()};
    let mut ret = unsafe {_ZNK9QTimeLine12currentValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

