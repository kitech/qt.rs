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
  // proto:  QTime QTime::addMSecs(int ms);
  fn _ZNK5QTime8addMSecsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto: static QTime QTime::fromMSecsSinceStartOfDay(int msecs);
  fn _ZN5QTime24fromMSecsSinceStartOfDayEi(arg0: c_int) -> *mut c_void;
  // proto: static QTime QTime::currentTime();
  fn _ZN5QTime11currentTimeEv() -> *mut c_void;
  // proto:  int QTime::second();
  fn _ZNK5QTime6secondEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTime::restart();
  fn _ZN5QTime7restartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTime::start();
  fn _ZN5QTime5startEv(qthis: *mut c_void) ;
  // proto:  bool QTime::isNull();
  fn _ZNK5QTime6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTime::msecsSinceStartOfDay();
  fn _ZNK5QTime20msecsSinceStartOfDayEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTime::hour();
  fn _ZNK5QTime4hourEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTime::elapsed();
  fn _ZNK5QTime7elapsedEv(qthis: *mut c_void) -> c_int;
  // proto:  QTime QTime::addSecs(int secs);
  fn _ZNK5QTime7addSecsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTime::isValid();
  fn _ZNK5QTime7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTime::NewQTime(int ms);
  fn _ZN5QTimeC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTime::msec();
  fn _ZNK5QTime4msecEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTime::NewQTime(int h, int m, int s, int ms);
  fn _ZN5QTimeC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  int QTime::secsTo(const QTime & );
  fn _ZNK5QTime6secsToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTime::NewQTime();
  fn _ZN5QTimeC1Ev(qthis: *mut c_void) ;
  // proto:  bool QTime::setHMS(int h, int m, int s, int ms);
  fn _ZN5QTime6setHMSEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> int8_t;
  // proto:  QString QTime::toString(const QString & format);
  fn _ZNK5QTime8toStringERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTime::msecsTo(const QTime & );
  fn _ZNK5QTime7msecsToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QTime::minute();
  fn _ZNK5QTime6minuteEv(qthis: *mut c_void) -> c_int;
  // proto: static bool QTime::isValid(int h, int m, int s, int ms);
  fn _ZN5QTime7isValidEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> int8_t;
  // proto: static QTime QTime::fromString(const QString & s, const QString & format);
  fn _ZN5QTime10fromStringERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QTime)=4
pub struct QTime {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTime {
  pub fn addMSecs<T: QTime_addMSecs>(&mut self, value: T) -> QTime {
    return value.addMSecs(self);
    // return 1;
  }
}

pub trait QTime_addMSecs {
  fn addMSecs(self, rsthis: &mut QTime) -> QTime;
}

// proto:  QTime QTime::addMSecs(int ms);
impl<'a> /*trait*/ QTime_addMSecs for (i32) {
  fn addMSecs(self, rsthis: &mut QTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime8addMSecsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QTime8addMSecsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn fromMSecsSinceStartOfDay<T: QTime_fromMSecsSinceStartOfDay>(&mut self, value: T) -> QTime {
    return value.fromMSecsSinceStartOfDay(self);
    // return 1;
  }
}

pub trait QTime_fromMSecsSinceStartOfDay {
  fn fromMSecsSinceStartOfDay(self, rsthis: &mut QTime) -> QTime;
}

// proto: static QTime QTime::fromMSecsSinceStartOfDay(int msecs);
impl<'a> /*trait*/ QTime_fromMSecsSinceStartOfDay for (i32) {
  fn fromMSecsSinceStartOfDay(self, rsthis: &mut QTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime24fromMSecsSinceStartOfDayEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN5QTime24fromMSecsSinceStartOfDayEi(arg0)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn currentTime<T: QTime_currentTime>(&mut self, value: T) -> QTime {
    return value.currentTime(self);
    // return 1;
  }
}

pub trait QTime_currentTime {
  fn currentTime(self, rsthis: &mut QTime) -> QTime;
}

// proto: static QTime QTime::currentTime();
impl<'a> /*trait*/ QTime_currentTime for () {
  fn currentTime(self, rsthis: &mut QTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime11currentTimeEv()};
    let mut ret = unsafe {_ZN5QTime11currentTimeEv()};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn second<T: QTime_second>(&mut self, value: T) -> i32 {
    return value.second(self);
    // return 1;
  }
}

