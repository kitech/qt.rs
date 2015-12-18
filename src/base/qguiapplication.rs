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
  pub fn FreeQGuiApplication<RetType, T: QGuiApplication_FreeQGuiApplication<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGuiApplication(self);
    // return 1;
  }
}

pub trait QGuiApplication_FreeQGuiApplication<RetType> {
  fn FreeQGuiApplication(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::FreeQGuiApplication();
impl<'a> /*trait*/ QGuiApplication_FreeQGuiApplication<()> for () {
  fn FreeQGuiApplication(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplicationD0Ev()};
     unsafe {_ZN15QGuiApplicationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setFont<RetType, T: QGuiApplication_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QGuiApplication_setFont<RetType> {
  fn setFont(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::setFont(const QFont & );
impl<'a> /*trait*/ QGuiApplication_setFont<()> for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication7setFontERK5QFont(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformName<RetType, T: QGuiApplication_platformName<RetType>>(&mut self, value: T) -> RetType {
    return value.platformName(self);
    // return 1;
  }
}

pub trait QGuiApplication_platformName<RetType> {
  fn platformName(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QString QGuiApplication::platformName();
impl<'a> /*trait*/ QGuiApplication_platformName<QString> for () {
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
  pub fn screens<RetType, T: QGuiApplication_screens<RetType>>(&mut self, value: T) -> RetType {
    return value.screens(self);
    // return 1;
  }
}

pub trait QGuiApplication_screens<RetType> {
  fn screens(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QList<QScreen *> QGuiApplication::screens();
impl<'a> /*trait*/ QGuiApplication_screens<()> for () {
  fn screens(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7screensEv()};
     unsafe {_ZN15QGuiApplication7screensEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setPalette<RetType, T: QGuiApplication_setPalette<RetType>>(&mut self, value: T) -> RetType {
    return value.setPalette(self);
    // return 1;
  }
}

pub trait QGuiApplication_setPalette<RetType> {
  fn setPalette(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::setPalette(const QPalette & pal);
impl<'a> /*trait*/ QGuiApplication_setPalette<()> for (&'a  QPalette) {
  fn setPalette(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication10setPaletteERK8QPalette(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn inputMethod<RetType, T: QGuiApplication_inputMethod<RetType>>(&mut self, value: T) -> RetType {
    return value.inputMethod(self);
    // return 1;
  }
}

pub trait QGuiApplication_inputMethod<RetType> {
  fn inputMethod(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QInputMethod * QGuiApplication::inputMethod();
impl<'a> /*trait*/ QGuiApplication_inputMethod<QInputMethod> for () {
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
  pub fn isSavingSession<RetType, T: QGuiApplication_isSavingSession<RetType>>(&mut self, value: T) -> RetType {
    return value.isSavingSession(self);
    // return 1;
  }
}

pub trait QGuiApplication_isSavingSession<RetType> {
  fn isSavingSession(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  bool QGuiApplication::isSavingSession();
impl<'a> /*trait*/ QGuiApplication_isSavingSession<i8> for () {
  fn isSavingSession(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication15isSavingSessionEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication15isSavingSessionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn paletteChanged<RetType, T: QGuiApplication_paletteChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.paletteChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_paletteChanged<RetType> {
  fn paletteChanged(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::paletteChanged(const QPalette & pal);
impl<'a> /*trait*/ QGuiApplication_paletteChanged<()> for (&'a  QPalette) {
  fn paletteChanged(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication14paletteChangedERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication14paletteChangedERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn font<RetType, T: QGuiApplication_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QGuiApplication_font<RetType> {
  fn font(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QFont QGuiApplication::font();
impl<'a> /*trait*/ QGuiApplication_font<QFont> for () {
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
  pub fn screenAdded<RetType, T: QGuiApplication_screenAdded<RetType>>(&mut self, value: T) -> RetType {
    return value.screenAdded(self);
    // return 1;
  }
}

pub trait QGuiApplication_screenAdded<RetType> {
  fn screenAdded(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::screenAdded(QScreen * screen);
impl<'a> /*trait*/ QGuiApplication_screenAdded<()> for (&'a mut QScreen) {
  fn screenAdded(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11screenAddedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication11screenAddedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isSessionRestored<RetType, T: QGuiApplication_isSessionRestored<RetType>>(&mut self, value: T) -> RetType {
    return value.isSessionRestored(self);
    // return 1;
  }
}

pub trait QGuiApplication_isSessionRestored<RetType> {
  fn isSessionRestored(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  bool QGuiApplication::isSessionRestored();
impl<'a> /*trait*/ QGuiApplication_isSessionRestored<i8> for () {
  fn isSessionRestored(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication17isSessionRestoredEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication17isSessionRestoredEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sessionKey<RetType, T: QGuiApplication_sessionKey<RetType>>(&mut self, value: T) -> RetType {
    return value.sessionKey(self);
    // return 1;
  }
}

pub trait QGuiApplication_sessionKey<RetType> {
  fn sessionKey(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  QString QGuiApplication::sessionKey();
impl<'a> /*trait*/ QGuiApplication_sessionKey<QString> for () {
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
  pub fn desktopSettingsAware<RetType, T: QGuiApplication_desktopSettingsAware<RetType>>(&mut self, value: T) -> RetType {
    return value.desktopSettingsAware(self);
    // return 1;
  }
}

pub trait QGuiApplication_desktopSettingsAware<RetType> {
  fn desktopSettingsAware(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static bool QGuiApplication::desktopSettingsAware();
impl<'a> /*trait*/ QGuiApplication_desktopSettingsAware<i8> for () {
  fn desktopSettingsAware(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication20desktopSettingsAwareEv()};
    let mut ret = unsafe {_ZN15QGuiApplication20desktopSettingsAwareEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sync<RetType, T: QGuiApplication_sync<RetType>>(&mut self, value: T) -> RetType {
    return value.sync(self);
    // return 1;
  }
}

pub trait QGuiApplication_sync<RetType> {
  fn sync(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::sync();
impl<'a> /*trait*/ QGuiApplication_sync<()> for () {
  fn sync(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4syncEv()};
     unsafe {_ZN15QGuiApplication4syncEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setQuitOnLastWindowClosed<RetType, T: QGuiApplication_setQuitOnLastWindowClosed<RetType>>(&mut self, value: T) -> RetType {
    return value.setQuitOnLastWindowClosed(self);
    // return 1;
  }
}

pub trait QGuiApplication_setQuitOnLastWindowClosed<RetType> {
  fn setQuitOnLastWindowClosed(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::setQuitOnLastWindowClosed(bool quit);
impl<'a> /*trait*/ QGuiApplication_setQuitOnLastWindowClosed<()> for (i8) {
  fn setQuitOnLastWindowClosed(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication25setQuitOnLastWindowClosedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QGuiApplication25setQuitOnLastWindowClosedEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn primaryScreen<RetType, T: QGuiApplication_primaryScreen<RetType>>(&mut self, value: T) -> RetType {
    return value.primaryScreen(self);
    // return 1;
  }
}

pub trait QGuiApplication_primaryScreen<RetType> {
  fn primaryScreen(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QScreen * QGuiApplication::primaryScreen();
impl<'a> /*trait*/ QGuiApplication_primaryScreen<QScreen> for () {
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
  pub fn overrideCursor<RetType, T: QGuiApplication_overrideCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.overrideCursor(self);
    // return 1;
  }
}

pub trait QGuiApplication_overrideCursor<RetType> {
  fn overrideCursor(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QCursor * QGuiApplication::overrideCursor();
impl<'a> /*trait*/ QGuiApplication_overrideCursor<QCursor> for () {
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
  pub fn windowIcon<RetType, T: QGuiApplication_windowIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.windowIcon(self);
    // return 1;
  }
}

pub trait QGuiApplication_windowIcon<RetType> {
  fn windowIcon(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QIcon QGuiApplication::windowIcon();
impl<'a> /*trait*/ QGuiApplication_windowIcon<QIcon> for () {
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
  pub fn styleHints<RetType, T: QGuiApplication_styleHints<RetType>>(&mut self, value: T) -> RetType {
    return value.styleHints(self);
    // return 1;
  }
}

pub trait QGuiApplication_styleHints<RetType> {
  fn styleHints(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QStyleHints * QGuiApplication::styleHints();
impl<'a> /*trait*/ QGuiApplication_styleHints<QStyleHints> for () {
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
  pub fn clipboard<RetType, T: QGuiApplication_clipboard<RetType>>(&mut self, value: T) -> RetType {
    return value.clipboard(self);
    // return 1;
  }
}

pub trait QGuiApplication_clipboard<RetType> {
  fn clipboard(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QClipboard * QGuiApplication::clipboard();
impl<'a> /*trait*/ QGuiApplication_clipboard<QClipboard> for () {
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
  pub fn palette<RetType, T: QGuiApplication_palette<RetType>>(&mut self, value: T) -> RetType {
    return value.palette(self);
    // return 1;
  }
}

pub trait QGuiApplication_palette<RetType> {
  fn palette(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QPalette QGuiApplication::palette();
impl<'a> /*trait*/ QGuiApplication_palette<QPalette> for () {
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
  pub fn notify<RetType, T: QGuiApplication_notify<RetType>>(&mut self, value: T) -> RetType {
    return value.notify(self);
    // return 1;
  }
}

pub trait QGuiApplication_notify<RetType> {
  fn notify(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  bool QGuiApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QGuiApplication_notify<i8> for (&'a mut QObject, &'a mut QEvent) {
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
  pub fn topLevelWindows<RetType, T: QGuiApplication_topLevelWindows<RetType>>(&mut self, value: T) -> RetType {
    return value.topLevelWindows(self);
    // return 1;
  }
}

pub trait QGuiApplication_topLevelWindows<RetType> {
  fn topLevelWindows(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QList<QWindow *> QGuiApplication::topLevelWindows();
impl<'a> /*trait*/ QGuiApplication_topLevelWindows<()> for () {
  fn topLevelWindows(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication15topLevelWindowsEv()};
     unsafe {_ZN15QGuiApplication15topLevelWindowsEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isRightToLeft<RetType, T: QGuiApplication_isRightToLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.isRightToLeft(self);
    // return 1;
  }
}

pub trait QGuiApplication_isRightToLeft<RetType> {
  fn isRightToLeft(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static bool QGuiApplication::isRightToLeft();
impl<'a> /*trait*/ QGuiApplication_isRightToLeft<i8> for () {
  fn isRightToLeft(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13isRightToLeftEv()};
    let mut ret = unsafe {_ZN15QGuiApplication13isRightToLeftEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusObjectChanged<RetType, T: QGuiApplication_focusObjectChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.focusObjectChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusObjectChanged<RetType> {
  fn focusObjectChanged(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::focusObjectChanged(QObject * focusObject);
impl<'a> /*trait*/ QGuiApplication_focusObjectChanged<()> for (&'a mut QObject) {
  fn focusObjectChanged(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication18focusObjectChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication18focusObjectChangedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn fontDatabaseChanged<RetType, T: QGuiApplication_fontDatabaseChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.fontDatabaseChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_fontDatabaseChanged<RetType> {
  fn fontDatabaseChanged(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::fontDatabaseChanged();
impl<'a> /*trait*/ QGuiApplication_fontDatabaseChanged<()> for () {
  fn fontDatabaseChanged(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication19fontDatabaseChangedEv()};
     unsafe {_ZN15QGuiApplication19fontDatabaseChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn changeOverrideCursor<RetType, T: QGuiApplication_changeOverrideCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.changeOverrideCursor(self);
    // return 1;
  }
}

pub trait QGuiApplication_changeOverrideCursor<RetType> {
  fn changeOverrideCursor(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::changeOverrideCursor(const QCursor & );
impl<'a> /*trait*/ QGuiApplication_changeOverrideCursor<()> for (&'a  QCursor) {
  fn changeOverrideCursor(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication20changeOverrideCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication20changeOverrideCursorERK7QCursor(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn allWindows<RetType, T: QGuiApplication_allWindows<RetType>>(&mut self, value: T) -> RetType {
    return value.allWindows(self);
    // return 1;
  }
}

pub trait QGuiApplication_allWindows<RetType> {
  fn allWindows(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QList<QWindow *> QGuiApplication::allWindows();
impl<'a> /*trait*/ QGuiApplication_allWindows<()> for () {
  fn allWindows(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10allWindowsEv()};
     unsafe {_ZN15QGuiApplication10allWindowsEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setOverrideCursor<RetType, T: QGuiApplication_setOverrideCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.setOverrideCursor(self);
    // return 1;
  }
}

pub trait QGuiApplication_setOverrideCursor<RetType> {
  fn setOverrideCursor(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::setOverrideCursor(const QCursor & );
impl<'a> /*trait*/ QGuiApplication_setOverrideCursor<()> for (&'a  QCursor) {
  fn setOverrideCursor(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication17setOverrideCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication17setOverrideCursorERK7QCursor(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn commitDataRequest<RetType, T: QGuiApplication_commitDataRequest<RetType>>(&mut self, value: T) -> RetType {
    return value.commitDataRequest(self);
    // return 1;
  }
}

pub trait QGuiApplication_commitDataRequest<RetType> {
  fn commitDataRequest(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::commitDataRequest(QSessionManager & sessionManager);
impl<'a> /*trait*/ QGuiApplication_commitDataRequest<()> for (&'a mut QSessionManager) {
  fn commitDataRequest(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication17commitDataRequestER15QSessionManager()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication17commitDataRequestER15QSessionManager(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setWindowIcon<RetType, T: QGuiApplication_setWindowIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowIcon(self);
    // return 1;
  }
}

pub trait QGuiApplication_setWindowIcon<RetType> {
  fn setWindowIcon(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QGuiApplication_setWindowIcon<()> for (&'a  QIcon) {
  fn setWindowIcon(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication13setWindowIconERK5QIcon(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sessionId<RetType, T: QGuiApplication_sessionId<RetType>>(&mut self, value: T) -> RetType {
    return value.sessionId(self);
    // return 1;
  }
}

pub trait QGuiApplication_sessionId<RetType> {
  fn sessionId(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  QString QGuiApplication::sessionId();
impl<'a> /*trait*/ QGuiApplication_sessionId<QString> for () {
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
  pub fn focusWindowChanged<RetType, T: QGuiApplication_focusWindowChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.focusWindowChanged(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusWindowChanged<RetType> {
  fn focusWindowChanged(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::focusWindowChanged(QWindow * focusWindow);
impl<'a> /*trait*/ QGuiApplication_focusWindowChanged<()> for (&'a mut QWindow) {
  fn focusWindowChanged(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication18focusWindowChangedEP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication18focusWindowChangedEP7QWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setApplicationDisplayName<RetType, T: QGuiApplication_setApplicationDisplayName<RetType>>(&mut self, value: T) -> RetType {
    return value.setApplicationDisplayName(self);
    // return 1;
  }
}

pub trait QGuiApplication_setApplicationDisplayName<RetType> {
  fn setApplicationDisplayName(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::setApplicationDisplayName(const QString & name);
impl<'a> /*trait*/ QGuiApplication_setApplicationDisplayName<()> for (&'a  QString) {
  fn setApplicationDisplayName(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication25setApplicationDisplayNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication25setApplicationDisplayNameERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isLeftToRight<RetType, T: QGuiApplication_isLeftToRight<RetType>>(&mut self, value: T) -> RetType {
    return value.isLeftToRight(self);
    // return 1;
  }
}

pub trait QGuiApplication_isLeftToRight<RetType> {
  fn isLeftToRight(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static bool QGuiApplication::isLeftToRight();
impl<'a> /*trait*/ QGuiApplication_isLeftToRight<i8> for () {
  fn isLeftToRight(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13isLeftToRightEv()};
    let mut ret = unsafe {_ZN15QGuiApplication13isLeftToRightEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn topLevelAt<RetType, T: QGuiApplication_topLevelAt<RetType>>(&mut self, value: T) -> RetType {
    return value.topLevelAt(self);
    // return 1;
  }
}

pub trait QGuiApplication_topLevelAt<RetType> {
  fn topLevelAt(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QWindow * QGuiApplication::topLevelAt(const QPoint & pos);
impl<'a> /*trait*/ QGuiApplication_topLevelAt<QWindow> for (&'a  QPoint) {
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
  pub fn setDesktopSettingsAware<RetType, T: QGuiApplication_setDesktopSettingsAware<RetType>>(&mut self, value: T) -> RetType {
    return value.setDesktopSettingsAware(self);
    // return 1;
  }
}

pub trait QGuiApplication_setDesktopSettingsAware<RetType> {
  fn setDesktopSettingsAware(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::setDesktopSettingsAware(bool on);
impl<'a> /*trait*/ QGuiApplication_setDesktopSettingsAware<()> for (i8) {
  fn setDesktopSettingsAware(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication23setDesktopSettingsAwareEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QGuiApplication23setDesktopSettingsAwareEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn modalWindow<RetType, T: QGuiApplication_modalWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.modalWindow(self);
    // return 1;
  }
}

pub trait QGuiApplication_modalWindow<RetType> {
  fn modalWindow(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QWindow * QGuiApplication::modalWindow();
impl<'a> /*trait*/ QGuiApplication_modalWindow<QWindow> for () {
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
  pub fn applicationDisplayName<RetType, T: QGuiApplication_applicationDisplayName<RetType>>(&mut self, value: T) -> RetType {
    return value.applicationDisplayName(self);
    // return 1;
  }
}

pub trait QGuiApplication_applicationDisplayName<RetType> {
  fn applicationDisplayName(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QString QGuiApplication::applicationDisplayName();
impl<'a> /*trait*/ QGuiApplication_applicationDisplayName<QString> for () {
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
  pub fn exec<RetType, T: QGuiApplication_exec<RetType>>(&mut self, value: T) -> RetType {
    return value.exec(self);
    // return 1;
  }
}

pub trait QGuiApplication_exec<RetType> {
  fn exec(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static int QGuiApplication::exec();
impl<'a> /*trait*/ QGuiApplication_exec<i32> for () {
  fn exec(self, rsthis: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4execEv()};
    let mut ret = unsafe {_ZN15QGuiApplication4execEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn quitOnLastWindowClosed<RetType, T: QGuiApplication_quitOnLastWindowClosed<RetType>>(&mut self, value: T) -> RetType {
    return value.quitOnLastWindowClosed(self);
    // return 1;
  }
}

pub trait QGuiApplication_quitOnLastWindowClosed<RetType> {
  fn quitOnLastWindowClosed(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static bool QGuiApplication::quitOnLastWindowClosed();
impl<'a> /*trait*/ QGuiApplication_quitOnLastWindowClosed<i8> for () {
  fn quitOnLastWindowClosed(self, rsthis: &mut QGuiApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication22quitOnLastWindowClosedEv()};
    let mut ret = unsafe {_ZN15QGuiApplication22quitOnLastWindowClosedEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn lastWindowClosed<RetType, T: QGuiApplication_lastWindowClosed<RetType>>(&mut self, value: T) -> RetType {
    return value.lastWindowClosed(self);
    // return 1;
  }
}

pub trait QGuiApplication_lastWindowClosed<RetType> {
  fn lastWindowClosed(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::lastWindowClosed();
impl<'a> /*trait*/ QGuiApplication_lastWindowClosed<()> for () {
  fn lastWindowClosed(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16lastWindowClosedEv()};
     unsafe {_ZN15QGuiApplication16lastWindowClosedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn restoreOverrideCursor<RetType, T: QGuiApplication_restoreOverrideCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.restoreOverrideCursor(self);
    // return 1;
  }
}

pub trait QGuiApplication_restoreOverrideCursor<RetType> {
  fn restoreOverrideCursor(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static void QGuiApplication::restoreOverrideCursor();
impl<'a> /*trait*/ QGuiApplication_restoreOverrideCursor<()> for () {
  fn restoreOverrideCursor(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication21restoreOverrideCursorEv()};
     unsafe {_ZN15QGuiApplication21restoreOverrideCursorEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformNativeInterface<RetType, T: QGuiApplication_platformNativeInterface<RetType>>(&mut self, value: T) -> RetType {
    return value.platformNativeInterface(self);
    // return 1;
  }
}

pub trait QGuiApplication_platformNativeInterface<RetType> {
  fn platformNativeInterface(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QPlatformNativeInterface * QGuiApplication::platformNativeInterface();
impl<'a> /*trait*/ QGuiApplication_platformNativeInterface<()> for () {
  fn platformNativeInterface(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication23platformNativeInterfaceEv()};
     unsafe {_ZN15QGuiApplication23platformNativeInterfaceEv()};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn metaObject<RetType, T: QGuiApplication_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QGuiApplication_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  const QMetaObject * QGuiApplication::metaObject();
impl<'a> /*trait*/ QGuiApplication_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication10metaObjectEv()};
     unsafe {_ZNK15QGuiApplication10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusObject<RetType, T: QGuiApplication_focusObject<RetType>>(&mut self, value: T) -> RetType {
    return value.focusObject(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusObject<RetType> {
  fn focusObject(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QObject * QGuiApplication::focusObject();
impl<'a> /*trait*/ QGuiApplication_focusObject<QObject> for () {
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
  pub fn screenRemoved<RetType, T: QGuiApplication_screenRemoved<RetType>>(&mut self, value: T) -> RetType {
    return value.screenRemoved(self);
    // return 1;
  }
}

pub trait QGuiApplication_screenRemoved<RetType> {
  fn screenRemoved(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::screenRemoved(QScreen * screen);
impl<'a> /*trait*/ QGuiApplication_screenRemoved<()> for (&'a mut QScreen) {
  fn screenRemoved(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13screenRemovedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication13screenRemovedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusWindow<RetType, T: QGuiApplication_focusWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.focusWindow(self);
    // return 1;
  }
}

pub trait QGuiApplication_focusWindow<RetType> {
  fn focusWindow(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QWindow * QGuiApplication::focusWindow();
impl<'a> /*trait*/ QGuiApplication_focusWindow<QWindow> for () {
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
  pub fn saveStateRequest<RetType, T: QGuiApplication_saveStateRequest<RetType>>(&mut self, value: T) -> RetType {
    return value.saveStateRequest(self);
    // return 1;
  }
}

pub trait QGuiApplication_saveStateRequest<RetType> {
  fn saveStateRequest(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  void QGuiApplication::saveStateRequest(QSessionManager & sessionManager);
impl<'a> /*trait*/ QGuiApplication_saveStateRequest<()> for (&'a mut QSessionManager) {
  fn saveStateRequest(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16saveStateRequestER15QSessionManager()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication16saveStateRequestER15QSessionManager(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn devicePixelRatio<RetType, T: QGuiApplication_devicePixelRatio<RetType>>(&mut self, value: T) -> RetType {
    return value.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QGuiApplication_devicePixelRatio<RetType> {
  fn devicePixelRatio(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto:  double QGuiApplication::devicePixelRatio();
impl<'a> /*trait*/ QGuiApplication_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self, rsthis: &mut QGuiApplication) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK15QGuiApplication16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformFunction<RetType, T: QGuiApplication_platformFunction<RetType>>(&mut self, value: T) -> RetType {
    return value.platformFunction(self);
    // return 1;
  }
}

pub trait QGuiApplication_platformFunction<RetType> {
  fn platformFunction(self, rsthis: &mut QGuiApplication) -> RetType;
}

// proto: static QFunctionPointer QGuiApplication::platformFunction(const QByteArray & function);
impl<'a> /*trait*/ QGuiApplication_platformFunction<()> for (&'a  QByteArray) {
  fn platformFunction(self, rsthis: &mut QGuiApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16platformFunctionERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGuiApplication16platformFunctionERK10QByteArray(arg0)};
    // return 1;
  }
}

