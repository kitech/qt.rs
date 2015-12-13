// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK5QTime8addMSecsEi(arg0: c_int) -> i32;
  fn _ZN5QTime24fromMSecsSinceStartOfDayEi(arg0: c_int) -> i32;
  fn _ZN5QTime11currentTimeEv() -> i32;
  fn _ZNK5QTime6secondEv() -> i32;
  fn _ZN5QTime7restartEv() -> i32;
  fn _ZN5QTime5startEv() -> i32;
  fn _ZNK5QTime6isNullEv() -> i32;
  fn _ZNK5QTime20msecsSinceStartOfDayEv() -> i32;
  fn _ZNK5QTime4hourEv() -> i32;
  fn _ZNK5QTime7elapsedEv() -> i32;
  fn _ZNK5QTime7addSecsEi(arg0: c_int) -> i32;
  fn _ZNK5QTime7isValidEv() -> i32;
  fn _ZN5QTimeC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  fn _ZNK5QTime4msecEv() -> i32;
  fn _ZN5QTimeC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZNK5QTime6secsToERKS_(arg0: *const c_void) -> i32;
  fn _ZN5QTimeC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN5QTime6setHMSEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZNK5QTime8toStringERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK5QTime7msecsToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK5QTime6minuteEv() -> i32;
  fn _ZN5QTime7isValidEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN5QTime10fromStringERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTime)=4
pub struct QTime {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTime {
  pub fn addMSecs<T: QTime_addMSecs>(&mut self, value: T) -> i32 {
    value.addMSecs(self);
    return 1;
  }
}

pub trait QTime_addMSecs {
  fn addMSecs(self, this: &mut QTime) -> i32;
}

// proto: QTime QTime::addMSecs(int ms);
impl<'a> /*trait*/ QTime_addMSecs for (i32) {
  fn addMSecs(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime8addMSecsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK5QTime8addMSecsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn fromMSecsSinceStartOfDay<T: QTime_fromMSecsSinceStartOfDay>(&mut self, value: T) -> i32 {
    value.fromMSecsSinceStartOfDay(self);
    return 1;
  }
}

pub trait QTime_fromMSecsSinceStartOfDay {
  fn fromMSecsSinceStartOfDay(self, this: &mut QTime) -> i32;
}

// proto: QTime QTime::fromMSecsSinceStartOfDay(int msecs);
impl<'a> /*trait*/ QTime_fromMSecsSinceStartOfDay for (i32) {
  fn fromMSecsSinceStartOfDay(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime24fromMSecsSinceStartOfDayEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QTime24fromMSecsSinceStartOfDayEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn currentTime<T: QTime_currentTime>(&mut self, value: T) -> i32 {
    value.currentTime(self);
    return 1;
  }
}

pub trait QTime_currentTime {
  fn currentTime(self, this: &mut QTime) -> i32;
}

// proto: QTime QTime::currentTime();
impl<'a> /*trait*/ QTime_currentTime for () {
  fn currentTime(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime11currentTimeEv()};
    unsafe {_ZN5QTime11currentTimeEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn second<T: QTime_second>(&mut self, value: T) -> i32 {
    value.second(self);
    return 1;
  }
}

pub trait QTime_second {
  fn second(self, this: &mut QTime) -> i32;
}

// proto: int QTime::second();
impl<'a> /*trait*/ QTime_second for () {
  fn second(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6secondEv()};
    unsafe {_ZNK5QTime6secondEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn restart<T: QTime_restart>(&mut self, value: T) -> i32 {
    value.restart(self);
    return 1;
  }
}

pub trait QTime_restart {
  fn restart(self, this: &mut QTime) -> i32;
}

// proto: int QTime::restart();
impl<'a> /*trait*/ QTime_restart for () {
  fn restart(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime7restartEv()};
    unsafe {_ZN5QTime7restartEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn start<T: QTime_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QTime_start {
  fn start(self, this: &mut QTime) -> i32;
}

// proto: void QTime::start();
impl<'a> /*trait*/ QTime_start for () {
  fn start(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime5startEv()};
    unsafe {_ZN5QTime5startEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn isNull<T: QTime_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QTime_isNull {
  fn isNull(self, this: &mut QTime) -> i32;
}

// proto: bool QTime::isNull();
impl<'a> /*trait*/ QTime_isNull for () {
  fn isNull(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6isNullEv()};
    unsafe {_ZNK5QTime6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn msecsSinceStartOfDay<T: QTime_msecsSinceStartOfDay>(&mut self, value: T) -> i32 {
    value.msecsSinceStartOfDay(self);
    return 1;
  }
}

pub trait QTime_msecsSinceStartOfDay {
  fn msecsSinceStartOfDay(self, this: &mut QTime) -> i32;
}

// proto: int QTime::msecsSinceStartOfDay();
impl<'a> /*trait*/ QTime_msecsSinceStartOfDay for () {
  fn msecsSinceStartOfDay(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime20msecsSinceStartOfDayEv()};
    unsafe {_ZNK5QTime20msecsSinceStartOfDayEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn hour<T: QTime_hour>(&mut self, value: T) -> i32 {
    value.hour(self);
    return 1;
  }
}

pub trait QTime_hour {
  fn hour(self, this: &mut QTime) -> i32;
}

// proto: int QTime::hour();
impl<'a> /*trait*/ QTime_hour for () {
  fn hour(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime4hourEv()};
    unsafe {_ZNK5QTime4hourEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn elapsed<T: QTime_elapsed>(&mut self, value: T) -> i32 {
    value.elapsed(self);
    return 1;
  }
}

pub trait QTime_elapsed {
  fn elapsed(self, this: &mut QTime) -> i32;
}

// proto: int QTime::elapsed();
impl<'a> /*trait*/ QTime_elapsed for () {
  fn elapsed(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7elapsedEv()};
    unsafe {_ZNK5QTime7elapsedEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn addSecs<T: QTime_addSecs>(&mut self, value: T) -> i32 {
    value.addSecs(self);
    return 1;
  }
}

pub trait QTime_addSecs {
  fn addSecs(self, this: &mut QTime) -> i32;
}

// proto: QTime QTime::addSecs(int secs);
impl<'a> /*trait*/ QTime_addSecs for (i32) {
  fn addSecs(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7addSecsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK5QTime7addSecsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn isValid<T: QTime_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTime_isValid {
  fn isValid(self, this: &mut QTime) -> i32;
}

// proto: bool QTime::isValid();
impl<'a> /*trait*/ QTime_isValid for () {
  fn isValid(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7isValidEv()};
    unsafe {_ZNK5QTime7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn NewQTime<T: QTime_NewQTime>(value: T) -> QTime {
    let rsthis = value.NewQTime();
    return rsthis;
    // return 1;
  }
}

pub trait QTime_NewQTime {
  fn NewQTime(self) -> QTime;
}

// proto: void QTime::NewQTime(int ms);
impl<'a> /*trait*/ QTime_NewQTime for (i32) {
  fn NewQTime(self) -> QTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTimeC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QTimeC1Ei(qthis, arg0)};
    let rsthis = QTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn msec<T: QTime_msec>(&mut self, value: T) -> i32 {
    value.msec(self);
    return 1;
  }
}

pub trait QTime_msec {
  fn msec(self, this: &mut QTime) -> i32;
}

// proto: int QTime::msec();
impl<'a> /*trait*/ QTime_msec for () {
  fn msec(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime4msecEv()};
    unsafe {_ZNK5QTime4msecEv()};
    return 1;
  }
}

