// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qfont::QFont;
use super::qstyle::QStyle;
use super::qsize::QSize;
use super::qpalette::QPalette;
use super::qicon::QIcon;
use super::qobject::QObject;
use super::qevent::QEvent;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QString QApplication::styleSheet();
  fn _ZNK12QApplication10styleSheetEv() -> i32;
  // proto: QPalette QApplication::palette(const char * className);
  fn _ZN12QApplication7paletteEPKc(arg0: *const c_char) -> i32;
  // proto: QWidget * QApplication::activeWindow();
  fn _ZN12QApplication12activeWindowEv() -> i32;
  // proto: void QApplication::setKeyboardInputInterval(int );
  fn _ZN12QApplication24setKeyboardInputIntervalEi(arg0: c_int) -> i32;
  // proto: QWidget * QApplication::focusWidget();
  fn _ZN12QApplication11focusWidgetEv() -> i32;
  // proto: QFontMetrics QApplication::fontMetrics();
  fn _ZN12QApplication11fontMetricsEv() -> i32;
  // proto: QFont QApplication::font(const char * className);
  fn _ZN12QApplication4fontEPKc(arg0: *const c_char) -> i32;
  // proto: QStyle * QApplication::style();
  fn _ZN12QApplication5styleEv() -> i32;
  // proto: QWidget * QApplication::widgetAt(const QPoint & p);
  fn _ZN12QApplication8widgetAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QApplication::setActiveWindow(QWidget * act);
  fn _ZN12QApplication15setActiveWindowEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: QFont QApplication::font();
  fn _ZN12QApplication4fontEv() -> i32;
  // proto: void QApplication::setWheelScrollLines(int );
  fn _ZN12QApplication19setWheelScrollLinesEi(arg0: c_int) -> i32;
  // proto: void QApplication::setStyleSheet(const QString & sheet);
  fn _ZN12QApplication13setStyleSheetERK7QString(arg0: *const c_void) -> i32;
  // proto: void QApplication::setAutoSipEnabled(const bool enabled);
  fn _ZN12QApplication17setAutoSipEnabledEb(arg0: bool) -> i32;
  // proto: const QMetaObject * QApplication::metaObject();
  fn _ZNK12QApplication10metaObjectEv() -> i32;
  // proto: int QApplication::keyboardInputInterval();
  fn _ZN12QApplication21keyboardInputIntervalEv() -> i32;
  // proto: int QApplication::cursorFlashTime();
  fn _ZN12QApplication15cursorFlashTimeEv() -> i32;
  // proto: int QApplication::startDragDistance();
  fn _ZN12QApplication17startDragDistanceEv() -> i32;
  // proto: QDesktopWidget * QApplication::desktop();
  fn _ZN12QApplication7desktopEv() -> i32;
  // proto: void QApplication::setStartDragDistance(int l);
  fn _ZN12QApplication20setStartDragDistanceEi(arg0: c_int) -> i32;
  // proto: QFont QApplication::font(const QWidget * );
  fn _ZN12QApplication4fontEPK7QWidget(arg0: *const c_void) -> i32;
  // proto: int QApplication::colorSpec();
  fn _ZN12QApplication9colorSpecEv() -> i32;
  // proto: void QApplication::setFont(const QFont & , const char * className);
  fn _ZN12QApplication7setFontERK5QFontPKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: void QApplication::closeAllWindows();
  fn _ZN12QApplication15closeAllWindowsEv() -> i32;
  // proto: void QApplication::setCursorFlashTime(int );
  fn _ZN12QApplication18setCursorFlashTimeEi(arg0: c_int) -> i32;
  // proto: QWidget * QApplication::widgetAt(int x, int y);
  fn _ZN12QApplication8widgetAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QApplication::alert(QWidget * widget, int duration);
  fn _ZN12QApplication5alertEP7QWidgeti(arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: QPalette QApplication::palette(const QWidget * );
  fn _ZN12QApplication7paletteEPK7QWidget(arg0: *const c_void) -> i32;
  // proto: int QApplication::wheelScrollLines();
  fn _ZN12QApplication16wheelScrollLinesEv() -> i32;
  // proto: void QApplication::NewQApplication(const QApplication & );
  fn _ZN12QApplicationC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QApplication::aboutQt();
  fn _ZN12QApplication7aboutQtEv() -> i32;
  // proto: QWidget * QApplication::activeModalWidget();
  fn _ZN12QApplication17activeModalWidgetEv() -> i32;
  // proto: QWidget * QApplication::activePopupWidget();
  fn _ZN12QApplication17activePopupWidgetEv() -> i32;
  // proto: void QApplication::NewQApplication(int & argc, char ** argv, int );
  fn _ZN12QApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) -> i32;
  // proto: void QApplication::focusChanged(QWidget * old, QWidget * now);
  fn _ZN12QApplication12focusChangedEP7QWidgetS1_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: void QApplication::setStartDragTime(int ms);
  fn _ZN12QApplication16setStartDragTimeEi(arg0: c_int) -> i32;
  // proto: QWidget * QApplication::topLevelAt(int x, int y);
  fn _ZN12QApplication10topLevelAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QApplication::setStyle(QStyle * );
  fn _ZN12QApplication8setStyleEP6QStyle(arg0: *mut c_void) -> i32;
  // proto: void QApplication::FreeQApplication();
  fn _ZN12QApplicationD0Ev() -> i32;
  // proto: void QApplication::setDoubleClickInterval(int );
  fn _ZN12QApplication22setDoubleClickIntervalEi(arg0: c_int) -> i32;
  // proto: void QApplication::setGlobalStrut(const QSize & );
  fn _ZN12QApplication14setGlobalStrutERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QApplication::setColorSpec(int );
  fn _ZN12QApplication12setColorSpecEi(arg0: c_int) -> i32;
  // proto: QList<QWidget *> QApplication::allWidgets();
  fn _ZN12QApplication10allWidgetsEv() -> i32;
  // proto: QSize QApplication::globalStrut();
  fn _ZN12QApplication11globalStrutEv() -> i32;
  // proto: void QApplication::setPalette(const QPalette & , const char * className);
  fn _ZN12QApplication10setPaletteERK8QPalettePKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: QStyle * QApplication::setStyle(const QString & );
  fn _ZN12QApplication8setStyleERK7QString(arg0: *const c_void) -> i32;
  // proto: QList<QWidget *> QApplication::topLevelWidgets();
  fn _ZN12QApplication15topLevelWidgetsEv() -> i32;
  // proto: int QApplication::exec();
  fn _ZN12QApplication4execEv() -> i32;
  // proto: void QApplication::setWindowIcon(const QIcon & icon);
  fn _ZN12QApplication13setWindowIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: void QApplication::beep();
  fn _ZN12QApplication4beepEv() -> i32;
  // proto: bool QApplication::notify(QObject * , QEvent * );
  fn _ZN12QApplication6notifyEP7QObjectP6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: bool QApplication::autoSipEnabled();
  fn _ZNK12QApplication14autoSipEnabledEv() -> i32;
  // proto: QWidget * QApplication::topLevelAt(const QPoint & p);
  fn _ZN12QApplication10topLevelAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: int QApplication::startDragTime();
  fn _ZN12QApplication13startDragTimeEv() -> i32;
  // proto: int QApplication::doubleClickInterval();
  fn _ZN12QApplication19doubleClickIntervalEv() -> i32;
  // proto: QIcon QApplication::windowIcon();
  fn _ZN12QApplication10windowIconEv() -> i32;
}

