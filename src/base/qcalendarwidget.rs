// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdate::QDate;
use super::qsize::QSize;
use super::qwidget::QWidget;
use super::qtextcharformat::QTextCharFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QCalendarWidget::selectionChanged();
  fn _ZN15QCalendarWidget16selectionChangedEv(qthis: *mut c_void) ;
  // proto:  void QCalendarWidget::showPreviousYear();
  fn _ZN15QCalendarWidget16showPreviousYearEv(qthis: *mut c_void) ;
  // proto:  QDate QCalendarWidget::maximumDate();
  fn _ZNK15QCalendarWidget11maximumDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::showPreviousMonth();
  fn _ZN15QCalendarWidget17showPreviousMonthEv(qthis: *mut c_void) ;
  // proto:  void QCalendarWidget::showSelectedDate();
  fn _ZN15QCalendarWidget16showSelectedDateEv(qthis: *mut c_void) ;
  // proto:  QSize QCalendarWidget::minimumSizeHint();
  fn _ZNK15QCalendarWidget15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::setDateEditAcceptDelay(int delay);
  fn _ZN15QCalendarWidget22setDateEditAcceptDelayEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QCalendarWidget::setGridVisible(bool show);
  fn _ZN15QCalendarWidget14setGridVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QCalendarWidget::FreeQCalendarWidget();
  fn _ZN15QCalendarWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  QSize QCalendarWidget::sizeHint();
  fn _ZNK15QCalendarWidget8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QCalendarWidget::monthShown();
  fn _ZNK15QCalendarWidget10monthShownEv(qthis: *mut c_void) -> c_int;
  // proto:  void QCalendarWidget::setSelectedDate(const QDate & date);
  fn _ZN15QCalendarWidget15setSelectedDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCalendarWidget::NewQCalendarWidget(QWidget * parent);
  fn _ZN15QCalendarWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCalendarWidget::currentPageChanged(int year, int month);
  fn _ZN15QCalendarWidget18currentPageChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  const QMetaObject * QCalendarWidget::metaObject();
  fn _ZNK15QCalendarWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QCalendarWidget::setNavigationBarVisible(bool visible);
  fn _ZN15QCalendarWidget23setNavigationBarVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QCalendarWidget::isNavigationBarVisible();
  fn _ZNK15QCalendarWidget22isNavigationBarVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  QTextCharFormat QCalendarWidget::dateTextFormat(const QDate & date);
  fn _ZNK15QCalendarWidget14dateTextFormatERK5QDate(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::setMinimumDate(const QDate & date);
  fn _ZN15QCalendarWidget14setMinimumDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QCalendarWidget::dateEditAcceptDelay();
  fn _ZNK15QCalendarWidget19dateEditAcceptDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  QDate QCalendarWidget::minimumDate();
  fn _ZNK15QCalendarWidget11minimumDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::NewQCalendarWidget(const QCalendarWidget & );
  fn _ZN15QCalendarWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QCalendarWidget::isDateEditEnabled();
  fn _ZNK15QCalendarWidget17isDateEditEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QCalendarWidget::clicked(const QDate & date);
  fn _ZN15QCalendarWidget7clickedERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCalendarWidget::setDateEditEnabled(bool enable);
  fn _ZN15QCalendarWidget18setDateEditEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QCalendarWidget::setDateTextFormat(const QDate & date, const QTextCharFormat & format);
  fn _ZN15QCalendarWidget17setDateTextFormatERK5QDateRK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QCalendarWidget::showNextMonth();
  fn _ZN15QCalendarWidget13showNextMonthEv(qthis: *mut c_void) ;
  // proto:  void QCalendarWidget::setDateRange(const QDate & min, const QDate & max);
  fn _ZN15QCalendarWidget12setDateRangeERK5QDateS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QDate QCalendarWidget::selectedDate();
  fn _ZNK15QCalendarWidget12selectedDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::setHeaderTextFormat(const QTextCharFormat & format);
  fn _ZN15QCalendarWidget19setHeaderTextFormatERK15QTextCharFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QCalendarWidget::isGridVisible();
  fn _ZNK15QCalendarWidget13isGridVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QCalendarWidget::yearShown();
  fn _ZNK15QCalendarWidget9yearShownEv(qthis: *mut c_void) -> c_int;
  // proto:  void QCalendarWidget::setMaximumDate(const QDate & date);
  fn _ZN15QCalendarWidget14setMaximumDateERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextCharFormat QCalendarWidget::headerTextFormat();
  fn _ZNK15QCalendarWidget16headerTextFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCalendarWidget::setCurrentPage(int year, int month);
  fn _ZN15QCalendarWidget14setCurrentPageEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QCalendarWidget::showToday();
  fn _ZN15QCalendarWidget9showTodayEv(qthis: *mut c_void) ;
  // proto:  void QCalendarWidget::activated(const QDate & date);
  fn _ZN15QCalendarWidget9activatedERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCalendarWidget::showNextYear();
  fn _ZN15QCalendarWidget12showNextYearEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QCalendarWidget)=1
