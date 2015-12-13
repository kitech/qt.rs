// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qpalette::QPalette;
use super::qscreen::QScreen;
use super::qobject::QObject;
use super::qevent::QEvent;
use super::qcursor::QCursor;
use super::qsessionmanager::QSessionManager;
use super::qicon::QIcon;
use super::qwindow::QWindow;
use super::qstring::QString;
use super::qpoint::QPoint;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGuiApplication::FreeQGuiApplication();
  fn _ZN15QGuiApplicationD0Ev() -> i32;
  // proto: void QGuiApplication::setFont(const QFont & );
  fn _ZN15QGuiApplication7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: QString QGuiApplication::platformName();
  fn _ZN15QGuiApplication12platformNameEv() -> i32;
  // proto: QList<QScreen *> QGuiApplication::screens();
  fn _ZN15QGuiApplication7screensEv() -> i32;
  // proto: void QGuiApplication::setPalette(const QPalette & pal);
  fn _ZN15QGuiApplication10setPaletteERK8QPalette(arg0: *const c_void) -> i32;
  // proto: QInputMethod * QGuiApplication::inputMethod();
  fn _ZN15QGuiApplication11inputMethodEv() -> i32;
  // proto: void QGuiApplication::NewQGuiApplication(const QGuiApplication & );
  fn _ZN15QGuiApplicationC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QGuiApplication::isSavingSession();
  fn _ZNK15QGuiApplication15isSavingSessionEv() -> i32;
  // proto: void QGuiApplication::paletteChanged(const QPalette & pal);
  fn _ZN15QGuiApplication14paletteChangedERK8QPalette(arg0: *const c_void) -> i32;
  // proto: QFont QGuiApplication::font();
  fn _ZN15QGuiApplication4fontEv() -> i32;
  // proto: void QGuiApplication::screenAdded(QScreen * screen);
  fn _ZN15QGuiApplication11screenAddedEP7QScreen(arg0: *mut c_void) -> i32;
  // proto: bool QGuiApplication::isSessionRestored();
  fn _ZNK15QGuiApplication17isSessionRestoredEv() -> i32;
  // proto: QString QGuiApplication::sessionKey();
  fn _ZNK15QGuiApplication10sessionKeyEv() -> i32;
  // proto: bool QGuiApplication::desktopSettingsAware();
  fn _ZN15QGuiApplication20desktopSettingsAwareEv() -> i32;
  // proto: void QGuiApplication::sync();
  fn _ZN15QGuiApplication4syncEv() -> i32;
  // proto: void QGuiApplication::setQuitOnLastWindowClosed(bool quit);
  fn _ZN15QGuiApplication25setQuitOnLastWindowClosedEb(arg0: int8_t) -> i32;
  // proto: QScreen * QGuiApplication::primaryScreen();
  fn _ZN15QGuiApplication13primaryScreenEv() -> i32;
  // proto: QCursor * QGuiApplication::overrideCursor();
  fn _ZN15QGuiApplication14overrideCursorEv() -> i32;
  // proto: QIcon QGuiApplication::windowIcon();
  fn _ZN15QGuiApplication10windowIconEv() -> i32;
  // proto: QStyleHints * QGuiApplication::styleHints();
  fn _ZN15QGuiApplication10styleHintsEv() -> i32;
  // proto: QClipboard * QGuiApplication::clipboard();
  fn _ZN15QGuiApplication9clipboardEv() -> i32;
  // proto: QPalette QGuiApplication::palette();
  fn _ZN15QGuiApplication7paletteEv() -> i32;
  // proto: bool QGuiApplication::notify(QObject * , QEvent * );
  fn _ZN15QGuiApplication6notifyEP7QObjectP6QEvent(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: QList<QWindow *> QGuiApplication::topLevelWindows();
  fn _ZN15QGuiApplication15topLevelWindowsEv() -> i32;
  // proto: bool QGuiApplication::isRightToLeft();
  fn _ZN15QGuiApplication13isRightToLeftEv() -> i32;
  // proto: void QGuiApplication::focusObjectChanged(QObject * focusObject);
  fn _ZN15QGuiApplication18focusObjectChangedEP7QObject(arg0: *mut c_void) -> i32;
  // proto: void QGuiApplication::fontDatabaseChanged();
  fn _ZN15QGuiApplication19fontDatabaseChangedEv() -> i32;
  // proto: void QGuiApplication::changeOverrideCursor(const QCursor & );
  fn _ZN15QGuiApplication20changeOverrideCursorERK7QCursor(arg0: *const c_void) -> i32;
  // proto: QList<QWindow *> QGuiApplication::allWindows();
  fn _ZN15QGuiApplication10allWindowsEv() -> i32;
  // proto: void QGuiApplication::setOverrideCursor(const QCursor & );
  fn _ZN15QGuiApplication17setOverrideCursorERK7QCursor(arg0: *const c_void) -> i32;
  // proto: void QGuiApplication::commitDataRequest(QSessionManager & sessionManager);
  fn _ZN15QGuiApplication17commitDataRequestER15QSessionManager(arg0: *mut c_void) -> i32;
  // proto: void QGuiApplication::setWindowIcon(const QIcon & icon);
  fn _ZN15QGuiApplication13setWindowIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: QString QGuiApplication::sessionId();
  fn _ZNK15QGuiApplication9sessionIdEv() -> i32;
  // proto: void QGuiApplication::focusWindowChanged(QWindow * focusWindow);
  fn _ZN15QGuiApplication18focusWindowChangedEP7QWindow(arg0: *mut c_void) -> i32;
  // proto: void QGuiApplication::setApplicationDisplayName(const QString & name);
  fn _ZN15QGuiApplication25setApplicationDisplayNameERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QGuiApplication::isLeftToRight();
  fn _ZN15QGuiApplication13isLeftToRightEv() -> i32;
  // proto: QWindow * QGuiApplication::topLevelAt(const QPoint & pos);
  fn _ZN15QGuiApplication10topLevelAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QGuiApplication::NewQGuiApplication(int & argc, char ** argv, int );
  fn _ZN15QGuiApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) -> i32;
  // proto: void QGuiApplication::setDesktopSettingsAware(bool on);
  fn _ZN15QGuiApplication23setDesktopSettingsAwareEb(arg0: int8_t) -> i32;
  // proto: QWindow * QGuiApplication::modalWindow();
  fn _ZN15QGuiApplication11modalWindowEv() -> i32;
  // proto: QString QGuiApplication::applicationDisplayName();
  fn _ZN15QGuiApplication22applicationDisplayNameEv() -> i32;
  // proto: int QGuiApplication::exec();
  fn _ZN15QGuiApplication4execEv() -> i32;
  // proto: bool QGuiApplication::quitOnLastWindowClosed();
  fn _ZN15QGuiApplication22quitOnLastWindowClosedEv() -> i32;
  // proto: void QGuiApplication::lastWindowClosed();
  fn _ZN15QGuiApplication16lastWindowClosedEv() -> i32;
  // proto: void QGuiApplication::restoreOverrideCursor();
  fn _ZN15QGuiApplication21restoreOverrideCursorEv() -> i32;
  // proto: QPlatformNativeInterface * QGuiApplication::platformNativeInterface();
  fn _ZN15QGuiApplication23platformNativeInterfaceEv() -> i32;
  // proto: const QMetaObject * QGuiApplication::metaObject();
  fn _ZNK15QGuiApplication10metaObjectEv() -> i32;
  // proto: QObject * QGuiApplication::focusObject();
  fn _ZN15QGuiApplication11focusObjectEv() -> i32;
  // proto: void QGuiApplication::screenRemoved(QScreen * screen);
  fn _ZN15QGuiApplication13screenRemovedEP7QScreen(arg0: *mut c_void) -> i32;
  // proto: QWindow * QGuiApplication::focusWindow();
  fn _ZN15QGuiApplication11focusWindowEv() -> i32;
  // proto: void QGuiApplication::saveStateRequest(QSessionManager & sessionManager);
  fn _ZN15QGuiApplication16saveStateRequestER15QSessionManager(arg0: *mut c_void) -> i32;
  // proto: double QGuiApplication::devicePixelRatio();
  fn _ZNK15QGuiApplication16devicePixelRatioEv() -> i32;
  // proto: QFunctionPointer QGuiApplication::platformFunction(const QByteArray & function);
  fn _ZN15QGuiApplication16platformFunctionERK10QByteArray(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGuiApplication)=1
