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
  pub fn addMSecs<RetType, T: QTime_addMSecs<RetType>>(&mut self, value: T) -> RetType {
    return value.addMSecs(self);
    // return 1;
  }
}

pub trait QTime_addMSecs<RetType> {
  fn addMSecs(self, rsthis: &mut QTime) -> RetType;
}

// proto:  QTime QTime::addMSecs(int ms);
impl<'a> /*trait*/ QTime_addMSecs<QTime> for (i32) {
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
  pub fn fromMSecsSinceStartOfDay<RetType, T: QTime_fromMSecsSinceStartOfDay<RetType>>(&mut self, value: T) -> RetType {
    return value.fromMSecsSinceStartOfDay(self);
    // return 1;
  }
}

pub trait QTime_fromMSecsSinceStartOfDay<RetType> {
  fn fromMSecsSinceStartOfDay(self, rsthis: &mut QTime) -> RetType;
}

// proto: static QTime QTime::fromMSecsSinceStartOfDay(int msecs);
impl<'a> /*trait*/ QTime_fromMSecsSinceStartOfDay<QTime> for (i32) {
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
  pub fn currentTime<RetType, T: QTime_currentTime<RetType>>(&mut self, value: T) -> RetType {
    return value.currentTime(self);
    // return 1;
  }
}

pub trait QTime_currentTime<RetType> {
  fn currentTime(self, rsthis: &mut QTime) -> RetType;
}

// proto: static QTime QTime::currentTime();
impl<'a> /*trait*/ QTime_currentTime<QTime> for () {
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
  pub fn second<RetType, T: QTime_second<RetType>>(&mut self, value: T) -> RetType {
    return value.second(self);
    // return 1;
  }
}