// proto: void QTime::NewQTime(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_NewQTime for (i32, i32, i32, i32) {
  fn NewQTime(self) -> QTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTimeC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QTimeC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn secsTo<T: QTime_secsTo>(&mut self, value: T) -> i32 {
    value.secsTo(self);
    return 1;
  }
}

pub trait QTime_secsTo {
  fn secsTo(self, this: &mut QTime) -> i32;
}

// proto: int QTime::secsTo(const QTime & );
impl<'a> /*trait*/ QTime_secsTo for (&'a  QTime) {
  fn secsTo(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6secsToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QTime6secsToERKS_(arg0)};
    return 1;
  }
}

// proto: void QTime::NewQTime();
impl<'a> /*trait*/ QTime_NewQTime for () {
  fn NewQTime(self) -> QTime {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTimeC1Ev()};
    unsafe {_ZN5QTimeC1Ev(qthis)};
    let rsthis = QTime{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn setHMS<T: QTime_setHMS>(&mut self, value: T) -> i32 {
    value.setHMS(self);
    return 1;
  }
}

pub trait QTime_setHMS {
  fn setHMS(self, this: &mut QTime) -> i32;
}

// proto: bool QTime::setHMS(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_setHMS for (i32, i32, i32, i32) {
  fn setHMS(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime6setHMSEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QTime6setHMSEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn toString<T: QTime_toString>(&mut self, value: T) -> i32 {
    value.toString(self);
    return 1;
  }
}

pub trait QTime_toString {
  fn toString(self, this: &mut QTime) -> i32;
}

// proto: QString QTime::toString(const QString & format);
impl<'a> /*trait*/ QTime_toString for (&'a  QString) {
  fn toString(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime8toStringERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QTime8toStringERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn msecsTo<T: QTime_msecsTo>(&mut self, value: T) -> i32 {
    value.msecsTo(self);
    return 1;
  }
}

pub trait QTime_msecsTo {
  fn msecsTo(self, this: &mut QTime) -> i32;
}

// proto: int QTime::msecsTo(const QTime & );
impl<'a> /*trait*/ QTime_msecsTo for (&'a  QTime) {
  fn msecsTo(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7msecsToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QTime7msecsToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn minute<T: QTime_minute>(&mut self, value: T) -> i32 {
    value.minute(self);
    return 1;
  }
}

pub trait QTime_minute {
  fn minute(self, this: &mut QTime) -> i32;
}

// proto: int QTime::minute();
impl<'a> /*trait*/ QTime_minute for () {
  fn minute(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6minuteEv()};
    unsafe {_ZNK5QTime6minuteEv()};
    return 1;
  }
}

// proto: bool QTime::isValid(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_isValid for (i32, i32, i32, i32) {
  fn isValid(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime7isValidEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QTime7isValidEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QTime {
  pub fn fromString<T: QTime_fromString>(&mut self, value: T) -> i32 {
    value.fromString(self);
    return 1;
  }
}

pub trait QTime_fromString {
  fn fromString(self, this: &mut QTime) -> i32;
}

// proto: QTime QTime::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QTime_fromString for (&'a  QString, &'a  QString) {
  fn fromString(self, this: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QTime10fromStringERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

