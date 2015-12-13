// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdate::QDate;
use super::qtime::QTime;
use super::qwidget::QWidget;
use super::qdatetime::QDateTime;
use super::qevent::QEvent;
use super::qcalendarwidget::QCalendarWidget;
use super::qstring::QString;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QDateTimeEdit::NewQDateTimeEdit(const QDateTimeEdit & );
  fn _ZN13QDateTimeEditC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDate QDateTimeEdit::date();
  fn _ZNK13QDateTimeEdit4dateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDateTimeEdit::clearMinimumDateTime();
  fn _ZN13QDateTimeEdit20clearMinimumDateTimeEv(qthis: *mut c_void) ;
  // proto:  void QDateTimeEdit::clear();
  fn _ZN13QDateTimeEdit5clearEv(qthis: *mut c_void) ;
  // proto:  void QDateTimeEdit::NewQDateTimeEdit(const QTime & t, QWidget * parent);
  fn _ZN13QDateTimeEditC1ERK5QTimeP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QDateTimeEdit::setDate(const QDate & date);
  fn _ZN13QDateTimeEdit7setDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDateTime QDateTimeEdit::maximumDateTime();
  fn _ZNK13QDateTimeEdit15maximumDateTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDateTimeEdit::setMinimumDate(const QDate & min);
  fn _ZN13QDateTimeEdit14setMinimumDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTimeEdit::NewQDateTimeEdit(const QDate & d, QWidget * parent);
  fn _ZN13QDateTimeEditC1ERK5QDateP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QDateTimeEdit::setMaximumDate(const QDate & max);
  fn _ZN13QDateTimeEdit14setMaximumDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTime QDateTimeEdit::maximumTime();
  fn _ZNK13QDateTimeEdit11maximumTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QDateTimeEdit::metaObject();
  fn _ZNK13QDateTimeEdit10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QDateTimeEdit::FreeQDateTimeEdit();
  fn _ZN13QDateTimeEditD0Ev(qthis: *mut c_void) ;
  // proto:  void QDateTimeEdit::dateChanged(const QDate & date);
  fn _ZN13QDateTimeEdit11dateChangedERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTimeEdit::clearMinimumDate();
  fn _ZN13QDateTimeEdit16clearMinimumDateEv(qthis: *mut c_void) ;
  // proto:  void QDateTimeEdit::setDateRange(const QDate & min, const QDate & max);
  fn _ZN13QDateTimeEdit12setDateRangeERK5QDateS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QDateTimeEdit::setMinimumTime(const QTime & min);
  fn _ZN13QDateTimeEdit14setMinimumTimeERK5QTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTime QDateTimeEdit::time();
  fn _ZNK13QDateTimeEdit4timeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QDateTimeEdit::currentSectionIndex();
  fn _ZNK13QDateTimeEdit19currentSectionIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QDateTimeEdit::event(QEvent * event);
  fn _ZN13QDateTimeEdit5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QDateTimeEdit::setDateTime(const QDateTime & dateTime);
  fn _ZN13QDateTimeEdit11setDateTimeERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTimeEdit::setCalendarWidget(QCalendarWidget * calendarWidget);
  fn _ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTimeEdit::setDateTimeRange(const QDateTime & min, const QDateTime & max);
  fn _ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QDateTimeEdit::setTime(const QTime & time);
  fn _ZN13QDateTimeEdit7setTimeERK5QTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDateTime QDateTimeEdit::dateTime();
  fn _ZNK13QDateTimeEdit8dateTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDateTimeEdit::setMaximumTime(const QTime & max);
  fn _ZN13QDateTimeEdit14setMaximumTimeERK5QTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTimeEdit::setMinimumDateTime(const QDateTime & dt);
  fn _ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTimeEdit::clearMaximumTime();
  fn _ZN13QDateTimeEdit16clearMaximumTimeEv(qthis: *mut c_void) ;
  // proto:  bool QDateTimeEdit::calendarPopup();
  fn _ZNK13QDateTimeEdit13calendarPopupEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDateTimeEdit::setMaximumDateTime(const QDateTime & dt);
  fn _ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTimeEdit::clearMaximumDateTime();
  fn _ZN13QDateTimeEdit20clearMaximumDateTimeEv(qthis: *mut c_void) ;
  // proto:  void QDateTimeEdit::NewQDateTimeEdit(QWidget * parent);
  fn _ZN13QDateTimeEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDateTime QDateTimeEdit::minimumDateTime();
  fn _ZNK13QDateTimeEdit15minimumDateTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QCalendarWidget * QDateTimeEdit::calendarWidget();
  fn _ZNK13QDateTimeEdit14calendarWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDateTimeEdit::setDisplayFormat(const QString & format);
  fn _ZN13QDateTimeEdit16setDisplayFormatERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDateTimeEdit::clearMaximumDate();
  fn _ZN13QDateTimeEdit16clearMaximumDateEv(qthis: *mut c_void) ;
  // proto:  void QDateTimeEdit::setCalendarPopup(bool enable);
  fn _ZN13QDateTimeEdit16setCalendarPopupEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QDateTimeEdit::stepBy(int steps);
  fn _ZN13QDateTimeEdit6stepByEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDateTimeEdit::NewQDateTimeEdit(const QDateTime & dt, QWidget * parent);
  fn _ZN13QDateTimeEditC1ERK9QDateTimeP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QDateTimeEdit::displayFormat();
  fn _ZNK13QDateTimeEdit13displayFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDateTimeEdit::dateTimeChanged(const QDateTime & dateTime);
  fn _ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTime QDateTimeEdit::minimumTime();
  fn _ZNK13QDateTimeEdit11minimumTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QDateTimeEdit::sizeHint();
  fn _ZNK13QDateTimeEdit8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QDateTimeEdit::sectionCount();
  fn _ZNK13QDateTimeEdit12sectionCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDateTimeEdit::setCurrentSectionIndex(int index);
  fn _ZN13QDateTimeEdit22setCurrentSectionIndexEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QDateTimeEdit::clearMinimumTime();
  fn _ZN13QDateTimeEdit16clearMinimumTimeEv(qthis: *mut c_void) ;
  // proto:  void QDateTimeEdit::setTimeRange(const QTime & min, const QTime & max);
  fn _ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QDateTimeEdit::timeChanged(const QTime & time);
  fn _ZN13QDateTimeEdit11timeChangedERK5QTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDate QDateTimeEdit::minimumDate();
  fn _ZNK13QDateTimeEdit11minimumDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDate QDateTimeEdit::maximumDate();
  fn _ZNK13QDateTimeEdit11maximumDateEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QDateTimeEdit)=1