pub trait QTime_second<RetType> {
  fn second(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::second();
impl<'a> /*trait*/ QTime_second<i32> for () {
  fn second(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6secondEv()};
    let mut ret = unsafe {_ZNK5QTime6secondEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn restart<RetType, T: QTime_restart<RetType>>(&mut self, value: T) -> RetType {
    return value.restart(self);
    // return 1;
  }
}

pub trait QTime_restart<RetType> {
  fn restart(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::restart();
impl<'a> /*trait*/ QTime_restart<i32> for () {
  fn restart(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime7restartEv()};
    let mut ret = unsafe {_ZN5QTime7restartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn start<RetType, T: QTime_start<RetType>>(&mut self, value: T) -> RetType {
    return value.start(self);
    // return 1;
  }
}

pub trait QTime_start<RetType> {
  fn start(self, rsthis: &mut QTime) -> RetType;
}

// proto:  void QTime::start();
impl<'a> /*trait*/ QTime_start<()> for () {
  fn start(self, rsthis: &mut QTime) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QTime5startEv()};
     unsafe {_ZN5QTime5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn isNull<RetType, T: QTime_isNull<RetType>>(&mut self, value: T) -> RetType {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QTime_isNull<RetType> {
  fn isNull(self, rsthis: &mut QTime) -> RetType;
}

// proto:  bool QTime::isNull();
impl<'a> /*trait*/ QTime_isNull<i8> for () {
  fn isNull(self, rsthis: &mut QTime) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6isNullEv()};
    let mut ret = unsafe {_ZNK5QTime6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn msecsSinceStartOfDay<RetType, T: QTime_msecsSinceStartOfDay<RetType>>(&mut self, value: T) -> RetType {
    return value.msecsSinceStartOfDay(self);
    // return 1;
  }
}

pub trait QTime_msecsSinceStartOfDay<RetType> {
  fn msecsSinceStartOfDay(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::msecsSinceStartOfDay();
impl<'a> /*trait*/ QTime_msecsSinceStartOfDay<i32> for () {
  fn msecsSinceStartOfDay(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime20msecsSinceStartOfDayEv()};
    let mut ret = unsafe {_ZNK5QTime20msecsSinceStartOfDayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn hour<RetType, T: QTime_hour<RetType>>(&mut self, value: T) -> RetType {
    return value.hour(self);
    // return 1;
  }
}

pub trait QTime_hour<RetType> {
  fn hour(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::hour();
impl<'a> /*trait*/ QTime_hour<i32> for () {
  fn hour(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime4hourEv()};
    let mut ret = unsafe {_ZNK5QTime4hourEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn elapsed<RetType, T: QTime_elapsed<RetType>>(&mut self, value: T) -> RetType {
    return value.elapsed(self);
    // return 1;
  }
}

pub trait QTime_elapsed<RetType> {
  fn elapsed(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::elapsed();
impl<'a> /*trait*/ QTime_elapsed<i32> for () {
  fn elapsed(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime7elapsedEv()};
    let mut ret = unsafe {_ZNK5QTime7elapsedEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTime {
  pub fn addSecs<RetType, T: QTime_addSecs<RetType>>(&mut self, value: T) -> RetType {
    return value.addSecs(self);
    // return 1;
  }
}

pub trait QTime_addSecs<RetType> {
  fn addSecs(self, rsthis: &mut QTime) -> RetType;
}

// proto:  QTime QTime::addSecs(int secs);
impl<'a> /*trait*/ QTime_addSecs<QTime> for (i32) {
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
  pub fn isValid<RetType, T: QTime_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTime_isValid<RetType> {
  fn isValid(self, rsthis: &mut QTime) -> RetType;
}

// proto:  bool QTime::isValid();
impl<'a> /*trait*/ QTime_isValid<i8> for () {
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
  pub fn msec<RetType, T: QTime_msec<RetType>>(&mut self, value: T) -> RetType {
    return value.msec(self);
    // return 1;
  }
}

pub trait QTime_msec<RetType> {
  fn msec(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::msec();
impl<'a> /*trait*/ QTime_msec<i32> for () {
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
  pub fn secsTo<RetType, T: QTime_secsTo<RetType>>(&mut self, value: T) -> RetType {
    return value.secsTo(self);
    // return 1;
  }
}

pub trait QTime_secsTo<RetType> {
  fn secsTo(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::secsTo(const QTime & );
impl<'a> /*trait*/ QTime_secsTo<i32> for (&'a  QTime) {
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
  pub fn setHMS<RetType, T: QTime_setHMS<RetType>>(&mut self, value: T) -> RetType {
    return value.setHMS(self);
    // return 1;
  }
}

pub trait QTime_setHMS<RetType> {
  fn setHMS(self, rsthis: &mut QTime) -> RetType;
}

// proto:  bool QTime::setHMS(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_setHMS<i8> for (i32, i32, i32, i32) {
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
  pub fn toString<RetType, T: QTime_toString<RetType>>(&mut self, value: T) -> RetType {
    return value.toString(self);
    // return 1;
  }
}

pub trait QTime_toString<RetType> {
  fn toString(self, rsthis: &mut QTime) -> RetType;
}

// proto:  QString QTime::toString(const QString & format);
impl<'a> /*trait*/ QTime_toString<QString> for (&'a  QString) {
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
  pub fn msecsTo<RetType, T: QTime_msecsTo<RetType>>(&mut self, value: T) -> RetType {
    return value.msecsTo(self);
    // return 1;
  }
}

pub trait QTime_msecsTo<RetType> {
  fn msecsTo(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::msecsTo(const QTime & );
impl<'a> /*trait*/ QTime_msecsTo<i32> for (&'a  QTime) {
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
  pub fn minute<RetType, T: QTime_minute<RetType>>(&mut self, value: T) -> RetType {
    return value.minute(self);
    // return 1;
  }
}

pub trait QTime_minute<RetType> {
  fn minute(self, rsthis: &mut QTime) -> RetType;
}

// proto:  int QTime::minute();
impl<'a> /*trait*/ QTime_minute<i32> for () {
  fn minute(self, rsthis: &mut QTime) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QTime6minuteEv()};
    let mut ret = unsafe {_ZNK5QTime6minuteEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: static bool QTime::isValid(int h, int m, int s, int ms);
impl<'a> /*trait*/ QTime_isValid<i8> for (i32, i32, i32, i32) {
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
  pub fn fromString<RetType, T: QTime_fromString<RetType>>(&mut self, value: T) -> RetType {
    return value.fromString(self);
    // return 1;
  }
}

pub trait QTime_fromString<RetType> {
  fn fromString(self, rsthis: &mut QTime) -> RetType;
}

// proto: static QTime QTime::fromString(const QString & s, const QString & format);
impl<'a> /*trait*/ QTime_fromString<QTime> for (&'a  QString, &'a  QString) {
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

