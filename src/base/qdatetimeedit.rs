// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtime::QTime;
use super::qwidget::QWidget;
use super::qdate::QDate;
use super::qevent::QEvent;
use super::qdatetime::QDateTime;
use super::qcalendarwidget::QCalendarWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QDateTimeEdit::NewQDateTimeEdit(const QDateTimeEdit & );
  fn _ZN13QDateTimeEditC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QDate QDateTimeEdit::date();
  fn _ZNK13QDateTimeEdit4dateEv() -> i32;
  // proto: void QDateTimeEdit::clearMinimumDateTime();
  fn _ZN13QDateTimeEdit20clearMinimumDateTimeEv() -> i32;
  // proto: void QDateTimeEdit::clear();
  fn _ZN13QDateTimeEdit5clearEv() -> i32;
  // proto: void QDateTimeEdit::NewQDateTimeEdit(const QTime & t, QWidget * parent);
  fn _ZN13QDateTimeEditC1ERK5QTimeP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QDateTimeEdit::setDate(const QDate & date);
  fn _ZN13QDateTimeEdit7setDateERK5QDate(arg0: *const c_void) -> i32;
  // proto: QDateTime QDateTimeEdit::maximumDateTime();
  fn _ZNK13QDateTimeEdit15maximumDateTimeEv() -> i32;
  // proto: void QDateTimeEdit::setMinimumDate(const QDate & min);
  fn _ZN13QDateTimeEdit14setMinimumDateERK5QDate(arg0: *const c_void) -> i32;
  // proto: void QDateTimeEdit::NewQDateTimeEdit(const QDate & d, QWidget * parent);
  fn _ZN13QDateTimeEditC1ERK5QDateP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QDateTimeEdit::setMaximumDate(const QDate & max);
  fn _ZN13QDateTimeEdit14setMaximumDateERK5QDate(arg0: *const c_void) -> i32;
  // proto: QTime QDateTimeEdit::maximumTime();
  fn _ZNK13QDateTimeEdit11maximumTimeEv() -> i32;
  // proto: const QMetaObject * QDateTimeEdit::metaObject();
  fn _ZNK13QDateTimeEdit10metaObjectEv() -> i32;
  // proto: void QDateTimeEdit::FreeQDateTimeEdit();
  fn _ZN13QDateTimeEditD0Ev() -> i32;
  // proto: void QDateTimeEdit::dateChanged(const QDate & date);
  fn _ZN13QDateTimeEdit11dateChangedERK5QDate(arg0: *const c_void) -> i32;
  // proto: void QDateTimeEdit::clearMinimumDate();
  fn _ZN13QDateTimeEdit16clearMinimumDateEv() -> i32;
  // proto: void QDateTimeEdit::setDateRange(const QDate & min, const QDate & max);
  fn _ZN13QDateTimeEdit12setDateRangeERK5QDateS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QDateTimeEdit::setMinimumTime(const QTime & min);
  fn _ZN13QDateTimeEdit14setMinimumTimeERK5QTime(arg0: *const c_void) -> i32;
  // proto: QTime QDateTimeEdit::time();
  fn _ZNK13QDateTimeEdit4timeEv() -> i32;
  // proto: int QDateTimeEdit::currentSectionIndex();
  fn _ZNK13QDateTimeEdit19currentSectionIndexEv() -> i32;
  // proto: bool QDateTimeEdit::event(QEvent * event);
  fn _ZN13QDateTimeEdit5eventEP6QEvent(arg0: *mut c_void) -> i32;
  // proto: void QDateTimeEdit::setDateTime(const QDateTime & dateTime);
  fn _ZN13QDateTimeEdit11setDateTimeERK9QDateTime(arg0: *const c_void) -> i32;
  // proto: void QDateTimeEdit::setCalendarWidget(QCalendarWidget * calendarWidget);
  fn _ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget(arg0: *mut c_void) -> i32;
  // proto: void QDateTimeEdit::setDateTimeRange(const QDateTime & min, const QDateTime & max);
  fn _ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QDateTimeEdit::setTime(const QTime & time);
  fn _ZN13QDateTimeEdit7setTimeERK5QTime(arg0: *const c_void) -> i32;
  // proto: QDateTime QDateTimeEdit::dateTime();
  fn _ZNK13QDateTimeEdit8dateTimeEv() -> i32;
  // proto: void QDateTimeEdit::setMaximumTime(const QTime & max);
  fn _ZN13QDateTimeEdit14setMaximumTimeERK5QTime(arg0: *const c_void) -> i32;
  // proto: void QDateTimeEdit::setMinimumDateTime(const QDateTime & dt);
  fn _ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime(arg0: *const c_void) -> i32;
  // proto: void QDateTimeEdit::clearMaximumTime();
  fn _ZN13QDateTimeEdit16clearMaximumTimeEv() -> i32;
  // proto: bool QDateTimeEdit::calendarPopup();
  fn _ZNK13QDateTimeEdit13calendarPopupEv() -> i32;
  // proto: void QDateTimeEdit::setMaximumDateTime(const QDateTime & dt);
  fn _ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime(arg0: *const c_void) -> i32;
  // proto: void QDateTimeEdit::clearMaximumDateTime();
  fn _ZN13QDateTimeEdit20clearMaximumDateTimeEv() -> i32;
  // proto: void QDateTimeEdit::NewQDateTimeEdit(QWidget * parent);
  fn _ZN13QDateTimeEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QDateTime QDateTimeEdit::minimumDateTime();
  fn _ZNK13QDateTimeEdit15minimumDateTimeEv() -> i32;
  // proto: QCalendarWidget * QDateTimeEdit::calendarWidget();
  fn _ZNK13QDateTimeEdit14calendarWidgetEv() -> i32;
  // proto: void QDateTimeEdit::setDisplayFormat(const QString & format);
  fn _ZN13QDateTimeEdit16setDisplayFormatERK7QString(arg0: *const c_void) -> i32;
  // proto: void QDateTimeEdit::clearMaximumDate();
  fn _ZN13QDateTimeEdit16clearMaximumDateEv() -> i32;
  // proto: void QDateTimeEdit::setCalendarPopup(bool enable);
  fn _ZN13QDateTimeEdit16setCalendarPopupEb(arg0: int8_t) -> i32;
  // proto: void QDateTimeEdit::stepBy(int steps);
  fn _ZN13QDateTimeEdit6stepByEi(arg0: c_int) -> i32;
  // proto: void QDateTimeEdit::NewQDateTimeEdit(const QDateTime & dt, QWidget * parent);
  fn _ZN13QDateTimeEditC1ERK9QDateTimeP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QString QDateTimeEdit::displayFormat();
  fn _ZNK13QDateTimeEdit13displayFormatEv() -> i32;
  // proto: void QDateTimeEdit::dateTimeChanged(const QDateTime & dateTime);
  fn _ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime(arg0: *const c_void) -> i32;
  // proto: QTime QDateTimeEdit::minimumTime();
  fn _ZNK13QDateTimeEdit11minimumTimeEv() -> i32;
  // proto: QSize QDateTimeEdit::sizeHint();
  fn _ZNK13QDateTimeEdit8sizeHintEv() -> i32;
  // proto: int QDateTimeEdit::sectionCount();
  fn _ZNK13QDateTimeEdit12sectionCountEv() -> i32;
  // proto: void QDateTimeEdit::setCurrentSectionIndex(int index);
  fn _ZN13QDateTimeEdit22setCurrentSectionIndexEi(arg0: c_int) -> i32;
  // proto: void QDateTimeEdit::clearMinimumTime();
  fn _ZN13QDateTimeEdit16clearMinimumTimeEv() -> i32;
  // proto: void QDateTimeEdit::setTimeRange(const QTime & min, const QTime & max);
  fn _ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QDateTimeEdit::timeChanged(const QTime & time);
  fn _ZN13QDateTimeEdit11timeChangedERK5QTime(arg0: *const c_void) -> i32;
  // proto: QDate QDateTimeEdit::minimumDate();
  fn _ZNK13QDateTimeEdit11minimumDateEv() -> i32;
  // proto: QDate QDateTimeEdit::maximumDate();
  fn _ZNK13QDateTimeEdit11maximumDateEv() -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEditC1ERKS_(qthis, arg0)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn date<T: QDateTimeEdit_date>(&mut self, value: T) -> i32 {
    value.date(self);
    return 1;
  }
}