pub struct QDateTimeEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDateTimeEdit {
  pub fn NewQDateTimeEdit<T: QDateTimeEdit_NewQDateTimeEdit>(value: T) -> QDateTimeEdit {
    let rsthis = value.NewQDateTimeEdit();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTimeEdit_NewQDateTimeEdit {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit;
}

// proto: void QDateTimeEdit::NewQDateTimeEdit(const QDateTimeEdit & );
impl<'a> /*trait*/ QDateTimeEdit_NewQDateTimeEdit for (&'a  QDateTimeEdit) {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEditC1ERKS_(qthis, arg0)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn date<T: QDateTimeEdit_date>(&mut self, value: T) -> QDate {
    return value.date(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_date {
  fn date(self, rsthis: &mut QDateTimeEdit) -> QDate;
}

// proto:  QDate QDateTimeEdit::date();
impl<'a> /*trait*/ QDateTimeEdit_date for () {
  fn date(self, rsthis: &mut QDateTimeEdit) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit4dateEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit4dateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumDateTime<T: QDateTimeEdit_clearMinimumDateTime>(&mut self, value: T)  {
     value.clearMinimumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumDateTime {
  fn clearMinimumDateTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::clearMinimumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumDateTime for () {
  fn clearMinimumDateTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit20clearMinimumDateTimeEv()};
     unsafe {_ZN13QDateTimeEdit20clearMinimumDateTimeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clear<T: QDateTimeEdit_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clear {
  fn clear(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::clear();
impl<'a> /*trait*/ QDateTimeEdit_clear for () {
  fn clear(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit5clearEv()};
     unsafe {_ZN13QDateTimeEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QDateTimeEdit::NewQDateTimeEdit(const QTime & t, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_NewQDateTimeEdit for (&'a  QTime, &'a mut QWidget) {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC1ERK5QTimeP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEditC1ERK5QTimeP7QWidget(qthis, arg0, arg1)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDate<T: QDateTimeEdit_setDate>(&mut self, value: T)  {
     value.setDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDate {
  fn setDate(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setDate(const QDate & date);
impl<'a> /*trait*/ QDateTimeEdit_setDate for (&'a  QDate) {
  fn setDate(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit7setDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit7setDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn maximumDateTime<T: QDateTimeEdit_maximumDateTime>(&mut self, value: T) -> QDateTime {
    return value.maximumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_maximumDateTime {
  fn maximumDateTime(self, rsthis: &mut QDateTimeEdit) -> QDateTime;
}

// proto:  QDateTime QDateTimeEdit::maximumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_maximumDateTime for () {
  fn maximumDateTime(self, rsthis: &mut QDateTimeEdit) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit15maximumDateTimeEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit15maximumDateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumDate<T: QDateTimeEdit_setMinimumDate>(&mut self, value: T)  {
     value.setMinimumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMinimumDate {
  fn setMinimumDate(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setMinimumDate(const QDate & min);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumDate for (&'a  QDate) {
  fn setMinimumDate(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMinimumDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit14setMinimumDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QDateTimeEdit::NewQDateTimeEdit(const QDate & d, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_NewQDateTimeEdit for (&'a  QDate, &'a mut QWidget) {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC1ERK5QDateP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEditC1ERK5QDateP7QWidget(qthis, arg0, arg1)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumDate<T: QDateTimeEdit_setMaximumDate>(&mut self, value: T)  {
     value.setMaximumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMaximumDate {
  fn setMaximumDate(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setMaximumDate(const QDate & max);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumDate for (&'a  QDate) {
  fn setMaximumDate(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMaximumDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit14setMaximumDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn maximumTime<T: QDateTimeEdit_maximumTime>(&mut self, value: T) -> QTime {
    return value.maximumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_maximumTime {
  fn maximumTime(self, rsthis: &mut QDateTimeEdit) -> QTime;
}

// proto:  QTime QDateTimeEdit::maximumTime();
impl<'a> /*trait*/ QDateTimeEdit_maximumTime for () {
  fn maximumTime(self, rsthis: &mut QDateTimeEdit) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11maximumTimeEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit11maximumTimeEv(rsthis.qclsinst)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn metaObject<T: QDateTimeEdit_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_metaObject {
  fn metaObject(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  const QMetaObject * QDateTimeEdit::metaObject();
impl<'a> /*trait*/ QDateTimeEdit_metaObject for () {
  fn metaObject(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit10metaObjectEv()};
     unsafe {_ZNK13QDateTimeEdit10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn FreeQDateTimeEdit<T: QDateTimeEdit_FreeQDateTimeEdit>(&mut self, value: T)  {
     value.FreeQDateTimeEdit(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_FreeQDateTimeEdit {
  fn FreeQDateTimeEdit(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::FreeQDateTimeEdit();
impl<'a> /*trait*/ QDateTimeEdit_FreeQDateTimeEdit for () {
  fn FreeQDateTimeEdit(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditD0Ev()};
     unsafe {_ZN13QDateTimeEditD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn dateChanged<T: QDateTimeEdit_dateChanged>(&mut self, value: T)  {
     value.dateChanged(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_dateChanged {
  fn dateChanged(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::dateChanged(const QDate & date);
impl<'a> /*trait*/ QDateTimeEdit_dateChanged for (&'a  QDate) {
  fn dateChanged(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit11dateChangedERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit11dateChangedERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumDate<T: QDateTimeEdit_clearMinimumDate>(&mut self, value: T)  {
     value.clearMinimumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumDate {
  fn clearMinimumDate(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::clearMinimumDate();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumDate for () {
  fn clearMinimumDate(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMinimumDateEv()};
     unsafe {_ZN13QDateTimeEdit16clearMinimumDateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDateRange<T: QDateTimeEdit_setDateRange>(&mut self, value: T)  {
     value.setDateRange(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDateRange {
  fn setDateRange(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setDateRange(const QDate & min, const QDate & max);
impl<'a> /*trait*/ QDateTimeEdit_setDateRange for (&'a  QDate, &'a  QDate) {
  fn setDateRange(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit12setDateRangeERK5QDateS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit12setDateRangeERK5QDateS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumTime<T: QDateTimeEdit_setMinimumTime>(&mut self, value: T)  {
     value.setMinimumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMinimumTime {
  fn setMinimumTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setMinimumTime(const QTime & min);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumTime for (&'a  QTime) {
  fn setMinimumTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMinimumTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit14setMinimumTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn time<T: QDateTimeEdit_time>(&mut self, value: T) -> QTime {
    return value.time(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_time {
  fn time(self, rsthis: &mut QDateTimeEdit) -> QTime;
}

// proto:  QTime QDateTimeEdit::time();
impl<'a> /*trait*/ QDateTimeEdit_time for () {
  fn time(self, rsthis: &mut QDateTimeEdit) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit4timeEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit4timeEv(rsthis.qclsinst)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn currentSectionIndex<T: QDateTimeEdit_currentSectionIndex>(&mut self, value: T) -> i32 {
    return value.currentSectionIndex(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_currentSectionIndex {
  fn currentSectionIndex(self, rsthis: &mut QDateTimeEdit) -> i32;
}

// proto:  int QDateTimeEdit::currentSectionIndex();
impl<'a> /*trait*/ QDateTimeEdit_currentSectionIndex for () {
  fn currentSectionIndex(self, rsthis: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit19currentSectionIndexEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit19currentSectionIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn event<T: QDateTimeEdit_event>(&mut self, value: T) -> i8 {
    return value.event(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_event {
  fn event(self, rsthis: &mut QDateTimeEdit) -> i8;
}

// proto:  bool QDateTimeEdit::event(QEvent * event);
impl<'a> /*trait*/ QDateTimeEdit_event for (&'a mut QEvent) {
  fn event(self, rsthis: &mut QDateTimeEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QDateTimeEdit5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDateTime<T: QDateTimeEdit_setDateTime>(&mut self, value: T)  {
     value.setDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDateTime {
  fn setDateTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setDateTime(const QDateTime & dateTime);
impl<'a> /*trait*/ QDateTimeEdit_setDateTime for (&'a  QDateTime) {
  fn setDateTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit11setDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit11setDateTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setCalendarWidget<T: QDateTimeEdit_setCalendarWidget>(&mut self, value: T)  {
     value.setCalendarWidget(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setCalendarWidget {
  fn setCalendarWidget(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setCalendarWidget(QCalendarWidget * calendarWidget);
impl<'a> /*trait*/ QDateTimeEdit_setCalendarWidget for (&'a mut QCalendarWidget) {
  fn setCalendarWidget(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDateTimeRange<T: QDateTimeEdit_setDateTimeRange>(&mut self, value: T)  {
     value.setDateTimeRange(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDateTimeRange {
  fn setDateTimeRange(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setDateTimeRange(const QDateTime & min, const QDateTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setDateTimeRange for (&'a  QDateTime, &'a  QDateTime) {
  fn setDateTimeRange(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setTime<T: QDateTimeEdit_setTime>(&mut self, value: T)  {
     value.setTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setTime {
  fn setTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setTime(const QTime & time);
impl<'a> /*trait*/ QDateTimeEdit_setTime for (&'a  QTime) {
  fn setTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit7setTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit7setTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn dateTime<T: QDateTimeEdit_dateTime>(&mut self, value: T) -> QDateTime {
    return value.dateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_dateTime {
  fn dateTime(self, rsthis: &mut QDateTimeEdit) -> QDateTime;
}

// proto:  QDateTime QDateTimeEdit::dateTime();
impl<'a> /*trait*/ QDateTimeEdit_dateTime for () {
  fn dateTime(self, rsthis: &mut QDateTimeEdit) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit8dateTimeEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit8dateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumTime<T: QDateTimeEdit_setMaximumTime>(&mut self, value: T)  {
     value.setMaximumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMaximumTime {
  fn setMaximumTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setMaximumTime(const QTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumTime for (&'a  QTime) {
  fn setMaximumTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMaximumTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit14setMaximumTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumDateTime<T: QDateTimeEdit_setMinimumDateTime>(&mut self, value: T)  {
     value.setMinimumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMinimumDateTime {
  fn setMinimumDateTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setMinimumDateTime(const QDateTime & dt);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumDateTime for (&'a  QDateTime) {
  fn setMinimumDateTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumTime<T: QDateTimeEdit_clearMaximumTime>(&mut self, value: T)  {
     value.clearMaximumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumTime {
  fn clearMaximumTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::clearMaximumTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumTime for () {
  fn clearMaximumTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMaximumTimeEv()};
     unsafe {_ZN13QDateTimeEdit16clearMaximumTimeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn calendarPopup<T: QDateTimeEdit_calendarPopup>(&mut self, value: T) -> i8 {
    return value.calendarPopup(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_calendarPopup {
  fn calendarPopup(self, rsthis: &mut QDateTimeEdit) -> i8;
}

// proto:  bool QDateTimeEdit::calendarPopup();
impl<'a> /*trait*/ QDateTimeEdit_calendarPopup for () {
  fn calendarPopup(self, rsthis: &mut QDateTimeEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit13calendarPopupEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit13calendarPopupEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumDateTime<T: QDateTimeEdit_setMaximumDateTime>(&mut self, value: T)  {
     value.setMaximumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMaximumDateTime {
  fn setMaximumDateTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setMaximumDateTime(const QDateTime & dt);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumDateTime for (&'a  QDateTime) {
  fn setMaximumDateTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumDateTime<T: QDateTimeEdit_clearMaximumDateTime>(&mut self, value: T)  {
     value.clearMaximumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumDateTime {
  fn clearMaximumDateTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::clearMaximumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumDateTime for () {
  fn clearMaximumDateTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit20clearMaximumDateTimeEv()};
     unsafe {_ZN13QDateTimeEdit20clearMaximumDateTimeEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QDateTimeEdit::NewQDateTimeEdit(QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_NewQDateTimeEdit for (&'a mut QWidget) {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEditC1EP7QWidget(qthis, arg0)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn minimumDateTime<T: QDateTimeEdit_minimumDateTime>(&mut self, value: T) -> QDateTime {
    return value.minimumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_minimumDateTime {
  fn minimumDateTime(self, rsthis: &mut QDateTimeEdit) -> QDateTime;
}

// proto:  QDateTime QDateTimeEdit::minimumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_minimumDateTime for () {
  fn minimumDateTime(self, rsthis: &mut QDateTimeEdit) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit15minimumDateTimeEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit15minimumDateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn calendarWidget<T: QDateTimeEdit_calendarWidget>(&mut self, value: T) -> QCalendarWidget {
    return value.calendarWidget(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_calendarWidget {
  fn calendarWidget(self, rsthis: &mut QDateTimeEdit) -> QCalendarWidget;
}

// proto:  QCalendarWidget * QDateTimeEdit::calendarWidget();
impl<'a> /*trait*/ QDateTimeEdit_calendarWidget for () {
  fn calendarWidget(self, rsthis: &mut QDateTimeEdit) -> QCalendarWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit14calendarWidgetEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit14calendarWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QCalendarWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDisplayFormat<T: QDateTimeEdit_setDisplayFormat>(&mut self, value: T)  {
     value.setDisplayFormat(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDisplayFormat {
  fn setDisplayFormat(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setDisplayFormat(const QString & format);
impl<'a> /*trait*/ QDateTimeEdit_setDisplayFormat for (&'a  QString) {
  fn setDisplayFormat(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setDisplayFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit16setDisplayFormatERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumDate<T: QDateTimeEdit_clearMaximumDate>(&mut self, value: T)  {
     value.clearMaximumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumDate {
  fn clearMaximumDate(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::clearMaximumDate();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumDate for () {
  fn clearMaximumDate(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMaximumDateEv()};
     unsafe {_ZN13QDateTimeEdit16clearMaximumDateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setCalendarPopup<T: QDateTimeEdit_setCalendarPopup>(&mut self, value: T)  {
     value.setCalendarPopup(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setCalendarPopup {
  fn setCalendarPopup(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setCalendarPopup(bool enable);
impl<'a> /*trait*/ QDateTimeEdit_setCalendarPopup for (i8) {
  fn setCalendarPopup(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setCalendarPopupEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QDateTimeEdit16setCalendarPopupEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn stepBy<T: QDateTimeEdit_stepBy>(&mut self, value: T)  {
     value.stepBy(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_stepBy {
  fn stepBy(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::stepBy(int steps);
impl<'a> /*trait*/ QDateTimeEdit_stepBy for (i32) {
  fn stepBy(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit6stepByEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QDateTimeEdit6stepByEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QDateTimeEdit::NewQDateTimeEdit(const QDateTime & dt, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_NewQDateTimeEdit for (&'a  QDateTime, &'a mut QWidget) {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC1ERK9QDateTimeP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEditC1ERK9QDateTimeP7QWidget(qthis, arg0, arg1)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn displayFormat<T: QDateTimeEdit_displayFormat>(&mut self, value: T) -> QString {
    return value.displayFormat(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_displayFormat {
  fn displayFormat(self, rsthis: &mut QDateTimeEdit) -> QString;
}

// proto:  QString QDateTimeEdit::displayFormat();
impl<'a> /*trait*/ QDateTimeEdit_displayFormat for () {
  fn displayFormat(self, rsthis: &mut QDateTimeEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit13displayFormatEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit13displayFormatEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn dateTimeChanged<T: QDateTimeEdit_dateTimeChanged>(&mut self, value: T)  {
     value.dateTimeChanged(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_dateTimeChanged {
  fn dateTimeChanged(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::dateTimeChanged(const QDateTime & dateTime);
impl<'a> /*trait*/ QDateTimeEdit_dateTimeChanged for (&'a  QDateTime) {
  fn dateTimeChanged(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn minimumTime<T: QDateTimeEdit_minimumTime>(&mut self, value: T) -> QTime {
    return value.minimumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_minimumTime {
  fn minimumTime(self, rsthis: &mut QDateTimeEdit) -> QTime;
}

// proto:  QTime QDateTimeEdit::minimumTime();
impl<'a> /*trait*/ QDateTimeEdit_minimumTime for () {
  fn minimumTime(self, rsthis: &mut QDateTimeEdit) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11minimumTimeEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit11minimumTimeEv(rsthis.qclsinst)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn sizeHint<T: QDateTimeEdit_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_sizeHint {
  fn sizeHint(self, rsthis: &mut QDateTimeEdit) -> QSize;
}

// proto:  QSize QDateTimeEdit::sizeHint();
impl<'a> /*trait*/ QDateTimeEdit_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QDateTimeEdit) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit8sizeHintEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn sectionCount<T: QDateTimeEdit_sectionCount>(&mut self, value: T) -> i32 {
    return value.sectionCount(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_sectionCount {
  fn sectionCount(self, rsthis: &mut QDateTimeEdit) -> i32;
}

// proto:  int QDateTimeEdit::sectionCount();
impl<'a> /*trait*/ QDateTimeEdit_sectionCount for () {
  fn sectionCount(self, rsthis: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit12sectionCountEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit12sectionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setCurrentSectionIndex<T: QDateTimeEdit_setCurrentSectionIndex>(&mut self, value: T)  {
     value.setCurrentSectionIndex(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setCurrentSectionIndex {
  fn setCurrentSectionIndex(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setCurrentSectionIndex(int index);
impl<'a> /*trait*/ QDateTimeEdit_setCurrentSectionIndex for (i32) {
  fn setCurrentSectionIndex(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit22setCurrentSectionIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QDateTimeEdit22setCurrentSectionIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumTime<T: QDateTimeEdit_clearMinimumTime>(&mut self, value: T)  {
     value.clearMinimumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumTime {
  fn clearMinimumTime(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::clearMinimumTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumTime for () {
  fn clearMinimumTime(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMinimumTimeEv()};
     unsafe {_ZN13QDateTimeEdit16clearMinimumTimeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setTimeRange<T: QDateTimeEdit_setTimeRange>(&mut self, value: T)  {
     value.setTimeRange(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setTimeRange {
  fn setTimeRange(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::setTimeRange(const QTime & min, const QTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setTimeRange for (&'a  QTime, &'a  QTime) {
  fn setTimeRange(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn timeChanged<T: QDateTimeEdit_timeChanged>(&mut self, value: T)  {
     value.timeChanged(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_timeChanged {
  fn timeChanged(self, rsthis: &mut QDateTimeEdit) ;
}

// proto:  void QDateTimeEdit::timeChanged(const QTime & time);
impl<'a> /*trait*/ QDateTimeEdit_timeChanged for (&'a  QTime) {
  fn timeChanged(self, rsthis: &mut QDateTimeEdit)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit11timeChangedERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QDateTimeEdit11timeChangedERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn minimumDate<T: QDateTimeEdit_minimumDate>(&mut self, value: T) -> QDate {
    return value.minimumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_minimumDate {
  fn minimumDate(self, rsthis: &mut QDateTimeEdit) -> QDate;
}

// proto:  QDate QDateTimeEdit::minimumDate();
impl<'a> /*trait*/ QDateTimeEdit_minimumDate for () {
  fn minimumDate(self, rsthis: &mut QDateTimeEdit) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11minimumDateEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit11minimumDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn maximumDate<T: QDateTimeEdit_maximumDate>(&mut self, value: T) -> QDate {
    return value.maximumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_maximumDate {
  fn maximumDate(self, rsthis: &mut QDateTimeEdit) -> QDate;
}

// proto:  QDate QDateTimeEdit::maximumDate();
impl<'a> /*trait*/ QDateTimeEdit_maximumDate for () {
  fn maximumDate(self, rsthis: &mut QDateTimeEdit) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11maximumDateEv()};
    let mut ret = unsafe {_ZNK13QDateTimeEdit11maximumDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