// body block begin
// class sizeof(QApplication)=1
pub struct QApplication {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QApplication {
  pub fn styleSheet<T: QApplication_styleSheet>(&mut self, value: T) -> i32 {
    value.styleSheet(self);
    return 1;
  }
}

pub trait QApplication_styleSheet {
  fn styleSheet(self, this: &mut QApplication) -> i32;
}

// proto: QString QApplication::styleSheet();
impl<'a> /*trait*/ QApplication_styleSheet for () {
  fn styleSheet(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication10styleSheetEv()};
    unsafe {_ZNK12QApplication10styleSheetEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn palette<T: QApplication_palette>(&mut self, value: T) -> i32 {
    value.palette(self);
    return 1;
  }
}

pub trait QApplication_palette {
  fn palette(self, this: &mut QApplication) -> i32;
}

// proto: QPalette QApplication::palette(const char * className);
impl<'a> /*trait*/ QApplication_palette for (&'a  String) {
  fn palette(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7paletteEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN12QApplication7paletteEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn activeWindow<T: QApplication_activeWindow>(&mut self, value: T) -> i32 {
    value.activeWindow(self);
    return 1;
  }
}

pub trait QApplication_activeWindow {
  fn activeWindow(self, this: &mut QApplication) -> i32;
}

// proto: QWidget * QApplication::activeWindow();
impl<'a> /*trait*/ QApplication_activeWindow for () {
  fn activeWindow(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12activeWindowEv()};
    unsafe {_ZN12QApplication12activeWindowEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setKeyboardInputInterval<T: QApplication_setKeyboardInputInterval>(&mut self, value: T) -> i32 {
    value.setKeyboardInputInterval(self);
    return 1;
  }
}

pub trait QApplication_setKeyboardInputInterval {
  fn setKeyboardInputInterval(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setKeyboardInputInterval(int );
impl<'a> /*trait*/ QApplication_setKeyboardInputInterval for (i32) {
  fn setKeyboardInputInterval(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication24setKeyboardInputIntervalEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QApplication24setKeyboardInputIntervalEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn focusWidget<T: QApplication_focusWidget>(&mut self, value: T) -> i32 {
    value.focusWidget(self);
    return 1;
  }
}

pub trait QApplication_focusWidget {
  fn focusWidget(self, this: &mut QApplication) -> i32;
}

// proto: QWidget * QApplication::focusWidget();
impl<'a> /*trait*/ QApplication_focusWidget for () {
  fn focusWidget(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11focusWidgetEv()};
    unsafe {_ZN12QApplication11focusWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn fontMetrics<T: QApplication_fontMetrics>(&mut self, value: T) -> i32 {
    value.fontMetrics(self);
    return 1;
  }
}

pub trait QApplication_fontMetrics {
  fn fontMetrics(self, this: &mut QApplication) -> i32;
}

// proto: QFontMetrics QApplication::fontMetrics();
impl<'a> /*trait*/ QApplication_fontMetrics for () {
  fn fontMetrics(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11fontMetricsEv()};
    unsafe {_ZN12QApplication11fontMetricsEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn font<T: QApplication_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QApplication_font {
  fn font(self, this: &mut QApplication) -> i32;
}

// proto: QFont QApplication::font(const char * className);
impl<'a> /*trait*/ QApplication_font for (&'a  String) {
  fn font(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN12QApplication4fontEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn style<T: QApplication_style>(&mut self, value: T) -> i32 {
    value.style(self);
    return 1;
  }
}

pub trait QApplication_style {
  fn style(self, this: &mut QApplication) -> i32;
}

// proto: QStyle * QApplication::style();
impl<'a> /*trait*/ QApplication_style for () {
  fn style(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication5styleEv()};
    unsafe {_ZN12QApplication5styleEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn widgetAt<T: QApplication_widgetAt>(&mut self, value: T) -> i32 {
    value.widgetAt(self);
    return 1;
  }
}

pub trait QApplication_widgetAt {
  fn widgetAt(self, this: &mut QApplication) -> i32;
}

// proto: QWidget * QApplication::widgetAt(const QPoint & p);
impl<'a> /*trait*/ QApplication_widgetAt for (&'a  QPoint) {
  fn widgetAt(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8widgetAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplication8widgetAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setActiveWindow<T: QApplication_setActiveWindow>(&mut self, value: T) -> i32 {
    value.setActiveWindow(self);
    return 1;
  }
}

pub trait QApplication_setActiveWindow {
  fn setActiveWindow(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setActiveWindow(QWidget * act);
impl<'a> /*trait*/ QApplication_setActiveWindow for (&'a mut QWidget) {
  fn setActiveWindow(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15setActiveWindowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QApplication15setActiveWindowEP7QWidget(arg0)};
    return 1;
  }
}

// proto: QFont QApplication::font();
impl<'a> /*trait*/ QApplication_font for () {
  fn font(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEv()};
    unsafe {_ZN12QApplication4fontEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setWheelScrollLines<T: QApplication_setWheelScrollLines>(&mut self, value: T) -> i32 {
    value.setWheelScrollLines(self);
    return 1;
  }
}

pub trait QApplication_setWheelScrollLines {
  fn setWheelScrollLines(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setWheelScrollLines(int );
impl<'a> /*trait*/ QApplication_setWheelScrollLines for (i32) {
  fn setWheelScrollLines(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication19setWheelScrollLinesEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QApplication19setWheelScrollLinesEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setStyleSheet<T: QApplication_setStyleSheet>(&mut self, value: T) -> i32 {
    value.setStyleSheet(self);
    return 1;
  }
}

pub trait QApplication_setStyleSheet {
  fn setStyleSheet(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setStyleSheet(const QString & sheet);
impl<'a> /*trait*/ QApplication_setStyleSheet for (&'a  QString) {
  fn setStyleSheet(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13setStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplication13setStyleSheetERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setAutoSipEnabled<T: QApplication_setAutoSipEnabled>(&mut self, value: T) -> i32 {
    value.setAutoSipEnabled(self);
    return 1;
  }
}

pub trait QApplication_setAutoSipEnabled {
  fn setAutoSipEnabled(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setAutoSipEnabled(const bool enabled);
impl<'a> /*trait*/ QApplication_setAutoSipEnabled for (bool) {
  fn setAutoSipEnabled(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17setAutoSipEnabledEb()};
    let arg0 = self  as bool;
    unsafe {_ZN12QApplication17setAutoSipEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn metaObject<T: QApplication_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QApplication_metaObject {
  fn metaObject(self, this: &mut QApplication) -> i32;
}

// proto: const QMetaObject * QApplication::metaObject();
impl<'a> /*trait*/ QApplication_metaObject for () {
  fn metaObject(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication10metaObjectEv()};
    unsafe {_ZNK12QApplication10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn keyboardInputInterval<T: QApplication_keyboardInputInterval>(&mut self, value: T) -> i32 {
    value.keyboardInputInterval(self);
    return 1;
  }
}

pub trait QApplication_keyboardInputInterval {
  fn keyboardInputInterval(self, this: &mut QApplication) -> i32;
}

// proto: int QApplication::keyboardInputInterval();
impl<'a> /*trait*/ QApplication_keyboardInputInterval for () {
  fn keyboardInputInterval(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication21keyboardInputIntervalEv()};
    unsafe {_ZN12QApplication21keyboardInputIntervalEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn cursorFlashTime<T: QApplication_cursorFlashTime>(&mut self, value: T) -> i32 {
    value.cursorFlashTime(self);
    return 1;
  }
}

pub trait QApplication_cursorFlashTime {
  fn cursorFlashTime(self, this: &mut QApplication) -> i32;
}

// proto: int QApplication::cursorFlashTime();
impl<'a> /*trait*/ QApplication_cursorFlashTime for () {
  fn cursorFlashTime(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15cursorFlashTimeEv()};
    unsafe {_ZN12QApplication15cursorFlashTimeEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn startDragDistance<T: QApplication_startDragDistance>(&mut self, value: T) -> i32 {
    value.startDragDistance(self);
    return 1;
  }
}

pub trait QApplication_startDragDistance {
  fn startDragDistance(self, this: &mut QApplication) -> i32;
}

// proto: int QApplication::startDragDistance();
impl<'a> /*trait*/ QApplication_startDragDistance for () {
  fn startDragDistance(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17startDragDistanceEv()};
    unsafe {_ZN12QApplication17startDragDistanceEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn desktop<T: QApplication_desktop>(&mut self, value: T) -> i32 {
    value.desktop(self);
    return 1;
  }
}

pub trait QApplication_desktop {
  fn desktop(self, this: &mut QApplication) -> i32;
}

// proto: QDesktopWidget * QApplication::desktop();
impl<'a> /*trait*/ QApplication_desktop for () {
  fn desktop(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7desktopEv()};
    unsafe {_ZN12QApplication7desktopEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setStartDragDistance<T: QApplication_setStartDragDistance>(&mut self, value: T) -> i32 {
    value.setStartDragDistance(self);
    return 1;
  }
}

pub trait QApplication_setStartDragDistance {
  fn setStartDragDistance(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setStartDragDistance(int l);
impl<'a> /*trait*/ QApplication_setStartDragDistance for (i32) {
  fn setStartDragDistance(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication20setStartDragDistanceEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QApplication20setStartDragDistanceEi(arg0)};
    return 1;
  }
}

// proto: QFont QApplication::font(const QWidget * );
impl<'a> /*trait*/ QApplication_font for (&'a  QWidget) {
  fn font(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEPK7QWidget()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplication4fontEPK7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn colorSpec<T: QApplication_colorSpec>(&mut self, value: T) -> i32 {
    value.colorSpec(self);
    return 1;
  }
}

pub trait QApplication_colorSpec {
  fn colorSpec(self, this: &mut QApplication) -> i32;
}

// proto: int QApplication::colorSpec();
impl<'a> /*trait*/ QApplication_colorSpec for () {
  fn colorSpec(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication9colorSpecEv()};
    unsafe {_ZN12QApplication9colorSpecEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setFont<T: QApplication_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QApplication_setFont {
  fn setFont(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setFont(const QFont & , const char * className);
impl<'a> /*trait*/ QApplication_setFont for (&'a  QFont, &'a  String) {
  fn setFont(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7setFontERK5QFontPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN12QApplication7setFontERK5QFontPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn closeAllWindows<T: QApplication_closeAllWindows>(&mut self, value: T) -> i32 {
    value.closeAllWindows(self);
    return 1;
  }
}

pub trait QApplication_closeAllWindows {
  fn closeAllWindows(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::closeAllWindows();
impl<'a> /*trait*/ QApplication_closeAllWindows for () {
  fn closeAllWindows(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15closeAllWindowsEv()};
    unsafe {_ZN12QApplication15closeAllWindowsEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setCursorFlashTime<T: QApplication_setCursorFlashTime>(&mut self, value: T) -> i32 {
    value.setCursorFlashTime(self);
    return 1;
  }
}

pub trait QApplication_setCursorFlashTime {
  fn setCursorFlashTime(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setCursorFlashTime(int );
impl<'a> /*trait*/ QApplication_setCursorFlashTime for (i32) {
  fn setCursorFlashTime(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication18setCursorFlashTimeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QApplication18setCursorFlashTimeEi(arg0)};
    return 1;
  }
}

// proto: QWidget * QApplication::widgetAt(int x, int y);
impl<'a> /*trait*/ QApplication_widgetAt for (i32, i32) {
  fn widgetAt(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8widgetAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QApplication8widgetAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn alert<T: QApplication_alert>(&mut self, value: T) -> i32 {
    value.alert(self);
    return 1;
  }
}

pub trait QApplication_alert {
  fn alert(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::alert(QWidget * widget, int duration);
impl<'a> /*trait*/ QApplication_alert for (&'a mut QWidget, i32) {
  fn alert(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication5alertEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QApplication5alertEP7QWidgeti(arg0, arg1)};
    return 1;
  }
}

// proto: QPalette QApplication::palette(const QWidget * );
impl<'a> /*trait*/ QApplication_palette for (&'a  QWidget) {
  fn palette(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7paletteEPK7QWidget()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplication7paletteEPK7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn wheelScrollLines<T: QApplication_wheelScrollLines>(&mut self, value: T) -> i32 {
    value.wheelScrollLines(self);
    return 1;
  }
}

pub trait QApplication_wheelScrollLines {
  fn wheelScrollLines(self, this: &mut QApplication) -> i32;
}

// proto: int QApplication::wheelScrollLines();
impl<'a> /*trait*/ QApplication_wheelScrollLines for () {
  fn wheelScrollLines(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication16wheelScrollLinesEv()};
    unsafe {_ZN12QApplication16wheelScrollLinesEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn NewQApplication<T: QApplication_NewQApplication>(value: T) -> QApplication {
    let rsthis = value.NewQApplication();
    return rsthis;
    // return 1;
  }
}

pub trait QApplication_NewQApplication {
  fn NewQApplication(self) -> QApplication;
}

// proto: void QApplication::NewQApplication(const QApplication & );
impl<'a> /*trait*/ QApplication_NewQApplication for (&'a  QApplication) {
  fn NewQApplication(self) -> QApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplicationC1ERKS_(qthis, arg0)};
    let rsthis = QApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn aboutQt<T: QApplication_aboutQt>(&mut self, value: T) -> i32 {
    value.aboutQt(self);
    return 1;
  }
}

pub trait QApplication_aboutQt {
  fn aboutQt(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::aboutQt();
impl<'a> /*trait*/ QApplication_aboutQt for () {
  fn aboutQt(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7aboutQtEv()};
    unsafe {_ZN12QApplication7aboutQtEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn activeModalWidget<T: QApplication_activeModalWidget>(&mut self, value: T) -> i32 {
    value.activeModalWidget(self);
    return 1;
  }
}

pub trait QApplication_activeModalWidget {
  fn activeModalWidget(self, this: &mut QApplication) -> i32;
}

// proto: QWidget * QApplication::activeModalWidget();
impl<'a> /*trait*/ QApplication_activeModalWidget for () {
  fn activeModalWidget(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17activeModalWidgetEv()};
    unsafe {_ZN12QApplication17activeModalWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn activePopupWidget<T: QApplication_activePopupWidget>(&mut self, value: T) -> i32 {
    value.activePopupWidget(self);
    return 1;
  }
}

pub trait QApplication_activePopupWidget {
  fn activePopupWidget(self, this: &mut QApplication) -> i32;
}

// proto: QWidget * QApplication::activePopupWidget();
impl<'a> /*trait*/ QApplication_activePopupWidget for () {
  fn activePopupWidget(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17activePopupWidgetEv()};
    unsafe {_ZN12QApplication17activePopupWidgetEv()};
    return 1;
  }
}

// proto: void QApplication::NewQApplication(int & argc, char ** argv, int );
impl<'a> /*trait*/ QApplication_NewQApplication for (&'a mut i32, &'a mut String, i32) {
  fn NewQApplication(self) -> QApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationC1ERiPPci()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN12QApplicationC1ERiPPci(qthis, arg0, arg1, arg2)};
    let rsthis = QApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn focusChanged<T: QApplication_focusChanged>(&mut self, value: T) -> i32 {
    value.focusChanged(self);
    return 1;
  }
}

pub trait QApplication_focusChanged {
  fn focusChanged(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::focusChanged(QWidget * old, QWidget * now);
impl<'a> /*trait*/ QApplication_focusChanged for (&'a mut QWidget, &'a mut QWidget) {
  fn focusChanged(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12focusChangedEP7QWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QApplication12focusChangedEP7QWidgetS1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setStartDragTime<T: QApplication_setStartDragTime>(&mut self, value: T) -> i32 {
    value.setStartDragTime(self);
    return 1;
  }
}

pub trait QApplication_setStartDragTime {
  fn setStartDragTime(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setStartDragTime(int ms);
impl<'a> /*trait*/ QApplication_setStartDragTime for (i32) {
  fn setStartDragTime(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication16setStartDragTimeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QApplication16setStartDragTimeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn topLevelAt<T: QApplication_topLevelAt>(&mut self, value: T) -> i32 {
    value.topLevelAt(self);
    return 1;
  }
}

pub trait QApplication_topLevelAt {
  fn topLevelAt(self, this: &mut QApplication) -> i32;
}

// proto: QWidget * QApplication::topLevelAt(int x, int y);
impl<'a> /*trait*/ QApplication_topLevelAt for (i32, i32) {
  fn topLevelAt(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10topLevelAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QApplication10topLevelAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setStyle<T: QApplication_setStyle>(&mut self, value: T) -> i32 {
    value.setStyle(self);
    return 1;
  }
}

pub trait QApplication_setStyle {
  fn setStyle(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setStyle(QStyle * );
impl<'a> /*trait*/ QApplication_setStyle for (&'a mut QStyle) {
  fn setStyle(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QApplication8setStyleEP6QStyle(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn FreeQApplication<T: QApplication_FreeQApplication>(&mut self, value: T) -> i32 {
    value.FreeQApplication(self);
    return 1;
  }
}

pub trait QApplication_FreeQApplication {
  fn FreeQApplication(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::FreeQApplication();
impl<'a> /*trait*/ QApplication_FreeQApplication for () {
  fn FreeQApplication(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationD0Ev()};
    unsafe {_ZN12QApplicationD0Ev()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setDoubleClickInterval<T: QApplication_setDoubleClickInterval>(&mut self, value: T) -> i32 {
    value.setDoubleClickInterval(self);
    return 1;
  }
}

pub trait QApplication_setDoubleClickInterval {
  fn setDoubleClickInterval(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setDoubleClickInterval(int );
impl<'a> /*trait*/ QApplication_setDoubleClickInterval for (i32) {
  fn setDoubleClickInterval(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication22setDoubleClickIntervalEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QApplication22setDoubleClickIntervalEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setGlobalStrut<T: QApplication_setGlobalStrut>(&mut self, value: T) -> i32 {
    value.setGlobalStrut(self);
    return 1;
  }
}

pub trait QApplication_setGlobalStrut {
  fn setGlobalStrut(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setGlobalStrut(const QSize & );
impl<'a> /*trait*/ QApplication_setGlobalStrut for (&'a  QSize) {
  fn setGlobalStrut(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication14setGlobalStrutERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplication14setGlobalStrutERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setColorSpec<T: QApplication_setColorSpec>(&mut self, value: T) -> i32 {
    value.setColorSpec(self);
    return 1;
  }
}

pub trait QApplication_setColorSpec {
  fn setColorSpec(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setColorSpec(int );
impl<'a> /*trait*/ QApplication_setColorSpec for (i32) {
  fn setColorSpec(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12setColorSpecEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QApplication12setColorSpecEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn allWidgets<T: QApplication_allWidgets>(&mut self, value: T) -> i32 {
    value.allWidgets(self);
    return 1;
  }
}

pub trait QApplication_allWidgets {
  fn allWidgets(self, this: &mut QApplication) -> i32;
}

// proto: QList<QWidget *> QApplication::allWidgets();
impl<'a> /*trait*/ QApplication_allWidgets for () {
  fn allWidgets(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10allWidgetsEv()};
    unsafe {_ZN12QApplication10allWidgetsEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn globalStrut<T: QApplication_globalStrut>(&mut self, value: T) -> i32 {
    value.globalStrut(self);
    return 1;
  }
}

pub trait QApplication_globalStrut {
  fn globalStrut(self, this: &mut QApplication) -> i32;
}

// proto: QSize QApplication::globalStrut();
impl<'a> /*trait*/ QApplication_globalStrut for () {
  fn globalStrut(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11globalStrutEv()};
    unsafe {_ZN12QApplication11globalStrutEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setPalette<T: QApplication_setPalette>(&mut self, value: T) -> i32 {
    value.setPalette(self);
    return 1;
  }
}

pub trait QApplication_setPalette {
  fn setPalette(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setPalette(const QPalette & , const char * className);
impl<'a> /*trait*/ QApplication_setPalette for (&'a  QPalette, &'a  String) {
  fn setPalette(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10setPaletteERK8QPalettePKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN12QApplication10setPaletteERK8QPalettePKc(arg0, arg1)};
    return 1;
  }
}

// proto: QStyle * QApplication::setStyle(const QString & );
impl<'a> /*trait*/ QApplication_setStyle for (&'a  QString) {
  fn setStyle(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8setStyleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplication8setStyleERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn topLevelWidgets<T: QApplication_topLevelWidgets>(&mut self, value: T) -> i32 {
    value.topLevelWidgets(self);
    return 1;
  }
}

pub trait QApplication_topLevelWidgets {
  fn topLevelWidgets(self, this: &mut QApplication) -> i32;
}

// proto: QList<QWidget *> QApplication::topLevelWidgets();
impl<'a> /*trait*/ QApplication_topLevelWidgets for () {
  fn topLevelWidgets(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15topLevelWidgetsEv()};
    unsafe {_ZN12QApplication15topLevelWidgetsEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn exec<T: QApplication_exec>(&mut self, value: T) -> i32 {
    value.exec(self);
    return 1;
  }
}

pub trait QApplication_exec {
  fn exec(self, this: &mut QApplication) -> i32;
}

// proto: int QApplication::exec();
impl<'a> /*trait*/ QApplication_exec for () {
  fn exec(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4execEv()};
    unsafe {_ZN12QApplication4execEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn setWindowIcon<T: QApplication_setWindowIcon>(&mut self, value: T) -> i32 {
    value.setWindowIcon(self);
    return 1;
  }
}

pub trait QApplication_setWindowIcon {
  fn setWindowIcon(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QApplication_setWindowIcon for (&'a  QIcon) {
  fn setWindowIcon(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplication13setWindowIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn beep<T: QApplication_beep>(&mut self, value: T) -> i32 {
    value.beep(self);
    return 1;
  }
}

pub trait QApplication_beep {
  fn beep(self, this: &mut QApplication) -> i32;
}

// proto: void QApplication::beep();
impl<'a> /*trait*/ QApplication_beep for () {
  fn beep(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4beepEv()};
    unsafe {_ZN12QApplication4beepEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn notify<T: QApplication_notify>(&mut self, value: T) -> i32 {
    value.notify(self);
    return 1;
  }
}

pub trait QApplication_notify {
  fn notify(self, this: &mut QApplication) -> i32;
}

// proto: bool QApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QApplication_notify for (&'a mut QObject, &'a mut QEvent) {
  fn notify(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QApplication6notifyEP7QObjectP6QEvent(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn autoSipEnabled<T: QApplication_autoSipEnabled>(&mut self, value: T) -> i32 {
    value.autoSipEnabled(self);
    return 1;
  }
}

pub trait QApplication_autoSipEnabled {
  fn autoSipEnabled(self, this: &mut QApplication) -> i32;
}

// proto: bool QApplication::autoSipEnabled();
impl<'a> /*trait*/ QApplication_autoSipEnabled for () {
  fn autoSipEnabled(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication14autoSipEnabledEv()};
    unsafe {_ZNK12QApplication14autoSipEnabledEv()};
    return 1;
  }
}

// proto: QWidget * QApplication::topLevelAt(const QPoint & p);
impl<'a> /*trait*/ QApplication_topLevelAt for (&'a  QPoint) {
  fn topLevelAt(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10topLevelAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QApplication10topLevelAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn startDragTime<T: QApplication_startDragTime>(&mut self, value: T) -> i32 {
    value.startDragTime(self);
    return 1;
  }
}

pub trait QApplication_startDragTime {
  fn startDragTime(self, this: &mut QApplication) -> i32;
}

// proto: int QApplication::startDragTime();
impl<'a> /*trait*/ QApplication_startDragTime for () {
  fn startDragTime(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13startDragTimeEv()};
    unsafe {_ZN12QApplication13startDragTimeEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn doubleClickInterval<T: QApplication_doubleClickInterval>(&mut self, value: T) -> i32 {
    value.doubleClickInterval(self);
    return 1;
  }
}

pub trait QApplication_doubleClickInterval {
  fn doubleClickInterval(self, this: &mut QApplication) -> i32;
}

// proto: int QApplication::doubleClickInterval();
impl<'a> /*trait*/ QApplication_doubleClickInterval for () {
  fn doubleClickInterval(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication19doubleClickIntervalEv()};
    unsafe {_ZN12QApplication19doubleClickIntervalEv()};
    return 1;
  }
}

impl /*struct*/ QApplication {
  pub fn windowIcon<T: QApplication_windowIcon>(&mut self, value: T) -> i32 {
    value.windowIcon(self);
    return 1;
  }
}

pub trait QApplication_windowIcon {
  fn windowIcon(self, this: &mut QApplication) -> i32;
}

// proto: QIcon QApplication::windowIcon();
impl<'a> /*trait*/ QApplication_windowIcon for () {
  fn windowIcon(self, this: &mut QApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10windowIconEv()};
    unsafe {_ZN12QApplication10windowIconEv()};
    return 1;
  }
}