pub trait QDateTimeEdit_date {
  fn date(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QDate QDateTimeEdit::date();
impl<'a> /*trait*/ QDateTimeEdit_date for () {
  fn date(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit4dateEv()};
    unsafe {_ZNK13QDateTimeEdit4dateEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumDateTime<T: QDateTimeEdit_clearMinimumDateTime>(&mut self, value: T) -> i32 {
    value.clearMinimumDateTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumDateTime {
  fn clearMinimumDateTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::clearMinimumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumDateTime for () {
  fn clearMinimumDateTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit20clearMinimumDateTimeEv()};
    unsafe {_ZN13QDateTimeEdit20clearMinimumDateTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clear<T: QDateTimeEdit_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QDateTimeEdit_clear {
  fn clear(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::clear();
impl<'a> /*trait*/ QDateTimeEdit_clear for () {
  fn clear(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit5clearEv()};
    unsafe {_ZN13QDateTimeEdit5clearEv()};
    return 1;
  }
}

// proto: void QDateTimeEdit::NewQDateTimeEdit(const QTime & t, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_NewQDateTimeEdit for (&'a  QTime, &'a mut QWidget) {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC1ERK5QTimeP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEditC1ERK5QTimeP7QWidget(qthis, arg0, arg1)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDate<T: QDateTimeEdit_setDate>(&mut self, value: T) -> i32 {
    value.setDate(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setDate {
  fn setDate(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setDate(const QDate & date);
impl<'a> /*trait*/ QDateTimeEdit_setDate for (&'a  QDate) {
  fn setDate(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit7setDateERK5QDate()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit7setDateERK5QDate(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn maximumDateTime<T: QDateTimeEdit_maximumDateTime>(&mut self, value: T) -> i32 {
    value.maximumDateTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_maximumDateTime {
  fn maximumDateTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QDateTime QDateTimeEdit::maximumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_maximumDateTime for () {
  fn maximumDateTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit15maximumDateTimeEv()};
    unsafe {_ZNK13QDateTimeEdit15maximumDateTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumDate<T: QDateTimeEdit_setMinimumDate>(&mut self, value: T) -> i32 {
    value.setMinimumDate(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setMinimumDate {
  fn setMinimumDate(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setMinimumDate(const QDate & min);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumDate for (&'a  QDate) {
  fn setMinimumDate(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMinimumDateERK5QDate()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit14setMinimumDateERK5QDate(arg0)};
    return 1;
  }
}

// proto: void QDateTimeEdit::NewQDateTimeEdit(const QDate & d, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_NewQDateTimeEdit for (&'a  QDate, &'a mut QWidget) {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC1ERK5QDateP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEditC1ERK5QDateP7QWidget(qthis, arg0, arg1)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumDate<T: QDateTimeEdit_setMaximumDate>(&mut self, value: T) -> i32 {
    value.setMaximumDate(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setMaximumDate {
  fn setMaximumDate(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setMaximumDate(const QDate & max);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumDate for (&'a  QDate) {
  fn setMaximumDate(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMaximumDateERK5QDate()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit14setMaximumDateERK5QDate(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn maximumTime<T: QDateTimeEdit_maximumTime>(&mut self, value: T) -> i32 {
    value.maximumTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_maximumTime {
  fn maximumTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QTime QDateTimeEdit::maximumTime();
impl<'a> /*trait*/ QDateTimeEdit_maximumTime for () {
  fn maximumTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11maximumTimeEv()};
    unsafe {_ZNK13QDateTimeEdit11maximumTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn metaObject<T: QDateTimeEdit_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDateTimeEdit_metaObject {
  fn metaObject(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: const QMetaObject * QDateTimeEdit::metaObject();
impl<'a> /*trait*/ QDateTimeEdit_metaObject for () {
  fn metaObject(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit10metaObjectEv()};
    unsafe {_ZNK13QDateTimeEdit10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn FreeQDateTimeEdit<T: QDateTimeEdit_FreeQDateTimeEdit>(&mut self, value: T) -> i32 {
    value.FreeQDateTimeEdit(self);
    return 1;
  }
}

pub trait QDateTimeEdit_FreeQDateTimeEdit {
  fn FreeQDateTimeEdit(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::FreeQDateTimeEdit();
impl<'a> /*trait*/ QDateTimeEdit_FreeQDateTimeEdit for () {
  fn FreeQDateTimeEdit(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditD0Ev()};
    unsafe {_ZN13QDateTimeEditD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn dateChanged<T: QDateTimeEdit_dateChanged>(&mut self, value: T) -> i32 {
    value.dateChanged(self);
    return 1;
  }
}

pub trait QDateTimeEdit_dateChanged {
  fn dateChanged(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::dateChanged(const QDate & date);
impl<'a> /*trait*/ QDateTimeEdit_dateChanged for (&'a  QDate) {
  fn dateChanged(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit11dateChangedERK5QDate()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit11dateChangedERK5QDate(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumDate<T: QDateTimeEdit_clearMinimumDate>(&mut self, value: T) -> i32 {
    value.clearMinimumDate(self);
    return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumDate {
  fn clearMinimumDate(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::clearMinimumDate();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumDate for () {
  fn clearMinimumDate(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMinimumDateEv()};
    unsafe {_ZN13QDateTimeEdit16clearMinimumDateEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDateRange<T: QDateTimeEdit_setDateRange>(&mut self, value: T) -> i32 {
    value.setDateRange(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setDateRange {
  fn setDateRange(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setDateRange(const QDate & min, const QDate & max);
impl<'a> /*trait*/ QDateTimeEdit_setDateRange for (&'a  QDate, &'a  QDate) {
  fn setDateRange(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit12setDateRangeERK5QDateS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit12setDateRangeERK5QDateS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumTime<T: QDateTimeEdit_setMinimumTime>(&mut self, value: T) -> i32 {
    value.setMinimumTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setMinimumTime {
  fn setMinimumTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setMinimumTime(const QTime & min);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumTime for (&'a  QTime) {
  fn setMinimumTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMinimumTimeERK5QTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit14setMinimumTimeERK5QTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn time<T: QDateTimeEdit_time>(&mut self, value: T) -> i32 {
    value.time(self);
    return 1;
  }
}

pub trait QDateTimeEdit_time {
  fn time(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QTime QDateTimeEdit::time();
impl<'a> /*trait*/ QDateTimeEdit_time for () {
  fn time(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit4timeEv()};
    unsafe {_ZNK13QDateTimeEdit4timeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn currentSectionIndex<T: QDateTimeEdit_currentSectionIndex>(&mut self, value: T) -> i32 {
    value.currentSectionIndex(self);
    return 1;
  }
}

pub trait QDateTimeEdit_currentSectionIndex {
  fn currentSectionIndex(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: int QDateTimeEdit::currentSectionIndex();
impl<'a> /*trait*/ QDateTimeEdit_currentSectionIndex for () {
  fn currentSectionIndex(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit19currentSectionIndexEv()};
    unsafe {_ZNK13QDateTimeEdit19currentSectionIndexEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn event<T: QDateTimeEdit_event>(&mut self, value: T) -> i32 {
    value.event(self);
    return 1;
  }
}

pub trait QDateTimeEdit_event {
  fn event(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: bool QDateTimeEdit::event(QEvent * event);
impl<'a> /*trait*/ QDateTimeEdit_event for (&'a mut QEvent) {
  fn event(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEdit5eventEP6QEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDateTime<T: QDateTimeEdit_setDateTime>(&mut self, value: T) -> i32 {
    value.setDateTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setDateTime {
  fn setDateTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setDateTime(const QDateTime & dateTime);
impl<'a> /*trait*/ QDateTimeEdit_setDateTime for (&'a  QDateTime) {
  fn setDateTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit11setDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit11setDateTimeERK9QDateTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setCalendarWidget<T: QDateTimeEdit_setCalendarWidget>(&mut self, value: T) -> i32 {
    value.setCalendarWidget(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setCalendarWidget {
  fn setCalendarWidget(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setCalendarWidget(QCalendarWidget * calendarWidget);
impl<'a> /*trait*/ QDateTimeEdit_setCalendarWidget for (&'a mut QCalendarWidget) {
  fn setCalendarWidget(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDateTimeRange<T: QDateTimeEdit_setDateTimeRange>(&mut self, value: T) -> i32 {
    value.setDateTimeRange(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setDateTimeRange {
  fn setDateTimeRange(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setDateTimeRange(const QDateTime & min, const QDateTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setDateTimeRange for (&'a  QDateTime, &'a  QDateTime) {
  fn setDateTimeRange(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setTime<T: QDateTimeEdit_setTime>(&mut self, value: T) -> i32 {
    value.setTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setTime {
  fn setTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setTime(const QTime & time);
impl<'a> /*trait*/ QDateTimeEdit_setTime for (&'a  QTime) {
  fn setTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit7setTimeERK5QTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit7setTimeERK5QTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn dateTime<T: QDateTimeEdit_dateTime>(&mut self, value: T) -> i32 {
    value.dateTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_dateTime {
  fn dateTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QDateTime QDateTimeEdit::dateTime();
impl<'a> /*trait*/ QDateTimeEdit_dateTime for () {
  fn dateTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit8dateTimeEv()};
    unsafe {_ZNK13QDateTimeEdit8dateTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumTime<T: QDateTimeEdit_setMaximumTime>(&mut self, value: T) -> i32 {
    value.setMaximumTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setMaximumTime {
  fn setMaximumTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setMaximumTime(const QTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumTime for (&'a  QTime) {
  fn setMaximumTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMaximumTimeERK5QTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit14setMaximumTimeERK5QTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumDateTime<T: QDateTimeEdit_setMinimumDateTime>(&mut self, value: T) -> i32 {
    value.setMinimumDateTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setMinimumDateTime {
  fn setMinimumDateTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setMinimumDateTime(const QDateTime & dt);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumDateTime for (&'a  QDateTime) {
  fn setMinimumDateTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumTime<T: QDateTimeEdit_clearMaximumTime>(&mut self, value: T) -> i32 {
    value.clearMaximumTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumTime {
  fn clearMaximumTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::clearMaximumTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumTime for () {
  fn clearMaximumTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMaximumTimeEv()};
    unsafe {_ZN13QDateTimeEdit16clearMaximumTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn calendarPopup<T: QDateTimeEdit_calendarPopup>(&mut self, value: T) -> i32 {
    value.calendarPopup(self);
    return 1;
  }
}

pub trait QDateTimeEdit_calendarPopup {
  fn calendarPopup(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: bool QDateTimeEdit::calendarPopup();
impl<'a> /*trait*/ QDateTimeEdit_calendarPopup for () {
  fn calendarPopup(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit13calendarPopupEv()};
    unsafe {_ZNK13QDateTimeEdit13calendarPopupEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumDateTime<T: QDateTimeEdit_setMaximumDateTime>(&mut self, value: T) -> i32 {
    value.setMaximumDateTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setMaximumDateTime {
  fn setMaximumDateTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setMaximumDateTime(const QDateTime & dt);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumDateTime for (&'a  QDateTime) {
  fn setMaximumDateTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumDateTime<T: QDateTimeEdit_clearMaximumDateTime>(&mut self, value: T) -> i32 {
    value.clearMaximumDateTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumDateTime {
  fn clearMaximumDateTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::clearMaximumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumDateTime for () {
  fn clearMaximumDateTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit20clearMaximumDateTimeEv()};
    unsafe {_ZN13QDateTimeEdit20clearMaximumDateTimeEv()};
    return 1;
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
  pub fn minimumDateTime<T: QDateTimeEdit_minimumDateTime>(&mut self, value: T) -> i32 {
    value.minimumDateTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_minimumDateTime {
  fn minimumDateTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QDateTime QDateTimeEdit::minimumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_minimumDateTime for () {
  fn minimumDateTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit15minimumDateTimeEv()};
    unsafe {_ZNK13QDateTimeEdit15minimumDateTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn calendarWidget<T: QDateTimeEdit_calendarWidget>(&mut self, value: T) -> i32 {
    value.calendarWidget(self);
    return 1;
  }
}

pub trait QDateTimeEdit_calendarWidget {
  fn calendarWidget(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QCalendarWidget * QDateTimeEdit::calendarWidget();
impl<'a> /*trait*/ QDateTimeEdit_calendarWidget for () {
  fn calendarWidget(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit14calendarWidgetEv()};
    unsafe {_ZNK13QDateTimeEdit14calendarWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setDisplayFormat<T: QDateTimeEdit_setDisplayFormat>(&mut self, value: T) -> i32 {
    value.setDisplayFormat(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setDisplayFormat {
  fn setDisplayFormat(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setDisplayFormat(const QString & format);
impl<'a> /*trait*/ QDateTimeEdit_setDisplayFormat for (&'a  QString) {
  fn setDisplayFormat(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setDisplayFormatERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit16setDisplayFormatERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumDate<T: QDateTimeEdit_clearMaximumDate>(&mut self, value: T) -> i32 {
    value.clearMaximumDate(self);
    return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumDate {
  fn clearMaximumDate(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::clearMaximumDate();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumDate for () {
  fn clearMaximumDate(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMaximumDateEv()};
    unsafe {_ZN13QDateTimeEdit16clearMaximumDateEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setCalendarPopup<T: QDateTimeEdit_setCalendarPopup>(&mut self, value: T) -> i32 {
    value.setCalendarPopup(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setCalendarPopup {
  fn setCalendarPopup(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setCalendarPopup(bool enable);
impl<'a> /*trait*/ QDateTimeEdit_setCalendarPopup for (i8) {
  fn setCalendarPopup(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setCalendarPopupEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QDateTimeEdit16setCalendarPopupEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn stepBy<T: QDateTimeEdit_stepBy>(&mut self, value: T) -> i32 {
    value.stepBy(self);
    return 1;
  }
}

pub trait QDateTimeEdit_stepBy {
  fn stepBy(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::stepBy(int steps);
impl<'a> /*trait*/ QDateTimeEdit_stepBy for (i32) {
  fn stepBy(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit6stepByEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QDateTimeEdit6stepByEi(arg0)};
    return 1;
  }
}

// proto: void QDateTimeEdit::NewQDateTimeEdit(const QDateTime & dt, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_NewQDateTimeEdit for (&'a  QDateTime, &'a mut QWidget) {
  fn NewQDateTimeEdit(self) -> QDateTimeEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC1ERK9QDateTimeP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QDateTimeEditC1ERK9QDateTimeP7QWidget(qthis, arg0, arg1)};
    let rsthis = QDateTimeEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn displayFormat<T: QDateTimeEdit_displayFormat>(&mut self, value: T) -> i32 {
    value.displayFormat(self);
    return 1;
  }
}

pub trait QDateTimeEdit_displayFormat {
  fn displayFormat(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QString QDateTimeEdit::displayFormat();
impl<'a> /*trait*/ QDateTimeEdit_displayFormat for () {
  fn displayFormat(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit13displayFormatEv()};
    unsafe {_ZNK13QDateTimeEdit13displayFormatEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn dateTimeChanged<T: QDateTimeEdit_dateTimeChanged>(&mut self, value: T) -> i32 {
    value.dateTimeChanged(self);
    return 1;
  }
}

pub trait QDateTimeEdit_dateTimeChanged {
  fn dateTimeChanged(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::dateTimeChanged(const QDateTime & dateTime);
impl<'a> /*trait*/ QDateTimeEdit_dateTimeChanged for (&'a  QDateTime) {
  fn dateTimeChanged(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn minimumTime<T: QDateTimeEdit_minimumTime>(&mut self, value: T) -> i32 {
    value.minimumTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_minimumTime {
  fn minimumTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QTime QDateTimeEdit::minimumTime();
impl<'a> /*trait*/ QDateTimeEdit_minimumTime for () {
  fn minimumTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11minimumTimeEv()};
    unsafe {_ZNK13QDateTimeEdit11minimumTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn sizeHint<T: QDateTimeEdit_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QDateTimeEdit_sizeHint {
  fn sizeHint(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QSize QDateTimeEdit::sizeHint();
impl<'a> /*trait*/ QDateTimeEdit_sizeHint for () {
  fn sizeHint(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit8sizeHintEv()};
    unsafe {_ZNK13QDateTimeEdit8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn sectionCount<T: QDateTimeEdit_sectionCount>(&mut self, value: T) -> i32 {
    value.sectionCount(self);
    return 1;
  }
}

pub trait QDateTimeEdit_sectionCount {
  fn sectionCount(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: int QDateTimeEdit::sectionCount();
impl<'a> /*trait*/ QDateTimeEdit_sectionCount for () {
  fn sectionCount(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit12sectionCountEv()};
    unsafe {_ZNK13QDateTimeEdit12sectionCountEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setCurrentSectionIndex<T: QDateTimeEdit_setCurrentSectionIndex>(&mut self, value: T) -> i32 {
    value.setCurrentSectionIndex(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setCurrentSectionIndex {
  fn setCurrentSectionIndex(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setCurrentSectionIndex(int index);
impl<'a> /*trait*/ QDateTimeEdit_setCurrentSectionIndex for (i32) {
  fn setCurrentSectionIndex(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit22setCurrentSectionIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QDateTimeEdit22setCurrentSectionIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumTime<T: QDateTimeEdit_clearMinimumTime>(&mut self, value: T) -> i32 {
    value.clearMinimumTime(self);
    return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumTime {
  fn clearMinimumTime(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::clearMinimumTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumTime for () {
  fn clearMinimumTime(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMinimumTimeEv()};
    unsafe {_ZN13QDateTimeEdit16clearMinimumTimeEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn setTimeRange<T: QDateTimeEdit_setTimeRange>(&mut self, value: T) -> i32 {
    value.setTimeRange(self);
    return 1;
  }
}

pub trait QDateTimeEdit_setTimeRange {
  fn setTimeRange(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::setTimeRange(const QTime & min, const QTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setTimeRange for (&'a  QTime, &'a  QTime) {
  fn setTimeRange(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn timeChanged<T: QDateTimeEdit_timeChanged>(&mut self, value: T) -> i32 {
    value.timeChanged(self);
    return 1;
  }
}

pub trait QDateTimeEdit_timeChanged {
  fn timeChanged(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: void QDateTimeEdit::timeChanged(const QTime & time);
impl<'a> /*trait*/ QDateTimeEdit_timeChanged for (&'a  QTime) {
  fn timeChanged(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit11timeChangedERK5QTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QDateTimeEdit11timeChangedERK5QTime(arg0)};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn minimumDate<T: QDateTimeEdit_minimumDate>(&mut self, value: T) -> i32 {
    value.minimumDate(self);
    return 1;
  }
}

pub trait QDateTimeEdit_minimumDate {
  fn minimumDate(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QDate QDateTimeEdit::minimumDate();
impl<'a> /*trait*/ QDateTimeEdit_minimumDate for () {
  fn minimumDate(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11minimumDateEv()};
    unsafe {_ZNK13QDateTimeEdit11minimumDateEv()};
    return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn maximumDate<T: QDateTimeEdit_maximumDate>(&mut self, value: T) -> i32 {
    value.maximumDate(self);
    return 1;
  }
}

pub trait QDateTimeEdit_maximumDate {
  fn maximumDate(self, this: &mut QDateTimeEdit) -> i32;
}

// proto: QDate QDateTimeEdit::maximumDate();
impl<'a> /*trait*/ QDateTimeEdit_maximumDate for () {
  fn maximumDate(self, this: &mut QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11maximumDateEv()};
    unsafe {_ZNK13QDateTimeEdit11maximumDateEv()};
    return 1;
  }
}

