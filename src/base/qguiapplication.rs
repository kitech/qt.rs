// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qstring::QString;
use super::qpalette::QPalette;
use super::qinputmethod::QInputMethod;
use super::qscreen::QScreen;
use super::qcursor::QCursor;
use super::qicon::QIcon;
use super::qstylehints::QStyleHints;
use super::qclipboard::QClipboard;
use super::qobject::QObject;
use super::qevent::QEvent;
use super::qsessionmanager::QSessionManager;
use super::qwindow::QWindow;
use super::qpoint::QPoint;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGuiApplication::FreeQGuiApplication();
  fn _ZN15QGuiApplicationD0Ev(qthis: *mut c_void) ;
  // proto: static void QGuiApplication::setFont(const QFont & );
  fn _ZN15QGuiApplication7setFontERK5QFont(arg0: *mut c_void) ;
  // proto: static QString QGuiApplication::platformName();
  fn _ZN15QGuiApplication12platformNameEv() -> *mut c_void;
  // proto: static QList<QScreen *> QGuiApplication::screens();
  fn _ZN15QGuiApplication7screensEv() ;
  // proto: static void QGuiApplication::setPalette(const QPalette & pal);
  fn _ZN15QGuiApplication10setPaletteERK8QPalette(arg0: *mut c_void) ;
  // proto: static QInputMethod * QGuiApplication::inputMethod();
  fn _ZN15QGuiApplication11inputMethodEv() -> *mut c_void;
  // proto:  void QGuiApplication::NewQGuiApplication(const QGuiApplication & );
  fn _ZN15QGuiApplicationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGuiApplication::isSavingSession();
  fn _ZNK15QGuiApplication15isSavingSessionEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGuiApplication::paletteChanged(const QPalette & pal);
  fn _ZN15QGuiApplication14paletteChangedERK8QPalette(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QFont QGuiApplication::font();
  fn _ZN15QGuiApplication4fontEv() -> *mut c_void;
  // proto:  void QGuiApplication::screenAdded(QScreen * screen);
  fn _ZN15QGuiApplication11screenAddedEP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGuiApplication::isSessionRestored();
  fn _ZNK15QGuiApplication17isSessionRestoredEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QGuiApplication::sessionKey();
  fn _ZNK15QGuiApplication10sessionKeyEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static bool QGuiApplication::desktopSettingsAware();
  fn _ZN15QGuiApplication20desktopSettingsAwareEv() -> int8_t;
  // proto: static void QGuiApplication::sync();
  fn _ZN15QGuiApplication4syncEv() ;
  // proto: static void QGuiApplication::setQuitOnLastWindowClosed(bool quit);
  fn _ZN15QGuiApplication25setQuitOnLastWindowClosedEb(arg0: int8_t) ;
  // proto: static QScreen * QGuiApplication::primaryScreen();
  fn _ZN15QGuiApplication13primaryScreenEv() -> *mut c_void;
  // proto: static QCursor * QGuiApplication::overrideCursor();
  fn _ZN15QGuiApplication14overrideCursorEv() -> *mut c_void;
  // proto: static QIcon QGuiApplication::windowIcon();
  fn _ZN15QGuiApplication10windowIconEv() -> *mut c_void;
  // proto: static QStyleHints * QGuiApplication::styleHints();
  fn _ZN15QGuiApplication10styleHintsEv() -> *mut c_void;
  // proto: static QClipboard * QGuiApplication::clipboard();
  fn _ZN15QGuiApplication9clipboardEv() -> *mut c_void;
  // proto: static QPalette QGuiApplication::palette();
  fn _ZN15QGuiApplication7paletteEv() -> *mut c_void;
  // proto:  bool QGuiApplication::notify(QObject * , QEvent * );
  fn _ZN15QGuiApplication6notifyEP7QObjectP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto: static QList<QWindow *> QGuiApplication::topLevelWindows();
  fn _ZN15QGuiApplication15topLevelWindowsEv() ;
  // proto: static bool QGuiApplication::isRightToLeft();
  fn _ZN15QGuiApplication13isRightToLeftEv() -> int8_t;
  // proto:  void QGuiApplication::focusObjectChanged(QObject * focusObject);
  fn _ZN15QGuiApplication18focusObjectChangedEP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGuiApplication::fontDatabaseChanged();
  fn _ZN15QGuiApplication19fontDatabaseChangedEv(qthis: *mut c_void) ;
  // proto: static void QGuiApplication::changeOverrideCursor(const QCursor & );
  fn _ZN15QGuiApplication20changeOverrideCursorERK7QCursor(arg0: *mut c_void) ;
  // proto: static QList<QWindow *> QGuiApplication::allWindows();
  fn _ZN15QGuiApplication10allWindowsEv() ;
  // proto: static void QGuiApplication::setOverrideCursor(const QCursor & );
  fn _ZN15QGuiApplication17setOverrideCursorERK7QCursor(arg0: *mut c_void) ;
  // proto:  void QGuiApplication::commitDataRequest(QSessionManager & sessionManager);
  fn _ZN15QGuiApplication17commitDataRequestER15QSessionManager(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QGuiApplication::setWindowIcon(const QIcon & icon);
  fn _ZN15QGuiApplication13setWindowIconERK5QIcon(arg0: *mut c_void) ;
  // proto:  QString QGuiApplication::sessionId();
  fn _ZNK15QGuiApplication9sessionIdEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGuiApplication::focusWindowChanged(QWindow * focusWindow);
  fn _ZN15QGuiApplication18focusWindowChangedEP7QWindow(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QGuiApplication::setApplicationDisplayName(const QString & name);
  fn _ZN15QGuiApplication25setApplicationDisplayNameERK7QString(arg0: *mut c_void) ;
  // proto: static bool QGuiApplication::isLeftToRight();
  fn _ZN15QGuiApplication13isLeftToRightEv() -> int8_t;
  // proto: static QWindow * QGuiApplication::topLevelAt(const QPoint & pos);
  fn _ZN15QGuiApplication10topLevelAtERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGuiApplication::NewQGuiApplication(int & argc, char ** argv, int );
  fn _ZN15QGuiApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) ;
  // proto: static void QGuiApplication::setDesktopSettingsAware(bool on);
  fn _ZN15QGuiApplication23setDesktopSettingsAwareEb(arg0: int8_t) ;
  // proto: static QWindow * QGuiApplication::modalWindow();
  fn _ZN15QGuiApplication11modalWindowEv() -> *mut c_void;
  // proto: static QString QGuiApplication::applicationDisplayName();
  fn _ZN15QGuiApplication22applicationDisplayNameEv() -> *mut c_void;
  // proto: static int QGuiApplication::exec();
  fn _ZN15QGuiApplication4execEv() -> c_int;
  // proto: static bool QGuiApplication::quitOnLastWindowClosed();
  fn _ZN15QGuiApplication22quitOnLastWindowClosedEv() -> int8_t;
  // proto:  void QGuiApplication::lastWindowClosed();
  fn _ZN15QGuiApplication16lastWindowClosedEv(qthis: *mut c_void) ;
  // proto: static void QGuiApplication::restoreOverrideCursor();
  fn _ZN15QGuiApplication21restoreOverrideCursorEv() ;
  // proto: static QPlatformNativeInterface * QGuiApplication::platformNativeInterface();
  fn _ZN15QGuiApplication23platformNativeInterfaceEv() ;
  // proto:  const QMetaObject * QGuiApplication::metaObject();
  fn _ZNK15QGuiApplication10metaObjectEv(qthis: *mut c_void) ;
  // proto: static QObject * QGuiApplication::focusObject();
  fn _ZN15QGuiApplication11focusObjectEv() -> *mut c_void;
  // proto:  void QGuiApplication::screenRemoved(QScreen * screen);
  fn _ZN15QGuiApplication13screenRemovedEP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QWindow * QGuiApplication::focusWindow();
  fn _ZN15QGuiApplication11focusWindowEv() -> *mut c_void;
  // proto:  void QGuiApplication::saveStateRequest(QSessionManager & sessionManager);
  fn _ZN15QGuiApplication16saveStateRequestER15QSessionManager(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGuiApplication::devicePixelRatio();
  fn _ZNK15QGuiApplication16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto: static QFunctionPointer QGuiApplication::platformFunction(const QByteArray & function);
  fn _ZN15QGuiApplication16platformFunctionERK10QByteArray(arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGuiApplication)=1
pub struct QGuiApplication {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGuiApplication {
  pub fn FreeQGuiApplication<T: QGuiApplication_FreeQGuiApplication>(&mut self, value: T)  {
     value.FreeQGuiApplication(self);
    // return 1;
  }
}

pub trait QGuiApplication_FreeQGuiApplication {
  fn FreeQGuiApplication(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::FreeQGuiApplication();
impl<'a> /*trait*/ QGuiApplication_FreeQGuiApplication for () {
  fn FreeQGuiApplication(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplicationD0Ev()};
     unsafe {_ZN15QGuiApplicationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setFont<T: QGuiApplication_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QGuiApplication_setFont {
  fn setFont(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::setFont(const QFont & );
impl<'a> /*trait*/ QGuiApplication_setFont for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication7setFontERK5QFont(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformName<T: QGuiApplication_platformName>(&mut self, value: T) -> QString {
    return value.platformName(self);
    // return 1;
  }
}

pub trait QGuiApplication_platformName {
  fn platformName(self, rsthis: &mut QGuiApplication) -> QString;
}

// proto: static QString QGuiApplication::platformName();
impl<'a> /*trait*/ QGuiApplication_platformName for () {
  fn platformName(self, rsthis: &mut QGuiApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication12platformNameEv()};
    let mut ret = unsafe {_ZN15QGuiApplication12platformNameEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn screens<T: QGuiApplication_screens>(&mut self, value: T)  {
     value.screens(self);
    // return 1;
  }
}

pub trait QGuiApplication_screens {
  fn screens(self, rsthis: &mut QGuiApplication) ;
}

// proto: static QList<QScreen *> QGuiApplication::screens();
impl<'a> /*trait*/ QGuiApplication_screens for () {
  fn screens(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7screensEv()};
     unsafe {_ZN15QGuiApplication7screensEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setPalette<T: QGuiApplication_setPalette>(&mut self, value: T)  {
     value.setPalette(self);
    // return 1;
  }
}

pub trait QGuiApplication_setPalette {
  fn setPalette(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::setPalette(const QPalette & pal);
impl<'a> /*trait*/ QGuiApplication_setPalette for (&'a  QPalette) {
  fn setPalette(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication10setPaletteERK8QPalette(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn inputMethod<T: QGuiApplication_inputMethod>(&mut self, value: T) -> QInputMethod {
    return value.inputMethod(self);
    // return 1;
  }
}

pub trait QGuiApplication_inputMethod {
  fn inputMethod(self, rsthis: &mut QGuiApplication) -> QInputMethod;
}

// proto: static QInputMethod * QGuiApplication::inputMethod();
impl<'a> /*trait*/ QGuiApplication_inputMethod for () {
  fn inputMethod(self, rsthis: &mut QGuiApplication) -> QInputMethod {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11inputMethodEv()};
    let mut ret = unsafe {_ZN15QGuiApplication11inputMethodEv()};
    let mut ret1 = QInputMethod{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn NewQGuiApplication<T: QGuiApplication_NewQGuiApplication>(value: T) -> QGuiApplication {
    let rsthis = value.NewQGuiApplication();
    return rsthis;
    // return 1;
  }
}

pub trait QGuiApplication_NewQGuiApplication {
  fn NewQGuiApplication(self) -> QGuiApplication;
}

// proto: void QGuiApplication::NewQGuiApplication(const QGuiApplication & );
impl<'a> /*trait*/ QGuiApplication_NewQGuiApplication for (&'a  QGuiApplication) {
  fn NewQGuiApplication(self) -> QGuiApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplicationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGuiApplicationC1ERKS_(qthis, arg0)};
    let rsthis = QGuiApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isSavingSession<T: QGuiApplication_isSavingSession>(&mut self, value: T) -> i8 {
    return value.isSavingSession(self);
    // return 1;
  }
}

pub trait QGuiApplication_isSavingSession {
  fn isSavingSession(self, rsthis: &mut QGuiApplication) -> i8;
}

// proto:  bool QGuiApplication::isSavingSession();
impl<'a> /*trait*/ QGuiApplication_isSavingSession for () {
  fn isSavingSession(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication15isSavingSessionEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication15isSavingSessionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn paletteChanged<T: QGuiApplication_paletteChanged>(&mut self, value: T)  {
     value.paletteChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_paletteChanged {
  fn paletteChanged(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::paletteChanged(const QPalette & pal);
impl<'a> /*trait*/ QGuiApplication_paletteChanged for (&'a  QPalette) {
  fn paletteChanged(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication14paletteChangedERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication14paletteChangedERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn font<T: QGuiApplication_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QGuiApplication_font {
  fn font(self, rsthis: &mut QGuiApplication) -> QFont;
}

// proto: static QFont QGuiApplication::font();
impl<'a> /*trait*/ QGuiApplication_font for () {
  fn font(self, rsthis: &mut QGuiApplication) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4fontEv()};
    let mut ret = unsafe {_ZN15QGuiApplication4fontEv()};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn screenAdded<T: QGuiApplication_screenAdded>(&mut self, value: T)  {
     value.screenAdded(self);
    // return 1;
  }
}

pub trait QGuiApplication_screenAdded {
  fn screenAdded(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::screenAdded(QScreen * screen);
impl<'a> /*trait*/ QGuiApplication_screenAdded for (&'a mut QScreen) {
  fn screenAdded(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11screenAddedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication11screenAddedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isSessionRestored<T: QGuiApplication_isSessionRestored>(&mut self, value: T) -> i8 {
    return value.isSessionRestored(self);
    // return 1;
  }
}

pub trait QGuiApplication_isSessionRestored {
  fn isSessionRestored(self, rsthis: &mut QGuiApplication) -> i8;
}

// proto:  bool QGuiApplication::isSessionRestored();
impl<'a> /*trait*/ QGuiApplication_isSessionRestored for () {
  fn isSessionRestored(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication17isSessionRestoredEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication17isSessionRestoredEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sessionKey<T: QGuiApplication_sessionKey>(&mut self, value: T) -> QString {
    return value.sessionKey(self);
    // return 1;
  }
}

pub trait QGuiApplication_sessionKey {
  fn sessionKey(self, rsthis: &mut QGuiApplication) -> QString;
}

// proto:  QString QGuiApplication::sessionKey();
impl<'a> /*trait*/ QGuiApplication_sessionKey for () {
  fn sessionKey(self, rsthis: &mut QGuiApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication10sessionKeyEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication10sessionKeyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn desktopSettingsAware<T: QGuiApplication_desktopSettingsAware>(&mut self, value: T) -> i8 {
    return value.desktopSettingsAware(self);
    // return 1;
  }
}

pub trait QGuiApplication_desktopSettingsAware {
  fn desktopSettingsAware(self, rsthis: &mut QGuiApplication) -> i8;
}

// proto: static bool QGuiApplication::desktopSettingsAware();
impl<'a> /*trait*/ QGuiApplication_desktopSettingsAware for () {
  fn desktopSettingsAware(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication20desktopSettingsAwareEv()};
    let mut ret = unsafe {_ZN15QGuiApplication20desktopSettingsAwareEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sync<T: QGuiApplication_sync>(&mut self, value: T)  {
     value.sync(self);
    // return 1;
  }
}

pub trait QGuiApplication_sync {
  fn sync(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::sync();
impl<'a> /*trait*/ QGuiApplication_sync for () {
  fn sync(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4syncEv()};
     unsafe {_ZN15QGuiApplication4syncEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setQuitOnLastWindowClosed<T: QGuiApplication_setQuitOnLastWindowClosed>(&mut self, value: T)  {
     value.setQuitOnLastWindowClosed(self);
    // return 1;
  }
}

pub trait QGuiApplication_setQuitOnLastWindowClosed {
  fn setQuitOnLastWindowClosed(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::setQuitOnLastWindowClosed(bool quit);
impl<'a> /*trait*/ QGuiApplication_setQuitOnLastWindowClosed for (i8) {
  fn setQuitOnLastWindowClosed(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication25setQuitOnLastWindowClosedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QGuiApplication25setQuitOnLastWindowClosedEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn primaryScreen<T: QGuiApplication_primaryScreen>(&mut self, value: T) -> QScreen {
    return value.primaryScreen(self);
    // return 1;
  }
}

pub trait QGuiApplication_primaryScreen {
  fn primaryScreen(self, rsthis: &mut QGuiApplication) -> QScreen;
}

// proto: static QScreen * QGuiApplication::primaryScreen();
impl<'a> /*trait*/ QGuiApplication_primaryScreen for () {
  fn primaryScreen(self, rsthis: &mut QGuiApplication) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13primaryScreenEv()};
    let mut ret = unsafe {_ZN15QGuiApplication13primaryScreenEv()};
    let mut ret1 = QScreen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn overrideCursor<T: QGuiApplication_overrideCursor>(&mut self, value: T) -> QCursor {
    return value.overrideCursor(self);
    // return 1;
  }
}

pub trait QGuiApplication_overrideCursor {
  fn overrideCursor(self, rsthis: &mut QGuiApplication) -> QCursor;
}

// proto: static QCursor * QGuiApplication::overrideCursor();
impl<'a> /*trait*/ QGuiApplication_overrideCursor for () {
  fn overrideCursor(self, rsthis: &mut QGuiApplication) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication14overrideCursorEv()};
    let mut ret = unsafe {_ZN15QGuiApplication14overrideCursorEv()};
    let mut ret1 = QCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn windowIcon<T: QGuiApplication_windowIcon>(&mut self, value: T) -> QIcon {
    return value.windowIcon(self);
    // return 1;
  }
}

pub trait QGuiApplication_windowIcon {
  fn windowIcon(self, rsthis: &mut QGuiApplication) -> QIcon;
}

// proto: static QIcon QGuiApplication::windowIcon();
impl<'a> /*trait*/ QGuiApplication_windowIcon for () {
  fn windowIcon(self, rsthis: &mut QGuiApplication) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10windowIconEv()};
    let mut ret = unsafe {_ZN15QGuiApplication10windowIconEv()};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn styleHints<T: QGuiApplication_styleHints>(&mut self, value: T) -> QStyleHints {
    return value.styleHints(self);
    // return 1;
  }
}

pub trait QGuiApplication_styleHints {
  fn styleHints(self, rsthis: &mut QGuiApplication) -> QStyleHints;
}

// proto: static QStyleHints * QGuiApplication::styleHints();
impl<'a> /*trait*/ QGuiApplication_styleHints for () {
  fn styleHints(self, rsthis: &mut QGuiApplication) -> QStyleHints {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10styleHintsEv()};
    let mut ret = unsafe {_ZN15QGuiApplication10styleHintsEv()};
    let mut ret1 = QStyleHints{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn clipboard<T: QGuiApplication_clipboard>(&mut self, value: T) -> QClipboard {
    return value.clipboard(self);
    // return 1;
  }
}

pub trait QGuiApplication_clipboard {
  fn clipboard(self, rsthis: &mut QGuiApplication) -> QClipboard;
}

// proto: static QClipboard * QGuiApplication::clipboard();
impl<'a> /*trait*/ QGuiApplication_clipboard for () {
  fn clipboard(self, rsthis: &mut QGuiApplication) -> QClipboard {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication9clipboardEv()};
    let mut ret = unsafe {_ZN15QGuiApplication9clipboardEv()};
    let mut ret1 = QClipboard{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn palette<T: QGuiApplication_palette>(&mut self, value: T) -> QPalette {
    return value.palette(self);
    // return 1;
  }
}

pub trait QGuiApplication_palette {
  fn palette(self, rsthis: &mut QGuiApplication) -> QPalette;
}

// proto: static QPalette QGuiApplication::palette();
impl<'a> /*trait*/ QGuiApplication_palette for () {
  fn palette(self, rsthis: &mut QGuiApplication) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7paletteEv()};
    let mut ret = unsafe {_ZN15QGuiApplication7paletteEv()};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn notify<T: QGuiApplication_notify>(&mut self, value: T) -> i8 {
    return value.notify(self);
    // return 1;
  }
}

pub trait QGuiApplication_notify {
  fn notify(self, rsthis: &mut QGuiApplication) -> i8;
}

// proto:  bool QGuiApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QGuiApplication_notify for (&'a mut QObject, &'a mut QEvent) {
  fn notify(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QGuiApplication6notifyEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn topLevelWindows<T: QGuiApplication_topLevelWindows>(&mut self, value: T)  {
     value.topLevelWindows(self);
    // return 1;
  }
}

pub trait QGuiApplication_topLevelWindows {
  fn topLevelWindows(self, rsthis: &mut QGuiApplication) ;
}

// proto: static QList<QWindow *> QGuiApplication::topLevelWindows();
impl<'a> /*trait*/ QGuiApplication_topLevelWindows for () {
  fn topLevelWindows(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication15topLevelWindowsEv()};
     unsafe {_ZN15QGuiApplication15topLevelWindowsEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isRightToLeft<T: QGuiApplication_isRightToLeft>(&mut self, value: T) -> i8 {
    return value.isRightToLeft(self);
    // return 1;
  }
}

pub trait QGuiApplication_isRightToLeft {
  fn isRightToLeft(self, rsthis: &mut QGuiApplication) -> i8;
}

// proto: static bool QGuiApplication::isRightToLeft();
impl<'a> /*trait*/ QGuiApplication_isRightToLeft for () {
  fn isRightToLeft(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13isRightToLeftEv()};
    let mut ret = unsafe {_ZN15QGuiApplication13isRightToLeftEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusObjectChanged<T: QGuiApplication_focusObjectChanged>(&mut self, value: T)  {
     value.focusObjectChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusObjectChanged {
  fn focusObjectChanged(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::focusObjectChanged(QObject * focusObject);
impl<'a> /*trait*/ QGuiApplication_focusObjectChanged for (&'a mut QObject) {
  fn focusObjectChanged(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication18focusObjectChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication18focusObjectChangedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn fontDatabaseChanged<T: QGuiApplication_fontDatabaseChanged>(&mut self, value: T)  {
     value.fontDatabaseChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_fontDatabaseChanged {
  fn fontDatabaseChanged(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::fontDatabaseChanged();
impl<'a> /*trait*/ QGuiApplication_fontDatabaseChanged for () {
  fn fontDatabaseChanged(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication19fontDatabaseChangedEv()};
     unsafe {_ZN15QGuiApplication19fontDatabaseChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn changeOverrideCursor<T: QGuiApplication_changeOverrideCursor>(&mut self, value: T)  {
     value.changeOverrideCursor(self);
    // return 1;
  }
}

pub trait QGuiApplication_changeOverrideCursor {
  fn changeOverrideCursor(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::changeOverrideCursor(const QCursor & );
impl<'a> /*trait*/ QGuiApplication_changeOverrideCursor for (&'a  QCursor) {
  fn changeOverrideCursor(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication20changeOverrideCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication20changeOverrideCursorERK7QCursor(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn allWindows<T: QGuiApplication_allWindows>(&mut self, value: T)  {
     value.allWindows(self);
    // return 1;
  }
}

pub trait QGuiApplication_allWindows {
  fn allWindows(self, rsthis: &mut QGuiApplication) ;
}

// proto: static QList<QWindow *> QGuiApplication::allWindows();
impl<'a> /*trait*/ QGuiApplication_allWindows for () {
  fn allWindows(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10allWindowsEv()};
     unsafe {_ZN15QGuiApplication10allWindowsEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setOverrideCursor<T: QGuiApplication_setOverrideCursor>(&mut self, value: T)  {
     value.setOverrideCursor(self);
    // return 1;
  }
}

pub trait QGuiApplication_setOverrideCursor {
  fn setOverrideCursor(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::setOverrideCursor(const QCursor & );
impl<'a> /*trait*/ QGuiApplication_setOverrideCursor for (&'a  QCursor) {
  fn setOverrideCursor(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication17setOverrideCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication17setOverrideCursorERK7QCursor(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn commitDataRequest<T: QGuiApplication_commitDataRequest>(&mut self, value: T)  {
     value.commitDataRequest(self);
    // return 1;
  }
}

pub trait QGuiApplication_commitDataRequest {
  fn commitDataRequest(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::commitDataRequest(QSessionManager & sessionManager);
impl<'a> /*trait*/ QGuiApplication_commitDataRequest for (&'a mut QSessionManager) {
  fn commitDataRequest(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication17commitDataRequestER15QSessionManager()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication17commitDataRequestER15QSessionManager(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setWindowIcon<T: QGuiApplication_setWindowIcon>(&mut self, value: T)  {
     value.setWindowIcon(self);
    // return 1;
  }
}

pub trait QGuiApplication_setWindowIcon {
  fn setWindowIcon(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QGuiApplication_setWindowIcon for (&'a  QIcon) {
  fn setWindowIcon(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication13setWindowIconERK5QIcon(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sessionId<T: QGuiApplication_sessionId>(&mut self, value: T) -> QString {
    return value.sessionId(self);
    // return 1;
  }
}

pub trait QGuiApplication_sessionId {
  fn sessionId(self, rsthis: &mut QGuiApplication) -> QString;
}

// proto:  QString QGuiApplication::sessionId();
impl<'a> /*trait*/ QGuiApplication_sessionId for () {
  fn sessionId(self, rsthis: &mut QGuiApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication9sessionIdEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication9sessionIdEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusWindowChanged<T: QGuiApplication_focusWindowChanged>(&mut self, value: T)  {
     value.focusWindowChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusWindowChanged {
  fn focusWindowChanged(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::focusWindowChanged(QWindow * focusWindow);
impl<'a> /*trait*/ QGuiApplication_focusWindowChanged for (&'a mut QWindow) {
  fn focusWindowChanged(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication18focusWindowChangedEP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication18focusWindowChangedEP7QWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setApplicationDisplayName<T: QGuiApplication_setApplicationDisplayName>(&mut self, value: T)  {
     value.setApplicationDisplayName(self);
    // return 1;
  }
}

pub trait QGuiApplication_setApplicationDisplayName {
  fn setApplicationDisplayName(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::setApplicationDisplayName(const QString & name);
impl<'a> /*trait*/ QGuiApplication_setApplicationDisplayName for (&'a  QString) {
  fn setApplicationDisplayName(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication25setApplicationDisplayNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication25setApplicationDisplayNameERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isLeftToRight<T: QGuiApplication_isLeftToRight>(&mut self, value: T) -> i8 {
    return value.isLeftToRight(self);
    // return 1;
  }
}

pub trait QGuiApplication_isLeftToRight {
  fn isLeftToRight(self, rsthis: &mut QGuiApplication) -> i8;
}

// proto: static bool QGuiApplication::isLeftToRight();
impl<'a> /*trait*/ QGuiApplication_isLeftToRight for () {
  fn isLeftToRight(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13isLeftToRightEv()};
    let mut ret = unsafe {_ZN15QGuiApplication13isLeftToRightEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn topLevelAt<T: QGuiApplication_topLevelAt>(&mut self, value: T) -> QWindow {
    return value.topLevelAt(self);
    // return 1;
  }
}

pub trait QGuiApplication_topLevelAt {
  fn topLevelAt(self, rsthis: &mut QGuiApplication) -> QWindow;
}

// proto: static QWindow * QGuiApplication::topLevelAt(const QPoint & pos);
impl<'a> /*trait*/ QGuiApplication_topLevelAt for (&'a  QPoint) {
  fn topLevelAt(self, rsthis: &mut QGuiApplication) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10topLevelAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QGuiApplication10topLevelAtERK6QPoint(arg0)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QGuiApplication::NewQGuiApplication(int & argc, char ** argv, int );
impl<'a> /*trait*/ QGuiApplication_NewQGuiApplication for (&'a mut i32, &'a mut String, i32) {
  fn NewQGuiApplication(self) -> QGuiApplication {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplicationC1ERiPPci()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QGuiApplicationC1ERiPPci(qthis, arg0, arg1, arg2)};
    let rsthis = QGuiApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setDesktopSettingsAware<T: QGuiApplication_setDesktopSettingsAware>(&mut self, value: T)  {
     value.setDesktopSettingsAware(self);
    // return 1;
  }
}

pub trait QGuiApplication_setDesktopSettingsAware {
  fn setDesktopSettingsAware(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::setDesktopSettingsAware(bool on);
impl<'a> /*trait*/ QGuiApplication_setDesktopSettingsAware for (i8) {
  fn setDesktopSettingsAware(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication23setDesktopSettingsAwareEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QGuiApplication23setDesktopSettingsAwareEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn modalWindow<T: QGuiApplication_modalWindow>(&mut self, value: T) -> QWindow {
    return value.modalWindow(self);
    // return 1;
  }
}

pub trait QGuiApplication_modalWindow {
  fn modalWindow(self, rsthis: &mut QGuiApplication) -> QWindow;
}

// proto: static QWindow * QGuiApplication::modalWindow();
impl<'a> /*trait*/ QGuiApplication_modalWindow for () {
  fn modalWindow(self, rsthis: &mut QGuiApplication) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11modalWindowEv()};
    let mut ret = unsafe {_ZN15QGuiApplication11modalWindowEv()};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn applicationDisplayName<T: QGuiApplication_applicationDisplayName>(&mut self, value: T) -> QString {
    return value.applicationDisplayName(self);
    // return 1;
  }
}

pub trait QGuiApplication_applicationDisplayName {
  fn applicationDisplayName(self, rsthis: &mut QGuiApplication) -> QString;
}

// proto: static QString QGuiApplication::applicationDisplayName();
impl<'a> /*trait*/ QGuiApplication_applicationDisplayName for () {
  fn applicationDisplayName(self, rsthis: &mut QGuiApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication22applicationDisplayNameEv()};
    let mut ret = unsafe {_ZN15QGuiApplication22applicationDisplayNameEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn exec<T: QGuiApplication_exec>(&mut self, value: T) -> i32 {
    return value.exec(self);
    // return 1;
  }
}

pub trait QGuiApplication_exec {
  fn exec(self, rsthis: &mut QGuiApplication) -> i32;
}

// proto: static int QGuiApplication::exec();
impl<'a> /*trait*/ QGuiApplication_exec for () {
  fn exec(self, rsthis: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4execEv()};
    let mut ret = unsafe {_ZN15QGuiApplication4execEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn quitOnLastWindowClosed<T: QGuiApplication_quitOnLastWindowClosed>(&mut self, value: T) -> i8 {
    return value.quitOnLastWindowClosed(self);
    // return 1;
  }
}

pub trait QGuiApplication_quitOnLastWindowClosed {
  fn quitOnLastWindowClosed(self, rsthis: &mut QGuiApplication) -> i8;
}

// proto: static bool QGuiApplication::quitOnLastWindowClosed();
impl<'a> /*trait*/ QGuiApplication_quitOnLastWindowClosed for () {
  fn quitOnLastWindowClosed(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication22quitOnLastWindowClosedEv()};
    let mut ret = unsafe {_ZN15QGuiApplication22quitOnLastWindowClosedEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn lastWindowClosed<T: QGuiApplication_lastWindowClosed>(&mut self, value: T)  {
     value.lastWindowClosed(self);
    // return 1;
  }
}

pub trait QGuiApplication_lastWindowClosed {
  fn lastWindowClosed(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::lastWindowClosed();
impl<'a> /*trait*/ QGuiApplication_lastWindowClosed for () {
  fn lastWindowClosed(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16lastWindowClosedEv()};
     unsafe {_ZN15QGuiApplication16lastWindowClosedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn restoreOverrideCursor<T: QGuiApplication_restoreOverrideCursor>(&mut self, value: T)  {
     value.restoreOverrideCursor(self);
    // return 1;
  }
}

pub trait QGuiApplication_restoreOverrideCursor {
  fn restoreOverrideCursor(self, rsthis: &mut QGuiApplication) ;
}

// proto: static void QGuiApplication::restoreOverrideCursor();
impl<'a> /*trait*/ QGuiApplication_restoreOverrideCursor for () {
  fn restoreOverrideCursor(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication21restoreOverrideCursorEv()};
     unsafe {_ZN15QGuiApplication21restoreOverrideCursorEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformNativeInterface<T: QGuiApplication_platformNativeInterface>(&mut self, value: T)  {
     value.platformNativeInterface(self);
    // return 1;
  }
}

pub trait QGuiApplication_platformNativeInterface {
  fn platformNativeInterface(self, rsthis: &mut QGuiApplication) ;
}

// proto: static QPlatformNativeInterface * QGuiApplication::platformNativeInterface();
impl<'a> /*trait*/ QGuiApplication_platformNativeInterface for () {
  fn platformNativeInterface(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication23platformNativeInterfaceEv()};
     unsafe {_ZN15QGuiApplication23platformNativeInterfaceEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn metaObject<T: QGuiApplication_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGuiApplication_metaObject {
  fn metaObject(self, rsthis: &mut QGuiApplication) ;
}

// proto:  const QMetaObject * QGuiApplication::metaObject();
impl<'a> /*trait*/ QGuiApplication_metaObject for () {
  fn metaObject(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication10metaObjectEv()};
     unsafe {_ZNK15QGuiApplication10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusObject<T: QGuiApplication_focusObject>(&mut self, value: T) -> QObject {
    return value.focusObject(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusObject {
  fn focusObject(self, rsthis: &mut QGuiApplication) -> QObject;
}

// proto: static QObject * QGuiApplication::focusObject();
impl<'a> /*trait*/ QGuiApplication_focusObject for () {
  fn focusObject(self, rsthis: &mut QGuiApplication) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11focusObjectEv()};
    let mut ret = unsafe {_ZN15QGuiApplication11focusObjectEv()};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn screenRemoved<T: QGuiApplication_screenRemoved>(&mut self, value: T)  {
     value.screenRemoved(self);
    // return 1;
  }
}

pub trait QGuiApplication_screenRemoved {
  fn screenRemoved(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::screenRemoved(QScreen * screen);
impl<'a> /*trait*/ QGuiApplication_screenRemoved for (&'a mut QScreen) {
  fn screenRemoved(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13screenRemovedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication13screenRemovedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusWindow<T: QGuiApplication_focusWindow>(&mut self, value: T) -> QWindow {
    return value.focusWindow(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusWindow {
  fn focusWindow(self, rsthis: &mut QGuiApplication) -> QWindow;
}

// proto: static QWindow * QGuiApplication::focusWindow();
impl<'a> /*trait*/ QGuiApplication_focusWindow for () {
  fn focusWindow(self, rsthis: &mut QGuiApplication) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11focusWindowEv()};
    let mut ret = unsafe {_ZN15QGuiApplication11focusWindowEv()};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn saveStateRequest<T: QGuiApplication_saveStateRequest>(&mut self, value: T)  {
     value.saveStateRequest(self);
    // return 1;
  }
}

pub trait QGuiApplication_saveStateRequest {
  fn saveStateRequest(self, rsthis: &mut QGuiApplication) ;
}

// proto:  void QGuiApplication::saveStateRequest(QSessionManager & sessionManager);
impl<'a> /*trait*/ QGuiApplication_saveStateRequest for (&'a mut QSessionManager) {
  fn saveStateRequest(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16saveStateRequestER15QSessionManager()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication16saveStateRequestER15QSessionManager(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn devicePixelRatio<T: QGuiApplication_devicePixelRatio>(&mut self, value: T) -> f64 {
    return value.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QGuiApplication_devicePixelRatio {
  fn devicePixelRatio(self, rsthis: &mut QGuiApplication) -> f64;
}

// proto:  double QGuiApplication::devicePixelRatio();
impl<'a> /*trait*/ QGuiApplication_devicePixelRatio for () {
  fn devicePixelRatio(self, rsthis: &mut QGuiApplication) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformFunction<T: QGuiApplication_platformFunction>(&mut self, value: T)  {
     value.platformFunction(self);
    // return 1;
  }
}

pub trait QGuiApplication_platformFunction {
  fn platformFunction(self, rsthis: &mut QGuiApplication) ;
}

// proto: static QFunctionPointer QGuiApplication::platformFunction(const QByteArray & function);
impl<'a> /*trait*/ QGuiApplication_platformFunction for (&'a  QByteArray) {
  fn platformFunction(self, rsthis: &mut QGuiApplication)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16platformFunctionERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication16platformFunctionERK10QByteArray(arg0)};
    // return 1;
  }
}

