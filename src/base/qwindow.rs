// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qscreen::QScreen;
use super::qicon::QIcon;
use super::qsize::QSize;
use super::qstring::QString;
use super::qpoint::QPoint;
use super::qobject::QObject;
use super::qcursor::QCursor;
use super::qrect::QRect;
use super::qsurfaceformat::QSurfaceFormat;
use super::qregion::QRegion;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QWindow::xChanged(int arg);
  fn _ZN7QWindow8xChangedEi(arg0: c_int) -> i32;
  // proto: void QWindow::unsetCursor();
  fn _ZN7QWindow11unsetCursorEv() -> i32;
  // proto: bool QWindow::isVisible();
  fn _ZNK7QWindow9isVisibleEv() -> i32;
  // proto: void QWindow::setScreen(QScreen * screen);
  fn _ZN7QWindow9setScreenEP7QScreen(arg0: *mut c_void) -> i32;
  // proto: QSize QWindow::maximumSize();
  fn _ZNK7QWindow11maximumSizeEv() -> i32;
  // proto: void QWindow::setTransientParent(QWindow * parent);
  fn _ZN7QWindow18setTransientParentEPS_(arg0: *mut c_void) -> i32;
  // proto: QSurfaceFormat QWindow::format();
  fn _ZNK7QWindow6formatEv() -> i32;
  // proto: bool QWindow::isTopLevel();
  fn _ZNK7QWindow10isTopLevelEv() -> i32;
  // proto: void QWindow::NewQWindow(const QWindow & );
  fn _ZN7QWindowC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QWindow::setIcon(const QIcon & icon);
  fn _ZN7QWindow7setIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: double QWindow::opacity();
  fn _ZNK7QWindow7opacityEv() -> i32;
  // proto: void QWindow::setMinimumSize(const QSize & size);
  fn _ZN7QWindow14setMinimumSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: int QWindow::minimumHeight();
  fn _ZNK7QWindow13minimumHeightEv() -> i32;
  // proto: QSize QWindow::sizeIncrement();
  fn _ZNK7QWindow13sizeIncrementEv() -> i32;
  // proto: void QWindow::resize(const QSize & newSize);
  fn _ZN7QWindow6resizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QWindow::screenChanged(QScreen * screen);
  fn _ZN7QWindow13screenChangedEP7QScreen(arg0: *mut c_void) -> i32;
  // proto: void QWindow::setTitle(const QString & );
  fn _ZN7QWindow8setTitleERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWindow::raise();
  fn _ZN7QWindow5raiseEv() -> i32;
  // proto: QSize QWindow::minimumSize();
  fn _ZNK7QWindow11minimumSizeEv() -> i32;
  // proto: QPoint QWindow::mapToGlobal(const QPoint & pos);
  fn _ZNK7QWindow11mapToGlobalERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QWindow * QWindow::fromWinId(WId id);
  fn _ZN7QWindow9fromWinIdEi(arg0: *mut c_uint) -> i32;
  // proto: QMargins QWindow::frameMargins();
  fn _ZNK7QWindow12frameMarginsEv() -> i32;
  // proto: void QWindow::setMaximumWidth(int w);
  fn _ZN7QWindow15setMaximumWidthEi(arg0: c_int) -> i32;
  // proto: int QWindow::maximumHeight();
  fn _ZNK7QWindow13maximumHeightEv() -> i32;
  // proto: void QWindow::focusObjectChanged(QObject * object);
  fn _ZN7QWindow18focusObjectChangedEP7QObject(arg0: *mut c_void) -> i32;
  // proto: bool QWindow::isModal();
  fn _ZNK7QWindow7isModalEv() -> i32;
  // proto: void QWindow::maximumWidthChanged(int arg);
  fn _ZN7QWindow19maximumWidthChangedEi(arg0: c_int) -> i32;
  // proto: QRect QWindow::geometry();
  fn _ZNK7QWindow8geometryEv() -> i32;
  // proto: void QWindow::setParent(QWindow * parent);
  fn _ZN7QWindow9setParentEPS_(arg0: *mut c_void) -> i32;
  // proto: QRect QWindow::frameGeometry();
  fn _ZNK7QWindow13frameGeometryEv() -> i32;
  // proto: QSurfaceFormat QWindow::requestedFormat();
  fn _ZNK7QWindow15requestedFormatEv() -> i32;
  // proto: void QWindow::setHeight(int arg);
  fn _ZN7QWindow9setHeightEi(arg0: c_int) -> i32;
  // proto: void QWindow::requestActivate();
  fn _ZN7QWindow15requestActivateEv() -> i32;
  // proto: QPoint QWindow::mapFromGlobal(const QPoint & pos);
  fn _ZNK7QWindow13mapFromGlobalERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QWindow::windowTitleChanged(const QString & title);
  fn _ZN7QWindow18windowTitleChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: int QWindow::y();
  fn _ZNK7QWindow1yEv() -> i32;
  // proto: int QWindow::width();
  fn _ZNK7QWindow5widthEv() -> i32;
  // proto: void QWindow::setFilePath(const QString & filePath);
  fn _ZN7QWindow11setFilePathERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWindow::setCursor(const QCursor & );
  fn _ZN7QWindow9setCursorERK7QCursor(arg0: *const c_void) -> i32;
  // proto: void QWindow::setVisible(bool visible);
  fn _ZN7QWindow10setVisibleEb(arg0: int8_t) -> i32;
  // proto: void QWindow::FreeQWindow();
  fn _ZN7QWindowD0Ev() -> i32;
  // proto: bool QWindow::setMouseGrabEnabled(bool grab);
  fn _ZN7QWindow19setMouseGrabEnabledEb(arg0: int8_t) -> i32;
  // proto: bool QWindow::isExposed();
  fn _ZNK7QWindow9isExposedEv() -> i32;
  // proto: void QWindow::heightChanged(int arg);
  fn _ZN7QWindow13heightChangedEi(arg0: c_int) -> i32;
  // proto: int QWindow::minimumWidth();
  fn _ZNK7QWindow12minimumWidthEv() -> i32;
  // proto: void QWindow::setPosition(const QPoint & pt);
  fn _ZN7QWindow11setPositionERK6QPoint(arg0: *const c_void) -> i32;
  // proto: bool QWindow::close();
  fn _ZN7QWindow5closeEv() -> i32;
  // proto: int QWindow::x();
  fn _ZNK7QWindow1xEv() -> i32;
  // proto: void QWindow::setMinimumWidth(int w);
  fn _ZN7QWindow15setMinimumWidthEi(arg0: c_int) -> i32;
  // proto: QRegion QWindow::mask();
  fn _ZNK7QWindow4maskEv() -> i32;
  // proto: void QWindow::widthChanged(int arg);
  fn _ZN7QWindow12widthChangedEi(arg0: c_int) -> i32;
  // proto: QWindow * QWindow::parent();
  fn _ZNK7QWindow6parentEv() -> i32;
  // proto: void QWindow::setFramePosition(const QPoint & point);
  fn _ZN7QWindow16setFramePositionERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QWindow::NewQWindow(QScreen * screen);
  fn _ZN7QWindowC1EP7QScreen(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QWindow::setGeometry(int posx, int posy, int w, int h);
  fn _ZN7QWindow11setGeometryEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: bool QWindow::setKeyboardGrabEnabled(bool grab);
  fn _ZN7QWindow22setKeyboardGrabEnabledEb(arg0: int8_t) -> i32;
  // proto: const QMetaObject * QWindow::metaObject();
  fn _ZNK7QWindow10metaObjectEv() -> i32;
  // proto: void QWindow::NewQWindow(QWindow * parent);
  fn _ZN7QWindowC1EPS_(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QWindow::activeChanged();
  fn _ZN7QWindow13activeChangedEv() -> i32;
  // proto: void QWindow::setWidth(int arg);
  fn _ZN7QWindow8setWidthEi(arg0: c_int) -> i32;
  // proto: void QWindow::setY(int arg);
  fn _ZN7QWindow4setYEi(arg0: c_int) -> i32;
  // proto: double QWindow::devicePixelRatio();
  fn _ZNK7QWindow16devicePixelRatioEv() -> i32;
  // proto: void QWindow::setBaseSize(const QSize & size);
  fn _ZN7QWindow11setBaseSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QWindow::alert(int msec);
  fn _ZN7QWindow5alertEi(arg0: c_int) -> i32;
  // proto: void QWindow::yChanged(int arg);
  fn _ZN7QWindow8yChangedEi(arg0: c_int) -> i32;
  // proto: QPlatformWindow * QWindow::handle();
  fn _ZNK7QWindow6handleEv() -> i32;
  // proto: void QWindow::destroy();
  fn _ZN7QWindow7destroyEv() -> i32;
  // proto: QWindow * QWindow::transientParent();
  fn _ZNK7QWindow15transientParentEv() -> i32;
  // proto: void QWindow::setMinimumHeight(int h);
  fn _ZN7QWindow16setMinimumHeightEi(arg0: c_int) -> i32;
  // proto: void QWindow::show();
  fn _ZN7QWindow4showEv() -> i32;
  // proto: void QWindow::minimumWidthChanged(int arg);
  fn _ZN7QWindow19minimumWidthChangedEi(arg0: c_int) -> i32;
  // proto: QSize QWindow::baseSize();
  fn _ZNK7QWindow8baseSizeEv() -> i32;
  // proto: QString QWindow::title();
  fn _ZNK7QWindow5titleEv() -> i32;
  // proto: void QWindow::showMaximized();
  fn _ZN7QWindow13showMaximizedEv() -> i32;
  // proto: void QWindow::create();
  fn _ZN7QWindow6createEv() -> i32;
  // proto: void QWindow::resize(int w, int h);
  fn _ZN7QWindow6resizeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QScreen * QWindow::screen();
  fn _ZNK7QWindow6screenEv() -> i32;
  // proto: void QWindow::setPosition(int posx, int posy);
  fn _ZN7QWindow11setPositionEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QWindow::setOpacity(qreal level);
  fn _ZN7QWindow10setOpacityEd(arg0: c_double) -> i32;
  // proto: void QWindow::setGeometry(const QRect & rect);
  fn _ZN7QWindow11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QWindow::setSizeIncrement(const QSize & size);
  fn _ZN7QWindow16setSizeIncrementERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QWindow::showMinimized();
  fn _ZN7QWindow13showMinimizedEv() -> i32;
  // proto: QObject * QWindow::focusObject();
  fn _ZNK7QWindow11focusObjectEv() -> i32;
  // proto: bool QWindow::isActive();
  fn _ZNK7QWindow8isActiveEv() -> i32;
  // proto: void QWindow::maximumHeightChanged(int arg);
  fn _ZN7QWindow20maximumHeightChangedEi(arg0: c_int) -> i32;
  // proto: QAccessibleInterface * QWindow::accessibleRoot();
  fn _ZNK7QWindow14accessibleRootEv() -> i32;
  // proto: QCursor QWindow::cursor();
  fn _ZNK7QWindow6cursorEv() -> i32;
  // proto: void QWindow::setFormat(const QSurfaceFormat & format);
  fn _ZN7QWindow9setFormatERK14QSurfaceFormat(arg0: *const c_void) -> i32;
  // proto: void QWindow::showFullScreen();
  fn _ZN7QWindow14showFullScreenEv() -> i32;
  // proto: void QWindow::setX(int arg);
  fn _ZN7QWindow4setXEi(arg0: c_int) -> i32;
  // proto: void QWindow::opacityChanged(qreal opacity);
  fn _ZN7QWindow14opacityChangedEd(arg0: c_double) -> i32;
  // proto: void QWindow::lower();
  fn _ZN7QWindow5lowerEv() -> i32;
  // proto: void QWindow::requestUpdate();
  fn _ZN7QWindow13requestUpdateEv() -> i32;
  // proto: void QWindow::hide();
  fn _ZN7QWindow4hideEv() -> i32;
  // proto: void QWindow::minimumHeightChanged(int arg);
  fn _ZN7QWindow20minimumHeightChangedEi(arg0: c_int) -> i32;
  // proto: void QWindow::setMask(const QRegion & region);
  fn _ZN7QWindow7setMaskERK7QRegion(arg0: *const c_void) -> i32;
  // proto: void QWindow::setMaximumSize(const QSize & size);
  fn _ZN7QWindow14setMaximumSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: int QWindow::height();
  fn _ZNK7QWindow6heightEv() -> i32;
  // proto: QSize QWindow::size();
  fn _ZNK7QWindow4sizeEv() -> i32;
  // proto: int QWindow::maximumWidth();
  fn _ZNK7QWindow12maximumWidthEv() -> i32;
  // proto: QPoint QWindow::position();
  fn _ZNK7QWindow8positionEv() -> i32;
  // proto: void QWindow::setMaximumHeight(int h);
  fn _ZN7QWindow16setMaximumHeightEi(arg0: c_int) -> i32;
  // proto: QString QWindow::filePath();
  fn _ZNK7QWindow8filePathEv() -> i32;
  // proto: void QWindow::showNormal();
  fn _ZN7QWindow10showNormalEv() -> i32;
  // proto: QPoint QWindow::framePosition();
  fn _ZNK7QWindow13framePositionEv() -> i32;
  // proto: void QWindow::visibleChanged(bool arg);
  fn _ZN7QWindow14visibleChangedEb(arg0: int8_t) -> i32;
  // proto: QIcon QWindow::icon();
  fn _ZNK7QWindow4iconEv() -> i32;
}

// body block begin
// class sizeof(QWindow)=1
pub struct QWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWindow {
  pub fn xChanged<T: QWindow_xChanged>(&mut self, value: T) -> i32 {
    value.xChanged(self);
    return 1;
  }
}

pub trait QWindow_xChanged {
  fn xChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::xChanged(int arg);
impl<'a> /*trait*/ QWindow_xChanged for (i32) {
  fn xChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8xChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow8xChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn unsetCursor<T: QWindow_unsetCursor>(&mut self, value: T) -> i32 {
    value.unsetCursor(self);
    return 1;
  }
}

pub trait QWindow_unsetCursor {
  fn unsetCursor(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::unsetCursor();
impl<'a> /*trait*/ QWindow_unsetCursor for () {
  fn unsetCursor(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11unsetCursorEv()};
    unsafe {_ZN7QWindow11unsetCursorEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isVisible<T: QWindow_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QWindow_isVisible {
  fn isVisible(self, this: &mut QWindow) -> i32;
}

// proto: bool QWindow::isVisible();
impl<'a> /*trait*/ QWindow_isVisible for () {
  fn isVisible(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow9isVisibleEv()};
    unsafe {_ZNK7QWindow9isVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setScreen<T: QWindow_setScreen>(&mut self, value: T) -> i32 {
    value.setScreen(self);
    return 1;
  }
}

pub trait QWindow_setScreen {
  fn setScreen(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setScreen(QScreen * screen);
impl<'a> /*trait*/ QWindow_setScreen for (&'a mut QScreen) {
  fn setScreen(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindow9setScreenEP7QScreen(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumSize<T: QWindow_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QWindow_maximumSize {
  fn maximumSize(self, this: &mut QWindow) -> i32;
}

// proto: QSize QWindow::maximumSize();
impl<'a> /*trait*/ QWindow_maximumSize for () {
  fn maximumSize(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11maximumSizeEv()};
    unsafe {_ZNK7QWindow11maximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setTransientParent<T: QWindow_setTransientParent>(&mut self, value: T) -> i32 {
    value.setTransientParent(self);
    return 1;
  }
}

pub trait QWindow_setTransientParent {
  fn setTransientParent(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setTransientParent(QWindow * parent);
impl<'a> /*trait*/ QWindow_setTransientParent for (&'a mut QWindow) {
  fn setTransientParent(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18setTransientParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindow18setTransientParentEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn format<T: QWindow_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QWindow_format {
  fn format(self, this: &mut QWindow) -> i32;
}

// proto: QSurfaceFormat QWindow::format();
impl<'a> /*trait*/ QWindow_format for () {
  fn format(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6formatEv()};
    unsafe {_ZNK7QWindow6formatEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isTopLevel<T: QWindow_isTopLevel>(&mut self, value: T) -> i32 {
    value.isTopLevel(self);
    return 1;
  }
}

pub trait QWindow_isTopLevel {
  fn isTopLevel(self, this: &mut QWindow) -> i32;
}

// proto: bool QWindow::isTopLevel();
impl<'a> /*trait*/ QWindow_isTopLevel for () {
  fn isTopLevel(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow10isTopLevelEv()};
    unsafe {_ZNK7QWindow10isTopLevelEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn NewQWindow<T: QWindow_NewQWindow>(value: T) -> QWindow {
    let rsthis = value.NewQWindow();
    return rsthis;
    // return 1;
  }
}

pub trait QWindow_NewQWindow {
  fn NewQWindow(self) -> QWindow;
}

// proto: void QWindow::NewQWindow(const QWindow & );
impl<'a> /*trait*/ QWindow_NewQWindow for (&'a  QWindow) {
  fn NewQWindow(self) -> QWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindowC1ERKS_(qthis, arg0)};
    let rsthis = QWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setIcon<T: QWindow_setIcon>(&mut self, value: T) -> i32 {
    value.setIcon(self);
    return 1;
  }
}

pub trait QWindow_setIcon {
  fn setIcon(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QWindow_setIcon for (&'a  QIcon) {
  fn setIcon(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow7setIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn opacity<T: QWindow_opacity>(&mut self, value: T) -> i32 {
    value.opacity(self);
    return 1;
  }
}

pub trait QWindow_opacity {
  fn opacity(self, this: &mut QWindow) -> i32;
}

// proto: double QWindow::opacity();
impl<'a> /*trait*/ QWindow_opacity for () {
  fn opacity(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow7opacityEv()};
    unsafe {_ZNK7QWindow7opacityEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMinimumSize<T: QWindow_setMinimumSize>(&mut self, value: T) -> i32 {
    value.setMinimumSize(self);
    return 1;
  }
}

pub trait QWindow_setMinimumSize {
  fn setMinimumSize(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setMinimumSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setMinimumSize for (&'a  QSize) {
  fn setMinimumSize(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14setMinimumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow14setMinimumSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumHeight<T: QWindow_minimumHeight>(&mut self, value: T) -> i32 {
    value.minimumHeight(self);
    return 1;
  }
}

pub trait QWindow_minimumHeight {
  fn minimumHeight(self, this: &mut QWindow) -> i32;
}

// proto: int QWindow::minimumHeight();
impl<'a> /*trait*/ QWindow_minimumHeight for () {
  fn minimumHeight(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13minimumHeightEv()};
    unsafe {_ZNK7QWindow13minimumHeightEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn sizeIncrement<T: QWindow_sizeIncrement>(&mut self, value: T) -> i32 {
    value.sizeIncrement(self);
    return 1;
  }
}

pub trait QWindow_sizeIncrement {
  fn sizeIncrement(self, this: &mut QWindow) -> i32;
}

// proto: QSize QWindow::sizeIncrement();
impl<'a> /*trait*/ QWindow_sizeIncrement for () {
  fn sizeIncrement(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13sizeIncrementEv()};
    unsafe {_ZNK7QWindow13sizeIncrementEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn resize<T: QWindow_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QWindow_resize {
  fn resize(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::resize(const QSize & newSize);
impl<'a> /*trait*/ QWindow_resize for (&'a  QSize) {
  fn resize(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow6resizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn screenChanged<T: QWindow_screenChanged>(&mut self, value: T) -> i32 {
    value.screenChanged(self);
    return 1;
  }
}

pub trait QWindow_screenChanged {
  fn screenChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::screenChanged(QScreen * screen);
impl<'a> /*trait*/ QWindow_screenChanged for (&'a mut QScreen) {
  fn screenChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13screenChangedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindow13screenChangedEP7QScreen(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setTitle<T: QWindow_setTitle>(&mut self, value: T) -> i32 {
    value.setTitle(self);
    return 1;
  }
}

pub trait QWindow_setTitle {
  fn setTitle(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setTitle(const QString & );
impl<'a> /*trait*/ QWindow_setTitle for (&'a  QString) {
  fn setTitle(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow8setTitleERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn raise<T: QWindow_raise>(&mut self, value: T) -> i32 {
    value.raise(self);
    return 1;
  }
}

pub trait QWindow_raise {
  fn raise(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::raise();
impl<'a> /*trait*/ QWindow_raise for () {
  fn raise(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5raiseEv()};
    unsafe {_ZN7QWindow5raiseEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumSize<T: QWindow_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QWindow_minimumSize {
  fn minimumSize(self, this: &mut QWindow) -> i32;
}

// proto: QSize QWindow::minimumSize();
impl<'a> /*trait*/ QWindow_minimumSize for () {
  fn minimumSize(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11minimumSizeEv()};
    unsafe {_ZNK7QWindow11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn mapToGlobal<T: QWindow_mapToGlobal>(&mut self, value: T) -> i32 {
    value.mapToGlobal(self);
    return 1;
  }
}

pub trait QWindow_mapToGlobal {
  fn mapToGlobal(self, this: &mut QWindow) -> i32;
}

// proto: QPoint QWindow::mapToGlobal(const QPoint & pos);
impl<'a> /*trait*/ QWindow_mapToGlobal for (&'a  QPoint) {
  fn mapToGlobal(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11mapToGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWindow11mapToGlobalERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn fromWinId<T: QWindow_fromWinId>(&mut self, value: T) -> i32 {
    value.fromWinId(self);
    return 1;
  }
}

pub trait QWindow_fromWinId {
  fn fromWinId(self, this: &mut QWindow) -> i32;
}

// proto: QWindow * QWindow::fromWinId(WId id);
impl<'a> /*trait*/ QWindow_fromWinId for (*mut i32) {
  fn fromWinId(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9fromWinIdEi()};
    let arg0 = self  as *mut c_uint;
    unsafe {_ZN7QWindow9fromWinIdEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn frameMargins<T: QWindow_frameMargins>(&mut self, value: T) -> i32 {
    value.frameMargins(self);
    return 1;
  }
}

pub trait QWindow_frameMargins {
  fn frameMargins(self, this: &mut QWindow) -> i32;
}

// proto: QMargins QWindow::frameMargins();
impl<'a> /*trait*/ QWindow_frameMargins for () {
  fn frameMargins(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12frameMarginsEv()};
    unsafe {_ZNK7QWindow12frameMarginsEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMaximumWidth<T: QWindow_setMaximumWidth>(&mut self, value: T) -> i32 {
    value.setMaximumWidth(self);
    return 1;
  }
}

pub trait QWindow_setMaximumWidth {
  fn setMaximumWidth(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setMaximumWidth(int w);
impl<'a> /*trait*/ QWindow_setMaximumWidth for (i32) {
  fn setMaximumWidth(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15setMaximumWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow15setMaximumWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumHeight<T: QWindow_maximumHeight>(&mut self, value: T) -> i32 {
    value.maximumHeight(self);
    return 1;
  }
}

pub trait QWindow_maximumHeight {
  fn maximumHeight(self, this: &mut QWindow) -> i32;
}

// proto: int QWindow::maximumHeight();
impl<'a> /*trait*/ QWindow_maximumHeight for () {
  fn maximumHeight(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13maximumHeightEv()};
    unsafe {_ZNK7QWindow13maximumHeightEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn focusObjectChanged<T: QWindow_focusObjectChanged>(&mut self, value: T) -> i32 {
    value.focusObjectChanged(self);
    return 1;
  }
}

pub trait QWindow_focusObjectChanged {
  fn focusObjectChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::focusObjectChanged(QObject * object);
impl<'a> /*trait*/ QWindow_focusObjectChanged for (&'a mut QObject) {
  fn focusObjectChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18focusObjectChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindow18focusObjectChangedEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isModal<T: QWindow_isModal>(&mut self, value: T) -> i32 {
    value.isModal(self);
    return 1;
  }
}

pub trait QWindow_isModal {
  fn isModal(self, this: &mut QWindow) -> i32;
}

// proto: bool QWindow::isModal();
impl<'a> /*trait*/ QWindow_isModal for () {
  fn isModal(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow7isModalEv()};
    unsafe {_ZNK7QWindow7isModalEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumWidthChanged<T: QWindow_maximumWidthChanged>(&mut self, value: T) -> i32 {
    value.maximumWidthChanged(self);
    return 1;
  }
}

pub trait QWindow_maximumWidthChanged {
  fn maximumWidthChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::maximumWidthChanged(int arg);
impl<'a> /*trait*/ QWindow_maximumWidthChanged for (i32) {
  fn maximumWidthChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19maximumWidthChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow19maximumWidthChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn geometry<T: QWindow_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QWindow_geometry {
  fn geometry(self, this: &mut QWindow) -> i32;
}

// proto: QRect QWindow::geometry();
impl<'a> /*trait*/ QWindow_geometry for () {
  fn geometry(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8geometryEv()};
    unsafe {_ZNK7QWindow8geometryEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setParent<T: QWindow_setParent>(&mut self, value: T) -> i32 {
    value.setParent(self);
    return 1;
  }
}

pub trait QWindow_setParent {
  fn setParent(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setParent(QWindow * parent);
impl<'a> /*trait*/ QWindow_setParent for (&'a mut QWindow) {
  fn setParent(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindow9setParentEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn frameGeometry<T: QWindow_frameGeometry>(&mut self, value: T) -> i32 {
    value.frameGeometry(self);
    return 1;
  }
}

pub trait QWindow_frameGeometry {
  fn frameGeometry(self, this: &mut QWindow) -> i32;
}

// proto: QRect QWindow::frameGeometry();
impl<'a> /*trait*/ QWindow_frameGeometry for () {
  fn frameGeometry(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13frameGeometryEv()};
    unsafe {_ZNK7QWindow13frameGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn requestedFormat<T: QWindow_requestedFormat>(&mut self, value: T) -> i32 {
    value.requestedFormat(self);
    return 1;
  }
}

pub trait QWindow_requestedFormat {
  fn requestedFormat(self, this: &mut QWindow) -> i32;
}

// proto: QSurfaceFormat QWindow::requestedFormat();
impl<'a> /*trait*/ QWindow_requestedFormat for () {
  fn requestedFormat(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow15requestedFormatEv()};
    unsafe {_ZNK7QWindow15requestedFormatEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setHeight<T: QWindow_setHeight>(&mut self, value: T) -> i32 {
    value.setHeight(self);
    return 1;
  }
}

pub trait QWindow_setHeight {
  fn setHeight(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setHeight(int arg);
impl<'a> /*trait*/ QWindow_setHeight for (i32) {
  fn setHeight(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow9setHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn requestActivate<T: QWindow_requestActivate>(&mut self, value: T) -> i32 {
    value.requestActivate(self);
    return 1;
  }
}

pub trait QWindow_requestActivate {
  fn requestActivate(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::requestActivate();
impl<'a> /*trait*/ QWindow_requestActivate for () {
  fn requestActivate(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15requestActivateEv()};
    unsafe {_ZN7QWindow15requestActivateEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn mapFromGlobal<T: QWindow_mapFromGlobal>(&mut self, value: T) -> i32 {
    value.mapFromGlobal(self);
    return 1;
  }
}

pub trait QWindow_mapFromGlobal {
  fn mapFromGlobal(self, this: &mut QWindow) -> i32;
}

// proto: QPoint QWindow::mapFromGlobal(const QPoint & pos);
impl<'a> /*trait*/ QWindow_mapFromGlobal for (&'a  QPoint) {
  fn mapFromGlobal(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13mapFromGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWindow13mapFromGlobalERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn windowTitleChanged<T: QWindow_windowTitleChanged>(&mut self, value: T) -> i32 {
    value.windowTitleChanged(self);
    return 1;
  }
}

pub trait QWindow_windowTitleChanged {
  fn windowTitleChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::windowTitleChanged(const QString & title);
impl<'a> /*trait*/ QWindow_windowTitleChanged for (&'a  QString) {
  fn windowTitleChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow18windowTitleChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow18windowTitleChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn y<T: QWindow_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QWindow_y {
  fn y(self, this: &mut QWindow) -> i32;
}

// proto: int QWindow::y();
impl<'a> /*trait*/ QWindow_y for () {
  fn y(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow1yEv()};
    unsafe {_ZNK7QWindow1yEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn width<T: QWindow_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QWindow_width {
  fn width(self, this: &mut QWindow) -> i32;
}

// proto: int QWindow::width();
impl<'a> /*trait*/ QWindow_width for () {
  fn width(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow5widthEv()};
    unsafe {_ZNK7QWindow5widthEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setFilePath<T: QWindow_setFilePath>(&mut self, value: T) -> i32 {
    value.setFilePath(self);
    return 1;
  }
}

pub trait QWindow_setFilePath {
  fn setFilePath(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setFilePath(const QString & filePath);
impl<'a> /*trait*/ QWindow_setFilePath for (&'a  QString) {
  fn setFilePath(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setFilePathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow11setFilePathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setCursor<T: QWindow_setCursor>(&mut self, value: T) -> i32 {
    value.setCursor(self);
    return 1;
  }
}

pub trait QWindow_setCursor {
  fn setCursor(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setCursor(const QCursor & );
impl<'a> /*trait*/ QWindow_setCursor for (&'a  QCursor) {
  fn setCursor(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow9setCursorERK7QCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setVisible<T: QWindow_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QWindow_setVisible {
  fn setVisible(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setVisible(bool visible);
impl<'a> /*trait*/ QWindow_setVisible for (i8) {
  fn setVisible(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWindow10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn FreeQWindow<T: QWindow_FreeQWindow>(&mut self, value: T) -> i32 {
    value.FreeQWindow(self);
    return 1;
  }
}

pub trait QWindow_FreeQWindow {
  fn FreeQWindow(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::FreeQWindow();
impl<'a> /*trait*/ QWindow_FreeQWindow for () {
  fn FreeQWindow(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowD0Ev()};
    unsafe {_ZN7QWindowD0Ev()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMouseGrabEnabled<T: QWindow_setMouseGrabEnabled>(&mut self, value: T) -> i32 {
    value.setMouseGrabEnabled(self);
    return 1;
  }
}

pub trait QWindow_setMouseGrabEnabled {
  fn setMouseGrabEnabled(self, this: &mut QWindow) -> i32;
}

// proto: bool QWindow::setMouseGrabEnabled(bool grab);
impl<'a> /*trait*/ QWindow_setMouseGrabEnabled for (i8) {
  fn setMouseGrabEnabled(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19setMouseGrabEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWindow19setMouseGrabEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isExposed<T: QWindow_isExposed>(&mut self, value: T) -> i32 {
    value.isExposed(self);
    return 1;
  }
}

pub trait QWindow_isExposed {
  fn isExposed(self, this: &mut QWindow) -> i32;
}

// proto: bool QWindow::isExposed();
impl<'a> /*trait*/ QWindow_isExposed for () {
  fn isExposed(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow9isExposedEv()};
    unsafe {_ZNK7QWindow9isExposedEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn heightChanged<T: QWindow_heightChanged>(&mut self, value: T) -> i32 {
    value.heightChanged(self);
    return 1;
  }
}

pub trait QWindow_heightChanged {
  fn heightChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::heightChanged(int arg);
impl<'a> /*trait*/ QWindow_heightChanged for (i32) {
  fn heightChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13heightChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow13heightChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumWidth<T: QWindow_minimumWidth>(&mut self, value: T) -> i32 {
    value.minimumWidth(self);
    return 1;
  }
}

pub trait QWindow_minimumWidth {
  fn minimumWidth(self, this: &mut QWindow) -> i32;
}

// proto: int QWindow::minimumWidth();
impl<'a> /*trait*/ QWindow_minimumWidth for () {
  fn minimumWidth(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12minimumWidthEv()};
    unsafe {_ZNK7QWindow12minimumWidthEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setPosition<T: QWindow_setPosition>(&mut self, value: T) -> i32 {
    value.setPosition(self);
    return 1;
  }
}

pub trait QWindow_setPosition {
  fn setPosition(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setPosition(const QPoint & pt);
impl<'a> /*trait*/ QWindow_setPosition for (&'a  QPoint) {
  fn setPosition(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setPositionERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow11setPositionERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn close<T: QWindow_close>(&mut self, value: T) -> i32 {
    value.close(self);
    return 1;
  }
}

pub trait QWindow_close {
  fn close(self, this: &mut QWindow) -> i32;
}

// proto: bool QWindow::close();
impl<'a> /*trait*/ QWindow_close for () {
  fn close(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5closeEv()};
    unsafe {_ZN7QWindow5closeEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn x<T: QWindow_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QWindow_x {
  fn x(self, this: &mut QWindow) -> i32;
}

// proto: int QWindow::x();
impl<'a> /*trait*/ QWindow_x for () {
  fn x(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow1xEv()};
    unsafe {_ZNK7QWindow1xEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMinimumWidth<T: QWindow_setMinimumWidth>(&mut self, value: T) -> i32 {
    value.setMinimumWidth(self);
    return 1;
  }
}

pub trait QWindow_setMinimumWidth {
  fn setMinimumWidth(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setMinimumWidth(int w);
impl<'a> /*trait*/ QWindow_setMinimumWidth for (i32) {
  fn setMinimumWidth(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow15setMinimumWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow15setMinimumWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn mask<T: QWindow_mask>(&mut self, value: T) -> i32 {
    value.mask(self);
    return 1;
  }
}

pub trait QWindow_mask {
  fn mask(self, this: &mut QWindow) -> i32;
}

// proto: QRegion QWindow::mask();
impl<'a> /*trait*/ QWindow_mask for () {
  fn mask(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4maskEv()};
    unsafe {_ZNK7QWindow4maskEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn widthChanged<T: QWindow_widthChanged>(&mut self, value: T) -> i32 {
    value.widthChanged(self);
    return 1;
  }
}

pub trait QWindow_widthChanged {
  fn widthChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::widthChanged(int arg);
impl<'a> /*trait*/ QWindow_widthChanged for (i32) {
  fn widthChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow12widthChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow12widthChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn parent<T: QWindow_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QWindow_parent {
  fn parent(self, this: &mut QWindow) -> i32;
}

// proto: QWindow * QWindow::parent();
impl<'a> /*trait*/ QWindow_parent for () {
  fn parent(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6parentEv()};
    unsafe {_ZNK7QWindow6parentEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setFramePosition<T: QWindow_setFramePosition>(&mut self, value: T) -> i32 {
    value.setFramePosition(self);
    return 1;
  }
}

pub trait QWindow_setFramePosition {
  fn setFramePosition(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setFramePosition(const QPoint & point);
impl<'a> /*trait*/ QWindow_setFramePosition for (&'a  QPoint) {
  fn setFramePosition(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setFramePositionERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow16setFramePositionERK6QPoint(arg0)};
    return 1;
  }
}

// proto: void QWindow::NewQWindow(QScreen * screen);
impl<'a> /*trait*/ QWindow_NewQWindow for (&'a mut QScreen) {
  fn NewQWindow(self) -> QWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowC1EP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindowC1EP7QScreen(qthis, arg0)};
    let rsthis = QWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setGeometry<T: QWindow_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QWindow_setGeometry {
  fn setGeometry(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setGeometry(int posx, int posy, int w, int h);
impl<'a> /*trait*/ QWindow_setGeometry for (i32, i32, i32, i32) {
  fn setGeometry(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setGeometryEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN7QWindow11setGeometryEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setKeyboardGrabEnabled<T: QWindow_setKeyboardGrabEnabled>(&mut self, value: T) -> i32 {
    value.setKeyboardGrabEnabled(self);
    return 1;
  }
}

pub trait QWindow_setKeyboardGrabEnabled {
  fn setKeyboardGrabEnabled(self, this: &mut QWindow) -> i32;
}

// proto: bool QWindow::setKeyboardGrabEnabled(bool grab);
impl<'a> /*trait*/ QWindow_setKeyboardGrabEnabled for (i8) {
  fn setKeyboardGrabEnabled(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow22setKeyboardGrabEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWindow22setKeyboardGrabEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn metaObject<T: QWindow_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QWindow_metaObject {
  fn metaObject(self, this: &mut QWindow) -> i32;
}

// proto: const QMetaObject * QWindow::metaObject();
impl<'a> /*trait*/ QWindow_metaObject for () {
  fn metaObject(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow10metaObjectEv()};
    unsafe {_ZNK7QWindow10metaObjectEv()};
    return 1;
  }
}

// proto: void QWindow::NewQWindow(QWindow * parent);
impl<'a> /*trait*/ QWindow_NewQWindow for (&'a mut QWindow) {
  fn NewQWindow(self) -> QWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindowC1EPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWindowC1EPS_(qthis, arg0)};
    let rsthis = QWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn activeChanged<T: QWindow_activeChanged>(&mut self, value: T) -> i32 {
    value.activeChanged(self);
    return 1;
  }
}

pub trait QWindow_activeChanged {
  fn activeChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::activeChanged();
impl<'a> /*trait*/ QWindow_activeChanged for () {
  fn activeChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13activeChangedEv()};
    unsafe {_ZN7QWindow13activeChangedEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setWidth<T: QWindow_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QWindow_setWidth {
  fn setWidth(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setWidth(int arg);
impl<'a> /*trait*/ QWindow_setWidth for (i32) {
  fn setWidth(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8setWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow8setWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setY<T: QWindow_setY>(&mut self, value: T) -> i32 {
    value.setY(self);
    return 1;
  }
}

pub trait QWindow_setY {
  fn setY(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setY(int arg);
impl<'a> /*trait*/ QWindow_setY for (i32) {
  fn setY(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4setYEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow4setYEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn devicePixelRatio<T: QWindow_devicePixelRatio>(&mut self, value: T) -> i32 {
    value.devicePixelRatio(self);
    return 1;
  }
}

pub trait QWindow_devicePixelRatio {
  fn devicePixelRatio(self, this: &mut QWindow) -> i32;
}

// proto: double QWindow::devicePixelRatio();
impl<'a> /*trait*/ QWindow_devicePixelRatio for () {
  fn devicePixelRatio(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow16devicePixelRatioEv()};
    unsafe {_ZNK7QWindow16devicePixelRatioEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setBaseSize<T: QWindow_setBaseSize>(&mut self, value: T) -> i32 {
    value.setBaseSize(self);
    return 1;
  }
}

pub trait QWindow_setBaseSize {
  fn setBaseSize(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setBaseSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setBaseSize for (&'a  QSize) {
  fn setBaseSize(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setBaseSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow11setBaseSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn alert<T: QWindow_alert>(&mut self, value: T) -> i32 {
    value.alert(self);
    return 1;
  }
}

pub trait QWindow_alert {
  fn alert(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::alert(int msec);
impl<'a> /*trait*/ QWindow_alert for (i32) {
  fn alert(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5alertEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow5alertEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn yChanged<T: QWindow_yChanged>(&mut self, value: T) -> i32 {
    value.yChanged(self);
    return 1;
  }
}

pub trait QWindow_yChanged {
  fn yChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::yChanged(int arg);
impl<'a> /*trait*/ QWindow_yChanged for (i32) {
  fn yChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow8yChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow8yChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn handle<T: QWindow_handle>(&mut self, value: T) -> i32 {
    value.handle(self);
    return 1;
  }
}

pub trait QWindow_handle {
  fn handle(self, this: &mut QWindow) -> i32;
}

// proto: QPlatformWindow * QWindow::handle();
impl<'a> /*trait*/ QWindow_handle for () {
  fn handle(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6handleEv()};
    unsafe {_ZNK7QWindow6handleEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn destroy<T: QWindow_destroy>(&mut self, value: T) -> i32 {
    value.destroy(self);
    return 1;
  }
}

pub trait QWindow_destroy {
  fn destroy(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::destroy();
impl<'a> /*trait*/ QWindow_destroy for () {
  fn destroy(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7destroyEv()};
    unsafe {_ZN7QWindow7destroyEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn transientParent<T: QWindow_transientParent>(&mut self, value: T) -> i32 {
    value.transientParent(self);
    return 1;
  }
}

pub trait QWindow_transientParent {
  fn transientParent(self, this: &mut QWindow) -> i32;
}

// proto: QWindow * QWindow::transientParent();
impl<'a> /*trait*/ QWindow_transientParent for () {
  fn transientParent(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow15transientParentEv()};
    unsafe {_ZNK7QWindow15transientParentEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMinimumHeight<T: QWindow_setMinimumHeight>(&mut self, value: T) -> i32 {
    value.setMinimumHeight(self);
    return 1;
  }
}

pub trait QWindow_setMinimumHeight {
  fn setMinimumHeight(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setMinimumHeight(int h);
impl<'a> /*trait*/ QWindow_setMinimumHeight for (i32) {
  fn setMinimumHeight(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setMinimumHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow16setMinimumHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn show<T: QWindow_show>(&mut self, value: T) -> i32 {
    value.show(self);
    return 1;
  }
}

pub trait QWindow_show {
  fn show(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::show();
impl<'a> /*trait*/ QWindow_show for () {
  fn show(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4showEv()};
    unsafe {_ZN7QWindow4showEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumWidthChanged<T: QWindow_minimumWidthChanged>(&mut self, value: T) -> i32 {
    value.minimumWidthChanged(self);
    return 1;
  }
}

pub trait QWindow_minimumWidthChanged {
  fn minimumWidthChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::minimumWidthChanged(int arg);
impl<'a> /*trait*/ QWindow_minimumWidthChanged for (i32) {
  fn minimumWidthChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow19minimumWidthChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow19minimumWidthChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn baseSize<T: QWindow_baseSize>(&mut self, value: T) -> i32 {
    value.baseSize(self);
    return 1;
  }
}

pub trait QWindow_baseSize {
  fn baseSize(self, this: &mut QWindow) -> i32;
}

// proto: QSize QWindow::baseSize();
impl<'a> /*trait*/ QWindow_baseSize for () {
  fn baseSize(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8baseSizeEv()};
    unsafe {_ZNK7QWindow8baseSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn title<T: QWindow_title>(&mut self, value: T) -> i32 {
    value.title(self);
    return 1;
  }
}

pub trait QWindow_title {
  fn title(self, this: &mut QWindow) -> i32;
}

// proto: QString QWindow::title();
impl<'a> /*trait*/ QWindow_title for () {
  fn title(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow5titleEv()};
    unsafe {_ZNK7QWindow5titleEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn showMaximized<T: QWindow_showMaximized>(&mut self, value: T) -> i32 {
    value.showMaximized(self);
    return 1;
  }
}

pub trait QWindow_showMaximized {
  fn showMaximized(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::showMaximized();
impl<'a> /*trait*/ QWindow_showMaximized for () {
  fn showMaximized(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13showMaximizedEv()};
    unsafe {_ZN7QWindow13showMaximizedEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn create<T: QWindow_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QWindow_create {
  fn create(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::create();
impl<'a> /*trait*/ QWindow_create for () {
  fn create(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6createEv()};
    unsafe {_ZN7QWindow6createEv()};
    return 1;
  }
}

// proto: void QWindow::resize(int w, int h);
impl<'a> /*trait*/ QWindow_resize for (i32, i32) {
  fn resize(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWindow6resizeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn screen<T: QWindow_screen>(&mut self, value: T) -> i32 {
    value.screen(self);
    return 1;
  }
}

pub trait QWindow_screen {
  fn screen(self, this: &mut QWindow) -> i32;
}

// proto: QScreen * QWindow::screen();
impl<'a> /*trait*/ QWindow_screen for () {
  fn screen(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6screenEv()};
    unsafe {_ZNK7QWindow6screenEv()};
    return 1;
  }
}

// proto: void QWindow::setPosition(int posx, int posy);
impl<'a> /*trait*/ QWindow_setPosition for (i32, i32) {
  fn setPosition(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setPositionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWindow11setPositionEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setOpacity<T: QWindow_setOpacity>(&mut self, value: T) -> i32 {
    value.setOpacity(self);
    return 1;
  }
}

pub trait QWindow_setOpacity {
  fn setOpacity(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setOpacity(qreal level);
impl<'a> /*trait*/ QWindow_setOpacity for (f64) {
  fn setOpacity(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10setOpacityEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QWindow10setOpacityEd(arg0)};
    return 1;
  }
}

// proto: void QWindow::setGeometry(const QRect & rect);
impl<'a> /*trait*/ QWindow_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setSizeIncrement<T: QWindow_setSizeIncrement>(&mut self, value: T) -> i32 {
    value.setSizeIncrement(self);
    return 1;
  }
}

pub trait QWindow_setSizeIncrement {
  fn setSizeIncrement(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setSizeIncrement(const QSize & size);
impl<'a> /*trait*/ QWindow_setSizeIncrement for (&'a  QSize) {
  fn setSizeIncrement(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setSizeIncrementERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow16setSizeIncrementERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn showMinimized<T: QWindow_showMinimized>(&mut self, value: T) -> i32 {
    value.showMinimized(self);
    return 1;
  }
}

pub trait QWindow_showMinimized {
  fn showMinimized(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::showMinimized();
impl<'a> /*trait*/ QWindow_showMinimized for () {
  fn showMinimized(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13showMinimizedEv()};
    unsafe {_ZN7QWindow13showMinimizedEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn focusObject<T: QWindow_focusObject>(&mut self, value: T) -> i32 {
    value.focusObject(self);
    return 1;
  }
}

pub trait QWindow_focusObject {
  fn focusObject(self, this: &mut QWindow) -> i32;
}

// proto: QObject * QWindow::focusObject();
impl<'a> /*trait*/ QWindow_focusObject for () {
  fn focusObject(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow11focusObjectEv()};
    unsafe {_ZNK7QWindow11focusObjectEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn isActive<T: QWindow_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QWindow_isActive {
  fn isActive(self, this: &mut QWindow) -> i32;
}

// proto: bool QWindow::isActive();
impl<'a> /*trait*/ QWindow_isActive for () {
  fn isActive(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8isActiveEv()};
    unsafe {_ZNK7QWindow8isActiveEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumHeightChanged<T: QWindow_maximumHeightChanged>(&mut self, value: T) -> i32 {
    value.maximumHeightChanged(self);
    return 1;
  }
}

pub trait QWindow_maximumHeightChanged {
  fn maximumHeightChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::maximumHeightChanged(int arg);
impl<'a> /*trait*/ QWindow_maximumHeightChanged for (i32) {
  fn maximumHeightChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow20maximumHeightChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow20maximumHeightChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn accessibleRoot<T: QWindow_accessibleRoot>(&mut self, value: T) -> i32 {
    value.accessibleRoot(self);
    return 1;
  }
}

pub trait QWindow_accessibleRoot {
  fn accessibleRoot(self, this: &mut QWindow) -> i32;
}

// proto: QAccessibleInterface * QWindow::accessibleRoot();
impl<'a> /*trait*/ QWindow_accessibleRoot for () {
  fn accessibleRoot(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow14accessibleRootEv()};
    unsafe {_ZNK7QWindow14accessibleRootEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn cursor<T: QWindow_cursor>(&mut self, value: T) -> i32 {
    value.cursor(self);
    return 1;
  }
}

pub trait QWindow_cursor {
  fn cursor(self, this: &mut QWindow) -> i32;
}

// proto: QCursor QWindow::cursor();
impl<'a> /*trait*/ QWindow_cursor for () {
  fn cursor(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6cursorEv()};
    unsafe {_ZNK7QWindow6cursorEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setFormat<T: QWindow_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QWindow_setFormat {
  fn setFormat(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QWindow_setFormat for (&'a  QSurfaceFormat) {
  fn setFormat(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow9setFormatERK14QSurfaceFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn showFullScreen<T: QWindow_showFullScreen>(&mut self, value: T) -> i32 {
    value.showFullScreen(self);
    return 1;
  }
}

pub trait QWindow_showFullScreen {
  fn showFullScreen(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::showFullScreen();
impl<'a> /*trait*/ QWindow_showFullScreen for () {
  fn showFullScreen(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14showFullScreenEv()};
    unsafe {_ZN7QWindow14showFullScreenEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setX<T: QWindow_setX>(&mut self, value: T) -> i32 {
    value.setX(self);
    return 1;
  }
}

pub trait QWindow_setX {
  fn setX(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setX(int arg);
impl<'a> /*trait*/ QWindow_setX for (i32) {
  fn setX(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4setXEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow4setXEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn opacityChanged<T: QWindow_opacityChanged>(&mut self, value: T) -> i32 {
    value.opacityChanged(self);
    return 1;
  }
}

pub trait QWindow_opacityChanged {
  fn opacityChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::opacityChanged(qreal opacity);
impl<'a> /*trait*/ QWindow_opacityChanged for (f64) {
  fn opacityChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14opacityChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QWindow14opacityChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn lower<T: QWindow_lower>(&mut self, value: T) -> i32 {
    value.lower(self);
    return 1;
  }
}

pub trait QWindow_lower {
  fn lower(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::lower();
impl<'a> /*trait*/ QWindow_lower for () {
  fn lower(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow5lowerEv()};
    unsafe {_ZN7QWindow5lowerEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn requestUpdate<T: QWindow_requestUpdate>(&mut self, value: T) -> i32 {
    value.requestUpdate(self);
    return 1;
  }
}

pub trait QWindow_requestUpdate {
  fn requestUpdate(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::requestUpdate();
impl<'a> /*trait*/ QWindow_requestUpdate for () {
  fn requestUpdate(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow13requestUpdateEv()};
    unsafe {_ZN7QWindow13requestUpdateEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn hide<T: QWindow_hide>(&mut self, value: T) -> i32 {
    value.hide(self);
    return 1;
  }
}

pub trait QWindow_hide {
  fn hide(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::hide();
impl<'a> /*trait*/ QWindow_hide for () {
  fn hide(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow4hideEv()};
    unsafe {_ZN7QWindow4hideEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn minimumHeightChanged<T: QWindow_minimumHeightChanged>(&mut self, value: T) -> i32 {
    value.minimumHeightChanged(self);
    return 1;
  }
}

pub trait QWindow_minimumHeightChanged {
  fn minimumHeightChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::minimumHeightChanged(int arg);
impl<'a> /*trait*/ QWindow_minimumHeightChanged for (i32) {
  fn minimumHeightChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow20minimumHeightChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow20minimumHeightChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMask<T: QWindow_setMask>(&mut self, value: T) -> i32 {
    value.setMask(self);
    return 1;
  }
}

pub trait QWindow_setMask {
  fn setMask(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setMask(const QRegion & region);
impl<'a> /*trait*/ QWindow_setMask for (&'a  QRegion) {
  fn setMask(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow7setMaskERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow7setMaskERK7QRegion(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMaximumSize<T: QWindow_setMaximumSize>(&mut self, value: T) -> i32 {
    value.setMaximumSize(self);
    return 1;
  }
}

pub trait QWindow_setMaximumSize {
  fn setMaximumSize(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setMaximumSize(const QSize & size);
impl<'a> /*trait*/ QWindow_setMaximumSize for (&'a  QSize) {
  fn setMaximumSize(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14setMaximumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWindow14setMaximumSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn height<T: QWindow_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QWindow_height {
  fn height(self, this: &mut QWindow) -> i32;
}

// proto: int QWindow::height();
impl<'a> /*trait*/ QWindow_height for () {
  fn height(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow6heightEv()};
    unsafe {_ZNK7QWindow6heightEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn size<T: QWindow_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QWindow_size {
  fn size(self, this: &mut QWindow) -> i32;
}

// proto: QSize QWindow::size();
impl<'a> /*trait*/ QWindow_size for () {
  fn size(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4sizeEv()};
    unsafe {_ZNK7QWindow4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn maximumWidth<T: QWindow_maximumWidth>(&mut self, value: T) -> i32 {
    value.maximumWidth(self);
    return 1;
  }
}

pub trait QWindow_maximumWidth {
  fn maximumWidth(self, this: &mut QWindow) -> i32;
}

// proto: int QWindow::maximumWidth();
impl<'a> /*trait*/ QWindow_maximumWidth for () {
  fn maximumWidth(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow12maximumWidthEv()};
    unsafe {_ZNK7QWindow12maximumWidthEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn position<T: QWindow_position>(&mut self, value: T) -> i32 {
    value.position(self);
    return 1;
  }
}

pub trait QWindow_position {
  fn position(self, this: &mut QWindow) -> i32;
}

// proto: QPoint QWindow::position();
impl<'a> /*trait*/ QWindow_position for () {
  fn position(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8positionEv()};
    unsafe {_ZNK7QWindow8positionEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn setMaximumHeight<T: QWindow_setMaximumHeight>(&mut self, value: T) -> i32 {
    value.setMaximumHeight(self);
    return 1;
  }
}

pub trait QWindow_setMaximumHeight {
  fn setMaximumHeight(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::setMaximumHeight(int h);
impl<'a> /*trait*/ QWindow_setMaximumHeight for (i32) {
  fn setMaximumHeight(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow16setMaximumHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWindow16setMaximumHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn filePath<T: QWindow_filePath>(&mut self, value: T) -> i32 {
    value.filePath(self);
    return 1;
  }
}

pub trait QWindow_filePath {
  fn filePath(self, this: &mut QWindow) -> i32;
}

// proto: QString QWindow::filePath();
impl<'a> /*trait*/ QWindow_filePath for () {
  fn filePath(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow8filePathEv()};
    unsafe {_ZNK7QWindow8filePathEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn showNormal<T: QWindow_showNormal>(&mut self, value: T) -> i32 {
    value.showNormal(self);
    return 1;
  }
}

pub trait QWindow_showNormal {
  fn showNormal(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::showNormal();
impl<'a> /*trait*/ QWindow_showNormal for () {
  fn showNormal(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow10showNormalEv()};
    unsafe {_ZN7QWindow10showNormalEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn framePosition<T: QWindow_framePosition>(&mut self, value: T) -> i32 {
    value.framePosition(self);
    return 1;
  }
}

pub trait QWindow_framePosition {
  fn framePosition(self, this: &mut QWindow) -> i32;
}

// proto: QPoint QWindow::framePosition();
impl<'a> /*trait*/ QWindow_framePosition for () {
  fn framePosition(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow13framePositionEv()};
    unsafe {_ZNK7QWindow13framePositionEv()};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn visibleChanged<T: QWindow_visibleChanged>(&mut self, value: T) -> i32 {
    value.visibleChanged(self);
    return 1;
  }
}

pub trait QWindow_visibleChanged {
  fn visibleChanged(self, this: &mut QWindow) -> i32;
}

// proto: void QWindow::visibleChanged(bool arg);
impl<'a> /*trait*/ QWindow_visibleChanged for (i8) {
  fn visibleChanged(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWindow14visibleChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWindow14visibleChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWindow {
  pub fn icon<T: QWindow_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QWindow_icon {
  fn icon(self, this: &mut QWindow) -> i32;
}

// proto: QIcon QWindow::icon();
impl<'a> /*trait*/ QWindow_icon for () {
  fn icon(self, this: &mut QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWindow4iconEv()};
    unsafe {_ZNK7QWindow4iconEv()};
    return 1;
  }
}