pub struct QCalendarWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCalendarWidget {
  pub fn selectionChanged<T: QCalendarWidget_selectionChanged>(&mut self, value: T)  {
     value.selectionChanged(self);
    // return 1;
  }
}

pub trait QCalendarWidget_selectionChanged {
  fn selectionChanged(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::selectionChanged();
impl<'a> /*trait*/ QCalendarWidget_selectionChanged for () {
  fn selectionChanged(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget16selectionChangedEv()};
     unsafe {_ZN15QCalendarWidget16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn showPreviousYear<T: QCalendarWidget_showPreviousYear>(&mut self, value: T)  {
     value.showPreviousYear(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showPreviousYear {
  fn showPreviousYear(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::showPreviousYear();
impl<'a> /*trait*/ QCalendarWidget_showPreviousYear for () {
  fn showPreviousYear(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget16showPreviousYearEv()};
     unsafe {_ZN15QCalendarWidget16showPreviousYearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn maximumDate<T: QCalendarWidget_maximumDate>(&mut self, value: T) -> QDate {
    return value.maximumDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_maximumDate {
  fn maximumDate(self, rsthis: &mut QCalendarWidget) -> QDate;
}

// proto:  QDate QCalendarWidget::maximumDate();
impl<'a> /*trait*/ QCalendarWidget_maximumDate for () {
  fn maximumDate(self, rsthis: &mut QCalendarWidget) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget11maximumDateEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget11maximumDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn showPreviousMonth<T: QCalendarWidget_showPreviousMonth>(&mut self, value: T)  {
     value.showPreviousMonth(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showPreviousMonth {
  fn showPreviousMonth(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::showPreviousMonth();
impl<'a> /*trait*/ QCalendarWidget_showPreviousMonth for () {
  fn showPreviousMonth(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget17showPreviousMonthEv()};
     unsafe {_ZN15QCalendarWidget17showPreviousMonthEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn showSelectedDate<T: QCalendarWidget_showSelectedDate>(&mut self, value: T)  {
     value.showSelectedDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showSelectedDate {
  fn showSelectedDate(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::showSelectedDate();
impl<'a> /*trait*/ QCalendarWidget_showSelectedDate for () {
  fn showSelectedDate(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget16showSelectedDateEv()};
     unsafe {_ZN15QCalendarWidget16showSelectedDateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn minimumSizeHint<T: QCalendarWidget_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QCalendarWidget_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QCalendarWidget) -> QSize;
}

// proto:  QSize QCalendarWidget::minimumSizeHint();
impl<'a> /*trait*/ QCalendarWidget_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QCalendarWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setDateEditAcceptDelay<T: QCalendarWidget_setDateEditAcceptDelay>(&mut self, value: T)  {
     value.setDateEditAcceptDelay(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setDateEditAcceptDelay {
  fn setDateEditAcceptDelay(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setDateEditAcceptDelay(int delay);
impl<'a> /*trait*/ QCalendarWidget_setDateEditAcceptDelay for (i32) {
  fn setDateEditAcceptDelay(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget22setDateEditAcceptDelayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QCalendarWidget22setDateEditAcceptDelayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setGridVisible<T: QCalendarWidget_setGridVisible>(&mut self, value: T)  {
     value.setGridVisible(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setGridVisible {
  fn setGridVisible(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setGridVisible(bool show);
impl<'a> /*trait*/ QCalendarWidget_setGridVisible for (i8) {
  fn setGridVisible(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget14setGridVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QCalendarWidget14setGridVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn FreeQCalendarWidget<T: QCalendarWidget_FreeQCalendarWidget>(&mut self, value: T)  {
     value.FreeQCalendarWidget(self);
    // return 1;
  }
}

pub trait QCalendarWidget_FreeQCalendarWidget {
  fn FreeQCalendarWidget(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::FreeQCalendarWidget();
impl<'a> /*trait*/ QCalendarWidget_FreeQCalendarWidget for () {
  fn FreeQCalendarWidget(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidgetD0Ev()};
     unsafe {_ZN15QCalendarWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn sizeHint<T: QCalendarWidget_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QCalendarWidget_sizeHint {
  fn sizeHint(self, rsthis: &mut QCalendarWidget) -> QSize;
}

// proto:  QSize QCalendarWidget::sizeHint();
impl<'a> /*trait*/ QCalendarWidget_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QCalendarWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget8sizeHintEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn monthShown<T: QCalendarWidget_monthShown>(&mut self, value: T) -> i32 {
    return value.monthShown(self);
    // return 1;
  }
}

pub trait QCalendarWidget_monthShown {
  fn monthShown(self, rsthis: &mut QCalendarWidget) -> i32;
}

// proto:  int QCalendarWidget::monthShown();
impl<'a> /*trait*/ QCalendarWidget_monthShown for () {
  fn monthShown(self, rsthis: &mut QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget10monthShownEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget10monthShownEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setSelectedDate<T: QCalendarWidget_setSelectedDate>(&mut self, value: T)  {
     value.setSelectedDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setSelectedDate {
  fn setSelectedDate(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setSelectedDate(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_setSelectedDate for (&'a  QDate) {
  fn setSelectedDate(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget15setSelectedDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget15setSelectedDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

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

// proto: void QCalendarWidget::NewQCalendarWidget(QWidget * parent);
impl<'a> /*trait*/ QCalendarWidget_NewQCalendarWidget for (&'a mut QWidget) {
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

impl /*struct*/ QCalendarWidget {
  pub fn currentPageChanged<T: QCalendarWidget_currentPageChanged>(&mut self, value: T)  {
     value.currentPageChanged(self);
    // return 1;
  }
}

pub trait QCalendarWidget_currentPageChanged {
  fn currentPageChanged(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::currentPageChanged(int year, int month);
impl<'a> /*trait*/ QCalendarWidget_currentPageChanged for (i32, i32) {
  fn currentPageChanged(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget18currentPageChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QCalendarWidget18currentPageChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn metaObject<T: QCalendarWidget_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QCalendarWidget_metaObject {
  fn metaObject(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  const QMetaObject * QCalendarWidget::metaObject();
impl<'a> /*trait*/ QCalendarWidget_metaObject for () {
  fn metaObject(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget10metaObjectEv()};
     unsafe {_ZNK15QCalendarWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setNavigationBarVisible<T: QCalendarWidget_setNavigationBarVisible>(&mut self, value: T)  {
     value.setNavigationBarVisible(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setNavigationBarVisible {
  fn setNavigationBarVisible(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setNavigationBarVisible(bool visible);
impl<'a> /*trait*/ QCalendarWidget_setNavigationBarVisible for (i8) {
  fn setNavigationBarVisible(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget23setNavigationBarVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QCalendarWidget23setNavigationBarVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn isNavigationBarVisible<T: QCalendarWidget_isNavigationBarVisible>(&mut self, value: T) -> i8 {
    return value.isNavigationBarVisible(self);
    // return 1;
  }
}

pub trait QCalendarWidget_isNavigationBarVisible {
  fn isNavigationBarVisible(self, rsthis: &mut QCalendarWidget) -> i8;
}

// proto:  bool QCalendarWidget::isNavigationBarVisible();
impl<'a> /*trait*/ QCalendarWidget_isNavigationBarVisible for () {
  fn isNavigationBarVisible(self, rsthis: &mut QCalendarWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget22isNavigationBarVisibleEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget22isNavigationBarVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn dateTextFormat<T: QCalendarWidget_dateTextFormat>(&mut self, value: T) -> QTextCharFormat {
    return value.dateTextFormat(self);
    // return 1;
  }
}

pub trait QCalendarWidget_dateTextFormat {
  fn dateTextFormat(self, rsthis: &mut QCalendarWidget) -> QTextCharFormat;
}

// proto:  QTextCharFormat QCalendarWidget::dateTextFormat(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_dateTextFormat for (&'a  QDate) {
  fn dateTextFormat(self, rsthis: &mut QCalendarWidget) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget14dateTextFormatERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QCalendarWidget14dateTextFormatERK5QDate(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setMinimumDate<T: QCalendarWidget_setMinimumDate>(&mut self, value: T)  {
     value.setMinimumDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setMinimumDate {
  fn setMinimumDate(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setMinimumDate(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_setMinimumDate for (&'a  QDate) {
  fn setMinimumDate(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget14setMinimumDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget14setMinimumDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn dateEditAcceptDelay<T: QCalendarWidget_dateEditAcceptDelay>(&mut self, value: T) -> i32 {
    return value.dateEditAcceptDelay(self);
    // return 1;
  }
}

pub trait QCalendarWidget_dateEditAcceptDelay {
  fn dateEditAcceptDelay(self, rsthis: &mut QCalendarWidget) -> i32;
}

// proto:  int QCalendarWidget::dateEditAcceptDelay();
impl<'a> /*trait*/ QCalendarWidget_dateEditAcceptDelay for () {
  fn dateEditAcceptDelay(self, rsthis: &mut QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget19dateEditAcceptDelayEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget19dateEditAcceptDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn minimumDate<T: QCalendarWidget_minimumDate>(&mut self, value: T) -> QDate {
    return value.minimumDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_minimumDate {
  fn minimumDate(self, rsthis: &mut QCalendarWidget) -> QDate;
}

// proto:  QDate QCalendarWidget::minimumDate();
impl<'a> /*trait*/ QCalendarWidget_minimumDate for () {
  fn minimumDate(self, rsthis: &mut QCalendarWidget) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget11minimumDateEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget11minimumDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QCalendarWidget::NewQCalendarWidget(const QCalendarWidget & );
impl<'a> /*trait*/ QCalendarWidget_NewQCalendarWidget for (&'a  QCalendarWidget) {
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

impl /*struct*/ QCalendarWidget {
  pub fn isDateEditEnabled<T: QCalendarWidget_isDateEditEnabled>(&mut self, value: T) -> i8 {
    return value.isDateEditEnabled(self);
    // return 1;
  }
}

pub trait QCalendarWidget_isDateEditEnabled {
  fn isDateEditEnabled(self, rsthis: &mut QCalendarWidget) -> i8;
}

// proto:  bool QCalendarWidget::isDateEditEnabled();
impl<'a> /*trait*/ QCalendarWidget_isDateEditEnabled for () {
  fn isDateEditEnabled(self, rsthis: &mut QCalendarWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget17isDateEditEnabledEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget17isDateEditEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn clicked<T: QCalendarWidget_clicked>(&mut self, value: T)  {
     value.clicked(self);
    // return 1;
  }
}

pub trait QCalendarWidget_clicked {
  fn clicked(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::clicked(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_clicked for (&'a  QDate) {
  fn clicked(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget7clickedERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget7clickedERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setDateEditEnabled<T: QCalendarWidget_setDateEditEnabled>(&mut self, value: T)  {
     value.setDateEditEnabled(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setDateEditEnabled {
  fn setDateEditEnabled(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setDateEditEnabled(bool enable);
impl<'a> /*trait*/ QCalendarWidget_setDateEditEnabled for (i8) {
  fn setDateEditEnabled(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget18setDateEditEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QCalendarWidget18setDateEditEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setDateTextFormat<T: QCalendarWidget_setDateTextFormat>(&mut self, value: T)  {
     value.setDateTextFormat(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setDateTextFormat {
  fn setDateTextFormat(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setDateTextFormat(const QDate & date, const QTextCharFormat & format);
impl<'a> /*trait*/ QCalendarWidget_setDateTextFormat for (&'a  QDate, &'a  QTextCharFormat) {
  fn setDateTextFormat(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget17setDateTextFormatERK5QDateRK15QTextCharFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget17setDateTextFormatERK5QDateRK15QTextCharFormat(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn showNextMonth<T: QCalendarWidget_showNextMonth>(&mut self, value: T)  {
     value.showNextMonth(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showNextMonth {
  fn showNextMonth(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::showNextMonth();
impl<'a> /*trait*/ QCalendarWidget_showNextMonth for () {
  fn showNextMonth(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget13showNextMonthEv()};
     unsafe {_ZN15QCalendarWidget13showNextMonthEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setDateRange<T: QCalendarWidget_setDateRange>(&mut self, value: T)  {
     value.setDateRange(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setDateRange {
  fn setDateRange(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setDateRange(const QDate & min, const QDate & max);
impl<'a> /*trait*/ QCalendarWidget_setDateRange for (&'a  QDate, &'a  QDate) {
  fn setDateRange(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget12setDateRangeERK5QDateS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget12setDateRangeERK5QDateS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn selectedDate<T: QCalendarWidget_selectedDate>(&mut self, value: T) -> QDate {
    return value.selectedDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_selectedDate {
  fn selectedDate(self, rsthis: &mut QCalendarWidget) -> QDate;
}

// proto:  QDate QCalendarWidget::selectedDate();
impl<'a> /*trait*/ QCalendarWidget_selectedDate for () {
  fn selectedDate(self, rsthis: &mut QCalendarWidget) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget12selectedDateEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget12selectedDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setHeaderTextFormat<T: QCalendarWidget_setHeaderTextFormat>(&mut self, value: T)  {
     value.setHeaderTextFormat(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setHeaderTextFormat {
  fn setHeaderTextFormat(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setHeaderTextFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QCalendarWidget_setHeaderTextFormat for (&'a  QTextCharFormat) {
  fn setHeaderTextFormat(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget19setHeaderTextFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget19setHeaderTextFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn isGridVisible<T: QCalendarWidget_isGridVisible>(&mut self, value: T) -> i8 {
    return value.isGridVisible(self);
    // return 1;
  }
}

pub trait QCalendarWidget_isGridVisible {
  fn isGridVisible(self, rsthis: &mut QCalendarWidget) -> i8;
}

// proto:  bool QCalendarWidget::isGridVisible();
impl<'a> /*trait*/ QCalendarWidget_isGridVisible for () {
  fn isGridVisible(self, rsthis: &mut QCalendarWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget13isGridVisibleEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget13isGridVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn yearShown<T: QCalendarWidget_yearShown>(&mut self, value: T) -> i32 {
    return value.yearShown(self);
    // return 1;
  }
}

pub trait QCalendarWidget_yearShown {
  fn yearShown(self, rsthis: &mut QCalendarWidget) -> i32;
}

// proto:  int QCalendarWidget::yearShown();
impl<'a> /*trait*/ QCalendarWidget_yearShown for () {
  fn yearShown(self, rsthis: &mut QCalendarWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget9yearShownEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget9yearShownEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setMaximumDate<T: QCalendarWidget_setMaximumDate>(&mut self, value: T)  {
     value.setMaximumDate(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setMaximumDate {
  fn setMaximumDate(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setMaximumDate(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_setMaximumDate for (&'a  QDate) {
  fn setMaximumDate(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget14setMaximumDateERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget14setMaximumDateERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn headerTextFormat<T: QCalendarWidget_headerTextFormat>(&mut self, value: T) -> QTextCharFormat {
    return value.headerTextFormat(self);
    // return 1;
  }
}

pub trait QCalendarWidget_headerTextFormat {
  fn headerTextFormat(self, rsthis: &mut QCalendarWidget) -> QTextCharFormat;
}

// proto:  QTextCharFormat QCalendarWidget::headerTextFormat();
impl<'a> /*trait*/ QCalendarWidget_headerTextFormat for () {
  fn headerTextFormat(self, rsthis: &mut QCalendarWidget) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QCalendarWidget16headerTextFormatEv()};
    let mut ret = unsafe {_ZNK15QCalendarWidget16headerTextFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn setCurrentPage<T: QCalendarWidget_setCurrentPage>(&mut self, value: T)  {
     value.setCurrentPage(self);
    // return 1;
  }
}

pub trait QCalendarWidget_setCurrentPage {
  fn setCurrentPage(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::setCurrentPage(int year, int month);
impl<'a> /*trait*/ QCalendarWidget_setCurrentPage for (i32, i32) {
  fn setCurrentPage(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget14setCurrentPageEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QCalendarWidget14setCurrentPageEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn showToday<T: QCalendarWidget_showToday>(&mut self, value: T)  {
     value.showToday(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showToday {
  fn showToday(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::showToday();
impl<'a> /*trait*/ QCalendarWidget_showToday for () {
  fn showToday(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget9showTodayEv()};
     unsafe {_ZN15QCalendarWidget9showTodayEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn activated<T: QCalendarWidget_activated>(&mut self, value: T)  {
     value.activated(self);
    // return 1;
  }
}

pub trait QCalendarWidget_activated {
  fn activated(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::activated(const QDate & date);
impl<'a> /*trait*/ QCalendarWidget_activated for (&'a  QDate) {
  fn activated(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget9activatedERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QCalendarWidget9activatedERK5QDate(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCalendarWidget {
  pub fn showNextYear<T: QCalendarWidget_showNextYear>(&mut self, value: T)  {
     value.showNextYear(self);
    // return 1;
  }
}

pub trait QCalendarWidget_showNextYear {
  fn showNextYear(self, rsthis: &mut QCalendarWidget) ;
}

// proto:  void QCalendarWidget::showNextYear();
impl<'a> /*trait*/ QCalendarWidget_showNextYear for () {
  fn showNextYear(self, rsthis: &mut QCalendarWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QCalendarWidget12showNextYearEv()};
     unsafe {_ZN15QCalendarWidget12showNextYearEv(rsthis.qclsinst)};
    // return 1;
  }
}