pub struct QGuiApplication {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGuiApplication {
  pub fn FreeQGuiApplication<T: QGuiApplication_FreeQGuiApplication>(&mut self, value: T) -> i32 {
    value.FreeQGuiApplication(self);
    return 1;
  }
}

pub trait QGuiApplication_FreeQGuiApplication {
  fn FreeQGuiApplication(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::FreeQGuiApplication();
impl<'a> /*trait*/ QGuiApplication_FreeQGuiApplication for () {
  fn FreeQGuiApplication(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplicationD0Ev()};
    unsafe {_ZN15QGuiApplicationD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setFont<T: QGuiApplication_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QGuiApplication_setFont {
  fn setFont(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::setFont(const QFont & );
impl<'a> /*trait*/ QGuiApplication_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformName<T: QGuiApplication_platformName>(&mut self, value: T) -> i32 {
    value.platformName(self);
    return 1;
  }
}

pub trait QGuiApplication_platformName {
  fn platformName(self, this: &mut QGuiApplication) -> i32;
}

// proto: QString QGuiApplication::platformName();
impl<'a> /*trait*/ QGuiApplication_platformName for () {
  fn platformName(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication12platformNameEv()};
    unsafe {_ZN15QGuiApplication12platformNameEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn screens<T: QGuiApplication_screens>(&mut self, value: T) -> i32 {
    value.screens(self);
    return 1;
  }
}

pub trait QGuiApplication_screens {
  fn screens(self, this: &mut QGuiApplication) -> i32;
}

// proto: QList<QScreen *> QGuiApplication::screens();
impl<'a> /*trait*/ QGuiApplication_screens for () {
  fn screens(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7screensEv()};
    unsafe {_ZN15QGuiApplication7screensEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setPalette<T: QGuiApplication_setPalette>(&mut self, value: T) -> i32 {
    value.setPalette(self);
    return 1;
  }
}

pub trait QGuiApplication_setPalette {
  fn setPalette(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::setPalette(const QPalette & pal);
impl<'a> /*trait*/ QGuiApplication_setPalette for (&'a  QPalette) {
  fn setPalette(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication10setPaletteERK8QPalette(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn inputMethod<T: QGuiApplication_inputMethod>(&mut self, value: T) -> i32 {
    value.inputMethod(self);
    return 1;
  }
}

pub trait QGuiApplication_inputMethod {
  fn inputMethod(self, this: &mut QGuiApplication) -> i32;
}

// proto: QInputMethod * QGuiApplication::inputMethod();
impl<'a> /*trait*/ QGuiApplication_inputMethod for () {
  fn inputMethod(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11inputMethodEv()};
    unsafe {_ZN15QGuiApplication11inputMethodEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplicationC1ERKS_(qthis, arg0)};
    let rsthis = QGuiApplication{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isSavingSession<T: QGuiApplication_isSavingSession>(&mut self, value: T) -> i32 {
    value.isSavingSession(self);
    return 1;
  }
}

pub trait QGuiApplication_isSavingSession {
  fn isSavingSession(self, this: &mut QGuiApplication) -> i32;
}

// proto: bool QGuiApplication::isSavingSession();
impl<'a> /*trait*/ QGuiApplication_isSavingSession for () {
  fn isSavingSession(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication15isSavingSessionEv()};
    unsafe {_ZNK15QGuiApplication15isSavingSessionEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn paletteChanged<T: QGuiApplication_paletteChanged>(&mut self, value: T) -> i32 {
    value.paletteChanged(self);
    return 1;
  }
}

pub trait QGuiApplication_paletteChanged {
  fn paletteChanged(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::paletteChanged(const QPalette & pal);
impl<'a> /*trait*/ QGuiApplication_paletteChanged for (&'a  QPalette) {
  fn paletteChanged(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication14paletteChangedERK8QPalette()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication14paletteChangedERK8QPalette(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn font<T: QGuiApplication_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QGuiApplication_font {
  fn font(self, this: &mut QGuiApplication) -> i32;
}

// proto: QFont QGuiApplication::font();
impl<'a> /*trait*/ QGuiApplication_font for () {
  fn font(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4fontEv()};
    unsafe {_ZN15QGuiApplication4fontEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn screenAdded<T: QGuiApplication_screenAdded>(&mut self, value: T) -> i32 {
    value.screenAdded(self);
    return 1;
  }
}

pub trait QGuiApplication_screenAdded {
  fn screenAdded(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::screenAdded(QScreen * screen);
impl<'a> /*trait*/ QGuiApplication_screenAdded for (&'a mut QScreen) {
  fn screenAdded(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11screenAddedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGuiApplication11screenAddedEP7QScreen(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isSessionRestored<T: QGuiApplication_isSessionRestored>(&mut self, value: T) -> i32 {
    value.isSessionRestored(self);
    return 1;
  }
}

pub trait QGuiApplication_isSessionRestored {
  fn isSessionRestored(self, this: &mut QGuiApplication) -> i32;
}

// proto: bool QGuiApplication::isSessionRestored();
impl<'a> /*trait*/ QGuiApplication_isSessionRestored for () {
  fn isSessionRestored(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication17isSessionRestoredEv()};
    unsafe {_ZNK15QGuiApplication17isSessionRestoredEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sessionKey<T: QGuiApplication_sessionKey>(&mut self, value: T) -> i32 {
    value.sessionKey(self);
    return 1;
  }
}

pub trait QGuiApplication_sessionKey {
  fn sessionKey(self, this: &mut QGuiApplication) -> i32;
}

// proto: QString QGuiApplication::sessionKey();
impl<'a> /*trait*/ QGuiApplication_sessionKey for () {
  fn sessionKey(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication10sessionKeyEv()};
    unsafe {_ZNK15QGuiApplication10sessionKeyEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn desktopSettingsAware<T: QGuiApplication_desktopSettingsAware>(&mut self, value: T) -> i32 {
    value.desktopSettingsAware(self);
    return 1;
  }
}

pub trait QGuiApplication_desktopSettingsAware {
  fn desktopSettingsAware(self, this: &mut QGuiApplication) -> i32;
}

// proto: bool QGuiApplication::desktopSettingsAware();
impl<'a> /*trait*/ QGuiApplication_desktopSettingsAware for () {
  fn desktopSettingsAware(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication20desktopSettingsAwareEv()};
    unsafe {_ZN15QGuiApplication20desktopSettingsAwareEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sync<T: QGuiApplication_sync>(&mut self, value: T) -> i32 {
    value.sync(self);
    return 1;
  }
}

pub trait QGuiApplication_sync {
  fn sync(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::sync();
impl<'a> /*trait*/ QGuiApplication_sync for () {
  fn sync(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4syncEv()};
    unsafe {_ZN15QGuiApplication4syncEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setQuitOnLastWindowClosed<T: QGuiApplication_setQuitOnLastWindowClosed>(&mut self, value: T) -> i32 {
    value.setQuitOnLastWindowClosed(self);
    return 1;
  }
}

pub trait QGuiApplication_setQuitOnLastWindowClosed {
  fn setQuitOnLastWindowClosed(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::setQuitOnLastWindowClosed(bool quit);
impl<'a> /*trait*/ QGuiApplication_setQuitOnLastWindowClosed for (i8) {
  fn setQuitOnLastWindowClosed(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication25setQuitOnLastWindowClosedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QGuiApplication25setQuitOnLastWindowClosedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn primaryScreen<T: QGuiApplication_primaryScreen>(&mut self, value: T) -> i32 {
    value.primaryScreen(self);
    return 1;
  }
}

pub trait QGuiApplication_primaryScreen {
  fn primaryScreen(self, this: &mut QGuiApplication) -> i32;
}

// proto: QScreen * QGuiApplication::primaryScreen();
impl<'a> /*trait*/ QGuiApplication_primaryScreen for () {
  fn primaryScreen(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13primaryScreenEv()};
    unsafe {_ZN15QGuiApplication13primaryScreenEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn overrideCursor<T: QGuiApplication_overrideCursor>(&mut self, value: T) -> i32 {
    value.overrideCursor(self);
    return 1;
  }
}

pub trait QGuiApplication_overrideCursor {
  fn overrideCursor(self, this: &mut QGuiApplication) -> i32;
}

// proto: QCursor * QGuiApplication::overrideCursor();
impl<'a> /*trait*/ QGuiApplication_overrideCursor for () {
  fn overrideCursor(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication14overrideCursorEv()};
    unsafe {_ZN15QGuiApplication14overrideCursorEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn windowIcon<T: QGuiApplication_windowIcon>(&mut self, value: T) -> i32 {
    value.windowIcon(self);
    return 1;
  }
}

pub trait QGuiApplication_windowIcon {
  fn windowIcon(self, this: &mut QGuiApplication) -> i32;
}

// proto: QIcon QGuiApplication::windowIcon();
impl<'a> /*trait*/ QGuiApplication_windowIcon for () {
  fn windowIcon(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10windowIconEv()};
    unsafe {_ZN15QGuiApplication10windowIconEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn styleHints<T: QGuiApplication_styleHints>(&mut self, value: T) -> i32 {
    value.styleHints(self);
    return 1;
  }
}

pub trait QGuiApplication_styleHints {
  fn styleHints(self, this: &mut QGuiApplication) -> i32;
}

// proto: QStyleHints * QGuiApplication::styleHints();
impl<'a> /*trait*/ QGuiApplication_styleHints for () {
  fn styleHints(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10styleHintsEv()};
    unsafe {_ZN15QGuiApplication10styleHintsEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn clipboard<T: QGuiApplication_clipboard>(&mut self, value: T) -> i32 {
    value.clipboard(self);
    return 1;
  }
}

pub trait QGuiApplication_clipboard {
  fn clipboard(self, this: &mut QGuiApplication) -> i32;
}

// proto: QClipboard * QGuiApplication::clipboard();
impl<'a> /*trait*/ QGuiApplication_clipboard for () {
  fn clipboard(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication9clipboardEv()};
    unsafe {_ZN15QGuiApplication9clipboardEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn palette<T: QGuiApplication_palette>(&mut self, value: T) -> i32 {
    value.palette(self);
    return 1;
  }
}

pub trait QGuiApplication_palette {
  fn palette(self, this: &mut QGuiApplication) -> i32;
}

// proto: QPalette QGuiApplication::palette();
impl<'a> /*trait*/ QGuiApplication_palette for () {
  fn palette(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication7paletteEv()};
    unsafe {_ZN15QGuiApplication7paletteEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn notify<T: QGuiApplication_notify>(&mut self, value: T) -> i32 {
    value.notify(self);
    return 1;
  }
}

pub trait QGuiApplication_notify {
  fn notify(self, this: &mut QGuiApplication) -> i32;
}

// proto: bool QGuiApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QGuiApplication_notify for (&'a mut QObject, &'a mut QEvent) {
  fn notify(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QGuiApplication6notifyEP7QObjectP6QEvent(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn topLevelWindows<T: QGuiApplication_topLevelWindows>(&mut self, value: T) -> i32 {
    value.topLevelWindows(self);
    return 1;
  }
}

pub trait QGuiApplication_topLevelWindows {
  fn topLevelWindows(self, this: &mut QGuiApplication) -> i32;
}

// proto: QList<QWindow *> QGuiApplication::topLevelWindows();
impl<'a> /*trait*/ QGuiApplication_topLevelWindows for () {
  fn topLevelWindows(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication15topLevelWindowsEv()};
    unsafe {_ZN15QGuiApplication15topLevelWindowsEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isRightToLeft<T: QGuiApplication_isRightToLeft>(&mut self, value: T) -> i32 {
    value.isRightToLeft(self);
    return 1;
  }
}

pub trait QGuiApplication_isRightToLeft {
  fn isRightToLeft(self, this: &mut QGuiApplication) -> i32;
}

// proto: bool QGuiApplication::isRightToLeft();
impl<'a> /*trait*/ QGuiApplication_isRightToLeft for () {
  fn isRightToLeft(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13isRightToLeftEv()};
    unsafe {_ZN15QGuiApplication13isRightToLeftEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusObjectChanged<T: QGuiApplication_focusObjectChanged>(&mut self, value: T) -> i32 {
    value.focusObjectChanged(self);
    return 1;
  }
}

pub trait QGuiApplication_focusObjectChanged {
  fn focusObjectChanged(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::focusObjectChanged(QObject * focusObject);
impl<'a> /*trait*/ QGuiApplication_focusObjectChanged for (&'a mut QObject) {
  fn focusObjectChanged(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication18focusObjectChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGuiApplication18focusObjectChangedEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn fontDatabaseChanged<T: QGuiApplication_fontDatabaseChanged>(&mut self, value: T) -> i32 {
    value.fontDatabaseChanged(self);
    return 1;
  }
}

pub trait QGuiApplication_fontDatabaseChanged {
  fn fontDatabaseChanged(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::fontDatabaseChanged();
impl<'a> /*trait*/ QGuiApplication_fontDatabaseChanged for () {
  fn fontDatabaseChanged(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication19fontDatabaseChangedEv()};
    unsafe {_ZN15QGuiApplication19fontDatabaseChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn changeOverrideCursor<T: QGuiApplication_changeOverrideCursor>(&mut self, value: T) -> i32 {
    value.changeOverrideCursor(self);
    return 1;
  }
}

pub trait QGuiApplication_changeOverrideCursor {
  fn changeOverrideCursor(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::changeOverrideCursor(const QCursor & );
impl<'a> /*trait*/ QGuiApplication_changeOverrideCursor for (&'a  QCursor) {
  fn changeOverrideCursor(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication20changeOverrideCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication20changeOverrideCursorERK7QCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn allWindows<T: QGuiApplication_allWindows>(&mut self, value: T) -> i32 {
    value.allWindows(self);
    return 1;
  }
}

pub trait QGuiApplication_allWindows {
  fn allWindows(self, this: &mut QGuiApplication) -> i32;
}

// proto: QList<QWindow *> QGuiApplication::allWindows();
impl<'a> /*trait*/ QGuiApplication_allWindows for () {
  fn allWindows(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10allWindowsEv()};
    unsafe {_ZN15QGuiApplication10allWindowsEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setOverrideCursor<T: QGuiApplication_setOverrideCursor>(&mut self, value: T) -> i32 {
    value.setOverrideCursor(self);
    return 1;
  }
}

pub trait QGuiApplication_setOverrideCursor {
  fn setOverrideCursor(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::setOverrideCursor(const QCursor & );
impl<'a> /*trait*/ QGuiApplication_setOverrideCursor for (&'a  QCursor) {
  fn setOverrideCursor(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication17setOverrideCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication17setOverrideCursorERK7QCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn commitDataRequest<T: QGuiApplication_commitDataRequest>(&mut self, value: T) -> i32 {
    value.commitDataRequest(self);
    return 1;
  }
}

pub trait QGuiApplication_commitDataRequest {
  fn commitDataRequest(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::commitDataRequest(QSessionManager & sessionManager);
impl<'a> /*trait*/ QGuiApplication_commitDataRequest for (&'a mut QSessionManager) {
  fn commitDataRequest(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication17commitDataRequestER15QSessionManager()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGuiApplication17commitDataRequestER15QSessionManager(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setWindowIcon<T: QGuiApplication_setWindowIcon>(&mut self, value: T) -> i32 {
    value.setWindowIcon(self);
    return 1;
  }
}

pub trait QGuiApplication_setWindowIcon {
  fn setWindowIcon(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QGuiApplication_setWindowIcon for (&'a  QIcon) {
  fn setWindowIcon(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication13setWindowIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn sessionId<T: QGuiApplication_sessionId>(&mut self, value: T) -> i32 {
    value.sessionId(self);
    return 1;
  }
}

pub trait QGuiApplication_sessionId {
  fn sessionId(self, this: &mut QGuiApplication) -> i32;
}

// proto: QString QGuiApplication::sessionId();
impl<'a> /*trait*/ QGuiApplication_sessionId for () {
  fn sessionId(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication9sessionIdEv()};
    unsafe {_ZNK15QGuiApplication9sessionIdEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusWindowChanged<T: QGuiApplication_focusWindowChanged>(&mut self, value: T) -> i32 {
    value.focusWindowChanged(self);
    return 1;
  }
}

pub trait QGuiApplication_focusWindowChanged {
  fn focusWindowChanged(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::focusWindowChanged(QWindow * focusWindow);
impl<'a> /*trait*/ QGuiApplication_focusWindowChanged for (&'a mut QWindow) {
  fn focusWindowChanged(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication18focusWindowChangedEP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGuiApplication18focusWindowChangedEP7QWindow(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn setApplicationDisplayName<T: QGuiApplication_setApplicationDisplayName>(&mut self, value: T) -> i32 {
    value.setApplicationDisplayName(self);
    return 1;
  }
}

pub trait QGuiApplication_setApplicationDisplayName {
  fn setApplicationDisplayName(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::setApplicationDisplayName(const QString & name);
impl<'a> /*trait*/ QGuiApplication_setApplicationDisplayName for (&'a  QString) {
  fn setApplicationDisplayName(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication25setApplicationDisplayNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication25setApplicationDisplayNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn isLeftToRight<T: QGuiApplication_isLeftToRight>(&mut self, value: T) -> i32 {
    value.isLeftToRight(self);
    return 1;
  }
}

pub trait QGuiApplication_isLeftToRight {
  fn isLeftToRight(self, this: &mut QGuiApplication) -> i32;
}

// proto: bool QGuiApplication::isLeftToRight();
impl<'a> /*trait*/ QGuiApplication_isLeftToRight for () {
  fn isLeftToRight(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13isLeftToRightEv()};
    unsafe {_ZN15QGuiApplication13isLeftToRightEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn topLevelAt<T: QGuiApplication_topLevelAt>(&mut self, value: T) -> i32 {
    value.topLevelAt(self);
    return 1;
  }
}

pub trait QGuiApplication_topLevelAt {
  fn topLevelAt(self, this: &mut QGuiApplication) -> i32;
}

// proto: QWindow * QGuiApplication::topLevelAt(const QPoint & pos);
impl<'a> /*trait*/ QGuiApplication_topLevelAt for (&'a  QPoint) {
  fn topLevelAt(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication10topLevelAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication10topLevelAtERK6QPoint(arg0)};
    return 1;
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
  pub fn setDesktopSettingsAware<T: QGuiApplication_setDesktopSettingsAware>(&mut self, value: T) -> i32 {
    value.setDesktopSettingsAware(self);
    return 1;
  }
}

pub trait QGuiApplication_setDesktopSettingsAware {
  fn setDesktopSettingsAware(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::setDesktopSettingsAware(bool on);
impl<'a> /*trait*/ QGuiApplication_setDesktopSettingsAware for (i8) {
  fn setDesktopSettingsAware(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication23setDesktopSettingsAwareEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QGuiApplication23setDesktopSettingsAwareEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn modalWindow<T: QGuiApplication_modalWindow>(&mut self, value: T) -> i32 {
    value.modalWindow(self);
    return 1;
  }
}

pub trait QGuiApplication_modalWindow {
  fn modalWindow(self, this: &mut QGuiApplication) -> i32;
}

// proto: QWindow * QGuiApplication::modalWindow();
impl<'a> /*trait*/ QGuiApplication_modalWindow for () {
  fn modalWindow(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11modalWindowEv()};
    unsafe {_ZN15QGuiApplication11modalWindowEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn applicationDisplayName<T: QGuiApplication_applicationDisplayName>(&mut self, value: T) -> i32 {
    value.applicationDisplayName(self);
    return 1;
  }
}

pub trait QGuiApplication_applicationDisplayName {
  fn applicationDisplayName(self, this: &mut QGuiApplication) -> i32;
}

// proto: QString QGuiApplication::applicationDisplayName();
impl<'a> /*trait*/ QGuiApplication_applicationDisplayName for () {
  fn applicationDisplayName(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication22applicationDisplayNameEv()};
    unsafe {_ZN15QGuiApplication22applicationDisplayNameEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn exec<T: QGuiApplication_exec>(&mut self, value: T) -> i32 {
    value.exec(self);
    return 1;
  }
}

pub trait QGuiApplication_exec {
  fn exec(self, this: &mut QGuiApplication) -> i32;
}

// proto: int QGuiApplication::exec();
impl<'a> /*trait*/ QGuiApplication_exec for () {
  fn exec(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication4execEv()};
    unsafe {_ZN15QGuiApplication4execEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn quitOnLastWindowClosed<T: QGuiApplication_quitOnLastWindowClosed>(&mut self, value: T) -> i32 {
    value.quitOnLastWindowClosed(self);
    return 1;
  }
}

pub trait QGuiApplication_quitOnLastWindowClosed {
  fn quitOnLastWindowClosed(self, this: &mut QGuiApplication) -> i32;
}

// proto: bool QGuiApplication::quitOnLastWindowClosed();
impl<'a> /*trait*/ QGuiApplication_quitOnLastWindowClosed for () {
  fn quitOnLastWindowClosed(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication22quitOnLastWindowClosedEv()};
    unsafe {_ZN15QGuiApplication22quitOnLastWindowClosedEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn lastWindowClosed<T: QGuiApplication_lastWindowClosed>(&mut self, value: T) -> i32 {
    value.lastWindowClosed(self);
    return 1;
  }
}

pub trait QGuiApplication_lastWindowClosed {
  fn lastWindowClosed(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::lastWindowClosed();
impl<'a> /*trait*/ QGuiApplication_lastWindowClosed for () {
  fn lastWindowClosed(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16lastWindowClosedEv()};
    unsafe {_ZN15QGuiApplication16lastWindowClosedEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn restoreOverrideCursor<T: QGuiApplication_restoreOverrideCursor>(&mut self, value: T) -> i32 {
    value.restoreOverrideCursor(self);
    return 1;
  }
}

pub trait QGuiApplication_restoreOverrideCursor {
  fn restoreOverrideCursor(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::restoreOverrideCursor();
impl<'a> /*trait*/ QGuiApplication_restoreOverrideCursor for () {
  fn restoreOverrideCursor(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication21restoreOverrideCursorEv()};
    unsafe {_ZN15QGuiApplication21restoreOverrideCursorEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformNativeInterface<T: QGuiApplication_platformNativeInterface>(&mut self, value: T) -> i32 {
    value.platformNativeInterface(self);
    return 1;
  }
}

pub trait QGuiApplication_platformNativeInterface {
  fn platformNativeInterface(self, this: &mut QGuiApplication) -> i32;
}

// proto: QPlatformNativeInterface * QGuiApplication::platformNativeInterface();
impl<'a> /*trait*/ QGuiApplication_platformNativeInterface for () {
  fn platformNativeInterface(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication23platformNativeInterfaceEv()};
    unsafe {_ZN15QGuiApplication23platformNativeInterfaceEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn metaObject<T: QGuiApplication_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGuiApplication_metaObject {
  fn metaObject(self, this: &mut QGuiApplication) -> i32;
}

// proto: const QMetaObject * QGuiApplication::metaObject();
impl<'a> /*trait*/ QGuiApplication_metaObject for () {
  fn metaObject(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication10metaObjectEv()};
    unsafe {_ZNK15QGuiApplication10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusObject<T: QGuiApplication_focusObject>(&mut self, value: T) -> i32 {
    value.focusObject(self);
    return 1;
  }
}

pub trait QGuiApplication_focusObject {
  fn focusObject(self, this: &mut QGuiApplication) -> i32;
}

// proto: QObject * QGuiApplication::focusObject();
impl<'a> /*trait*/ QGuiApplication_focusObject for () {
  fn focusObject(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11focusObjectEv()};
    unsafe {_ZN15QGuiApplication11focusObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn screenRemoved<T: QGuiApplication_screenRemoved>(&mut self, value: T) -> i32 {
    value.screenRemoved(self);
    return 1;
  }
}

pub trait QGuiApplication_screenRemoved {
  fn screenRemoved(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::screenRemoved(QScreen * screen);
impl<'a> /*trait*/ QGuiApplication_screenRemoved for (&'a mut QScreen) {
  fn screenRemoved(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication13screenRemovedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGuiApplication13screenRemovedEP7QScreen(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn focusWindow<T: QGuiApplication_focusWindow>(&mut self, value: T) -> i32 {
    value.focusWindow(self);
    return 1;
  }
}

pub trait QGuiApplication_focusWindow {
  fn focusWindow(self, this: &mut QGuiApplication) -> i32;
}

// proto: QWindow * QGuiApplication::focusWindow();
impl<'a> /*trait*/ QGuiApplication_focusWindow for () {
  fn focusWindow(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication11focusWindowEv()};
    unsafe {_ZN15QGuiApplication11focusWindowEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn saveStateRequest<T: QGuiApplication_saveStateRequest>(&mut self, value: T) -> i32 {
    value.saveStateRequest(self);
    return 1;
  }
}

pub trait QGuiApplication_saveStateRequest {
  fn saveStateRequest(self, this: &mut QGuiApplication) -> i32;
}

// proto: void QGuiApplication::saveStateRequest(QSessionManager & sessionManager);
impl<'a> /*trait*/ QGuiApplication_saveStateRequest for (&'a mut QSessionManager) {
  fn saveStateRequest(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16saveStateRequestER15QSessionManager()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGuiApplication16saveStateRequestER15QSessionManager(arg0)};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn devicePixelRatio<T: QGuiApplication_devicePixelRatio>(&mut self, value: T) -> i32 {
    value.devicePixelRatio(self);
    return 1;
  }
}

pub trait QGuiApplication_devicePixelRatio {
  fn devicePixelRatio(self, this: &mut QGuiApplication) -> i32;
}

// proto: double QGuiApplication::devicePixelRatio();
impl<'a> /*trait*/ QGuiApplication_devicePixelRatio for () {
  fn devicePixelRatio(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGuiApplication16devicePixelRatioEv()};
    unsafe {_ZNK15QGuiApplication16devicePixelRatioEv()};
    return 1;
  }
}

impl /*struct*/ QGuiApplication {
  pub fn platformFunction<T: QGuiApplication_platformFunction>(&mut self, value: T) -> i32 {
    value.platformFunction(self);
    return 1;
  }
}

pub trait QGuiApplication_platformFunction {
  fn platformFunction(self, this: &mut QGuiApplication) -> i32;
}

// proto: QFunctionPointer QGuiApplication::platformFunction(const QByteArray & function);
impl<'a> /*trait*/ QGuiApplication_platformFunction for (&'a  QByteArray) {
  fn platformFunction(self, this: &mut QGuiApplication) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGuiApplication16platformFunctionERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGuiApplication16platformFunctionERK10QByteArray(arg0)};
    return 1;
  }
}