pub trait QTime_second {
  fn second(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::second();
impl<'a> /*trait*/ QTime_second for () {
  fn second(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6secondEv()};
    let mut ret = unsafe {_ZNK5QTime6secondEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn restart<T: QTime_restart>(&mut self, value: T) -> i32 {
    return value.restart(self);
    // return 1;
  }
}

pub trait QTime_restart {
  fn restart(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::restart();
impl<'a> /*trait*/ QTime_restart for () {
  fn restart(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime7restartEv()};
    let mut ret = unsafe {_ZN5QTime7restartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn start<T: QTime_start>(&mut self, value: T)  {
     value.start(self);
    // return 1;
  }
}

pub trait QTime_start {
  fn start(self, rsthis: &mut QTime) ;
}

// proto:  void QTime::start();
impl<'a> /*trait*/ QTime_start for () {
  fn start(self, rsthis: &mut QTime)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime5startEv()};
     unsafe {_ZN5QTime5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn isNull<T: QTime_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QTime_isNull {
  fn isNull(self, rsthis: &mut QTime) -> i8;
}

// proto:  bool QTime::isNull();
impl<'a> /*trait*/ QTime_isNull for () {
  fn isNull(self, rsthis: &mut QTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6isNullEv()};
    let mut ret = unsafe {_ZNK5QTime6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn msecsSinceStartOfDay<T: QTime_msecsSinceStartOfDay>(&mut self, value: T) -> i32 {
    return value.msecsSinceStartOfDay(self);
    // return 1;
  }
}

pub trait QTime_msecsSinceStartOfDay {
  fn msecsSinceStartOfDay(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::msecsSinceStartOfDay();
impl<'a> /*trait*/ QTime_msecsSinceStartOfDay for () {
  fn msecsSinceStartOfDay(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime20msecsSinceStartOfDayEv()};
    let mut ret = unsafe {_ZNK5QTime20msecsSinceStartOfDayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn hour<T: QTime_hour>(&mut self, value: T) -> i32 {
    return value.hour(self);
    // return 1;
  }
}

pub trait QTime_hour {
  fn hour(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::hour();
impl<'a> /*trait*/ QTime_hour for () {
  fn hour(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime4hourEv()};
    let mut ret = unsafe {_ZNK5QTime4hourEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn elapsed<T: QTime_elapsed>(&mut self, value: T) -> i32 {
    return value.elapsed(self);
    // return 1;
  }
}

pub trait QTime_elapsed {
  fn elapsed(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::elapsed();
impl<'a> /*trait*/ QTime_elapsed for () {
  fn elapsed(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7elapsedEv()};
    let mut ret = unsafe {_ZNK5QTime7elapsedEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn addSecs<T: QTime_addSecs>(&mut self, value: T) -> QTime {
    return value.addSecs(self);
    // return 1;
  }
}

pub trait QTime_addSecs {
  fn addSecs(self, rsthis: &mut QTime) -> QTime;
}

// proto:  QTime QTime::addSecs(int secs);
impl<'a> /*trait*/ QTime_addSecs for (i32) {
  fn addSecs(self, rsthis: &mut QTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7addSecsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK5QTime7addSecsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn isValid<T: QTime_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTime_isValid {
  fn isValid(self, rsthis: &mut QTime) -> i8;
}

// proto:  bool QTime::isValid();
impl<'a> /*trait*/ QTime_isValid for () {
  fn isValid(self, rsthis: &mut QTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7isValidEv()};
    let mut ret = unsafe {_ZNK5QTime7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
    return value.msec(self);
    // return 1;
  }
}

pub trait QTime_msec {
  fn msec(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::msec();
impl<'a> /*trait*/ QTime_msec for () {
  fn msec(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime4msecEv()};
    let mut ret = unsafe {_ZNK5QTime4msecEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
    return value.secsTo(self);
    // return 1;
  }
}

pub trait QTime_secsTo {
  fn secsTo(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::secsTo(const QTime & );
impl<'a> /*trait*/ QTime_secsTo for (&'a  QTime) {
  fn secsTo(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6secsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QTime6secsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
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
  pub fn setHMS<T: QTime_setHMS>(&mut self, value: T) -> i8 {
    return value.setHMS(self);
    // return 1;
  }
}

pub trait QTime_setHMS {
  fn setHMS(self, rsthis: &mut QTime) -> i8;
}

// proto:  bool QTime::setHMS(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_setHMS for (i32, i32, i32, i32) {
  fn setHMS(self, rsthis: &mut QTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime6setHMSEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN5QTime6setHMSEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn toString<T: QTime_toString>(&mut self, value: T) -> QString {
    return value.toString(self);
    // return 1;
  }
}

pub trait QTime_toString {
  fn toString(self, rsthis: &mut QTime) -> QString;
}

// proto:  QString QTime::toString(const QString & format);
impl<'a> /*trait*/ QTime_toString for (&'a  QString) {
  fn toString(self, rsthis: &mut QTime) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime8toStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QTime8toStringERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn msecsTo<T: QTime_msecsTo>(&mut self, value: T) -> i32 {
    return value.msecsTo(self);
    // return 1;
  }
}

pub trait QTime_msecsTo {
  fn msecsTo(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::msecsTo(const QTime & );
impl<'a> /*trait*/ QTime_msecsTo for (&'a  QTime) {
  fn msecsTo(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7msecsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QTime7msecsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn minute<T: QTime_minute>(&mut self, value: T) -> i32 {
    return value.minute(self);
    // return 1;
  }
}

pub trait QTime_minute {
  fn minute(self, rsthis: &mut QTime) -> i32;
}

// proto:  int QTime::minute();
impl<'a> /*trait*/ QTime_minute for () {
  fn minute(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6minuteEv()};
    let mut ret = unsafe {_ZNK5QTime6minuteEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: static bool QTime::isValid(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_isValid for (i32, i32, i32, i32) {
  fn isValid(self, rsthis: &mut QTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime7isValidEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN5QTime7isValidEiiii(arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn fromString<T: QTime_fromString>(&mut self, value: T) -> QTime {
    return value.fromString(self);
    // return 1;
  }
}

pub trait QTime_fromString {
  fn fromString(self, rsthis: &mut QTime) -> QTime;
}

// proto: static QTime QTime::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QTime_fromString for (&'a  QString, &'a  QString) {
  fn fromString(self, rsthis: &mut QTime) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime10fromStringERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QTime10fromStringERK7QStringS2_(arg0, arg1)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

