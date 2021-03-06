// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qdatetimeedit.h
// dst-file: /src/widgets/qdatetimeedit.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
// use super::qdatetimeedit::QDateTimeEdit; // 773
use std::ops::Deref;
use super::qwidget::*; // 773
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qdatetime::*; // 771
use super::qabstractspinbox::*; // 773
use super::super::core::qcoreevent::*; // 771
use super::qcalendarwidget::*; // 773
use super::super::core::qstring::*; // 771
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTimeEdit_Class_Size() -> c_int;
  // proto:  void QTimeEdit::QTimeEdit(QWidget * parent);
  fn C_ZN9QTimeEditC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QTimeEdit::metaObject();
  fn C_ZNK9QTimeEdit10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTimeEdit::QTimeEdit(const QTime & time, QWidget * parent);
  fn C_ZN9QTimeEditC2ERK5QTimeP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QTimeEdit::~QTimeEdit();
  fn C_ZN9QTimeEditD2Ev(qthis: u64 /* *mut c_void*/);
  fn QDateEdit_Class_Size() -> c_int;
  // proto:  void QDateEdit::QDateEdit(const QDate & date, QWidget * parent);
  fn C_ZN9QDateEditC2ERK5QDateP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  const QMetaObject * QDateEdit::metaObject();
  fn C_ZNK9QDateEdit10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDateEdit::QDateEdit(QWidget * parent);
  fn C_ZN9QDateEditC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QDateEdit::~QDateEdit();
  fn C_ZN9QDateEditD2Ev(qthis: u64 /* *mut c_void*/);
  fn QDateTimeEdit_Class_Size() -> c_int;
  // proto:  QDate QDateTimeEdit::date();
  fn C_ZNK13QDateTimeEdit4dateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDateTimeEdit::clearMinimumDateTime();
  fn C_ZN13QDateTimeEdit20clearMinimumDateTimeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDateTimeEdit::clear();
  fn C_ZN13QDateTimeEdit5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDateTimeEdit::QDateTimeEdit(const QTime & t, QWidget * parent);
  fn C_ZN13QDateTimeEditC2ERK5QTimeP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QDateTimeEdit::setDate(const QDate & date);
  fn C_ZN13QDateTimeEdit7setDateERK5QDate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QDateTime QDateTimeEdit::maximumDateTime();
  fn C_ZNK13QDateTimeEdit15maximumDateTimeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDateTimeEdit::setMinimumDate(const QDate & min);
  fn C_ZN13QDateTimeEdit14setMinimumDateERK5QDate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDateTimeEdit::QDateTimeEdit(const QDate & d, QWidget * parent);
  fn C_ZN13QDateTimeEditC2ERK5QDateP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QDateTimeEdit::setMaximumDate(const QDate & max);
  fn C_ZN13QDateTimeEdit14setMaximumDateERK5QDate(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTime QDateTimeEdit::maximumTime();
  fn C_ZNK13QDateTimeEdit11maximumTimeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QDateTimeEdit::metaObject();
  fn C_ZNK13QDateTimeEdit10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDateTimeEdit::~QDateTimeEdit();
  fn C_ZN13QDateTimeEditD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDateTimeEdit::clearMinimumDate();
  fn C_ZN13QDateTimeEdit16clearMinimumDateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDateTimeEdit::setDateRange(const QDate & min, const QDate & max);
  fn C_ZN13QDateTimeEdit12setDateRangeERK5QDateS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QDateTimeEdit::setMinimumTime(const QTime & min);
  fn C_ZN13QDateTimeEdit14setMinimumTimeERK5QTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTime QDateTimeEdit::time();
  fn C_ZNK13QDateTimeEdit4timeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QDateTimeEdit::currentSectionIndex();
  fn C_ZNK13QDateTimeEdit19currentSectionIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QDateTimeEdit::event(QEvent * event);
  fn C_ZN13QDateTimeEdit5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QDateTimeEdit::setDateTime(const QDateTime & dateTime);
  fn C_ZN13QDateTimeEdit11setDateTimeERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDateTimeEdit::setCalendarWidget(QCalendarWidget * calendarWidget);
  fn C_ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDateTimeEdit::setDateTimeRange(const QDateTime & min, const QDateTime & max);
  fn C_ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QDateTimeEdit::setTime(const QTime & time);
  fn C_ZN13QDateTimeEdit7setTimeERK5QTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QDateTime QDateTimeEdit::dateTime();
  fn C_ZNK13QDateTimeEdit8dateTimeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDateTimeEdit::setMaximumTime(const QTime & max);
  fn C_ZN13QDateTimeEdit14setMaximumTimeERK5QTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDateTimeEdit::setMinimumDateTime(const QDateTime & dt);
  fn C_ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDateTimeEdit::clearMaximumTime();
  fn C_ZN13QDateTimeEdit16clearMaximumTimeEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QDateTimeEdit::calendarPopup();
  fn C_ZNK13QDateTimeEdit13calendarPopupEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QDateTimeEdit::setMaximumDateTime(const QDateTime & dt);
  fn C_ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDateTimeEdit::clearMaximumDateTime();
  fn C_ZN13QDateTimeEdit20clearMaximumDateTimeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDateTimeEdit::QDateTimeEdit(QWidget * parent);
  fn C_ZN13QDateTimeEditC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  QDateTime QDateTimeEdit::minimumDateTime();
  fn C_ZNK13QDateTimeEdit15minimumDateTimeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QCalendarWidget * QDateTimeEdit::calendarWidget();
  fn C_ZNK13QDateTimeEdit14calendarWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDateTimeEdit::setDisplayFormat(const QString & format);
  fn C_ZN13QDateTimeEdit16setDisplayFormatERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDateTimeEdit::clearMaximumDate();
  fn C_ZN13QDateTimeEdit16clearMaximumDateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDateTimeEdit::setCalendarPopup(bool enable);
  fn C_ZN13QDateTimeEdit16setCalendarPopupEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QDateTimeEdit::stepBy(int steps);
  fn C_ZN13QDateTimeEdit6stepByEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QDateTimeEdit::QDateTimeEdit(const QDateTime & dt, QWidget * parent);
  fn C_ZN13QDateTimeEditC2ERK9QDateTimeP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  QString QDateTimeEdit::displayFormat();
  fn C_ZNK13QDateTimeEdit13displayFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTime QDateTimeEdit::minimumTime();
  fn C_ZNK13QDateTimeEdit11minimumTimeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QDateTimeEdit::sizeHint();
  fn C_ZNK13QDateTimeEdit8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QDateTimeEdit::sectionCount();
  fn C_ZNK13QDateTimeEdit12sectionCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDateTimeEdit::setCurrentSectionIndex(int index);
  fn C_ZN13QDateTimeEdit22setCurrentSectionIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QDateTimeEdit::clearMinimumTime();
  fn C_ZN13QDateTimeEdit16clearMinimumTimeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDateTimeEdit::setTimeRange(const QTime & min, const QTime & max);
  fn C_ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QDate QDateTimeEdit::minimumDate();
  fn C_ZNK13QDateTimeEdit11minimumDateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QDate QDateTimeEdit::maximumDate();
  fn C_ZNK13QDateTimeEdit11maximumDateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QTimeEdit_SlotProxy_connect__ZN9QTimeEdit15userTimeChangedERK5QTime(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDateEdit_SlotProxy_connect__ZN9QDateEdit15userDateChangedERK5QDate(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit11dateChangedERK5QDate(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit11timeChangedERK5QTime(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTimeEdit)=1
#[derive(Default)]
pub struct QTimeEdit {
  qbase: QDateTimeEdit,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _userTimeChanged: QTimeEdit_userTimeChanged_signal,
}

// class sizeof(QDateEdit)=1
#[derive(Default)]
pub struct QDateEdit {
  qbase: QDateTimeEdit,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _userDateChanged: QDateEdit_userDateChanged_signal,
}

// class sizeof(QDateTimeEdit)=1
#[derive(Default)]
pub struct QDateTimeEdit {
  qbase: QAbstractSpinBox,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _dateChanged: QDateTimeEdit_dateChanged_signal,
  pub _timeChanged: QDateTimeEdit_timeChanged_signal,
  pub _dateTimeChanged: QDateTimeEdit_dateTimeChanged_signal,
}

impl /*struct*/ QTimeEdit {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTimeEdit {
    return QTimeEdit{qbase: QDateTimeEdit::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTimeEdit {
  type Target = QDateTimeEdit;

  fn deref(&self) -> &QDateTimeEdit {
    return & self.qbase;
  }
}
impl AsRef<QDateTimeEdit> for QTimeEdit {
  fn as_ref(& self) -> & QDateTimeEdit {
    return & self.qbase;
  }
}
  // proto:  void QTimeEdit::QTimeEdit(QWidget * parent);
impl /*struct*/ QTimeEdit {
  pub fn new<T: QTimeEdit_new>(value: T) -> QTimeEdit {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeEdit_new {
  fn new(self) -> QTimeEdit;
}

  // proto:  void QTimeEdit::QTimeEdit(QWidget * parent);
impl<'a> /*trait*/ QTimeEdit_new for (Option<&'a QWidget>) {
  fn new(self) -> QTimeEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeEditC2EP7QWidget()};
    let ctysz: c_int = unsafe{QTimeEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QTimeEditC2EP7QWidget(arg0)};
    let rsthis = QTimeEdit{qbase: QDateTimeEdit::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTimeEdit::metaObject();
impl /*struct*/ QTimeEdit {
  pub fn metaObject<RetType, T: QTimeEdit_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTimeEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTimeEdit) -> RetType;
}

  // proto:  const QMetaObject * QTimeEdit::metaObject();
impl<'a> /*trait*/ QTimeEdit_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTimeEdit) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeEdit10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QTimeEdit10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTimeEdit::QTimeEdit(const QTime & time, QWidget * parent);
impl<'a> /*trait*/ QTimeEdit_new for (&'a QTime, Option<&'a QWidget>) {
  fn new(self) -> QTimeEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeEditC2ERK5QTimeP7QWidget()};
    let ctysz: c_int = unsafe{QTimeEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QTimeEditC2ERK5QTimeP7QWidget(arg0, arg1)};
    let rsthis = QTimeEdit{qbase: QDateTimeEdit::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimeEdit::~QTimeEdit();
impl /*struct*/ QTimeEdit {
  pub fn free<RetType, T: QTimeEdit_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTimeEdit_free<RetType> {
  fn free(self , rsthis: & QTimeEdit) -> RetType;
}

  // proto:  void QTimeEdit::~QTimeEdit();
impl<'a> /*trait*/ QTimeEdit_free<()> for () {
  fn free(self , rsthis: & QTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeEditD2Ev()};
     unsafe {C_ZN9QTimeEditD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateEdit {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDateEdit {
    return QDateEdit{qbase: QDateTimeEdit::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDateEdit {
  type Target = QDateTimeEdit;

  fn deref(&self) -> &QDateTimeEdit {
    return & self.qbase;
  }
}
impl AsRef<QDateTimeEdit> for QDateEdit {
  fn as_ref(& self) -> & QDateTimeEdit {
    return & self.qbase;
  }
}
  // proto:  void QDateEdit::QDateEdit(const QDate & date, QWidget * parent);
impl /*struct*/ QDateEdit {
  pub fn new<T: QDateEdit_new>(value: T) -> QDateEdit {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDateEdit_new {
  fn new(self) -> QDateEdit;
}

  // proto:  void QDateEdit::QDateEdit(const QDate & date, QWidget * parent);
impl<'a> /*trait*/ QDateEdit_new for (&'a QDate, Option<&'a QWidget>) {
  fn new(self) -> QDateEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateEditC2ERK5QDateP7QWidget()};
    let ctysz: c_int = unsafe{QDateEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QDateEditC2ERK5QDateP7QWidget(arg0, arg1)};
    let rsthis = QDateEdit{qbase: QDateTimeEdit::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDateEdit::metaObject();
impl /*struct*/ QDateEdit {
  pub fn metaObject<RetType, T: QDateEdit_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDateEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDateEdit) -> RetType;
}

  // proto:  const QMetaObject * QDateEdit::metaObject();
impl<'a> /*trait*/ QDateEdit_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QDateEdit) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDateEdit10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QDateEdit10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateEdit::QDateEdit(QWidget * parent);
impl<'a> /*trait*/ QDateEdit_new for (Option<&'a QWidget>) {
  fn new(self) -> QDateEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateEditC2EP7QWidget()};
    let ctysz: c_int = unsafe{QDateEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QDateEditC2EP7QWidget(arg0)};
    let rsthis = QDateEdit{qbase: QDateTimeEdit::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDateEdit::~QDateEdit();
impl /*struct*/ QDateEdit {
  pub fn free<RetType, T: QDateEdit_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDateEdit_free<RetType> {
  fn free(self , rsthis: & QDateEdit) -> RetType;
}

  // proto:  void QDateEdit::~QDateEdit();
impl<'a> /*trait*/ QDateEdit_free<()> for () {
  fn free(self , rsthis: & QDateEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDateEditD2Ev()};
     unsafe {C_ZN9QDateEditD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDateTimeEdit {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDateTimeEdit {
    return QDateTimeEdit{qbase: QAbstractSpinBox::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDateTimeEdit {
  type Target = QAbstractSpinBox;

  fn deref(&self) -> &QAbstractSpinBox {
    return & self.qbase;
  }
}
impl AsRef<QAbstractSpinBox> for QDateTimeEdit {
  fn as_ref(& self) -> & QAbstractSpinBox {
    return & self.qbase;
  }
}
  // proto:  QDate QDateTimeEdit::date();
impl /*struct*/ QDateTimeEdit {
  pub fn date<RetType, T: QDateTimeEdit_date<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.date(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_date<RetType> {
  fn date(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QDate QDateTimeEdit::date();
impl<'a> /*trait*/ QDateTimeEdit_date<QDate> for () {
  fn date(self , rsthis: & QDateTimeEdit) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit4dateEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit4dateEv(rsthis.qclsinst)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::clearMinimumDateTime();
impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumDateTime<RetType, T: QDateTimeEdit_clearMinimumDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMinimumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumDateTime<RetType> {
  fn clearMinimumDateTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::clearMinimumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumDateTime<()> for () {
  fn clearMinimumDateTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit20clearMinimumDateTimeEv()};
     unsafe {C_ZN13QDateTimeEdit20clearMinimumDateTimeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::clear();
impl /*struct*/ QDateTimeEdit {
  pub fn clear<RetType, T: QDateTimeEdit_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clear<RetType> {
  fn clear(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::clear();
impl<'a> /*trait*/ QDateTimeEdit_clear<()> for () {
  fn clear(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit5clearEv()};
     unsafe {C_ZN13QDateTimeEdit5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::QDateTimeEdit(const QTime & t, QWidget * parent);
impl /*struct*/ QDateTimeEdit {
  pub fn new<T: QDateTimeEdit_new>(value: T) -> QDateTimeEdit {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDateTimeEdit_new {
  fn new(self) -> QDateTimeEdit;
}

  // proto:  void QDateTimeEdit::QDateTimeEdit(const QTime & t, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_new for (&'a QTime, Option<&'a QWidget>) {
  fn new(self) -> QDateTimeEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC2ERK5QTimeP7QWidget()};
    let ctysz: c_int = unsafe{QDateTimeEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QDateTimeEditC2ERK5QTimeP7QWidget(arg0, arg1)};
    let rsthis = QDateTimeEdit{qbase: QAbstractSpinBox::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setDate(const QDate & date);
impl /*struct*/ QDateTimeEdit {
  pub fn setDate<RetType, T: QDateTimeEdit_setDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDate<RetType> {
  fn setDate(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setDate(const QDate & date);
impl<'a> /*trait*/ QDateTimeEdit_setDate<()> for (&'a QDate) {
  fn setDate(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit7setDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit7setDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDateTime QDateTimeEdit::maximumDateTime();
impl /*struct*/ QDateTimeEdit {
  pub fn maximumDateTime<RetType, T: QDateTimeEdit_maximumDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_maximumDateTime<RetType> {
  fn maximumDateTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QDateTime QDateTimeEdit::maximumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_maximumDateTime<QDateTime> for () {
  fn maximumDateTime(self , rsthis: & QDateTimeEdit) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit15maximumDateTimeEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit15maximumDateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setMinimumDate(const QDate & min);
impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumDate<RetType, T: QDateTimeEdit_setMinimumDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMinimumDate<RetType> {
  fn setMinimumDate(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setMinimumDate(const QDate & min);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumDate<()> for (&'a QDate) {
  fn setMinimumDate(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMinimumDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit14setMinimumDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::QDateTimeEdit(const QDate & d, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_new for (&'a QDate, Option<&'a QWidget>) {
  fn new(self) -> QDateTimeEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC2ERK5QDateP7QWidget()};
    let ctysz: c_int = unsafe{QDateTimeEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QDateTimeEditC2ERK5QDateP7QWidget(arg0, arg1)};
    let rsthis = QDateTimeEdit{qbase: QAbstractSpinBox::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setMaximumDate(const QDate & max);
impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumDate<RetType, T: QDateTimeEdit_setMaximumDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMaximumDate<RetType> {
  fn setMaximumDate(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setMaximumDate(const QDate & max);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumDate<()> for (&'a QDate) {
  fn setMaximumDate(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMaximumDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit14setMaximumDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTime QDateTimeEdit::maximumTime();
impl /*struct*/ QDateTimeEdit {
  pub fn maximumTime<RetType, T: QDateTimeEdit_maximumTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_maximumTime<RetType> {
  fn maximumTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QTime QDateTimeEdit::maximumTime();
impl<'a> /*trait*/ QDateTimeEdit_maximumTime<QTime> for () {
  fn maximumTime(self , rsthis: & QDateTimeEdit) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11maximumTimeEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit11maximumTimeEv(rsthis.qclsinst)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDateTimeEdit::metaObject();
impl /*struct*/ QDateTimeEdit {
  pub fn metaObject<RetType, T: QDateTimeEdit_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  const QMetaObject * QDateTimeEdit::metaObject();
impl<'a> /*trait*/ QDateTimeEdit_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QDateTimeEdit) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::~QDateTimeEdit();
impl /*struct*/ QDateTimeEdit {
  pub fn free<RetType, T: QDateTimeEdit_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_free<RetType> {
  fn free(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::~QDateTimeEdit();
impl<'a> /*trait*/ QDateTimeEdit_free<()> for () {
  fn free(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditD2Ev()};
     unsafe {C_ZN13QDateTimeEditD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::clearMinimumDate();
impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumDate<RetType, T: QDateTimeEdit_clearMinimumDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMinimumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumDate<RetType> {
  fn clearMinimumDate(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::clearMinimumDate();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumDate<()> for () {
  fn clearMinimumDate(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMinimumDateEv()};
     unsafe {C_ZN13QDateTimeEdit16clearMinimumDateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setDateRange(const QDate & min, const QDate & max);
impl /*struct*/ QDateTimeEdit {
  pub fn setDateRange<RetType, T: QDateTimeEdit_setDateRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDateRange(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDateRange<RetType> {
  fn setDateRange(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setDateRange(const QDate & min, const QDate & max);
impl<'a> /*trait*/ QDateTimeEdit_setDateRange<()> for (&'a QDate, &'a QDate) {
  fn setDateRange(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit12setDateRangeERK5QDateS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit12setDateRangeERK5QDateS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setMinimumTime(const QTime & min);
impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumTime<RetType, T: QDateTimeEdit_setMinimumTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMinimumTime<RetType> {
  fn setMinimumTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setMinimumTime(const QTime & min);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumTime<()> for (&'a QTime) {
  fn setMinimumTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMinimumTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit14setMinimumTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTime QDateTimeEdit::time();
impl /*struct*/ QDateTimeEdit {
  pub fn time<RetType, T: QDateTimeEdit_time<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.time(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_time<RetType> {
  fn time(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QTime QDateTimeEdit::time();
impl<'a> /*trait*/ QDateTimeEdit_time<QTime> for () {
  fn time(self , rsthis: & QDateTimeEdit) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit4timeEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit4timeEv(rsthis.qclsinst)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QDateTimeEdit::currentSectionIndex();
impl /*struct*/ QDateTimeEdit {
  pub fn currentSectionIndex<RetType, T: QDateTimeEdit_currentSectionIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentSectionIndex(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_currentSectionIndex<RetType> {
  fn currentSectionIndex(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  int QDateTimeEdit::currentSectionIndex();
impl<'a> /*trait*/ QDateTimeEdit_currentSectionIndex<i32> for () {
  fn currentSectionIndex(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit19currentSectionIndexEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit19currentSectionIndexEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QDateTimeEdit::event(QEvent * event);
impl /*struct*/ QDateTimeEdit {
  pub fn event<RetType, T: QDateTimeEdit_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_event<RetType> {
  fn event(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  bool QDateTimeEdit::event(QEvent * event);
impl<'a> /*trait*/ QDateTimeEdit_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QDateTimeEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QDateTimeEdit5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setDateTime(const QDateTime & dateTime);
impl /*struct*/ QDateTimeEdit {
  pub fn setDateTime<RetType, T: QDateTimeEdit_setDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDateTime<RetType> {
  fn setDateTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setDateTime(const QDateTime & dateTime);
impl<'a> /*trait*/ QDateTimeEdit_setDateTime<()> for (&'a QDateTime) {
  fn setDateTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit11setDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit11setDateTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setCalendarWidget(QCalendarWidget * calendarWidget);
impl /*struct*/ QDateTimeEdit {
  pub fn setCalendarWidget<RetType, T: QDateTimeEdit_setCalendarWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCalendarWidget(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setCalendarWidget<RetType> {
  fn setCalendarWidget(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setCalendarWidget(QCalendarWidget * calendarWidget);
impl<'a> /*trait*/ QDateTimeEdit_setCalendarWidget<()> for (&'a QCalendarWidget) {
  fn setCalendarWidget(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit17setCalendarWidgetEP15QCalendarWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setDateTimeRange(const QDateTime & min, const QDateTime & max);
impl /*struct*/ QDateTimeEdit {
  pub fn setDateTimeRange<RetType, T: QDateTimeEdit_setDateTimeRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDateTimeRange(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDateTimeRange<RetType> {
  fn setDateTimeRange(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setDateTimeRange(const QDateTime & min, const QDateTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setDateTimeRange<()> for (&'a QDateTime, &'a QDateTime) {
  fn setDateTimeRange(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit16setDateTimeRangeERK9QDateTimeS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setTime(const QTime & time);
impl /*struct*/ QDateTimeEdit {
  pub fn setTime<RetType, T: QDateTimeEdit_setTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setTime<RetType> {
  fn setTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setTime(const QTime & time);
impl<'a> /*trait*/ QDateTimeEdit_setTime<()> for (&'a QTime) {
  fn setTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit7setTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit7setTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QDateTime QDateTimeEdit::dateTime();
impl /*struct*/ QDateTimeEdit {
  pub fn dateTime<RetType, T: QDateTimeEdit_dateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_dateTime<RetType> {
  fn dateTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QDateTime QDateTimeEdit::dateTime();
impl<'a> /*trait*/ QDateTimeEdit_dateTime<QDateTime> for () {
  fn dateTime(self , rsthis: & QDateTimeEdit) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit8dateTimeEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit8dateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setMaximumTime(const QTime & max);
impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumTime<RetType, T: QDateTimeEdit_setMaximumTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMaximumTime<RetType> {
  fn setMaximumTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setMaximumTime(const QTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumTime<()> for (&'a QTime) {
  fn setMaximumTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit14setMaximumTimeERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit14setMaximumTimeERK5QTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setMinimumDateTime(const QDateTime & dt);
impl /*struct*/ QDateTimeEdit {
  pub fn setMinimumDateTime<RetType, T: QDateTimeEdit_setMinimumDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMinimumDateTime<RetType> {
  fn setMinimumDateTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setMinimumDateTime(const QDateTime & dt);
impl<'a> /*trait*/ QDateTimeEdit_setMinimumDateTime<()> for (&'a QDateTime) {
  fn setMinimumDateTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit18setMinimumDateTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::clearMaximumTime();
impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumTime<RetType, T: QDateTimeEdit_clearMaximumTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMaximumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumTime<RetType> {
  fn clearMaximumTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::clearMaximumTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumTime<()> for () {
  fn clearMaximumTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMaximumTimeEv()};
     unsafe {C_ZN13QDateTimeEdit16clearMaximumTimeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QDateTimeEdit::calendarPopup();
impl /*struct*/ QDateTimeEdit {
  pub fn calendarPopup<RetType, T: QDateTimeEdit_calendarPopup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.calendarPopup(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_calendarPopup<RetType> {
  fn calendarPopup(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  bool QDateTimeEdit::calendarPopup();
impl<'a> /*trait*/ QDateTimeEdit_calendarPopup<i8> for () {
  fn calendarPopup(self , rsthis: & QDateTimeEdit) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit13calendarPopupEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit13calendarPopupEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setMaximumDateTime(const QDateTime & dt);
impl /*struct*/ QDateTimeEdit {
  pub fn setMaximumDateTime<RetType, T: QDateTimeEdit_setMaximumDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setMaximumDateTime<RetType> {
  fn setMaximumDateTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setMaximumDateTime(const QDateTime & dt);
impl<'a> /*trait*/ QDateTimeEdit_setMaximumDateTime<()> for (&'a QDateTime) {
  fn setMaximumDateTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit18setMaximumDateTimeERK9QDateTime(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::clearMaximumDateTime();
impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumDateTime<RetType, T: QDateTimeEdit_clearMaximumDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMaximumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumDateTime<RetType> {
  fn clearMaximumDateTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::clearMaximumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumDateTime<()> for () {
  fn clearMaximumDateTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit20clearMaximumDateTimeEv()};
     unsafe {C_ZN13QDateTimeEdit20clearMaximumDateTimeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::QDateTimeEdit(QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_new for (Option<&'a QWidget>) {
  fn new(self) -> QDateTimeEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC2EP7QWidget()};
    let ctysz: c_int = unsafe{QDateTimeEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QDateTimeEditC2EP7QWidget(arg0)};
    let rsthis = QDateTimeEdit{qbase: QAbstractSpinBox::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QDateTime QDateTimeEdit::minimumDateTime();
impl /*struct*/ QDateTimeEdit {
  pub fn minimumDateTime<RetType, T: QDateTimeEdit_minimumDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumDateTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_minimumDateTime<RetType> {
  fn minimumDateTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QDateTime QDateTimeEdit::minimumDateTime();
impl<'a> /*trait*/ QDateTimeEdit_minimumDateTime<QDateTime> for () {
  fn minimumDateTime(self , rsthis: & QDateTimeEdit) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit15minimumDateTimeEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit15minimumDateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QCalendarWidget * QDateTimeEdit::calendarWidget();
impl /*struct*/ QDateTimeEdit {
  pub fn calendarWidget<RetType, T: QDateTimeEdit_calendarWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.calendarWidget(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_calendarWidget<RetType> {
  fn calendarWidget(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QCalendarWidget * QDateTimeEdit::calendarWidget();
impl<'a> /*trait*/ QDateTimeEdit_calendarWidget<QCalendarWidget> for () {
  fn calendarWidget(self , rsthis: & QDateTimeEdit) -> QCalendarWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit14calendarWidgetEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit14calendarWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QCalendarWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setDisplayFormat(const QString & format);
impl /*struct*/ QDateTimeEdit {
  pub fn setDisplayFormat<RetType, T: QDateTimeEdit_setDisplayFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDisplayFormat(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setDisplayFormat<RetType> {
  fn setDisplayFormat(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setDisplayFormat(const QString & format);
impl<'a> /*trait*/ QDateTimeEdit_setDisplayFormat<()> for (&'a QString) {
  fn setDisplayFormat(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setDisplayFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit16setDisplayFormatERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::clearMaximumDate();
impl /*struct*/ QDateTimeEdit {
  pub fn clearMaximumDate<RetType, T: QDateTimeEdit_clearMaximumDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMaximumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMaximumDate<RetType> {
  fn clearMaximumDate(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::clearMaximumDate();
impl<'a> /*trait*/ QDateTimeEdit_clearMaximumDate<()> for () {
  fn clearMaximumDate(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMaximumDateEv()};
     unsafe {C_ZN13QDateTimeEdit16clearMaximumDateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setCalendarPopup(bool enable);
impl /*struct*/ QDateTimeEdit {
  pub fn setCalendarPopup<RetType, T: QDateTimeEdit_setCalendarPopup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCalendarPopup(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setCalendarPopup<RetType> {
  fn setCalendarPopup(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setCalendarPopup(bool enable);
impl<'a> /*trait*/ QDateTimeEdit_setCalendarPopup<()> for (i8) {
  fn setCalendarPopup(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16setCalendarPopupEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QDateTimeEdit16setCalendarPopupEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::stepBy(int steps);
impl /*struct*/ QDateTimeEdit {
  pub fn stepBy<RetType, T: QDateTimeEdit_stepBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stepBy(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_stepBy<RetType> {
  fn stepBy(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::stepBy(int steps);
impl<'a> /*trait*/ QDateTimeEdit_stepBy<()> for (i32) {
  fn stepBy(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit6stepByEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QDateTimeEdit6stepByEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::QDateTimeEdit(const QDateTime & dt, QWidget * parent);
impl<'a> /*trait*/ QDateTimeEdit_new for (&'a QDateTime, Option<&'a QWidget>) {
  fn new(self) -> QDateTimeEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEditC2ERK9QDateTimeP7QWidget()};
    let ctysz: c_int = unsafe{QDateTimeEdit_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QDateTimeEditC2ERK9QDateTimeP7QWidget(arg0, arg1)};
    let rsthis = QDateTimeEdit{qbase: QAbstractSpinBox::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QDateTimeEdit::displayFormat();
impl /*struct*/ QDateTimeEdit {
  pub fn displayFormat<RetType, T: QDateTimeEdit_displayFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.displayFormat(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_displayFormat<RetType> {
  fn displayFormat(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QString QDateTimeEdit::displayFormat();
impl<'a> /*trait*/ QDateTimeEdit_displayFormat<QString> for () {
  fn displayFormat(self , rsthis: & QDateTimeEdit) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit13displayFormatEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit13displayFormatEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTime QDateTimeEdit::minimumTime();
impl /*struct*/ QDateTimeEdit {
  pub fn minimumTime<RetType, T: QDateTimeEdit_minimumTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_minimumTime<RetType> {
  fn minimumTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QTime QDateTimeEdit::minimumTime();
impl<'a> /*trait*/ QDateTimeEdit_minimumTime<QTime> for () {
  fn minimumTime(self , rsthis: & QDateTimeEdit) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11minimumTimeEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit11minimumTimeEv(rsthis.qclsinst)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QDateTimeEdit::sizeHint();
impl /*struct*/ QDateTimeEdit {
  pub fn sizeHint<RetType, T: QDateTimeEdit_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QSize QDateTimeEdit::sizeHint();
impl<'a> /*trait*/ QDateTimeEdit_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QDateTimeEdit) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit8sizeHintEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QDateTimeEdit::sectionCount();
impl /*struct*/ QDateTimeEdit {
  pub fn sectionCount<RetType, T: QDateTimeEdit_sectionCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sectionCount(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_sectionCount<RetType> {
  fn sectionCount(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  int QDateTimeEdit::sectionCount();
impl<'a> /*trait*/ QDateTimeEdit_sectionCount<i32> for () {
  fn sectionCount(self , rsthis: & QDateTimeEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit12sectionCountEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit12sectionCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setCurrentSectionIndex(int index);
impl /*struct*/ QDateTimeEdit {
  pub fn setCurrentSectionIndex<RetType, T: QDateTimeEdit_setCurrentSectionIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentSectionIndex(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setCurrentSectionIndex<RetType> {
  fn setCurrentSectionIndex(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setCurrentSectionIndex(int index);
impl<'a> /*trait*/ QDateTimeEdit_setCurrentSectionIndex<()> for (i32) {
  fn setCurrentSectionIndex(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit22setCurrentSectionIndexEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QDateTimeEdit22setCurrentSectionIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::clearMinimumTime();
impl /*struct*/ QDateTimeEdit {
  pub fn clearMinimumTime<RetType, T: QDateTimeEdit_clearMinimumTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMinimumTime(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_clearMinimumTime<RetType> {
  fn clearMinimumTime(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::clearMinimumTime();
impl<'a> /*trait*/ QDateTimeEdit_clearMinimumTime<()> for () {
  fn clearMinimumTime(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit16clearMinimumTimeEv()};
     unsafe {C_ZN13QDateTimeEdit16clearMinimumTimeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDateTimeEdit::setTimeRange(const QTime & min, const QTime & max);
impl /*struct*/ QDateTimeEdit {
  pub fn setTimeRange<RetType, T: QDateTimeEdit_setTimeRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTimeRange(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_setTimeRange<RetType> {
  fn setTimeRange(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  void QDateTimeEdit::setTimeRange(const QTime & min, const QTime & max);
impl<'a> /*trait*/ QDateTimeEdit_setTimeRange<()> for (&'a QTime, &'a QTime) {
  fn setTimeRange(self , rsthis: & QDateTimeEdit) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QDateTimeEdit12setTimeRangeERK5QTimeS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QDate QDateTimeEdit::minimumDate();
impl /*struct*/ QDateTimeEdit {
  pub fn minimumDate<RetType, T: QDateTimeEdit_minimumDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_minimumDate<RetType> {
  fn minimumDate(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QDate QDateTimeEdit::minimumDate();
impl<'a> /*trait*/ QDateTimeEdit_minimumDate<QDate> for () {
  fn minimumDate(self , rsthis: & QDateTimeEdit) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11minimumDateEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit11minimumDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDate QDateTimeEdit::maximumDate();
impl /*struct*/ QDateTimeEdit {
  pub fn maximumDate<RetType, T: QDateTimeEdit_maximumDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumDate(self);
    // return 1;
  }
}

pub trait QDateTimeEdit_maximumDate<RetType> {
  fn maximumDate(self , rsthis: & QDateTimeEdit) -> RetType;
}

  // proto:  QDate QDateTimeEdit::maximumDate();
impl<'a> /*trait*/ QDateTimeEdit_maximumDate<QDate> for () {
  fn maximumDate(self , rsthis: & QDateTimeEdit) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QDateTimeEdit11maximumDateEv()};
    let mut ret = unsafe {C_ZNK13QDateTimeEdit11maximumDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QTimeEdit_userTimeChanged
pub struct QTimeEdit_userTimeChanged_signal{poi:u64}
impl /* struct */ QTimeEdit {
  pub fn userTimeChanged(&self) -> QTimeEdit_userTimeChanged_signal {
     return QTimeEdit_userTimeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTimeEdit_userTimeChanged_signal {
  pub fn connect<T: QTimeEdit_userTimeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTimeEdit_userTimeChanged_signal_connect {
  fn connect(self, sigthis: QTimeEdit_userTimeChanged_signal);
}

// userTimeChanged(const class QTime &)
extern fn QTimeEdit_userTimeChanged_signal_connect_cb_0(rsfptr:fn(QTime), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTime::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTimeEdit_userTimeChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QTime)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTime::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTimeEdit_userTimeChanged_signal_connect for fn(QTime) {
  fn connect(self, sigthis: QTimeEdit_userTimeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTimeEdit_userTimeChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTimeEdit_SlotProxy_connect__ZN9QTimeEdit15userTimeChangedERK5QTime(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTimeEdit_userTimeChanged_signal_connect for Box<Fn(QTime)> {
  fn connect(self, sigthis: QTimeEdit_userTimeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTimeEdit_userTimeChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTimeEdit_SlotProxy_connect__ZN9QTimeEdit15userTimeChangedERK5QTime(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QDateEdit_userDateChanged
pub struct QDateEdit_userDateChanged_signal{poi:u64}
impl /* struct */ QDateEdit {
  pub fn userDateChanged(&self) -> QDateEdit_userDateChanged_signal {
     return QDateEdit_userDateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDateEdit_userDateChanged_signal {
  pub fn connect<T: QDateEdit_userDateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDateEdit_userDateChanged_signal_connect {
  fn connect(self, sigthis: QDateEdit_userDateChanged_signal);
}

// userDateChanged(const class QDate &)
extern fn QDateEdit_userDateChanged_signal_connect_cb_0(rsfptr:fn(QDate), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QDate::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QDateEdit_userDateChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QDate)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QDate::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDateEdit_userDateChanged_signal_connect for fn(QDate) {
  fn connect(self, sigthis: QDateEdit_userDateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDateEdit_userDateChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDateEdit_SlotProxy_connect__ZN9QDateEdit15userDateChangedERK5QDate(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDateEdit_userDateChanged_signal_connect for Box<Fn(QDate)> {
  fn connect(self, sigthis: QDateEdit_userDateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDateEdit_userDateChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDateEdit_SlotProxy_connect__ZN9QDateEdit15userDateChangedERK5QDate(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QDateTimeEdit_dateChanged
pub struct QDateTimeEdit_dateChanged_signal{poi:u64}
impl /* struct */ QDateTimeEdit {
  pub fn dateChanged(&self) -> QDateTimeEdit_dateChanged_signal {
     return QDateTimeEdit_dateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDateTimeEdit_dateChanged_signal {
  pub fn connect<T: QDateTimeEdit_dateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDateTimeEdit_dateChanged_signal_connect {
  fn connect(self, sigthis: QDateTimeEdit_dateChanged_signal);
}

#[derive(Default)] // for QDateTimeEdit_timeChanged
pub struct QDateTimeEdit_timeChanged_signal{poi:u64}
impl /* struct */ QDateTimeEdit {
  pub fn timeChanged(&self) -> QDateTimeEdit_timeChanged_signal {
     return QDateTimeEdit_timeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDateTimeEdit_timeChanged_signal {
  pub fn connect<T: QDateTimeEdit_timeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDateTimeEdit_timeChanged_signal_connect {
  fn connect(self, sigthis: QDateTimeEdit_timeChanged_signal);
}

#[derive(Default)] // for QDateTimeEdit_dateTimeChanged
pub struct QDateTimeEdit_dateTimeChanged_signal{poi:u64}
impl /* struct */ QDateTimeEdit {
  pub fn dateTimeChanged(&self) -> QDateTimeEdit_dateTimeChanged_signal {
     return QDateTimeEdit_dateTimeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDateTimeEdit_dateTimeChanged_signal {
  pub fn connect<T: QDateTimeEdit_dateTimeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDateTimeEdit_dateTimeChanged_signal_connect {
  fn connect(self, sigthis: QDateTimeEdit_dateTimeChanged_signal);
}

// dateTimeChanged(const class QDateTime &)
extern fn QDateTimeEdit_dateTimeChanged_signal_connect_cb_0(rsfptr:fn(QDateTime), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QDateTime::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QDateTimeEdit_dateTimeChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QDateTime)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QDateTime::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDateTimeEdit_dateTimeChanged_signal_connect for fn(QDateTime) {
  fn connect(self, sigthis: QDateTimeEdit_dateTimeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDateTimeEdit_dateTimeChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDateTimeEdit_dateTimeChanged_signal_connect for Box<Fn(QDateTime)> {
  fn connect(self, sigthis: QDateTimeEdit_dateTimeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDateTimeEdit_dateTimeChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit15dateTimeChangedERK9QDateTime(arg0, arg1, arg2)};
  }
}
// dateChanged(const class QDate &)
extern fn QDateTimeEdit_dateChanged_signal_connect_cb_1(rsfptr:fn(QDate), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QDate::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QDateTimeEdit_dateChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QDate)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QDate::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDateTimeEdit_dateChanged_signal_connect for fn(QDate) {
  fn connect(self, sigthis: QDateTimeEdit_dateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDateTimeEdit_dateChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit11dateChangedERK5QDate(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDateTimeEdit_dateChanged_signal_connect for Box<Fn(QDate)> {
  fn connect(self, sigthis: QDateTimeEdit_dateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDateTimeEdit_dateChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit11dateChangedERK5QDate(arg0, arg1, arg2)};
  }
}
// timeChanged(const class QTime &)
extern fn QDateTimeEdit_timeChanged_signal_connect_cb_2(rsfptr:fn(QTime), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTime::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QDateTimeEdit_timeChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QTime)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTime::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QDateTimeEdit_timeChanged_signal_connect for fn(QTime) {
  fn connect(self, sigthis: QDateTimeEdit_timeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDateTimeEdit_timeChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit11timeChangedERK5QTime(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDateTimeEdit_timeChanged_signal_connect for Box<Fn(QTime)> {
  fn connect(self, sigthis: QDateTimeEdit_timeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDateTimeEdit_timeChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QDateTimeEdit_SlotProxy_connect__ZN13QDateTimeEdit11timeChangedERK5QTime(arg0, arg1, arg2)};
  }
}
// <= body block end

