// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qcalendarwidget.h
// dst-file: /src/widgets/qcalendarwidget.rs
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
use super::super::core::qdatetime::QDate; // 771
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
use super::super::gui::qtextformat::QTextCharFormat; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QCalendarWidget::selectionChanged();
  fn _ZN15QCalendarWidget16selectionChangedEv(qthis: *mut c_void);
  // proto:  void QCalendarWidget::showPreviousYear();
  fn _ZN15QCalendarWidget16showPreviousYearEv(qthis: *mut c_void);
  // proto:  QDate QCalendarWidget::maximumDate();
  fn _ZNK15QCalendarWidget11maximumDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::showPreviousMonth();
  fn _ZN15QCalendarWidget17showPreviousMonthEv(qthis: *mut c_void);
  // proto:  void QCalendarWidget::showSelectedDate();
  fn _ZN15QCalendarWidget16showSelectedDateEv(qthis: *mut c_void);
  // proto:  QSize QCalendarWidget::minimumSizeHint();
  fn _ZNK15QCalendarWidget15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::setDateEditAcceptDelay(int delay);
  fn _ZN15QCalendarWidget22setDateEditAcceptDelayEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QCalendarWidget::setGridVisible(bool show);
  fn _ZN15QCalendarWidget14setGridVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QCalendarWidget::~QCalendarWidget();
  fn _ZN15QCalendarWidgetD0Ev(qthis: *mut c_void);
  // proto:  QSize QCalendarWidget::sizeHint();
  fn _ZNK15QCalendarWidget8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QCalendarWidget::monthShown();
  fn _ZNK15QCalendarWidget10monthShownEv(qthis: *mut c_void) -> c_int;
  // proto:  void QCalendarWidget::setSelectedDate(const QDate & date);
  fn _ZN15QCalendarWidget15setSelectedDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCalendarWidget::QCalendarWidget(QWidget * parent);
  fn _ZN15QCalendarWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCalendarWidget::currentPageChanged(int year, int month);
  fn _ZN15QCalendarWidget18currentPageChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  const QMetaObject * QCalendarWidget::metaObject();
  fn _ZNK15QCalendarWidget10metaObjectEv(qthis: *mut c_void);
  // proto:  void QCalendarWidget::setNavigationBarVisible(bool visible);
  fn _ZN15QCalendarWidget23setNavigationBarVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QCalendarWidget::isNavigationBarVisible();
  fn _ZNK15QCalendarWidget22isNavigationBarVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  QTextCharFormat QCalendarWidget::dateTextFormat(const QDate & date);
  fn _ZNK15QCalendarWidget14dateTextFormatERK5QDate(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::setMinimumDate(const QDate & date);
  fn _ZN15QCalendarWidget14setMinimumDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QCalendarWidget::dateEditAcceptDelay();
  fn _ZNK15QCalendarWidget19dateEditAcceptDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  QDate QCalendarWidget::minimumDate();
  fn _ZNK15QCalendarWidget11minimumDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::QCalendarWidget(const QCalendarWidget & );
  fn _ZN15QCalendarWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QCalendarWidget::isDateEditEnabled();
  fn _ZNK15QCalendarWidget17isDateEditEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  QMap<QDate, QTextCharFormat> QCalendarWidget::dateTextFormat();
  fn _ZNK15QCalendarWidget14dateTextFormatEv(qthis: *mut c_void);
  // proto:  void QCalendarWidget::clicked(const QDate & date);
  fn _ZN15QCalendarWidget7clickedERK5QDate(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCalendarWidget::setDateEditEnabled(bool enable);
  fn _ZN15QCalendarWidget18setDateEditEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QCalendarWidget::setDateTextFormat(const QDate & date, const QTextCharFormat & format);
  fn _ZN15QCalendarWidget17setDateTextFormatERK5QDateRK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QCalendarWidget::showNextMonth();
  fn _ZN15QCalendarWidget13showNextMonthEv(qthis: *mut c_void);
  // proto:  void QCalendarWidget::setDateRange(const QDate & min, const QDate & max);
  fn _ZN15QCalendarWidget12setDateRangeERK5QDateS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QDate QCalendarWidget::selectedDate();
  fn _ZNK15QCalendarWidget12selectedDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::setHeaderTextFormat(const QTextCharFormat & format);
  fn _ZN15QCalendarWidget19setHeaderTextFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QCalendarWidget::isGridVisible();
  fn _ZNK15QCalendarWidget13isGridVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  int QCalendarWidget::yearShown();
  fn _ZNK15QCalendarWidget9yearShownEv(qthis: *mut c_void) -> c_int;
  // proto:  void QCalendarWidget::setMaximumDate(const QDate & date);
  fn _ZN15QCalendarWidget14setMaximumDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextCharFormat QCalendarWidget::headerTextFormat();
  fn _ZNK15QCalendarWidget16headerTextFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::setCurrentPage(int year, int month);
  fn _ZN15QCalendarWidget14setCurrentPageEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QCalendarWidget::showToday();
  fn _ZN15QCalendarWidget9showTodayEv(qthis: *mut c_void);
  // proto:  void QCalendarWidget::activated(const QDate & date);
  fn _ZN15QCalendarWidget9activatedERK5QDate(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCalendarWidget::showNextYear();
  fn _ZN15QCalendarWidget12showNextYearEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCalendarWidget)=1
pub struct QCalendarWidget {
  pub qclsinst: *mut c_void,
}

  // proto:  void QCalendarWidget::selectionChanged();
impl /*struct*/ QCalendarWidget {
  pub fn selectionChanged<RetType, T: QCalendarWidget_selectionChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QCalendarWidget_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::selectionChanged();
impl<'a> /*trait*/ QCalendarWidget_selectionChanged<()> for () {
  fn selectionChanged(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget16selectionChangedEv()};
     unsafe {_ZN15QCalendarWidget16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::showPreviousYear();
impl /*struct*/ QCalendarWidget {
  pub fn showPreviousYear<RetType, T: QCalendarWidget_showPreviousYear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showPreviousYear(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showPreviousYear<RetType> {
  fn showPreviousYear(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::showPreviousYear();
impl<'a> /*trait*/ QCalendarWidget_showPreviousYear<()> for () {
  fn showPreviousYear(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget16showPreviousYearEv()};
     unsafe {_ZN15QCalendarWidget16showPreviousYearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QDate QCalendarWidget::maximumDate();
impl /*struct*/ QCalendarWidget {
  pub fn maximumDate<RetType, T: QCalendarWidget_maximumDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_maximumDate<RetType> {
  fn maximumDate(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  QDate QCalendarWidget::maximumDate();
impl<'a> /*trait*/ QCalendarWidget_maximumDate<QDate> for () {
  fn maximumDate(self , rsthis: &mut QCalendarWidget) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget11maximumDateEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget11maximumDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::showPreviousMonth();
impl /*struct*/ QCalendarWidget {
  pub fn showPreviousMonth<RetType, T: QCalendarWidget_showPreviousMonth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showPreviousMonth(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showPreviousMonth<RetType> {
  fn showPreviousMonth(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::showPreviousMonth();
impl<'a> /*trait*/ QCalendarWidget_showPreviousMonth<()> for () {
  fn showPreviousMonth(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget17showPreviousMonthEv()};
     unsafe {_ZN15QCalendarWidget17showPreviousMonthEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::showSelectedDate();
impl /*struct*/ QCalendarWidget {
  pub fn showSelectedDate<RetType, T: QCalendarWidget_showSelectedDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showSelectedDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showSelectedDate<RetType> {
  fn showSelectedDate(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::showSelectedDate();
impl<'a> /*trait*/ QCalendarWidget_showSelectedDate<()> for () {
  fn showSelectedDate(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget16showSelectedDateEv()};
     unsafe {_ZN15QCalendarWidget16showSelectedDateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QCalendarWidget::minimumSizeHint();
impl /*struct*/ QCalendarWidget {
  pub fn minimumSizeHint<RetType, T: QCalendarWidget_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QCalendarWidget_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  QSize QCalendarWidget::minimumSizeHint();
impl<'a> /*trait*/ QCalendarWidget_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QCalendarWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setDateEditAcceptDelay(int delay);
impl /*struct*/ QCalendarWidget {
  pub fn setDateEditAcceptDelay<RetType, T: QCalendarWidget_setDateEditAcceptDelay<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDateEditAcceptDelay(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setDateEditAcceptDelay<RetType> {
  fn setDateEditAcceptDelay(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setDateEditAcceptDelay(int delay);
impl<'a> /*trait*/ QCalendarWidget_setDateEditAcceptDelay<()> for (i32) {
  fn setDateEditAcceptDelay(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget22setDateEditAcceptDelayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QCalendarWidget22setDateEditAcceptDelayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setGridVisible(bool show);
impl /*struct*/ QCalendarWidget {
  pub fn setGridVisible<RetType, T: QCalendarWidget_setGridVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGridVisible(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setGridVisible<RetType> {
  fn setGridVisible(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setGridVisible(bool show);
impl<'a> /*trait*/ QCalendarWidget_setGridVisible<()> for (i8) {
  fn setGridVisible(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget14setGridVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QCalendarWidget14setGridVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::~QCalendarWidget();
impl /*struct*/ QCalendarWidget {
  pub fn FreeQCalendarWidget<RetType, T: QCalendarWidget_FreeQCalendarWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQCalendarWidget(self);
    // return 1;
  }
}

pub trait QCalendarWidget_FreeQCalendarWidget<RetType> {
  fn FreeQCalendarWidget(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::~QCalendarWidget();
impl<'a> /*trait*/ QCalendarWidget_FreeQCalendarWidget<()> for () {
  fn FreeQCalendarWidget(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidgetD0Ev()};
     unsafe {_ZN15QCalendarWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QCalendarWidget::sizeHint();
impl /*struct*/ QCalendarWidget {
  pub fn sizeHint<RetType, T: QCalendarWidget_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QCalendarWidget_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  QSize QCalendarWidget::sizeHint();
impl<'a> /*trait*/ QCalendarWidget_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QCalendarWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget8sizeHintEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QCalendarWidget::monthShown();
impl /*struct*/ QCalendarWidget {
  pub fn monthShown<RetType, T: QCalendarWidget_monthShown<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.monthShown(self);
    // return 1;
  }
}

pub trait QCalendarWidget_monthShown<RetType> {
  fn monthShown(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  int QCalendarWidget::monthShown();
impl<'a> /*trait*/ QCalendarWidget_monthShown<i32> for () {
  fn monthShown(self , rsthis: &mut QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget10monthShownEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget10monthShownEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setSelectedDate(const QDate & date);
impl /*struct*/ QCalendarWidget {
  pub fn setSelectedDate<RetType, T: QCalendarWidget_setSelectedDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSelectedDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setSelectedDate<RetType> {
  fn setSelectedDate(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setSelectedDate(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_setSelectedDate<()> for (QDate) {
  fn setSelectedDate(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget15setSelectedDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget15setSelectedDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::QCalendarWidget(QWidget * parent);
impl /*struct*/ QCalendarWidget {
  pub fn NewQCalendarWidget<T: QCalendarWidget_NewQCalendarWidget>(value: T) -> QCalendarWidget {
    let rsthis = value.NewQCalendarWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QCalendarWidget_NewQCalendarWidget {
  fn NewQCalendarWidget(self) -> QCalendarWidget;
}

  // proto:  void QCalendarWidget::QCalendarWidget(QWidget * parent);
impl<'a> /*trait*/ QCalendarWidget_NewQCalendarWidget for (QWidget) {
  fn NewQCalendarWidget(self) -> QCalendarWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QCalendarWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QCalendarWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::currentPageChanged(int year, int month);
impl /*struct*/ QCalendarWidget {
  pub fn currentPageChanged<RetType, T: QCalendarWidget_currentPageChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentPageChanged(self);
    // return 1;
  }
}

pub trait QCalendarWidget_currentPageChanged<RetType> {
  fn currentPageChanged(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::currentPageChanged(int year, int month);
impl<'a> /*trait*/ QCalendarWidget_currentPageChanged<()> for (i32, i32) {
  fn currentPageChanged(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget18currentPageChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QCalendarWidget18currentPageChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QCalendarWidget::metaObject();
impl /*struct*/ QCalendarWidget {
  pub fn metaObject<RetType, T: QCalendarWidget_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCalendarWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  const QMetaObject * QCalendarWidget::metaObject();
impl<'a> /*trait*/ QCalendarWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget10metaObjectEv()};
     unsafe {_ZNK15QCalendarWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setNavigationBarVisible(bool visible);
impl /*struct*/ QCalendarWidget {
  pub fn setNavigationBarVisible<RetType, T: QCalendarWidget_setNavigationBarVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNavigationBarVisible(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setNavigationBarVisible<RetType> {
  fn setNavigationBarVisible(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setNavigationBarVisible(bool visible);
impl<'a> /*trait*/ QCalendarWidget_setNavigationBarVisible<()> for (i8) {
  fn setNavigationBarVisible(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget23setNavigationBarVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QCalendarWidget23setNavigationBarVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QCalendarWidget::isNavigationBarVisible();
impl /*struct*/ QCalendarWidget {
  pub fn isNavigationBarVisible<RetType, T: QCalendarWidget_isNavigationBarVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNavigationBarVisible(self);
    // return 1;
  }
}

pub trait QCalendarWidget_isNavigationBarVisible<RetType> {
  fn isNavigationBarVisible(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  bool QCalendarWidget::isNavigationBarVisible();
impl<'a> /*trait*/ QCalendarWidget_isNavigationBarVisible<i8> for () {
  fn isNavigationBarVisible(self , rsthis: &mut QCalendarWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget22isNavigationBarVisibleEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget22isNavigationBarVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextCharFormat QCalendarWidget::dateTextFormat(const QDate & date);
impl /*struct*/ QCalendarWidget {
  pub fn dateTextFormat<RetType, T: QCalendarWidget_dateTextFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dateTextFormat(self);
    // return 1;
  }
}

pub trait QCalendarWidget_dateTextFormat<RetType> {
  fn dateTextFormat(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  QTextCharFormat QCalendarWidget::dateTextFormat(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_dateTextFormat<QTextCharFormat> for (QDate) {
  fn dateTextFormat(self , rsthis: &mut QCalendarWidget) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget14dateTextFormatERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QCalendarWidget14dateTextFormatERK5QDate(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setMinimumDate(const QDate & date);
impl /*struct*/ QCalendarWidget {
  pub fn setMinimumDate<RetType, T: QCalendarWidget_setMinimumDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinimumDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setMinimumDate<RetType> {
  fn setMinimumDate(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setMinimumDate(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_setMinimumDate<()> for (QDate) {
  fn setMinimumDate(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget14setMinimumDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget14setMinimumDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QCalendarWidget::dateEditAcceptDelay();
impl /*struct*/ QCalendarWidget {
  pub fn dateEditAcceptDelay<RetType, T: QCalendarWidget_dateEditAcceptDelay<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dateEditAcceptDelay(self);
    // return 1;
  }
}

pub trait QCalendarWidget_dateEditAcceptDelay<RetType> {
  fn dateEditAcceptDelay(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  int QCalendarWidget::dateEditAcceptDelay();
impl<'a> /*trait*/ QCalendarWidget_dateEditAcceptDelay<i32> for () {
  fn dateEditAcceptDelay(self , rsthis: &mut QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget19dateEditAcceptDelayEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget19dateEditAcceptDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QDate QCalendarWidget::minimumDate();
impl /*struct*/ QCalendarWidget {
  pub fn minimumDate<RetType, T: QCalendarWidget_minimumDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_minimumDate<RetType> {
  fn minimumDate(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  QDate QCalendarWidget::minimumDate();
impl<'a> /*trait*/ QCalendarWidget_minimumDate<QDate> for () {
  fn minimumDate(self , rsthis: &mut QCalendarWidget) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget11minimumDateEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget11minimumDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::QCalendarWidget(const QCalendarWidget & );
impl<'a> /*trait*/ QCalendarWidget_NewQCalendarWidget for (QCalendarWidget) {
  fn NewQCalendarWidget(self) -> QCalendarWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QCalendarWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QCalendarWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QCalendarWidget::isDateEditEnabled();
impl /*struct*/ QCalendarWidget {
  pub fn isDateEditEnabled<RetType, T: QCalendarWidget_isDateEditEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDateEditEnabled(self);
    // return 1;
  }
}

pub trait QCalendarWidget_isDateEditEnabled<RetType> {
  fn isDateEditEnabled(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  bool QCalendarWidget::isDateEditEnabled();
impl<'a> /*trait*/ QCalendarWidget_isDateEditEnabled<i8> for () {
  fn isDateEditEnabled(self , rsthis: &mut QCalendarWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget17isDateEditEnabledEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget17isDateEditEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QMap<QDate, QTextCharFormat> QCalendarWidget::dateTextFormat();
impl<'a> /*trait*/ QCalendarWidget_dateTextFormat<()> for () {
  fn dateTextFormat(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget14dateTextFormatEv()};
     unsafe {_ZNK15QCalendarWidget14dateTextFormatEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::clicked(const QDate & date);
impl /*struct*/ QCalendarWidget {
  pub fn clicked<RetType, T: QCalendarWidget_clicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clicked(self);
    // return 1;
  }
}

pub trait QCalendarWidget_clicked<RetType> {
  fn clicked(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::clicked(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_clicked<()> for (QDate) {
  fn clicked(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget7clickedERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget7clickedERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setDateEditEnabled(bool enable);
impl /*struct*/ QCalendarWidget {
  pub fn setDateEditEnabled<RetType, T: QCalendarWidget_setDateEditEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDateEditEnabled(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setDateEditEnabled<RetType> {
  fn setDateEditEnabled(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setDateEditEnabled(bool enable);
impl<'a> /*trait*/ QCalendarWidget_setDateEditEnabled<()> for (i8) {
  fn setDateEditEnabled(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget18setDateEditEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QCalendarWidget18setDateEditEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setDateTextFormat(const QDate & date, const QTextCharFormat & format);
impl /*struct*/ QCalendarWidget {
  pub fn setDateTextFormat<RetType, T: QCalendarWidget_setDateTextFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDateTextFormat(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setDateTextFormat<RetType> {
  fn setDateTextFormat(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setDateTextFormat(const QDate & date, const QTextCharFormat & format);
impl<'a> /*trait*/ QCalendarWidget_setDateTextFormat<()> for (QDate, QTextCharFormat) {
  fn setDateTextFormat(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget17setDateTextFormatERK5QDateRK15QTextCharFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget17setDateTextFormatERK5QDateRK15QTextCharFormat(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::showNextMonth();
impl /*struct*/ QCalendarWidget {
  pub fn showNextMonth<RetType, T: QCalendarWidget_showNextMonth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showNextMonth(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showNextMonth<RetType> {
  fn showNextMonth(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::showNextMonth();
impl<'a> /*trait*/ QCalendarWidget_showNextMonth<()> for () {
  fn showNextMonth(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget13showNextMonthEv()};
     unsafe {_ZN15QCalendarWidget13showNextMonthEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setDateRange(const QDate & min, const QDate & max);
impl /*struct*/ QCalendarWidget {
  pub fn setDateRange<RetType, T: QCalendarWidget_setDateRange<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDateRange(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setDateRange<RetType> {
  fn setDateRange(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setDateRange(const QDate & min, const QDate & max);
impl<'a> /*trait*/ QCalendarWidget_setDateRange<()> for (QDate, QDate) {
  fn setDateRange(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget12setDateRangeERK5QDateS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget12setDateRangeERK5QDateS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QDate QCalendarWidget::selectedDate();
impl /*struct*/ QCalendarWidget {
  pub fn selectedDate<RetType, T: QCalendarWidget_selectedDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_selectedDate<RetType> {
  fn selectedDate(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  QDate QCalendarWidget::selectedDate();
impl<'a> /*trait*/ QCalendarWidget_selectedDate<QDate> for () {
  fn selectedDate(self , rsthis: &mut QCalendarWidget) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget12selectedDateEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget12selectedDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setHeaderTextFormat(const QTextCharFormat & format);
impl /*struct*/ QCalendarWidget {
  pub fn setHeaderTextFormat<RetType, T: QCalendarWidget_setHeaderTextFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeaderTextFormat(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setHeaderTextFormat<RetType> {
  fn setHeaderTextFormat(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setHeaderTextFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QCalendarWidget_setHeaderTextFormat<()> for (QTextCharFormat) {
  fn setHeaderTextFormat(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget19setHeaderTextFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget19setHeaderTextFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QCalendarWidget::isGridVisible();
impl /*struct*/ QCalendarWidget {
  pub fn isGridVisible<RetType, T: QCalendarWidget_isGridVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isGridVisible(self);
    // return 1;
  }
}

pub trait QCalendarWidget_isGridVisible<RetType> {
  fn isGridVisible(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  bool QCalendarWidget::isGridVisible();
impl<'a> /*trait*/ QCalendarWidget_isGridVisible<i8> for () {
  fn isGridVisible(self , rsthis: &mut QCalendarWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget13isGridVisibleEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget13isGridVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QCalendarWidget::yearShown();
impl /*struct*/ QCalendarWidget {
  pub fn yearShown<RetType, T: QCalendarWidget_yearShown<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.yearShown(self);
    // return 1;
  }
}

pub trait QCalendarWidget_yearShown<RetType> {
  fn yearShown(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  int QCalendarWidget::yearShown();
impl<'a> /*trait*/ QCalendarWidget_yearShown<i32> for () {
  fn yearShown(self , rsthis: &mut QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget9yearShownEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget9yearShownEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setMaximumDate(const QDate & date);
impl /*struct*/ QCalendarWidget {
  pub fn setMaximumDate<RetType, T: QCalendarWidget_setMaximumDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setMaximumDate<RetType> {
  fn setMaximumDate(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setMaximumDate(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_setMaximumDate<()> for (QDate) {
  fn setMaximumDate(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget14setMaximumDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget14setMaximumDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextCharFormat QCalendarWidget::headerTextFormat();
impl /*struct*/ QCalendarWidget {
  pub fn headerTextFormat<RetType, T: QCalendarWidget_headerTextFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.headerTextFormat(self);
    // return 1;
  }
}

pub trait QCalendarWidget_headerTextFormat<RetType> {
  fn headerTextFormat(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  QTextCharFormat QCalendarWidget::headerTextFormat();
impl<'a> /*trait*/ QCalendarWidget_headerTextFormat<QTextCharFormat> for () {
  fn headerTextFormat(self , rsthis: &mut QCalendarWidget) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget16headerTextFormatEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget16headerTextFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCalendarWidget::setCurrentPage(int year, int month);
impl /*struct*/ QCalendarWidget {
  pub fn setCurrentPage<RetType, T: QCalendarWidget_setCurrentPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentPage(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setCurrentPage<RetType> {
  fn setCurrentPage(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::setCurrentPage(int year, int month);
impl<'a> /*trait*/ QCalendarWidget_setCurrentPage<()> for (i32, i32) {
  fn setCurrentPage(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget14setCurrentPageEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QCalendarWidget14setCurrentPageEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::showToday();
impl /*struct*/ QCalendarWidget {
  pub fn showToday<RetType, T: QCalendarWidget_showToday<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showToday(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showToday<RetType> {
  fn showToday(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::showToday();
impl<'a> /*trait*/ QCalendarWidget_showToday<()> for () {
  fn showToday(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget9showTodayEv()};
     unsafe {_ZN15QCalendarWidget9showTodayEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::activated(const QDate & date);
impl /*struct*/ QCalendarWidget {
  pub fn activated<RetType, T: QCalendarWidget_activated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.activated(self);
    // return 1;
  }
}

pub trait QCalendarWidget_activated<RetType> {
  fn activated(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::activated(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_activated<()> for (QDate) {
  fn activated(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget9activatedERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget9activatedERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCalendarWidget::showNextYear();
impl /*struct*/ QCalendarWidget {
  pub fn showNextYear<RetType, T: QCalendarWidget_showNextYear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showNextYear(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showNextYear<RetType> {
  fn showNextYear(self , rsthis: &mut QCalendarWidget) -> RetType;
}

  // proto:  void QCalendarWidget::showNextYear();
impl<'a> /*trait*/ QCalendarWidget_showNextYear<()> for () {
  fn showNextYear(self , rsthis: &mut QCalendarWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget12showNextYearEv()};
     unsafe {_ZN15QCalendarWidget12showNextYearEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

