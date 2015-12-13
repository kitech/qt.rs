// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicseffect::QGraphicsEffect;
use super::qstring::QString;
use super::qsize::QSize;
use super::qpoint::QPoint;
use super::qrect::QRect;
use super::qregion::QRegion;
use super::qaction::QAction;
use super::qpalette::QPalette;
use super::qstyle::QStyle;
use super::qcursor::QCursor;
use super::qmargins::QMargins;
use super::qsizepolicy::QSizePolicy;
use super::qicon::QIcon;
use super::qbytearray::QByteArray;
use super::qbitmap::QBitmap;
use super::qlayout::QLayout;
use super::qfont::QFont;
use super::qlocale::QLocale;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QWidget::setGraphicsEffect(QGraphicsEffect * effect);
  fn _ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect(arg0: *mut c_void) -> i32;
  // proto: QString QWidget::accessibleDescription();
  fn _ZNK7QWidget21accessibleDescriptionEv() -> i32;
  // proto: QGraphicsEffect * QWidget::graphicsEffect();
  fn _ZNK7QWidget14graphicsEffectEv() -> i32;
  // proto: bool QWidget::isFullScreen();
  fn _ZNK7QWidget12isFullScreenEv() -> i32;
  // proto: QSize QWidget::maximumSize();
  fn _ZNK7QWidget11maximumSizeEv() -> i32;
  // proto: bool QWidget::isEnabledToTLW();
  fn _ZNK7QWidget14isEnabledToTLWEv() -> i32;
  // proto: void QWidget::setStatusTip(const QString & );
  fn _ZN7QWidget12setStatusTipERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWidget::setSizeIncrement(const QSize & );
  fn _ZN7QWidget16setSizeIncrementERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QWidget::customContextMenuRequested(const QPoint & pos);
  fn _ZN7QWidget26customContextMenuRequestedERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QWidget * QWidget::focusWidget();
  fn _ZNK7QWidget11focusWidgetEv() -> i32;
  // proto: bool QWidget::isTopLevel();
  fn _ZNK7QWidget10isTopLevelEv() -> i32;
  // proto: bool QWidget::acceptDrops();
  fn _ZNK7QWidget11acceptDropsEv() -> i32;
  // proto: bool QWidget::isWindow();
  fn _ZNK7QWidget8isWindowEv() -> i32;
  // proto: void QWidget::setFixedSize(const QSize & );
  fn _ZN7QWidget12setFixedSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: bool QWidget::isVisible();
  fn _ZNK7QWidget9isVisibleEv() -> i32;
  // proto: int QWidget::minimumHeight();
  fn _ZNK7QWidget13minimumHeightEv() -> i32;
  // proto: QSize QWidget::sizeIncrement();
  fn _ZNK7QWidget13sizeIncrementEv() -> i32;
  // proto: void QWidget::repaint(const QRect & );
  fn _ZN7QWidget7repaintERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QWidget::update(int x, int y, int w, int h);
  fn _ZN7QWidget6updateEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: QString QWidget::windowFilePath();
  fn _ZNK7QWidget14windowFilePathEv() -> i32;
  // proto: void QWidget::clearMask();
  fn _ZN7QWidget9clearMaskEv() -> i32;
  // proto: QPoint QWidget::mapFromParent(const QPoint & );
  fn _ZNK7QWidget13mapFromParentERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QRect QWidget::rect();
  fn _ZNK7QWidget4rectEv() -> i32;
  // proto: void QWidget::unsetLayoutDirection();
  fn _ZN7QWidget20unsetLayoutDirectionEv() -> i32;
  // proto: void QWidget::setMinimumSize(const QSize & );
  fn _ZN7QWidget14setMinimumSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: bool QWidget::isActiveWindow();
  fn _ZNK7QWidget14isActiveWindowEv() -> i32;
  // proto: void QWidget::grabKeyboard();
  fn _ZN7QWidget12grabKeyboardEv() -> i32;
  // proto: QSize QWidget::frameSize();
  fn _ZNK7QWidget9frameSizeEv() -> i32;
  // proto: void QWidget::setFocus();
  fn _ZN7QWidget8setFocusEv() -> i32;
  // proto: QPoint QWidget::mapToParent(const QPoint & );
  fn _ZNK7QWidget11mapToParentERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QWidget::updateGeometry();
  fn _ZN7QWidget14updateGeometryEv() -> i32;
  // proto: void QWidget::repaint(const QRegion & );
  fn _ZN7QWidget7repaintERK7QRegion(arg0: *const c_void) -> i32;
  // proto: void QWidget::insertAction(QAction * before, QAction * action);
  fn _ZN7QWidget12insertActionEP7QActionS1_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: void QWidget::setWindowRole(const QString & );
  fn _ZN7QWidget13setWindowRoleERK7QString(arg0: *const c_void) -> i32;
  // proto: int QWidget::toolTipDuration();
  fn _ZNK7QWidget15toolTipDurationEv() -> i32;
  // proto: void QWidget::setPalette(const QPalette & );
  fn _ZN7QWidget10setPaletteERK8QPalette(arg0: *const c_void) -> i32;
  // proto: void QWidget::setStyleSheet(const QString & styleSheet);
  fn _ZN7QWidget13setStyleSheetERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QWidget::windowIconText();
  fn _ZNK7QWidget14windowIconTextEv() -> i32;
  // proto: void QWidget::releaseMouse();
  fn _ZN7QWidget12releaseMouseEv() -> i32;
  // proto: bool QWidget::isModal();
  fn _ZNK7QWidget7isModalEv() -> i32;
  // proto: void QWidget::setStyle(QStyle * );
  fn _ZN7QWidget8setStyleEP6QStyle(arg0: *mut c_void) -> i32;
  // proto: void QWidget::repaint();
  fn _ZN7QWidget7repaintEv() -> i32;
  // proto: void QWidget::setBaseSize(int basew, int baseh);
  fn _ZN7QWidget11setBaseSizeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QWidget::isWindowModified();
  fn _ZNK7QWidget16isWindowModifiedEv() -> i32;
  // proto: const QRect & QWidget::geometry();
  fn _ZNK7QWidget8geometryEv() -> i32;
  // proto: void QWidget::setAccessibleDescription(const QString & description);
  fn _ZN7QWidget24setAccessibleDescriptionERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWidget::windowTitleChanged(const QString & title);
  fn _ZN7QWidget18windowTitleChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: QWidget * QWidget::nextInFocusChain();
  fn _ZNK7QWidget16nextInFocusChainEv() -> i32;
  // proto: void QWidget::setTabOrder(QWidget * , QWidget * );
  fn _ZN7QWidget11setTabOrderEPS_S0_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: QRect QWidget::frameGeometry();
  fn _ZNK7QWidget13frameGeometryEv() -> i32;
  // proto: QSize QWidget::sizeHint();
  fn _ZNK7QWidget8sizeHintEv() -> i32;
  // proto: void QWidget::setMinimumWidth(int minw);
  fn _ZN7QWidget15setMinimumWidthEi(arg0: c_int) -> i32;
  // proto: bool QWidget::isVisibleTo(const QWidget * );
  fn _ZNK7QWidget11isVisibleToEPKS_(arg0: *const c_void) -> i32;
  // proto: void QWidget::setMaximumSize(int maxw, int maxh);
  fn _ZN7QWidget14setMaximumSizeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QWidget::hasMouseTracking();
  fn _ZNK7QWidget16hasMouseTrackingEv() -> i32;
  // proto: void QWidget::update(const QRect & );
  fn _ZN7QWidget6updateERK5QRect(arg0: *const c_void) -> i32;
  // proto: bool QWidget::isHidden();
  fn _ZNK7QWidget8isHiddenEv() -> i32;
  // proto: int QWidget::devType();
  fn _ZNK7QWidget7devTypeEv() -> i32;
  // proto: QWidget * QWidget::childAt(int x, int y);
  fn _ZNK7QWidget7childAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QWidget::activateWindow();
  fn _ZN7QWidget14activateWindowEv() -> i32;
  // proto: QRect QWidget::normalGeometry();
  fn _ZNK7QWidget14normalGeometryEv() -> i32;
  // proto: QString QWidget::windowTitle();
  fn _ZNK7QWidget11windowTitleEv() -> i32;
  // proto: void QWidget::grabMouse(const QCursor & );
  fn _ZN7QWidget9grabMouseERK7QCursor(arg0: *const c_void) -> i32;
  // proto: QPixmap QWidget::grab(const QRect & rectangle);
  fn _ZN7QWidget4grabERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QWidget::setVisible(bool visible);
  fn _ZN7QWidget10setVisibleEb(arg0: int8_t) -> i32;
  // proto: bool QWidget::isEnabledTo(const QWidget * );
  fn _ZNK7QWidget11isEnabledToEPKS_(arg0: *const c_void) -> i32;
  // proto: bool QWidget::isLeftToRight();
  fn _ZNK7QWidget13isLeftToRightEv() -> i32;
  // proto: void QWidget::setGeometry(const QRect & );
  fn _ZN7QWidget11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QWidget::unsetLocale();
  fn _ZN7QWidget11unsetLocaleEv() -> i32;
  // proto: void QWidget::showNormal();
  fn _ZN7QWidget10showNormalEv() -> i32;
  // proto: int QWidget::y();
  fn _ZNK7QWidget1yEv() -> i32;
  // proto: int QWidget::width();
  fn _ZNK7QWidget5widthEv() -> i32;
  // proto: bool QWidget::isMaximized();
  fn _ZNK7QWidget11isMaximizedEv() -> i32;
  // proto: void QWidget::resize(const QSize & );
  fn _ZN7QWidget6resizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: QWindow * QWidget::windowHandle();
  fn _ZNK7QWidget12windowHandleEv() -> i32;
  // proto: QString QWidget::accessibleName();
  fn _ZNK7QWidget14accessibleNameEv() -> i32;
  // proto: void QWidget::setContentsMargins(const QMargins & margins);
  fn _ZN7QWidget18setContentsMarginsERK8QMargins(arg0: *const c_void) -> i32;
  // proto: QByteArray QWidget::saveGeometry();
  fn _ZNK7QWidget12saveGeometryEv() -> i32;
  // proto: bool QWidget::isEnabled();
  fn _ZNK7QWidget9isEnabledEv() -> i32;
  // proto: void QWidget::setFixedHeight(int h);
  fn _ZN7QWidget14setFixedHeightEi(arg0: c_int) -> i32;
  // proto: QRegion QWidget::mask();
  fn _ZNK7QWidget4maskEv() -> i32;
  // proto: void QWidget::stackUnder(QWidget * );
  fn _ZN7QWidget10stackUnderEPS_(arg0: *mut c_void) -> i32;
  // proto: QPaintEngine * QWidget::paintEngine();
  fn _ZNK7QWidget11paintEngineEv() -> i32;
  // proto: void QWidget::setAcceptDrops(bool on);
  fn _ZN7QWidget14setAcceptDropsEb(arg0: int8_t) -> i32;
  // proto: void QWidget::move_(const QPoint & );
  fn _ZN7QWidget4moveERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QList<QAction *> QWidget::actions();
  fn _ZNK7QWidget7actionsEv() -> i32;
  // proto: void QWidget::show();
  fn _ZN7QWidget4showEv() -> i32;
  // proto: QWidget * QWidget::keyboardGrabber();
  fn _ZN7QWidget15keyboardGrabberEv() -> i32;
  // proto: QPoint QWidget::mapTo(const QWidget * , const QPoint & );
  fn _ZNK7QWidget5mapToEPKS_RK6QPoint(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: int QWidget::minimumWidth();
  fn _ZNK7QWidget12minimumWidthEv() -> i32;
  // proto: QFontInfo QWidget::fontInfo();
  fn _ZNK7QWidget8fontInfoEv() -> i32;
  // proto: bool QWidget::autoFillBackground();
  fn _ZNK7QWidget18autoFillBackgroundEv() -> i32;
  // proto: void QWidget::scroll(int dx, int dy, const QRect & );
  fn _ZN7QWidget6scrollEiiRK5QRect(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: QFontMetrics QWidget::fontMetrics();
  fn _ZNK7QWidget11fontMetricsEv() -> i32;
  // proto: void QWidget::adjustSize();
  fn _ZN7QWidget10adjustSizeEv() -> i32;
  // proto: QWidget * QWidget::previousInFocusChain();
  fn _ZNK7QWidget20previousInFocusChainEv() -> i32;
  // proto: bool QWidget::updatesEnabled();
  fn _ZNK7QWidget14updatesEnabledEv() -> i32;
  // proto: void QWidget::setMaximumHeight(int maxh);
  fn _ZN7QWidget16setMaximumHeightEi(arg0: c_int) -> i32;
  // proto: void QWidget::showMaximized();
  fn _ZN7QWidget13showMaximizedEv() -> i32;
  // proto: QPoint QWidget::mapFrom(const QWidget * , const QPoint & );
  fn _ZNK7QWidget7mapFromEPKS_RK6QPoint(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: int QWidget::x();
  fn _ZNK7QWidget1xEv() -> i32;
  // proto: void QWidget::clearFocus();
  fn _ZN7QWidget10clearFocusEv() -> i32;
  // proto: QWidget * QWidget::find(WId );
  fn _ZN7QWidget4findEi(arg0: *mut c_uint) -> i32;
  // proto: const QPalette & QWidget::palette();
  fn _ZNK7QWidget7paletteEv() -> i32;
  // proto: void QWidget::setSizePolicy(QSizePolicy );
  fn _ZN7QWidget13setSizePolicyE11QSizePolicy(arg0: *mut c_void) -> i32;
  // proto: void QWidget::setMask(const QRegion & );
  fn _ZN7QWidget7setMaskERK7QRegion(arg0: *const c_void) -> i32;
  // proto: void QWidget::setMaximumWidth(int maxw);
  fn _ZN7QWidget15setMaximumWidthEi(arg0: c_int) -> i32;
  // proto: void QWidget::setWindowIconText(const QString & );
  fn _ZN7QWidget17setWindowIconTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWidget::setWindowIcon(const QIcon & icon);
  fn _ZN7QWidget13setWindowIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: void QWidget::FreeQWidget();
  fn _ZN7QWidgetD0Ev() -> i32;
  // proto: void QWidget::getContentsMargins(int * left, int * top, int * right, int * bottom);
  fn _ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: QSize QWidget::minimumSizeHint();
  fn _ZNK7QWidget15minimumSizeHintEv() -> i32;
  // proto: void QWidget::setWindowModified(bool );
  fn _ZN7QWidget17setWindowModifiedEb(arg0: int8_t) -> i32;
  // proto: bool QWidget::restoreGeometry(const QByteArray & geometry);
  fn _ZN7QWidget15restoreGeometryERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: QLayout * QWidget::layout();
  fn _ZNK7QWidget6layoutEv() -> i32;
  // proto: QRect QWidget::contentsRect();
  fn _ZNK7QWidget12contentsRectEv() -> i32;
  // proto: QBackingStore * QWidget::backingStore();
  fn _ZNK7QWidget12backingStoreEv() -> i32;
  // proto: QWidget * QWidget::focusProxy();
  fn _ZNK7QWidget10focusProxyEv() -> i32;
  // proto: QString QWidget::styleSheet();
  fn _ZNK7QWidget10styleSheetEv() -> i32;
  // proto: QWidget * QWidget::childAt(const QPoint & p);
  fn _ZNK7QWidget7childAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QWidget::repaint(int x, int y, int w, int h);
  fn _ZN7QWidget7repaintEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: QString QWidget::whatsThis();
  fn _ZNK7QWidget9whatsThisEv() -> i32;
  // proto: const QFont & QWidget::font();
  fn _ZNK7QWidget4fontEv() -> i32;
  // proto: void QWidget::setMinimumSize(int minw, int minh);
  fn _ZN7QWidget14setMinimumSizeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: const QMetaObject * QWidget::metaObject();
  fn _ZNK7QWidget10metaObjectEv() -> i32;
  // proto: void QWidget::setMinimumHeight(int minh);
  fn _ZN7QWidget16setMinimumHeightEi(arg0: c_int) -> i32;
  // proto: void QWidget::update(const QRegion & );
  fn _ZN7QWidget6updateERK7QRegion(arg0: *const c_void) -> i32;
  // proto: double QWidget::windowOpacity();
  fn _ZNK7QWidget13windowOpacityEv() -> i32;
  // proto: QRegion QWidget::childrenRegion();
  fn _ZNK7QWidget14childrenRegionEv() -> i32;
  // proto: void QWidget::setWindowFilePath(const QString & filePath);
  fn _ZN7QWidget17setWindowFilePathERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWidget::setShortcutEnabled(int id, bool enable);
  fn _ZN7QWidget18setShortcutEnabledEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QWidget::raise();
  fn _ZN7QWidget5raiseEv() -> i32;
  // proto: QString QWidget::statusTip();
  fn _ZNK7QWidget9statusTipEv() -> i32;
  // proto: QRect QWidget::childrenRect();
  fn _ZNK7QWidget12childrenRectEv() -> i32;
  // proto: void QWidget::setParent(QWidget * parent);
  fn _ZN7QWidget9setParentEPS_(arg0: *mut c_void) -> i32;
  // proto: QRegion QWidget::visibleRegion();
  fn _ZNK7QWidget13visibleRegionEv() -> i32;
  // proto: QLocale QWidget::locale();
  fn _ZNK7QWidget6localeEv() -> i32;
  // proto: void QWidget::releaseKeyboard();
  fn _ZN7QWidget15releaseKeyboardEv() -> i32;
  // proto: QWidget * QWidget::mouseGrabber();
  fn _ZN7QWidget12mouseGrabberEv() -> i32;
  // proto: void QWidget::setFixedWidth(int w);
  fn _ZN7QWidget13setFixedWidthEi(arg0: c_int) -> i32;
  // proto: void QWidget::addAction(QAction * action);
  fn _ZN7QWidget9addActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QWidget::setDisabled(bool );
  fn _ZN7QWidget11setDisabledEb(arg0: int8_t) -> i32;
  // proto: QIcon QWidget::windowIcon();
  fn _ZNK7QWidget10windowIconEv() -> i32;
  // proto: void QWidget::setContentsMargins(int left, int top, int right, int bottom);
  fn _ZN7QWidget18setContentsMarginsEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: QString QWidget::windowRole();
  fn _ZNK7QWidget10windowRoleEv() -> i32;
  // proto: void QWidget::setShortcutAutoRepeat(int id, bool enable);
  fn _ZN7QWidget21setShortcutAutoRepeatEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: void QWidget::showFullScreen();
  fn _ZN7QWidget14showFullScreenEv() -> i32;
  // proto: void QWidget::grabMouse();
  fn _ZN7QWidget9grabMouseEv() -> i32;
  // proto: void QWidget::setMaximumSize(const QSize & );
  fn _ZN7QWidget14setMaximumSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: QPoint QWidget::mapToGlobal(const QPoint & );
  fn _ZNK7QWidget11mapToGlobalERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QString QWidget::toolTip();
  fn _ZNK7QWidget7toolTipEv() -> i32;
  // proto: void QWidget::setWhatsThis(const QString & );
  fn _ZN7QWidget12setWhatsThisERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWidget::resize(int w, int h);
  fn _ZN7QWidget6resizeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QWidget * QWidget::parentWidget();
  fn _ZNK7QWidget12parentWidgetEv() -> i32;
  // proto: QPoint QWidget::pos();
  fn _ZNK7QWidget3posEv() -> i32;
  // proto: void QWidget::setAutoFillBackground(bool enabled);
  fn _ZN7QWidget21setAutoFillBackgroundEb(arg0: int8_t) -> i32;
  // proto: bool QWidget::hasFocus();
  fn _ZNK7QWidget8hasFocusEv() -> i32;
  // proto: QSize QWidget::baseSize();
  fn _ZNK7QWidget8baseSizeEv() -> i32;
  // proto: void QWidget::setMask(const QBitmap & );
  fn _ZN7QWidget7setMaskERK7QBitmap(arg0: *const c_void) -> i32;
  // proto: void QWidget::ensurePolished();
  fn _ZNK7QWidget14ensurePolishedEv() -> i32;
  // proto: void QWidget::setWindowTitle(const QString & );
  fn _ZN7QWidget14setWindowTitleERK7QString(arg0: *const c_void) -> i32;
  // proto: QWidget * QWidget::window();
  fn _ZNK7QWidget6windowEv() -> i32;
  // proto: void QWidget::scroll(int dx, int dy);
  fn _ZN7QWidget6scrollEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QWidget::releaseShortcut(int id);
  fn _ZN7QWidget15releaseShortcutEi(arg0: c_int) -> i32;
  // proto: void QWidget::setToolTipDuration(int msec);
  fn _ZN7QWidget18setToolTipDurationEi(arg0: c_int) -> i32;
  // proto: void QWidget::setGeometry(int x, int y, int w, int h);
  fn _ZN7QWidget11setGeometryEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QWidget::setSizeIncrement(int w, int h);
  fn _ZN7QWidget16setSizeIncrementEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QWidget::setUpdatesEnabled(bool enable);
  fn _ZN7QWidget17setUpdatesEnabledEb(arg0: int8_t) -> i32;
  // proto: void QWidget::lower();
  fn _ZN7QWidget5lowerEv() -> i32;
  // proto: void QWidget::setMouseTracking(bool enable);
  fn _ZN7QWidget16setMouseTrackingEb(arg0: int8_t) -> i32;
  // proto: void QWidget::setBaseSize(const QSize & );
  fn _ZN7QWidget11setBaseSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QWidget::hide();
  fn _ZN7QWidget4hideEv() -> i32;
  // proto: void QWidget::removeAction(QAction * action);
  fn _ZN7QWidget12removeActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QWidget::setFocusProxy(QWidget * );
  fn _ZN7QWidget13setFocusProxyEPS_(arg0: *mut c_void) -> i32;
  // proto: bool QWidget::close();
  fn _ZN7QWidget5closeEv() -> i32;
  // proto: void QWidget::showMinimized();
  fn _ZN7QWidget13showMinimizedEv() -> i32;
  // proto: void QWidget::setFixedSize(int w, int h);
  fn _ZN7QWidget12setFixedSizeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QSize QWidget::minimumSize();
  fn _ZNK7QWidget11minimumSizeEv() -> i32;
  // proto: void QWidget::setEnabled(bool );
  fn _ZN7QWidget10setEnabledEb(arg0: int8_t) -> i32;
  // proto: int QWidget::maximumHeight();
  fn _ZNK7QWidget13maximumHeightEv() -> i32;
  // proto: void QWidget::move_(int x, int y);
  fn _ZN7QWidget4moveEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: bool QWidget::isAncestorOf(const QWidget * child);
  fn _ZNK7QWidget12isAncestorOfEPKS_(arg0: *const c_void) -> i32;
  // proto: void QWidget::NewQWidget(const QWidget & );
  fn _ZN7QWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QCursor QWidget::cursor();
  fn _ZNK7QWidget6cursorEv() -> i32;
  // proto: QPoint QWidget::mapFromGlobal(const QPoint & );
  fn _ZNK7QWidget13mapFromGlobalERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QWidget::setToolTip(const QString & );
  fn _ZN7QWidget10setToolTipERK7QString(arg0: *const c_void) -> i32;
  // proto: QSizePolicy QWidget::sizePolicy();
  fn _ZNK7QWidget10sizePolicyEv() -> i32;
  // proto: bool QWidget::hasHeightForWidth();
  fn _ZNK7QWidget17hasHeightForWidthEv() -> i32;
  // proto: QGraphicsProxyWidget * QWidget::graphicsProxyWidget();
  fn _ZNK7QWidget19graphicsProxyWidgetEv() -> i32;
  // proto: QMargins QWidget::contentsMargins();
  fn _ZNK7QWidget15contentsMarginsEv() -> i32;
  // proto: QWidget * QWidget::topLevelWidget();
  fn _ZNK7QWidget14topLevelWidgetEv() -> i32;
  // proto: void QWidget::setLayout(QLayout * );
  fn _ZN7QWidget9setLayoutEP7QLayout(arg0: *mut c_void) -> i32;
  // proto: bool QWidget::underMouse();
  fn _ZNK7QWidget10underMouseEv() -> i32;
  // proto: int QWidget::heightForWidth(int );
  fn _ZNK7QWidget14heightForWidthEi(arg0: c_int) -> i32;
  // proto: void QWidget::setFont(const QFont & );
  fn _ZN7QWidget7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: QWidget * QWidget::nativeParentWidget();
  fn _ZNK7QWidget18nativeParentWidgetEv() -> i32;
  // proto: void QWidget::setLocale(const QLocale & locale);
  fn _ZN7QWidget9setLocaleERK7QLocale(arg0: *const c_void) -> i32;
  // proto: int QWidget::height();
  fn _ZNK7QWidget6heightEv() -> i32;
  // proto: void QWidget::setHidden(bool hidden);
  fn _ZN7QWidget9setHiddenEb(arg0: int8_t) -> i32;
  // proto: QSize QWidget::size();
  fn _ZNK7QWidget4sizeEv() -> i32;
  // proto: int QWidget::maximumWidth();
  fn _ZNK7QWidget12maximumWidthEv() -> i32;
  // proto: bool QWidget::isMinimized();
  fn _ZNK7QWidget11isMinimizedEv() -> i32;
  // proto: void QWidget::update();
  fn _ZN7QWidget6updateEv() -> i32;
  // proto: void QWidget::setCursor(const QCursor & );
  fn _ZN7QWidget9setCursorERK7QCursor(arg0: *const c_void) -> i32;
  // proto: void QWidget::windowIconChanged(const QIcon & icon);
  fn _ZN7QWidget17windowIconChangedERK5QIcon(arg0: *const c_void) -> i32;
  // proto: QStyle * QWidget::style();
  fn _ZNK7QWidget5styleEv() -> i32;
  // proto: void QWidget::createWinId();
  fn _ZN7QWidget11createWinIdEv() -> i32;
  // proto: void QWidget::setWindowOpacity(qreal level);
  fn _ZN7QWidget16setWindowOpacityEd(arg0: c_double) -> i32;
  // proto: bool QWidget::isRightToLeft();
  fn _ZNK7QWidget13isRightToLeftEv() -> i32;
  // proto: void QWidget::setAccessibleName(const QString & name);
  fn _ZN7QWidget17setAccessibleNameERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWidget::windowIconTextChanged(const QString & iconText);
  fn _ZN7QWidget21windowIconTextChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWidget::unsetCursor();
  fn _ZN7QWidget11unsetCursorEv() -> i32;
}

// body block begin
// class sizeof(QWidget)=1
pub struct QWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWidget {
  pub fn setGraphicsEffect<T: QWidget_setGraphicsEffect>(&mut self, value: T) -> i32 {
    value.setGraphicsEffect(self);
    return 1;
  }
}

pub trait QWidget_setGraphicsEffect {
  fn setGraphicsEffect(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setGraphicsEffect(QGraphicsEffect * effect);
impl<'a> /*trait*/ QWidget_setGraphicsEffect for (&'a mut QGraphicsEffect) {
  fn setGraphicsEffect(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn accessibleDescription<T: QWidget_accessibleDescription>(&mut self, value: T) -> i32 {
    value.accessibleDescription(self);
    return 1;
  }
}

pub trait QWidget_accessibleDescription {
  fn accessibleDescription(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::accessibleDescription();
impl<'a> /*trait*/ QWidget_accessibleDescription for () {
  fn accessibleDescription(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget21accessibleDescriptionEv()};
    unsafe {_ZNK7QWidget21accessibleDescriptionEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn graphicsEffect<T: QWidget_graphicsEffect>(&mut self, value: T) -> i32 {
    value.graphicsEffect(self);
    return 1;
  }
}

pub trait QWidget_graphicsEffect {
  fn graphicsEffect(self, this: &mut QWidget) -> i32;
}

// proto: QGraphicsEffect * QWidget::graphicsEffect();
impl<'a> /*trait*/ QWidget_graphicsEffect for () {
  fn graphicsEffect(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14graphicsEffectEv()};
    unsafe {_ZNK7QWidget14graphicsEffectEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isFullScreen<T: QWidget_isFullScreen>(&mut self, value: T) -> i32 {
    value.isFullScreen(self);
    return 1;
  }
}

pub trait QWidget_isFullScreen {
  fn isFullScreen(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isFullScreen();
impl<'a> /*trait*/ QWidget_isFullScreen for () {
  fn isFullScreen(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12isFullScreenEv()};
    unsafe {_ZNK7QWidget12isFullScreenEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn maximumSize<T: QWidget_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QWidget_maximumSize {
  fn maximumSize(self, this: &mut QWidget) -> i32;
}

// proto: QSize QWidget::maximumSize();
impl<'a> /*trait*/ QWidget_maximumSize for () {
  fn maximumSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11maximumSizeEv()};
    unsafe {_ZNK7QWidget11maximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isEnabledToTLW<T: QWidget_isEnabledToTLW>(&mut self, value: T) -> i32 {
    value.isEnabledToTLW(self);
    return 1;
  }
}

pub trait QWidget_isEnabledToTLW {
  fn isEnabledToTLW(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isEnabledToTLW();
impl<'a> /*trait*/ QWidget_isEnabledToTLW for () {
  fn isEnabledToTLW(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14isEnabledToTLWEv()};
    unsafe {_ZNK7QWidget14isEnabledToTLWEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setStatusTip<T: QWidget_setStatusTip>(&mut self, value: T) -> i32 {
    value.setStatusTip(self);
    return 1;
  }
}

pub trait QWidget_setStatusTip {
  fn setStatusTip(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setStatusTip(const QString & );
impl<'a> /*trait*/ QWidget_setStatusTip for (&'a  QString) {
  fn setStatusTip(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget12setStatusTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setSizeIncrement<T: QWidget_setSizeIncrement>(&mut self, value: T) -> i32 {
    value.setSizeIncrement(self);
    return 1;
  }
}

pub trait QWidget_setSizeIncrement {
  fn setSizeIncrement(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setSizeIncrement(const QSize & );
impl<'a> /*trait*/ QWidget_setSizeIncrement for (&'a  QSize) {
  fn setSizeIncrement(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setSizeIncrementERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget16setSizeIncrementERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn customContextMenuRequested<T: QWidget_customContextMenuRequested>(&mut self, value: T) -> i32 {
    value.customContextMenuRequested(self);
    return 1;
  }
}

pub trait QWidget_customContextMenuRequested {
  fn customContextMenuRequested(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::customContextMenuRequested(const QPoint & pos);
impl<'a> /*trait*/ QWidget_customContextMenuRequested for (&'a  QPoint) {
  fn customContextMenuRequested(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget26customContextMenuRequestedERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget26customContextMenuRequestedERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn focusWidget<T: QWidget_focusWidget>(&mut self, value: T) -> i32 {
    value.focusWidget(self);
    return 1;
  }
}

pub trait QWidget_focusWidget {
  fn focusWidget(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::focusWidget();
impl<'a> /*trait*/ QWidget_focusWidget for () {
  fn focusWidget(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11focusWidgetEv()};
    unsafe {_ZNK7QWidget11focusWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isTopLevel<T: QWidget_isTopLevel>(&mut self, value: T) -> i32 {
    value.isTopLevel(self);
    return 1;
  }
}

pub trait QWidget_isTopLevel {
  fn isTopLevel(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isTopLevel();
impl<'a> /*trait*/ QWidget_isTopLevel for () {
  fn isTopLevel(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10isTopLevelEv()};
    unsafe {_ZNK7QWidget10isTopLevelEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn acceptDrops<T: QWidget_acceptDrops>(&mut self, value: T) -> i32 {
    value.acceptDrops(self);
    return 1;
  }
}

pub trait QWidget_acceptDrops {
  fn acceptDrops(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::acceptDrops();
impl<'a> /*trait*/ QWidget_acceptDrops for () {
  fn acceptDrops(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11acceptDropsEv()};
    unsafe {_ZNK7QWidget11acceptDropsEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isWindow<T: QWidget_isWindow>(&mut self, value: T) -> i32 {
    value.isWindow(self);
    return 1;
  }
}

pub trait QWidget_isWindow {
  fn isWindow(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isWindow();
impl<'a> /*trait*/ QWidget_isWindow for () {
  fn isWindow(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8isWindowEv()};
    unsafe {_ZNK7QWidget8isWindowEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFixedSize<T: QWidget_setFixedSize>(&mut self, value: T) -> i32 {
    value.setFixedSize(self);
    return 1;
  }
}

pub trait QWidget_setFixedSize {
  fn setFixedSize(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setFixedSize(const QSize & );
impl<'a> /*trait*/ QWidget_setFixedSize for (&'a  QSize) {
  fn setFixedSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setFixedSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget12setFixedSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isVisible<T: QWidget_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QWidget_isVisible {
  fn isVisible(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isVisible();
impl<'a> /*trait*/ QWidget_isVisible for () {
  fn isVisible(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9isVisibleEv()};
    unsafe {_ZNK7QWidget9isVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn minimumHeight<T: QWidget_minimumHeight>(&mut self, value: T) -> i32 {
    value.minimumHeight(self);
    return 1;
  }
}

pub trait QWidget_minimumHeight {
  fn minimumHeight(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::minimumHeight();
impl<'a> /*trait*/ QWidget_minimumHeight for () {
  fn minimumHeight(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13minimumHeightEv()};
    unsafe {_ZNK7QWidget13minimumHeightEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn sizeIncrement<T: QWidget_sizeIncrement>(&mut self, value: T) -> i32 {
    value.sizeIncrement(self);
    return 1;
  }
}

pub trait QWidget_sizeIncrement {
  fn sizeIncrement(self, this: &mut QWidget) -> i32;
}

// proto: QSize QWidget::sizeIncrement();
impl<'a> /*trait*/ QWidget_sizeIncrement for () {
  fn sizeIncrement(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13sizeIncrementEv()};
    unsafe {_ZNK7QWidget13sizeIncrementEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn repaint<T: QWidget_repaint>(&mut self, value: T) -> i32 {
    value.repaint(self);
    return 1;
  }
}

pub trait QWidget_repaint {
  fn repaint(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::repaint(const QRect & );
impl<'a> /*trait*/ QWidget_repaint for (&'a  QRect) {
  fn repaint(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget7repaintERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn update<T: QWidget_update>(&mut self, value: T) -> i32 {
    value.update(self);
    return 1;
  }
}

pub trait QWidget_update {
  fn update(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::update(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_update for (i32, i32, i32, i32) {
  fn update(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN7QWidget6updateEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowFilePath<T: QWidget_windowFilePath>(&mut self, value: T) -> i32 {
    value.windowFilePath(self);
    return 1;
  }
}

pub trait QWidget_windowFilePath {
  fn windowFilePath(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::windowFilePath();
impl<'a> /*trait*/ QWidget_windowFilePath for () {
  fn windowFilePath(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14windowFilePathEv()};
    unsafe {_ZNK7QWidget14windowFilePathEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn clearMask<T: QWidget_clearMask>(&mut self, value: T) -> i32 {
    value.clearMask(self);
    return 1;
  }
}

pub trait QWidget_clearMask {
  fn clearMask(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::clearMask();
impl<'a> /*trait*/ QWidget_clearMask for () {
  fn clearMask(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9clearMaskEv()};
    unsafe {_ZN7QWidget9clearMaskEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapFromParent<T: QWidget_mapFromParent>(&mut self, value: T) -> i32 {
    value.mapFromParent(self);
    return 1;
  }
}

pub trait QWidget_mapFromParent {
  fn mapFromParent(self, this: &mut QWidget) -> i32;
}

// proto: QPoint QWidget::mapFromParent(const QPoint & );
impl<'a> /*trait*/ QWidget_mapFromParent for (&'a  QPoint) {
  fn mapFromParent(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13mapFromParentERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget13mapFromParentERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn rect<T: QWidget_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QWidget_rect {
  fn rect(self, this: &mut QWidget) -> i32;
}

// proto: QRect QWidget::rect();
impl<'a> /*trait*/ QWidget_rect for () {
  fn rect(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4rectEv()};
    unsafe {_ZNK7QWidget4rectEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn unsetLayoutDirection<T: QWidget_unsetLayoutDirection>(&mut self, value: T) -> i32 {
    value.unsetLayoutDirection(self);
    return 1;
  }
}

pub trait QWidget_unsetLayoutDirection {
  fn unsetLayoutDirection(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::unsetLayoutDirection();
impl<'a> /*trait*/ QWidget_unsetLayoutDirection for () {
  fn unsetLayoutDirection(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget20unsetLayoutDirectionEv()};
    unsafe {_ZN7QWidget20unsetLayoutDirectionEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMinimumSize<T: QWidget_setMinimumSize>(&mut self, value: T) -> i32 {
    value.setMinimumSize(self);
    return 1;
  }
}

pub trait QWidget_setMinimumSize {
  fn setMinimumSize(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setMinimumSize(const QSize & );
impl<'a> /*trait*/ QWidget_setMinimumSize for (&'a  QSize) {
  fn setMinimumSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMinimumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget14setMinimumSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isActiveWindow<T: QWidget_isActiveWindow>(&mut self, value: T) -> i32 {
    value.isActiveWindow(self);
    return 1;
  }
}

pub trait QWidget_isActiveWindow {
  fn isActiveWindow(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isActiveWindow();
impl<'a> /*trait*/ QWidget_isActiveWindow for () {
  fn isActiveWindow(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14isActiveWindowEv()};
    unsafe {_ZNK7QWidget14isActiveWindowEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn grabKeyboard<T: QWidget_grabKeyboard>(&mut self, value: T) -> i32 {
    value.grabKeyboard(self);
    return 1;
  }
}

pub trait QWidget_grabKeyboard {
  fn grabKeyboard(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::grabKeyboard();
impl<'a> /*trait*/ QWidget_grabKeyboard for () {
  fn grabKeyboard(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12grabKeyboardEv()};
    unsafe {_ZN7QWidget12grabKeyboardEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn frameSize<T: QWidget_frameSize>(&mut self, value: T) -> i32 {
    value.frameSize(self);
    return 1;
  }
}

pub trait QWidget_frameSize {
  fn frameSize(self, this: &mut QWidget) -> i32;
}

// proto: QSize QWidget::frameSize();
impl<'a> /*trait*/ QWidget_frameSize for () {
  fn frameSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9frameSizeEv()};
    unsafe {_ZNK7QWidget9frameSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFocus<T: QWidget_setFocus>(&mut self, value: T) -> i32 {
    value.setFocus(self);
    return 1;
  }
}

pub trait QWidget_setFocus {
  fn setFocus(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setFocus();
impl<'a> /*trait*/ QWidget_setFocus for () {
  fn setFocus(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget8setFocusEv()};
    unsafe {_ZN7QWidget8setFocusEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapToParent<T: QWidget_mapToParent>(&mut self, value: T) -> i32 {
    value.mapToParent(self);
    return 1;
  }
}

pub trait QWidget_mapToParent {
  fn mapToParent(self, this: &mut QWidget) -> i32;
}

// proto: QPoint QWidget::mapToParent(const QPoint & );
impl<'a> /*trait*/ QWidget_mapToParent for (&'a  QPoint) {
  fn mapToParent(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11mapToParentERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget11mapToParentERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn updateGeometry<T: QWidget_updateGeometry>(&mut self, value: T) -> i32 {
    value.updateGeometry(self);
    return 1;
  }
}

pub trait QWidget_updateGeometry {
  fn updateGeometry(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::updateGeometry();
impl<'a> /*trait*/ QWidget_updateGeometry for () {
  fn updateGeometry(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14updateGeometryEv()};
    unsafe {_ZN7QWidget14updateGeometryEv()};
    return 1;
  }
}

// proto: void QWidget::repaint(const QRegion & );
impl<'a> /*trait*/ QWidget_repaint for (&'a  QRegion) {
  fn repaint(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget7repaintERK7QRegion(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn insertAction<T: QWidget_insertAction>(&mut self, value: T) -> i32 {
    value.insertAction(self);
    return 1;
  }
}

pub trait QWidget_insertAction {
  fn insertAction(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::insertAction(QAction * before, QAction * action);
impl<'a> /*trait*/ QWidget_insertAction for (&'a mut QAction, &'a mut QAction) {
  fn insertAction(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12insertActionEP7QActionS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget12insertActionEP7QActionS1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowRole<T: QWidget_setWindowRole>(&mut self, value: T) -> i32 {
    value.setWindowRole(self);
    return 1;
  }
}

pub trait QWidget_setWindowRole {
  fn setWindowRole(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setWindowRole(const QString & );
impl<'a> /*trait*/ QWidget_setWindowRole for (&'a  QString) {
  fn setWindowRole(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setWindowRoleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget13setWindowRoleERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn toolTipDuration<T: QWidget_toolTipDuration>(&mut self, value: T) -> i32 {
    value.toolTipDuration(self);
    return 1;
  }
}

pub trait QWidget_toolTipDuration {
  fn toolTipDuration(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::toolTipDuration();
impl<'a> /*trait*/ QWidget_toolTipDuration for () {
  fn toolTipDuration(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15toolTipDurationEv()};
    unsafe {_ZNK7QWidget15toolTipDurationEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setPalette<T: QWidget_setPalette>(&mut self, value: T) -> i32 {
    value.setPalette(self);
    return 1;
  }
}

pub trait QWidget_setPalette {
  fn setPalette(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setPalette(const QPalette & );
impl<'a> /*trait*/ QWidget_setPalette for (&'a  QPalette) {
  fn setPalette(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget10setPaletteERK8QPalette(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setStyleSheet<T: QWidget_setStyleSheet>(&mut self, value: T) -> i32 {
    value.setStyleSheet(self);
    return 1;
  }
}

pub trait QWidget_setStyleSheet {
  fn setStyleSheet(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setStyleSheet(const QString & styleSheet);
impl<'a> /*trait*/ QWidget_setStyleSheet for (&'a  QString) {
  fn setStyleSheet(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget13setStyleSheetERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowIconText<T: QWidget_windowIconText>(&mut self, value: T) -> i32 {
    value.windowIconText(self);
    return 1;
  }
}

pub trait QWidget_windowIconText {
  fn windowIconText(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::windowIconText();
impl<'a> /*trait*/ QWidget_windowIconText for () {
  fn windowIconText(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14windowIconTextEv()};
    unsafe {_ZNK7QWidget14windowIconTextEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn releaseMouse<T: QWidget_releaseMouse>(&mut self, value: T) -> i32 {
    value.releaseMouse(self);
    return 1;
  }
}

pub trait QWidget_releaseMouse {
  fn releaseMouse(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::releaseMouse();
impl<'a> /*trait*/ QWidget_releaseMouse for () {
  fn releaseMouse(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12releaseMouseEv()};
    unsafe {_ZN7QWidget12releaseMouseEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isModal<T: QWidget_isModal>(&mut self, value: T) -> i32 {
    value.isModal(self);
    return 1;
  }
}

pub trait QWidget_isModal {
  fn isModal(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isModal();
impl<'a> /*trait*/ QWidget_isModal for () {
  fn isModal(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7isModalEv()};
    unsafe {_ZNK7QWidget7isModalEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setStyle<T: QWidget_setStyle>(&mut self, value: T) -> i32 {
    value.setStyle(self);
    return 1;
  }
}

pub trait QWidget_setStyle {
  fn setStyle(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setStyle(QStyle * );
impl<'a> /*trait*/ QWidget_setStyle for (&'a mut QStyle) {
  fn setStyle(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget8setStyleEP6QStyle(arg0)};
    return 1;
  }
}

// proto: void QWidget::repaint();
impl<'a> /*trait*/ QWidget_repaint for () {
  fn repaint(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintEv()};
    unsafe {_ZN7QWidget7repaintEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setBaseSize<T: QWidget_setBaseSize>(&mut self, value: T) -> i32 {
    value.setBaseSize(self);
    return 1;
  }
}

pub trait QWidget_setBaseSize {
  fn setBaseSize(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setBaseSize(int basew, int baseh);
impl<'a> /*trait*/ QWidget_setBaseSize for (i32, i32) {
  fn setBaseSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setBaseSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWidget11setBaseSizeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isWindowModified<T: QWidget_isWindowModified>(&mut self, value: T) -> i32 {
    value.isWindowModified(self);
    return 1;
  }
}

pub trait QWidget_isWindowModified {
  fn isWindowModified(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isWindowModified();
impl<'a> /*trait*/ QWidget_isWindowModified for () {
  fn isWindowModified(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16isWindowModifiedEv()};
    unsafe {_ZNK7QWidget16isWindowModifiedEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn geometry<T: QWidget_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QWidget_geometry {
  fn geometry(self, this: &mut QWidget) -> i32;
}

// proto: const QRect & QWidget::geometry();
impl<'a> /*trait*/ QWidget_geometry for () {
  fn geometry(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8geometryEv()};
    unsafe {_ZNK7QWidget8geometryEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setAccessibleDescription<T: QWidget_setAccessibleDescription>(&mut self, value: T) -> i32 {
    value.setAccessibleDescription(self);
    return 1;
  }
}

pub trait QWidget_setAccessibleDescription {
  fn setAccessibleDescription(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setAccessibleDescription(const QString & description);
impl<'a> /*trait*/ QWidget_setAccessibleDescription for (&'a  QString) {
  fn setAccessibleDescription(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget24setAccessibleDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget24setAccessibleDescriptionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowTitleChanged<T: QWidget_windowTitleChanged>(&mut self, value: T) -> i32 {
    value.windowTitleChanged(self);
    return 1;
  }
}

pub trait QWidget_windowTitleChanged {
  fn windowTitleChanged(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::windowTitleChanged(const QString & title);
impl<'a> /*trait*/ QWidget_windowTitleChanged for (&'a  QString) {
  fn windowTitleChanged(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18windowTitleChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget18windowTitleChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn nextInFocusChain<T: QWidget_nextInFocusChain>(&mut self, value: T) -> i32 {
    value.nextInFocusChain(self);
    return 1;
  }
}

pub trait QWidget_nextInFocusChain {
  fn nextInFocusChain(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::nextInFocusChain();
impl<'a> /*trait*/ QWidget_nextInFocusChain for () {
  fn nextInFocusChain(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16nextInFocusChainEv()};
    unsafe {_ZNK7QWidget16nextInFocusChainEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setTabOrder<T: QWidget_setTabOrder>(&mut self, value: T) -> i32 {
    value.setTabOrder(self);
    return 1;
  }
}

pub trait QWidget_setTabOrder {
  fn setTabOrder(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setTabOrder(QWidget * , QWidget * );
impl<'a> /*trait*/ QWidget_setTabOrder for (&'a mut QWidget, &'a mut QWidget) {
  fn setTabOrder(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setTabOrderEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget11setTabOrderEPS_S0_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn frameGeometry<T: QWidget_frameGeometry>(&mut self, value: T) -> i32 {
    value.frameGeometry(self);
    return 1;
  }
}

pub trait QWidget_frameGeometry {
  fn frameGeometry(self, this: &mut QWidget) -> i32;
}

// proto: QRect QWidget::frameGeometry();
impl<'a> /*trait*/ QWidget_frameGeometry for () {
  fn frameGeometry(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13frameGeometryEv()};
    unsafe {_ZNK7QWidget13frameGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn sizeHint<T: QWidget_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QWidget_sizeHint {
  fn sizeHint(self, this: &mut QWidget) -> i32;
}

// proto: QSize QWidget::sizeHint();
impl<'a> /*trait*/ QWidget_sizeHint for () {
  fn sizeHint(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8sizeHintEv()};
    unsafe {_ZNK7QWidget8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMinimumWidth<T: QWidget_setMinimumWidth>(&mut self, value: T) -> i32 {
    value.setMinimumWidth(self);
    return 1;
  }
}

pub trait QWidget_setMinimumWidth {
  fn setMinimumWidth(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setMinimumWidth(int minw);
impl<'a> /*trait*/ QWidget_setMinimumWidth for (i32) {
  fn setMinimumWidth(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15setMinimumWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWidget15setMinimumWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isVisibleTo<T: QWidget_isVisibleTo>(&mut self, value: T) -> i32 {
    value.isVisibleTo(self);
    return 1;
  }
}

pub trait QWidget_isVisibleTo {
  fn isVisibleTo(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isVisibleTo(const QWidget * );
impl<'a> /*trait*/ QWidget_isVisibleTo for (&'a  QWidget) {
  fn isVisibleTo(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isVisibleToEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget11isVisibleToEPKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMaximumSize<T: QWidget_setMaximumSize>(&mut self, value: T) -> i32 {
    value.setMaximumSize(self);
    return 1;
  }
}

pub trait QWidget_setMaximumSize {
  fn setMaximumSize(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setMaximumSize(int maxw, int maxh);
impl<'a> /*trait*/ QWidget_setMaximumSize for (i32, i32) {
  fn setMaximumSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMaximumSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWidget14setMaximumSizeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn hasMouseTracking<T: QWidget_hasMouseTracking>(&mut self, value: T) -> i32 {
    value.hasMouseTracking(self);
    return 1;
  }
}

pub trait QWidget_hasMouseTracking {
  fn hasMouseTracking(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::hasMouseTracking();
impl<'a> /*trait*/ QWidget_hasMouseTracking for () {
  fn hasMouseTracking(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16hasMouseTrackingEv()};
    unsafe {_ZNK7QWidget16hasMouseTrackingEv()};
    return 1;
  }
}

// proto: void QWidget::update(const QRect & );
impl<'a> /*trait*/ QWidget_update for (&'a  QRect) {
  fn update(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget6updateERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isHidden<T: QWidget_isHidden>(&mut self, value: T) -> i32 {
    value.isHidden(self);
    return 1;
  }
}

pub trait QWidget_isHidden {
  fn isHidden(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isHidden();
impl<'a> /*trait*/ QWidget_isHidden for () {
  fn isHidden(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8isHiddenEv()};
    unsafe {_ZNK7QWidget8isHiddenEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn devType<T: QWidget_devType>(&mut self, value: T) -> i32 {
    value.devType(self);
    return 1;
  }
}

pub trait QWidget_devType {
  fn devType(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::devType();
impl<'a> /*trait*/ QWidget_devType for () {
  fn devType(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7devTypeEv()};
    unsafe {_ZNK7QWidget7devTypeEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn childAt<T: QWidget_childAt>(&mut self, value: T) -> i32 {
    value.childAt(self);
    return 1;
  }
}

pub trait QWidget_childAt {
  fn childAt(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::childAt(int x, int y);
impl<'a> /*trait*/ QWidget_childAt for (i32, i32) {
  fn childAt(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QWidget7childAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn activateWindow<T: QWidget_activateWindow>(&mut self, value: T) -> i32 {
    value.activateWindow(self);
    return 1;
  }
}

pub trait QWidget_activateWindow {
  fn activateWindow(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::activateWindow();
impl<'a> /*trait*/ QWidget_activateWindow for () {
  fn activateWindow(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14activateWindowEv()};
    unsafe {_ZN7QWidget14activateWindowEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn normalGeometry<T: QWidget_normalGeometry>(&mut self, value: T) -> i32 {
    value.normalGeometry(self);
    return 1;
  }
}

pub trait QWidget_normalGeometry {
  fn normalGeometry(self, this: &mut QWidget) -> i32;
}

// proto: QRect QWidget::normalGeometry();
impl<'a> /*trait*/ QWidget_normalGeometry for () {
  fn normalGeometry(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14normalGeometryEv()};
    unsafe {_ZNK7QWidget14normalGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowTitle<T: QWidget_windowTitle>(&mut self, value: T) -> i32 {
    value.windowTitle(self);
    return 1;
  }
}

pub trait QWidget_windowTitle {
  fn windowTitle(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::windowTitle();
impl<'a> /*trait*/ QWidget_windowTitle for () {
  fn windowTitle(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11windowTitleEv()};
    unsafe {_ZNK7QWidget11windowTitleEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn grabMouse<T: QWidget_grabMouse>(&mut self, value: T) -> i32 {
    value.grabMouse(self);
    return 1;
  }
}

pub trait QWidget_grabMouse {
  fn grabMouse(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::grabMouse(const QCursor & );
impl<'a> /*trait*/ QWidget_grabMouse for (&'a  QCursor) {
  fn grabMouse(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9grabMouseERK7QCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget9grabMouseERK7QCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn grab<T: QWidget_grab>(&mut self, value: T) -> i32 {
    value.grab(self);
    return 1;
  }
}

pub trait QWidget_grab {
  fn grab(self, this: &mut QWidget) -> i32;
}

// proto: QPixmap QWidget::grab(const QRect & rectangle);
impl<'a> /*trait*/ QWidget_grab for (&'a  QRect) {
  fn grab(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4grabERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget4grabERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setVisible<T: QWidget_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QWidget_setVisible {
  fn setVisible(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setVisible(bool visible);
impl<'a> /*trait*/ QWidget_setVisible for (i8) {
  fn setVisible(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isEnabledTo<T: QWidget_isEnabledTo>(&mut self, value: T) -> i32 {
    value.isEnabledTo(self);
    return 1;
  }
}

pub trait QWidget_isEnabledTo {
  fn isEnabledTo(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isEnabledTo(const QWidget * );
impl<'a> /*trait*/ QWidget_isEnabledTo for (&'a  QWidget) {
  fn isEnabledTo(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isEnabledToEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget11isEnabledToEPKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isLeftToRight<T: QWidget_isLeftToRight>(&mut self, value: T) -> i32 {
    value.isLeftToRight(self);
    return 1;
  }
}

pub trait QWidget_isLeftToRight {
  fn isLeftToRight(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isLeftToRight();
impl<'a> /*trait*/ QWidget_isLeftToRight for () {
  fn isLeftToRight(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13isLeftToRightEv()};
    unsafe {_ZNK7QWidget13isLeftToRightEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setGeometry<T: QWidget_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QWidget_setGeometry {
  fn setGeometry(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setGeometry(const QRect & );
impl<'a> /*trait*/ QWidget_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn unsetLocale<T: QWidget_unsetLocale>(&mut self, value: T) -> i32 {
    value.unsetLocale(self);
    return 1;
  }
}

pub trait QWidget_unsetLocale {
  fn unsetLocale(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::unsetLocale();
impl<'a> /*trait*/ QWidget_unsetLocale for () {
  fn unsetLocale(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11unsetLocaleEv()};
    unsafe {_ZN7QWidget11unsetLocaleEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn showNormal<T: QWidget_showNormal>(&mut self, value: T) -> i32 {
    value.showNormal(self);
    return 1;
  }
}

pub trait QWidget_showNormal {
  fn showNormal(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::showNormal();
impl<'a> /*trait*/ QWidget_showNormal for () {
  fn showNormal(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10showNormalEv()};
    unsafe {_ZN7QWidget10showNormalEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn y<T: QWidget_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QWidget_y {
  fn y(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::y();
impl<'a> /*trait*/ QWidget_y for () {
  fn y(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget1yEv()};
    unsafe {_ZNK7QWidget1yEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn width<T: QWidget_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QWidget_width {
  fn width(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::width();
impl<'a> /*trait*/ QWidget_width for () {
  fn width(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5widthEv()};
    unsafe {_ZNK7QWidget5widthEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isMaximized<T: QWidget_isMaximized>(&mut self, value: T) -> i32 {
    value.isMaximized(self);
    return 1;
  }
}

pub trait QWidget_isMaximized {
  fn isMaximized(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isMaximized();
impl<'a> /*trait*/ QWidget_isMaximized for () {
  fn isMaximized(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isMaximizedEv()};
    unsafe {_ZNK7QWidget11isMaximizedEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn resize<T: QWidget_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QWidget_resize {
  fn resize(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::resize(const QSize & );
impl<'a> /*trait*/ QWidget_resize for (&'a  QSize) {
  fn resize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget6resizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowHandle<T: QWidget_windowHandle>(&mut self, value: T) -> i32 {
    value.windowHandle(self);
    return 1;
  }
}

pub trait QWidget_windowHandle {
  fn windowHandle(self, this: &mut QWidget) -> i32;
}

// proto: QWindow * QWidget::windowHandle();
impl<'a> /*trait*/ QWidget_windowHandle for () {
  fn windowHandle(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12windowHandleEv()};
    unsafe {_ZNK7QWidget12windowHandleEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn accessibleName<T: QWidget_accessibleName>(&mut self, value: T) -> i32 {
    value.accessibleName(self);
    return 1;
  }
}

pub trait QWidget_accessibleName {
  fn accessibleName(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::accessibleName();
impl<'a> /*trait*/ QWidget_accessibleName for () {
  fn accessibleName(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14accessibleNameEv()};
    unsafe {_ZNK7QWidget14accessibleNameEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setContentsMargins<T: QWidget_setContentsMargins>(&mut self, value: T) -> i32 {
    value.setContentsMargins(self);
    return 1;
  }
}

pub trait QWidget_setContentsMargins {
  fn setContentsMargins(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setContentsMargins(const QMargins & margins);
impl<'a> /*trait*/ QWidget_setContentsMargins for (&'a  QMargins) {
  fn setContentsMargins(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setContentsMarginsERK8QMargins()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget18setContentsMarginsERK8QMargins(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn saveGeometry<T: QWidget_saveGeometry>(&mut self, value: T) -> i32 {
    value.saveGeometry(self);
    return 1;
  }
}

pub trait QWidget_saveGeometry {
  fn saveGeometry(self, this: &mut QWidget) -> i32;
}

// proto: QByteArray QWidget::saveGeometry();
impl<'a> /*trait*/ QWidget_saveGeometry for () {
  fn saveGeometry(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12saveGeometryEv()};
    unsafe {_ZNK7QWidget12saveGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isEnabled<T: QWidget_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QWidget_isEnabled {
  fn isEnabled(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isEnabled();
impl<'a> /*trait*/ QWidget_isEnabled for () {
  fn isEnabled(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9isEnabledEv()};
    unsafe {_ZNK7QWidget9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFixedHeight<T: QWidget_setFixedHeight>(&mut self, value: T) -> i32 {
    value.setFixedHeight(self);
    return 1;
  }
}

pub trait QWidget_setFixedHeight {
  fn setFixedHeight(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setFixedHeight(int h);
impl<'a> /*trait*/ QWidget_setFixedHeight for (i32) {
  fn setFixedHeight(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setFixedHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWidget14setFixedHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mask<T: QWidget_mask>(&mut self, value: T) -> i32 {
    value.mask(self);
    return 1;
  }
}

pub trait QWidget_mask {
  fn mask(self, this: &mut QWidget) -> i32;
}

// proto: QRegion QWidget::mask();
impl<'a> /*trait*/ QWidget_mask for () {
  fn mask(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4maskEv()};
    unsafe {_ZNK7QWidget4maskEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn stackUnder<T: QWidget_stackUnder>(&mut self, value: T) -> i32 {
    value.stackUnder(self);
    return 1;
  }
}

pub trait QWidget_stackUnder {
  fn stackUnder(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::stackUnder(QWidget * );
impl<'a> /*trait*/ QWidget_stackUnder for (&'a mut QWidget) {
  fn stackUnder(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10stackUnderEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget10stackUnderEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn paintEngine<T: QWidget_paintEngine>(&mut self, value: T) -> i32 {
    value.paintEngine(self);
    return 1;
  }
}

pub trait QWidget_paintEngine {
  fn paintEngine(self, this: &mut QWidget) -> i32;
}

// proto: QPaintEngine * QWidget::paintEngine();
impl<'a> /*trait*/ QWidget_paintEngine for () {
  fn paintEngine(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11paintEngineEv()};
    unsafe {_ZNK7QWidget11paintEngineEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setAcceptDrops<T: QWidget_setAcceptDrops>(&mut self, value: T) -> i32 {
    value.setAcceptDrops(self);
    return 1;
  }
}

pub trait QWidget_setAcceptDrops {
  fn setAcceptDrops(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setAcceptDrops(bool on);
impl<'a> /*trait*/ QWidget_setAcceptDrops for (i8) {
  fn setAcceptDrops(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setAcceptDropsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget14setAcceptDropsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn move_<T: QWidget_move_>(&mut self, value: T) -> i32 {
    value.move_(self);
    return 1;
  }
}

pub trait QWidget_move_ {
  fn move_(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::move_(const QPoint & );
impl<'a> /*trait*/ QWidget_move_ for (&'a  QPoint) {
  fn move_(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4moveERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget4moveERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn actions<T: QWidget_actions>(&mut self, value: T) -> i32 {
    value.actions(self);
    return 1;
  }
}

pub trait QWidget_actions {
  fn actions(self, this: &mut QWidget) -> i32;
}

// proto: QList<QAction *> QWidget::actions();
impl<'a> /*trait*/ QWidget_actions for () {
  fn actions(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7actionsEv()};
    unsafe {_ZNK7QWidget7actionsEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn show<T: QWidget_show>(&mut self, value: T) -> i32 {
    value.show(self);
    return 1;
  }
}

pub trait QWidget_show {
  fn show(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::show();
impl<'a> /*trait*/ QWidget_show for () {
  fn show(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4showEv()};
    unsafe {_ZN7QWidget4showEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn keyboardGrabber<T: QWidget_keyboardGrabber>(&mut self, value: T) -> i32 {
    value.keyboardGrabber(self);
    return 1;
  }
}

pub trait QWidget_keyboardGrabber {
  fn keyboardGrabber(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::keyboardGrabber();
impl<'a> /*trait*/ QWidget_keyboardGrabber for () {
  fn keyboardGrabber(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15keyboardGrabberEv()};
    unsafe {_ZN7QWidget15keyboardGrabberEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapTo<T: QWidget_mapTo>(&mut self, value: T) -> i32 {
    value.mapTo(self);
    return 1;
  }
}

pub trait QWidget_mapTo {
  fn mapTo(self, this: &mut QWidget) -> i32;
}

// proto: QPoint QWidget::mapTo(const QWidget * , const QPoint & );
impl<'a> /*trait*/ QWidget_mapTo for (&'a  QWidget, &'a  QPoint) {
  fn mapTo(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5mapToEPKS_RK6QPoint()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget5mapToEPKS_RK6QPoint(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn minimumWidth<T: QWidget_minimumWidth>(&mut self, value: T) -> i32 {
    value.minimumWidth(self);
    return 1;
  }
}

pub trait QWidget_minimumWidth {
  fn minimumWidth(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::minimumWidth();
impl<'a> /*trait*/ QWidget_minimumWidth for () {
  fn minimumWidth(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12minimumWidthEv()};
    unsafe {_ZNK7QWidget12minimumWidthEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn fontInfo<T: QWidget_fontInfo>(&mut self, value: T) -> i32 {
    value.fontInfo(self);
    return 1;
  }
}

pub trait QWidget_fontInfo {
  fn fontInfo(self, this: &mut QWidget) -> i32;
}

// proto: QFontInfo QWidget::fontInfo();
impl<'a> /*trait*/ QWidget_fontInfo for () {
  fn fontInfo(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8fontInfoEv()};
    unsafe {_ZNK7QWidget8fontInfoEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn autoFillBackground<T: QWidget_autoFillBackground>(&mut self, value: T) -> i32 {
    value.autoFillBackground(self);
    return 1;
  }
}

pub trait QWidget_autoFillBackground {
  fn autoFillBackground(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::autoFillBackground();
impl<'a> /*trait*/ QWidget_autoFillBackground for () {
  fn autoFillBackground(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18autoFillBackgroundEv()};
    unsafe {_ZNK7QWidget18autoFillBackgroundEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn scroll<T: QWidget_scroll>(&mut self, value: T) -> i32 {
    value.scroll(self);
    return 1;
  }
}

pub trait QWidget_scroll {
  fn scroll(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::scroll(int dx, int dy, const QRect & );
impl<'a> /*trait*/ QWidget_scroll for (i32, i32, &'a  QRect) {
  fn scroll(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6scrollEiiRK5QRect()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget6scrollEiiRK5QRect(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn fontMetrics<T: QWidget_fontMetrics>(&mut self, value: T) -> i32 {
    value.fontMetrics(self);
    return 1;
  }
}

pub trait QWidget_fontMetrics {
  fn fontMetrics(self, this: &mut QWidget) -> i32;
}

// proto: QFontMetrics QWidget::fontMetrics();
impl<'a> /*trait*/ QWidget_fontMetrics for () {
  fn fontMetrics(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11fontMetricsEv()};
    unsafe {_ZNK7QWidget11fontMetricsEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn adjustSize<T: QWidget_adjustSize>(&mut self, value: T) -> i32 {
    value.adjustSize(self);
    return 1;
  }
}

pub trait QWidget_adjustSize {
  fn adjustSize(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::adjustSize();
impl<'a> /*trait*/ QWidget_adjustSize for () {
  fn adjustSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10adjustSizeEv()};
    unsafe {_ZN7QWidget10adjustSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn previousInFocusChain<T: QWidget_previousInFocusChain>(&mut self, value: T) -> i32 {
    value.previousInFocusChain(self);
    return 1;
  }
}

pub trait QWidget_previousInFocusChain {
  fn previousInFocusChain(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::previousInFocusChain();
impl<'a> /*trait*/ QWidget_previousInFocusChain for () {
  fn previousInFocusChain(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget20previousInFocusChainEv()};
    unsafe {_ZNK7QWidget20previousInFocusChainEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn updatesEnabled<T: QWidget_updatesEnabled>(&mut self, value: T) -> i32 {
    value.updatesEnabled(self);
    return 1;
  }
}

pub trait QWidget_updatesEnabled {
  fn updatesEnabled(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::updatesEnabled();
impl<'a> /*trait*/ QWidget_updatesEnabled for () {
  fn updatesEnabled(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14updatesEnabledEv()};
    unsafe {_ZNK7QWidget14updatesEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMaximumHeight<T: QWidget_setMaximumHeight>(&mut self, value: T) -> i32 {
    value.setMaximumHeight(self);
    return 1;
  }
}

pub trait QWidget_setMaximumHeight {
  fn setMaximumHeight(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setMaximumHeight(int maxh);
impl<'a> /*trait*/ QWidget_setMaximumHeight for (i32) {
  fn setMaximumHeight(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMaximumHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWidget16setMaximumHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn showMaximized<T: QWidget_showMaximized>(&mut self, value: T) -> i32 {
    value.showMaximized(self);
    return 1;
  }
}

pub trait QWidget_showMaximized {
  fn showMaximized(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::showMaximized();
impl<'a> /*trait*/ QWidget_showMaximized for () {
  fn showMaximized(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13showMaximizedEv()};
    unsafe {_ZN7QWidget13showMaximizedEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapFrom<T: QWidget_mapFrom>(&mut self, value: T) -> i32 {
    value.mapFrom(self);
    return 1;
  }
}

pub trait QWidget_mapFrom {
  fn mapFrom(self, this: &mut QWidget) -> i32;
}

// proto: QPoint QWidget::mapFrom(const QWidget * , const QPoint & );
impl<'a> /*trait*/ QWidget_mapFrom for (&'a  QWidget, &'a  QPoint) {
  fn mapFrom(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7mapFromEPKS_RK6QPoint()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget7mapFromEPKS_RK6QPoint(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn x<T: QWidget_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QWidget_x {
  fn x(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::x();
impl<'a> /*trait*/ QWidget_x for () {
  fn x(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget1xEv()};
    unsafe {_ZNK7QWidget1xEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn clearFocus<T: QWidget_clearFocus>(&mut self, value: T) -> i32 {
    value.clearFocus(self);
    return 1;
  }
}

pub trait QWidget_clearFocus {
  fn clearFocus(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::clearFocus();
impl<'a> /*trait*/ QWidget_clearFocus for () {
  fn clearFocus(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10clearFocusEv()};
    unsafe {_ZN7QWidget10clearFocusEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn find<T: QWidget_find>(&mut self, value: T) -> i32 {
    value.find(self);
    return 1;
  }
}

pub trait QWidget_find {
  fn find(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::find(WId );
impl<'a> /*trait*/ QWidget_find for (*mut i32) {
  fn find(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4findEi()};
    let arg0 = self  as *mut c_uint;
    unsafe {_ZN7QWidget4findEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn palette<T: QWidget_palette>(&mut self, value: T) -> i32 {
    value.palette(self);
    return 1;
  }
}

pub trait QWidget_palette {
  fn palette(self, this: &mut QWidget) -> i32;
}

// proto: const QPalette & QWidget::palette();
impl<'a> /*trait*/ QWidget_palette for () {
  fn palette(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7paletteEv()};
    unsafe {_ZNK7QWidget7paletteEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setSizePolicy<T: QWidget_setSizePolicy>(&mut self, value: T) -> i32 {
    value.setSizePolicy(self);
    return 1;
  }
}

pub trait QWidget_setSizePolicy {
  fn setSizePolicy(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setSizePolicy(QSizePolicy );
impl<'a> /*trait*/ QWidget_setSizePolicy for (QSizePolicy) {
  fn setSizePolicy(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setSizePolicyE11QSizePolicy()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget13setSizePolicyE11QSizePolicy(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMask<T: QWidget_setMask>(&mut self, value: T) -> i32 {
    value.setMask(self);
    return 1;
  }
}

pub trait QWidget_setMask {
  fn setMask(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setMask(const QRegion & );
impl<'a> /*trait*/ QWidget_setMask for (&'a  QRegion) {
  fn setMask(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setMaskERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget7setMaskERK7QRegion(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMaximumWidth<T: QWidget_setMaximumWidth>(&mut self, value: T) -> i32 {
    value.setMaximumWidth(self);
    return 1;
  }
}

pub trait QWidget_setMaximumWidth {
  fn setMaximumWidth(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setMaximumWidth(int maxw);
impl<'a> /*trait*/ QWidget_setMaximumWidth for (i32) {
  fn setMaximumWidth(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15setMaximumWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWidget15setMaximumWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowIconText<T: QWidget_setWindowIconText>(&mut self, value: T) -> i32 {
    value.setWindowIconText(self);
    return 1;
  }
}

pub trait QWidget_setWindowIconText {
  fn setWindowIconText(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setWindowIconText(const QString & );
impl<'a> /*trait*/ QWidget_setWindowIconText for (&'a  QString) {
  fn setWindowIconText(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowIconTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget17setWindowIconTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowIcon<T: QWidget_setWindowIcon>(&mut self, value: T) -> i32 {
    value.setWindowIcon(self);
    return 1;
  }
}

pub trait QWidget_setWindowIcon {
  fn setWindowIcon(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QWidget_setWindowIcon for (&'a  QIcon) {
  fn setWindowIcon(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget13setWindowIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn FreeQWidget<T: QWidget_FreeQWidget>(&mut self, value: T) -> i32 {
    value.FreeQWidget(self);
    return 1;
  }
}

pub trait QWidget_FreeQWidget {
  fn FreeQWidget(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::FreeQWidget();
impl<'a> /*trait*/ QWidget_FreeQWidget for () {
  fn FreeQWidget(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidgetD0Ev()};
    unsafe {_ZN7QWidgetD0Ev()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn getContentsMargins<T: QWidget_getContentsMargins>(&mut self, value: T) -> i32 {
    value.getContentsMargins(self);
    return 1;
  }
}

pub trait QWidget_getContentsMargins {
  fn getContentsMargins(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::getContentsMargins(int * left, int * top, int * right, int * bottom);
impl<'a> /*trait*/ QWidget_getContentsMargins for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getContentsMargins(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn minimumSizeHint<T: QWidget_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QWidget_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QWidget) -> i32;
}

// proto: QSize QWidget::minimumSizeHint();
impl<'a> /*trait*/ QWidget_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15minimumSizeHintEv()};
    unsafe {_ZNK7QWidget15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowModified<T: QWidget_setWindowModified>(&mut self, value: T) -> i32 {
    value.setWindowModified(self);
    return 1;
  }
}

pub trait QWidget_setWindowModified {
  fn setWindowModified(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setWindowModified(bool );
impl<'a> /*trait*/ QWidget_setWindowModified for (i8) {
  fn setWindowModified(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowModifiedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget17setWindowModifiedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn restoreGeometry<T: QWidget_restoreGeometry>(&mut self, value: T) -> i32 {
    value.restoreGeometry(self);
    return 1;
  }
}

pub trait QWidget_restoreGeometry {
  fn restoreGeometry(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::restoreGeometry(const QByteArray & geometry);
impl<'a> /*trait*/ QWidget_restoreGeometry for (&'a  QByteArray) {
  fn restoreGeometry(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15restoreGeometryERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget15restoreGeometryERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn layout<T: QWidget_layout>(&mut self, value: T) -> i32 {
    value.layout(self);
    return 1;
  }
}

pub trait QWidget_layout {
  fn layout(self, this: &mut QWidget) -> i32;
}

// proto: QLayout * QWidget::layout();
impl<'a> /*trait*/ QWidget_layout for () {
  fn layout(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6layoutEv()};
    unsafe {_ZNK7QWidget6layoutEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn contentsRect<T: QWidget_contentsRect>(&mut self, value: T) -> i32 {
    value.contentsRect(self);
    return 1;
  }
}

pub trait QWidget_contentsRect {
  fn contentsRect(self, this: &mut QWidget) -> i32;
}

// proto: QRect QWidget::contentsRect();
impl<'a> /*trait*/ QWidget_contentsRect for () {
  fn contentsRect(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12contentsRectEv()};
    unsafe {_ZNK7QWidget12contentsRectEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn backingStore<T: QWidget_backingStore>(&mut self, value: T) -> i32 {
    value.backingStore(self);
    return 1;
  }
}

pub trait QWidget_backingStore {
  fn backingStore(self, this: &mut QWidget) -> i32;
}

// proto: QBackingStore * QWidget::backingStore();
impl<'a> /*trait*/ QWidget_backingStore for () {
  fn backingStore(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12backingStoreEv()};
    unsafe {_ZNK7QWidget12backingStoreEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn focusProxy<T: QWidget_focusProxy>(&mut self, value: T) -> i32 {
    value.focusProxy(self);
    return 1;
  }
}

pub trait QWidget_focusProxy {
  fn focusProxy(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::focusProxy();
impl<'a> /*trait*/ QWidget_focusProxy for () {
  fn focusProxy(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10focusProxyEv()};
    unsafe {_ZNK7QWidget10focusProxyEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn styleSheet<T: QWidget_styleSheet>(&mut self, value: T) -> i32 {
    value.styleSheet(self);
    return 1;
  }
}

pub trait QWidget_styleSheet {
  fn styleSheet(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::styleSheet();
impl<'a> /*trait*/ QWidget_styleSheet for () {
  fn styleSheet(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10styleSheetEv()};
    unsafe {_ZNK7QWidget10styleSheetEv()};
    return 1;
  }
}

// proto: QWidget * QWidget::childAt(const QPoint & p);
impl<'a> /*trait*/ QWidget_childAt for (&'a  QPoint) {
  fn childAt(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7childAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget7childAtERK6QPoint(arg0)};
    return 1;
  }
}

// proto: void QWidget::repaint(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_repaint for (i32, i32, i32, i32) {
  fn repaint(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN7QWidget7repaintEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn whatsThis<T: QWidget_whatsThis>(&mut self, value: T) -> i32 {
    value.whatsThis(self);
    return 1;
  }
}

pub trait QWidget_whatsThis {
  fn whatsThis(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::whatsThis();
impl<'a> /*trait*/ QWidget_whatsThis for () {
  fn whatsThis(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9whatsThisEv()};
    unsafe {_ZNK7QWidget9whatsThisEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn font<T: QWidget_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QWidget_font {
  fn font(self, this: &mut QWidget) -> i32;
}

// proto: const QFont & QWidget::font();
impl<'a> /*trait*/ QWidget_font for () {
  fn font(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4fontEv()};
    unsafe {_ZNK7QWidget4fontEv()};
    return 1;
  }
}

// proto: void QWidget::setMinimumSize(int minw, int minh);
impl<'a> /*trait*/ QWidget_setMinimumSize for (i32, i32) {
  fn setMinimumSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMinimumSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWidget14setMinimumSizeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn metaObject<T: QWidget_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QWidget_metaObject {
  fn metaObject(self, this: &mut QWidget) -> i32;
}

// proto: const QMetaObject * QWidget::metaObject();
impl<'a> /*trait*/ QWidget_metaObject for () {
  fn metaObject(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10metaObjectEv()};
    unsafe {_ZNK7QWidget10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMinimumHeight<T: QWidget_setMinimumHeight>(&mut self, value: T) -> i32 {
    value.setMinimumHeight(self);
    return 1;
  }
}

pub trait QWidget_setMinimumHeight {
  fn setMinimumHeight(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setMinimumHeight(int minh);
impl<'a> /*trait*/ QWidget_setMinimumHeight for (i32) {
  fn setMinimumHeight(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMinimumHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWidget16setMinimumHeightEi(arg0)};
    return 1;
  }
}

// proto: void QWidget::update(const QRegion & );
impl<'a> /*trait*/ QWidget_update for (&'a  QRegion) {
  fn update(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget6updateERK7QRegion(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowOpacity<T: QWidget_windowOpacity>(&mut self, value: T) -> i32 {
    value.windowOpacity(self);
    return 1;
  }
}

pub trait QWidget_windowOpacity {
  fn windowOpacity(self, this: &mut QWidget) -> i32;
}

// proto: double QWidget::windowOpacity();
impl<'a> /*trait*/ QWidget_windowOpacity for () {
  fn windowOpacity(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13windowOpacityEv()};
    unsafe {_ZNK7QWidget13windowOpacityEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn childrenRegion<T: QWidget_childrenRegion>(&mut self, value: T) -> i32 {
    value.childrenRegion(self);
    return 1;
  }
}

pub trait QWidget_childrenRegion {
  fn childrenRegion(self, this: &mut QWidget) -> i32;
}

// proto: QRegion QWidget::childrenRegion();
impl<'a> /*trait*/ QWidget_childrenRegion for () {
  fn childrenRegion(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14childrenRegionEv()};
    unsafe {_ZNK7QWidget14childrenRegionEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowFilePath<T: QWidget_setWindowFilePath>(&mut self, value: T) -> i32 {
    value.setWindowFilePath(self);
    return 1;
  }
}

pub trait QWidget_setWindowFilePath {
  fn setWindowFilePath(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setWindowFilePath(const QString & filePath);
impl<'a> /*trait*/ QWidget_setWindowFilePath for (&'a  QString) {
  fn setWindowFilePath(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowFilePathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget17setWindowFilePathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setShortcutEnabled<T: QWidget_setShortcutEnabled>(&mut self, value: T) -> i32 {
    value.setShortcutEnabled(self);
    return 1;
  }
}

pub trait QWidget_setShortcutEnabled {
  fn setShortcutEnabled(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setShortcutEnabled(int id, bool enable);
impl<'a> /*trait*/ QWidget_setShortcutEnabled for (i32, i8) {
  fn setShortcutEnabled(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setShortcutEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN7QWidget18setShortcutEnabledEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn raise<T: QWidget_raise>(&mut self, value: T) -> i32 {
    value.raise(self);
    return 1;
  }
}

pub trait QWidget_raise {
  fn raise(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::raise();
impl<'a> /*trait*/ QWidget_raise for () {
  fn raise(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5raiseEv()};
    unsafe {_ZN7QWidget5raiseEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn statusTip<T: QWidget_statusTip>(&mut self, value: T) -> i32 {
    value.statusTip(self);
    return 1;
  }
}

pub trait QWidget_statusTip {
  fn statusTip(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::statusTip();
impl<'a> /*trait*/ QWidget_statusTip for () {
  fn statusTip(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9statusTipEv()};
    unsafe {_ZNK7QWidget9statusTipEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn childrenRect<T: QWidget_childrenRect>(&mut self, value: T) -> i32 {
    value.childrenRect(self);
    return 1;
  }
}

pub trait QWidget_childrenRect {
  fn childrenRect(self, this: &mut QWidget) -> i32;
}

// proto: QRect QWidget::childrenRect();
impl<'a> /*trait*/ QWidget_childrenRect for () {
  fn childrenRect(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12childrenRectEv()};
    unsafe {_ZNK7QWidget12childrenRectEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setParent<T: QWidget_setParent>(&mut self, value: T) -> i32 {
    value.setParent(self);
    return 1;
  }
}

pub trait QWidget_setParent {
  fn setParent(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setParent(QWidget * parent);
impl<'a> /*trait*/ QWidget_setParent for (&'a mut QWidget) {
  fn setParent(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget9setParentEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn visibleRegion<T: QWidget_visibleRegion>(&mut self, value: T) -> i32 {
    value.visibleRegion(self);
    return 1;
  }
}

pub trait QWidget_visibleRegion {
  fn visibleRegion(self, this: &mut QWidget) -> i32;
}

// proto: QRegion QWidget::visibleRegion();
impl<'a> /*trait*/ QWidget_visibleRegion for () {
  fn visibleRegion(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13visibleRegionEv()};
    unsafe {_ZNK7QWidget13visibleRegionEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn locale<T: QWidget_locale>(&mut self, value: T) -> i32 {
    value.locale(self);
    return 1;
  }
}

pub trait QWidget_locale {
  fn locale(self, this: &mut QWidget) -> i32;
}

// proto: QLocale QWidget::locale();
impl<'a> /*trait*/ QWidget_locale for () {
  fn locale(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6localeEv()};
    unsafe {_ZNK7QWidget6localeEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn releaseKeyboard<T: QWidget_releaseKeyboard>(&mut self, value: T) -> i32 {
    value.releaseKeyboard(self);
    return 1;
  }
}

pub trait QWidget_releaseKeyboard {
  fn releaseKeyboard(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::releaseKeyboard();
impl<'a> /*trait*/ QWidget_releaseKeyboard for () {
  fn releaseKeyboard(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15releaseKeyboardEv()};
    unsafe {_ZN7QWidget15releaseKeyboardEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mouseGrabber<T: QWidget_mouseGrabber>(&mut self, value: T) -> i32 {
    value.mouseGrabber(self);
    return 1;
  }
}

pub trait QWidget_mouseGrabber {
  fn mouseGrabber(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::mouseGrabber();
impl<'a> /*trait*/ QWidget_mouseGrabber for () {
  fn mouseGrabber(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12mouseGrabberEv()};
    unsafe {_ZN7QWidget12mouseGrabberEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFixedWidth<T: QWidget_setFixedWidth>(&mut self, value: T) -> i32 {
    value.setFixedWidth(self);
    return 1;
  }
}

pub trait QWidget_setFixedWidth {
  fn setFixedWidth(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setFixedWidth(int w);
impl<'a> /*trait*/ QWidget_setFixedWidth for (i32) {
  fn setFixedWidth(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setFixedWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWidget13setFixedWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn addAction<T: QWidget_addAction>(&mut self, value: T) -> i32 {
    value.addAction(self);
    return 1;
  }
}

pub trait QWidget_addAction {
  fn addAction(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::addAction(QAction * action);
impl<'a> /*trait*/ QWidget_addAction for (&'a mut QAction) {
  fn addAction(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget9addActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setDisabled<T: QWidget_setDisabled>(&mut self, value: T) -> i32 {
    value.setDisabled(self);
    return 1;
  }
}

pub trait QWidget_setDisabled {
  fn setDisabled(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setDisabled(bool );
impl<'a> /*trait*/ QWidget_setDisabled for (i8) {
  fn setDisabled(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setDisabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget11setDisabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowIcon<T: QWidget_windowIcon>(&mut self, value: T) -> i32 {
    value.windowIcon(self);
    return 1;
  }
}

pub trait QWidget_windowIcon {
  fn windowIcon(self, this: &mut QWidget) -> i32;
}

// proto: QIcon QWidget::windowIcon();
impl<'a> /*trait*/ QWidget_windowIcon for () {
  fn windowIcon(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10windowIconEv()};
    unsafe {_ZNK7QWidget10windowIconEv()};
    return 1;
  }
}

// proto: void QWidget::setContentsMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QWidget_setContentsMargins for (i32, i32, i32, i32) {
  fn setContentsMargins(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setContentsMarginsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN7QWidget18setContentsMarginsEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowRole<T: QWidget_windowRole>(&mut self, value: T) -> i32 {
    value.windowRole(self);
    return 1;
  }
}

pub trait QWidget_windowRole {
  fn windowRole(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::windowRole();
impl<'a> /*trait*/ QWidget_windowRole for () {
  fn windowRole(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10windowRoleEv()};
    unsafe {_ZNK7QWidget10windowRoleEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setShortcutAutoRepeat<T: QWidget_setShortcutAutoRepeat>(&mut self, value: T) -> i32 {
    value.setShortcutAutoRepeat(self);
    return 1;
  }
}

pub trait QWidget_setShortcutAutoRepeat {
  fn setShortcutAutoRepeat(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setShortcutAutoRepeat(int id, bool enable);
impl<'a> /*trait*/ QWidget_setShortcutAutoRepeat for (i32, i8) {
  fn setShortcutAutoRepeat(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget21setShortcutAutoRepeatEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN7QWidget21setShortcutAutoRepeatEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn showFullScreen<T: QWidget_showFullScreen>(&mut self, value: T) -> i32 {
    value.showFullScreen(self);
    return 1;
  }
}

pub trait QWidget_showFullScreen {
  fn showFullScreen(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::showFullScreen();
impl<'a> /*trait*/ QWidget_showFullScreen for () {
  fn showFullScreen(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14showFullScreenEv()};
    unsafe {_ZN7QWidget14showFullScreenEv()};
    return 1;
  }
}

// proto: void QWidget::grabMouse();
impl<'a> /*trait*/ QWidget_grabMouse for () {
  fn grabMouse(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9grabMouseEv()};
    unsafe {_ZN7QWidget9grabMouseEv()};
    return 1;
  }
}

// proto: void QWidget::setMaximumSize(const QSize & );
impl<'a> /*trait*/ QWidget_setMaximumSize for (&'a  QSize) {
  fn setMaximumSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMaximumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget14setMaximumSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapToGlobal<T: QWidget_mapToGlobal>(&mut self, value: T) -> i32 {
    value.mapToGlobal(self);
    return 1;
  }
}

pub trait QWidget_mapToGlobal {
  fn mapToGlobal(self, this: &mut QWidget) -> i32;
}

// proto: QPoint QWidget::mapToGlobal(const QPoint & );
impl<'a> /*trait*/ QWidget_mapToGlobal for (&'a  QPoint) {
  fn mapToGlobal(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11mapToGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget11mapToGlobalERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn toolTip<T: QWidget_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QWidget_toolTip {
  fn toolTip(self, this: &mut QWidget) -> i32;
}

// proto: QString QWidget::toolTip();
impl<'a> /*trait*/ QWidget_toolTip for () {
  fn toolTip(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7toolTipEv()};
    unsafe {_ZNK7QWidget7toolTipEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWhatsThis<T: QWidget_setWhatsThis>(&mut self, value: T) -> i32 {
    value.setWhatsThis(self);
    return 1;
  }
}

pub trait QWidget_setWhatsThis {
  fn setWhatsThis(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setWhatsThis(const QString & );
impl<'a> /*trait*/ QWidget_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget12setWhatsThisERK7QString(arg0)};
    return 1;
  }
}

// proto: void QWidget::resize(int w, int h);
impl<'a> /*trait*/ QWidget_resize for (i32, i32) {
  fn resize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWidget6resizeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn parentWidget<T: QWidget_parentWidget>(&mut self, value: T) -> i32 {
    value.parentWidget(self);
    return 1;
  }
}

pub trait QWidget_parentWidget {
  fn parentWidget(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::parentWidget();
impl<'a> /*trait*/ QWidget_parentWidget for () {
  fn parentWidget(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12parentWidgetEv()};
    unsafe {_ZNK7QWidget12parentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn pos<T: QWidget_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QWidget_pos {
  fn pos(self, this: &mut QWidget) -> i32;
}

// proto: QPoint QWidget::pos();
impl<'a> /*trait*/ QWidget_pos for () {
  fn pos(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget3posEv()};
    unsafe {_ZNK7QWidget3posEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setAutoFillBackground<T: QWidget_setAutoFillBackground>(&mut self, value: T) -> i32 {
    value.setAutoFillBackground(self);
    return 1;
  }
}

pub trait QWidget_setAutoFillBackground {
  fn setAutoFillBackground(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setAutoFillBackground(bool enabled);
impl<'a> /*trait*/ QWidget_setAutoFillBackground for (i8) {
  fn setAutoFillBackground(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget21setAutoFillBackgroundEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget21setAutoFillBackgroundEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn hasFocus<T: QWidget_hasFocus>(&mut self, value: T) -> i32 {
    value.hasFocus(self);
    return 1;
  }
}

pub trait QWidget_hasFocus {
  fn hasFocus(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::hasFocus();
impl<'a> /*trait*/ QWidget_hasFocus for () {
  fn hasFocus(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8hasFocusEv()};
    unsafe {_ZNK7QWidget8hasFocusEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn baseSize<T: QWidget_baseSize>(&mut self, value: T) -> i32 {
    value.baseSize(self);
    return 1;
  }
}

pub trait QWidget_baseSize {
  fn baseSize(self, this: &mut QWidget) -> i32;
}

// proto: QSize QWidget::baseSize();
impl<'a> /*trait*/ QWidget_baseSize for () {
  fn baseSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8baseSizeEv()};
    unsafe {_ZNK7QWidget8baseSizeEv()};
    return 1;
  }
}

// proto: void QWidget::setMask(const QBitmap & );
impl<'a> /*trait*/ QWidget_setMask for (&'a  QBitmap) {
  fn setMask(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setMaskERK7QBitmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget7setMaskERK7QBitmap(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn ensurePolished<T: QWidget_ensurePolished>(&mut self, value: T) -> i32 {
    value.ensurePolished(self);
    return 1;
  }
}

pub trait QWidget_ensurePolished {
  fn ensurePolished(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::ensurePolished();
impl<'a> /*trait*/ QWidget_ensurePolished for () {
  fn ensurePolished(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14ensurePolishedEv()};
    unsafe {_ZNK7QWidget14ensurePolishedEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowTitle<T: QWidget_setWindowTitle>(&mut self, value: T) -> i32 {
    value.setWindowTitle(self);
    return 1;
  }
}

pub trait QWidget_setWindowTitle {
  fn setWindowTitle(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setWindowTitle(const QString & );
impl<'a> /*trait*/ QWidget_setWindowTitle for (&'a  QString) {
  fn setWindowTitle(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setWindowTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget14setWindowTitleERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn window<T: QWidget_window>(&mut self, value: T) -> i32 {
    value.window(self);
    return 1;
  }
}

pub trait QWidget_window {
  fn window(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::window();
impl<'a> /*trait*/ QWidget_window for () {
  fn window(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6windowEv()};
    unsafe {_ZNK7QWidget6windowEv()};
    return 1;
  }
}

// proto: void QWidget::scroll(int dx, int dy);
impl<'a> /*trait*/ QWidget_scroll for (i32, i32) {
  fn scroll(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6scrollEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWidget6scrollEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn releaseShortcut<T: QWidget_releaseShortcut>(&mut self, value: T) -> i32 {
    value.releaseShortcut(self);
    return 1;
  }
}

pub trait QWidget_releaseShortcut {
  fn releaseShortcut(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::releaseShortcut(int id);
impl<'a> /*trait*/ QWidget_releaseShortcut for (i32) {
  fn releaseShortcut(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15releaseShortcutEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWidget15releaseShortcutEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setToolTipDuration<T: QWidget_setToolTipDuration>(&mut self, value: T) -> i32 {
    value.setToolTipDuration(self);
    return 1;
  }
}

pub trait QWidget_setToolTipDuration {
  fn setToolTipDuration(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setToolTipDuration(int msec);
impl<'a> /*trait*/ QWidget_setToolTipDuration for (i32) {
  fn setToolTipDuration(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setToolTipDurationEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWidget18setToolTipDurationEi(arg0)};
    return 1;
  }
}

// proto: void QWidget::setGeometry(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_setGeometry for (i32, i32, i32, i32) {
  fn setGeometry(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setGeometryEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN7QWidget11setGeometryEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QWidget::setSizeIncrement(int w, int h);
impl<'a> /*trait*/ QWidget_setSizeIncrement for (i32, i32) {
  fn setSizeIncrement(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setSizeIncrementEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWidget16setSizeIncrementEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setUpdatesEnabled<T: QWidget_setUpdatesEnabled>(&mut self, value: T) -> i32 {
    value.setUpdatesEnabled(self);
    return 1;
  }
}

pub trait QWidget_setUpdatesEnabled {
  fn setUpdatesEnabled(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setUpdatesEnabled(bool enable);
impl<'a> /*trait*/ QWidget_setUpdatesEnabled for (i8) {
  fn setUpdatesEnabled(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setUpdatesEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget17setUpdatesEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn lower<T: QWidget_lower>(&mut self, value: T) -> i32 {
    value.lower(self);
    return 1;
  }
}

pub trait QWidget_lower {
  fn lower(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::lower();
impl<'a> /*trait*/ QWidget_lower for () {
  fn lower(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5lowerEv()};
    unsafe {_ZN7QWidget5lowerEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMouseTracking<T: QWidget_setMouseTracking>(&mut self, value: T) -> i32 {
    value.setMouseTracking(self);
    return 1;
  }
}

pub trait QWidget_setMouseTracking {
  fn setMouseTracking(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setMouseTracking(bool enable);
impl<'a> /*trait*/ QWidget_setMouseTracking for (i8) {
  fn setMouseTracking(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMouseTrackingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget16setMouseTrackingEb(arg0)};
    return 1;
  }
}

// proto: void QWidget::setBaseSize(const QSize & );
impl<'a> /*trait*/ QWidget_setBaseSize for (&'a  QSize) {
  fn setBaseSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setBaseSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget11setBaseSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn hide<T: QWidget_hide>(&mut self, value: T) -> i32 {
    value.hide(self);
    return 1;
  }
}

pub trait QWidget_hide {
  fn hide(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::hide();
impl<'a> /*trait*/ QWidget_hide for () {
  fn hide(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4hideEv()};
    unsafe {_ZN7QWidget4hideEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn removeAction<T: QWidget_removeAction>(&mut self, value: T) -> i32 {
    value.removeAction(self);
    return 1;
  }
}

pub trait QWidget_removeAction {
  fn removeAction(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::removeAction(QAction * action);
impl<'a> /*trait*/ QWidget_removeAction for (&'a mut QAction) {
  fn removeAction(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget12removeActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFocusProxy<T: QWidget_setFocusProxy>(&mut self, value: T) -> i32 {
    value.setFocusProxy(self);
    return 1;
  }
}

pub trait QWidget_setFocusProxy {
  fn setFocusProxy(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setFocusProxy(QWidget * );
impl<'a> /*trait*/ QWidget_setFocusProxy for (&'a mut QWidget) {
  fn setFocusProxy(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setFocusProxyEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget13setFocusProxyEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn close<T: QWidget_close>(&mut self, value: T) -> i32 {
    value.close(self);
    return 1;
  }
}

pub trait QWidget_close {
  fn close(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::close();
impl<'a> /*trait*/ QWidget_close for () {
  fn close(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5closeEv()};
    unsafe {_ZN7QWidget5closeEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn showMinimized<T: QWidget_showMinimized>(&mut self, value: T) -> i32 {
    value.showMinimized(self);
    return 1;
  }
}

pub trait QWidget_showMinimized {
  fn showMinimized(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::showMinimized();
impl<'a> /*trait*/ QWidget_showMinimized for () {
  fn showMinimized(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13showMinimizedEv()};
    unsafe {_ZN7QWidget13showMinimizedEv()};
    return 1;
  }
}

// proto: void QWidget::setFixedSize(int w, int h);
impl<'a> /*trait*/ QWidget_setFixedSize for (i32, i32) {
  fn setFixedSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setFixedSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWidget12setFixedSizeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn minimumSize<T: QWidget_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QWidget_minimumSize {
  fn minimumSize(self, this: &mut QWidget) -> i32;
}

// proto: QSize QWidget::minimumSize();
impl<'a> /*trait*/ QWidget_minimumSize for () {
  fn minimumSize(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11minimumSizeEv()};
    unsafe {_ZNK7QWidget11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setEnabled<T: QWidget_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QWidget_setEnabled {
  fn setEnabled(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setEnabled(bool );
impl<'a> /*trait*/ QWidget_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget10setEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn maximumHeight<T: QWidget_maximumHeight>(&mut self, value: T) -> i32 {
    value.maximumHeight(self);
    return 1;
  }
}

pub trait QWidget_maximumHeight {
  fn maximumHeight(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::maximumHeight();
impl<'a> /*trait*/ QWidget_maximumHeight for () {
  fn maximumHeight(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13maximumHeightEv()};
    unsafe {_ZNK7QWidget13maximumHeightEv()};
    return 1;
  }
}

// proto: void QWidget::move_(int x, int y);
impl<'a> /*trait*/ QWidget_move_ for (i32, i32) {
  fn move_(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4moveEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QWidget4moveEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isAncestorOf<T: QWidget_isAncestorOf>(&mut self, value: T) -> i32 {
    value.isAncestorOf(self);
    return 1;
  }
}

pub trait QWidget_isAncestorOf {
  fn isAncestorOf(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isAncestorOf(const QWidget * child);
impl<'a> /*trait*/ QWidget_isAncestorOf for (&'a  QWidget) {
  fn isAncestorOf(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12isAncestorOfEPKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget12isAncestorOfEPKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn NewQWidget<T: QWidget_NewQWidget>(value: T) -> QWidget {
    let rsthis = value.NewQWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QWidget_NewQWidget {
  fn NewQWidget(self) -> QWidget;
}

// proto: void QWidget::NewQWidget(const QWidget & );
impl<'a> /*trait*/ QWidget_NewQWidget for (&'a  QWidget) {
  fn NewQWidget(self) -> QWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn cursor<T: QWidget_cursor>(&mut self, value: T) -> i32 {
    value.cursor(self);
    return 1;
  }
}

pub trait QWidget_cursor {
  fn cursor(self, this: &mut QWidget) -> i32;
}

// proto: QCursor QWidget::cursor();
impl<'a> /*trait*/ QWidget_cursor for () {
  fn cursor(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6cursorEv()};
    unsafe {_ZNK7QWidget6cursorEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapFromGlobal<T: QWidget_mapFromGlobal>(&mut self, value: T) -> i32 {
    value.mapFromGlobal(self);
    return 1;
  }
}

pub trait QWidget_mapFromGlobal {
  fn mapFromGlobal(self, this: &mut QWidget) -> i32;
}

// proto: QPoint QWidget::mapFromGlobal(const QPoint & );
impl<'a> /*trait*/ QWidget_mapFromGlobal for (&'a  QPoint) {
  fn mapFromGlobal(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13mapFromGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWidget13mapFromGlobalERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setToolTip<T: QWidget_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QWidget_setToolTip {
  fn setToolTip(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setToolTip(const QString & );
impl<'a> /*trait*/ QWidget_setToolTip for (&'a  QString) {
  fn setToolTip(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget10setToolTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn sizePolicy<T: QWidget_sizePolicy>(&mut self, value: T) -> i32 {
    value.sizePolicy(self);
    return 1;
  }
}

pub trait QWidget_sizePolicy {
  fn sizePolicy(self, this: &mut QWidget) -> i32;
}

// proto: QSizePolicy QWidget::sizePolicy();
impl<'a> /*trait*/ QWidget_sizePolicy for () {
  fn sizePolicy(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10sizePolicyEv()};
    unsafe {_ZNK7QWidget10sizePolicyEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn hasHeightForWidth<T: QWidget_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QWidget_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::hasHeightForWidth();
impl<'a> /*trait*/ QWidget_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget17hasHeightForWidthEv()};
    unsafe {_ZNK7QWidget17hasHeightForWidthEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn graphicsProxyWidget<T: QWidget_graphicsProxyWidget>(&mut self, value: T) -> i32 {
    value.graphicsProxyWidget(self);
    return 1;
  }
}

pub trait QWidget_graphicsProxyWidget {
  fn graphicsProxyWidget(self, this: &mut QWidget) -> i32;
}

// proto: QGraphicsProxyWidget * QWidget::graphicsProxyWidget();
impl<'a> /*trait*/ QWidget_graphicsProxyWidget for () {
  fn graphicsProxyWidget(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget19graphicsProxyWidgetEv()};
    unsafe {_ZNK7QWidget19graphicsProxyWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn contentsMargins<T: QWidget_contentsMargins>(&mut self, value: T) -> i32 {
    value.contentsMargins(self);
    return 1;
  }
}

pub trait QWidget_contentsMargins {
  fn contentsMargins(self, this: &mut QWidget) -> i32;
}

// proto: QMargins QWidget::contentsMargins();
impl<'a> /*trait*/ QWidget_contentsMargins for () {
  fn contentsMargins(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15contentsMarginsEv()};
    unsafe {_ZNK7QWidget15contentsMarginsEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn topLevelWidget<T: QWidget_topLevelWidget>(&mut self, value: T) -> i32 {
    value.topLevelWidget(self);
    return 1;
  }
}

pub trait QWidget_topLevelWidget {
  fn topLevelWidget(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::topLevelWidget();
impl<'a> /*trait*/ QWidget_topLevelWidget for () {
  fn topLevelWidget(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14topLevelWidgetEv()};
    unsafe {_ZNK7QWidget14topLevelWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setLayout<T: QWidget_setLayout>(&mut self, value: T) -> i32 {
    value.setLayout(self);
    return 1;
  }
}

pub trait QWidget_setLayout {
  fn setLayout(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setLayout(QLayout * );
impl<'a> /*trait*/ QWidget_setLayout for (&'a mut QLayout) {
  fn setLayout(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setLayoutEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidget9setLayoutEP7QLayout(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn underMouse<T: QWidget_underMouse>(&mut self, value: T) -> i32 {
    value.underMouse(self);
    return 1;
  }
}

pub trait QWidget_underMouse {
  fn underMouse(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::underMouse();
impl<'a> /*trait*/ QWidget_underMouse for () {
  fn underMouse(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10underMouseEv()};
    unsafe {_ZNK7QWidget10underMouseEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn heightForWidth<T: QWidget_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QWidget_heightForWidth {
  fn heightForWidth(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::heightForWidth(int );
impl<'a> /*trait*/ QWidget_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QWidget14heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFont<T: QWidget_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QWidget_setFont {
  fn setFont(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setFont(const QFont & );
impl<'a> /*trait*/ QWidget_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn nativeParentWidget<T: QWidget_nativeParentWidget>(&mut self, value: T) -> i32 {
    value.nativeParentWidget(self);
    return 1;
  }
}

pub trait QWidget_nativeParentWidget {
  fn nativeParentWidget(self, this: &mut QWidget) -> i32;
}

// proto: QWidget * QWidget::nativeParentWidget();
impl<'a> /*trait*/ QWidget_nativeParentWidget for () {
  fn nativeParentWidget(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18nativeParentWidgetEv()};
    unsafe {_ZNK7QWidget18nativeParentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setLocale<T: QWidget_setLocale>(&mut self, value: T) -> i32 {
    value.setLocale(self);
    return 1;
  }
}

pub trait QWidget_setLocale {
  fn setLocale(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QWidget_setLocale for (&'a  QLocale) {
  fn setLocale(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget9setLocaleERK7QLocale(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn height<T: QWidget_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QWidget_height {
  fn height(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::height();
impl<'a> /*trait*/ QWidget_height for () {
  fn height(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6heightEv()};
    unsafe {_ZNK7QWidget6heightEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setHidden<T: QWidget_setHidden>(&mut self, value: T) -> i32 {
    value.setHidden(self);
    return 1;
  }
}

pub trait QWidget_setHidden {
  fn setHidden(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setHidden(bool hidden);
impl<'a> /*trait*/ QWidget_setHidden for (i8) {
  fn setHidden(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setHiddenEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWidget9setHiddenEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn size<T: QWidget_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QWidget_size {
  fn size(self, this: &mut QWidget) -> i32;
}

// proto: QSize QWidget::size();
impl<'a> /*trait*/ QWidget_size for () {
  fn size(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4sizeEv()};
    unsafe {_ZNK7QWidget4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn maximumWidth<T: QWidget_maximumWidth>(&mut self, value: T) -> i32 {
    value.maximumWidth(self);
    return 1;
  }
}

pub trait QWidget_maximumWidth {
  fn maximumWidth(self, this: &mut QWidget) -> i32;
}

// proto: int QWidget::maximumWidth();
impl<'a> /*trait*/ QWidget_maximumWidth for () {
  fn maximumWidth(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12maximumWidthEv()};
    unsafe {_ZNK7QWidget12maximumWidthEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isMinimized<T: QWidget_isMinimized>(&mut self, value: T) -> i32 {
    value.isMinimized(self);
    return 1;
  }
}

pub trait QWidget_isMinimized {
  fn isMinimized(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isMinimized();
impl<'a> /*trait*/ QWidget_isMinimized for () {
  fn isMinimized(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isMinimizedEv()};
    unsafe {_ZNK7QWidget11isMinimizedEv()};
    return 1;
  }
}

// proto: void QWidget::update();
impl<'a> /*trait*/ QWidget_update for () {
  fn update(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateEv()};
    unsafe {_ZN7QWidget6updateEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setCursor<T: QWidget_setCursor>(&mut self, value: T) -> i32 {
    value.setCursor(self);
    return 1;
  }
}

pub trait QWidget_setCursor {
  fn setCursor(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setCursor(const QCursor & );
impl<'a> /*trait*/ QWidget_setCursor for (&'a  QCursor) {
  fn setCursor(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget9setCursorERK7QCursor(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowIconChanged<T: QWidget_windowIconChanged>(&mut self, value: T) -> i32 {
    value.windowIconChanged(self);
    return 1;
  }
}

pub trait QWidget_windowIconChanged {
  fn windowIconChanged(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::windowIconChanged(const QIcon & icon);
impl<'a> /*trait*/ QWidget_windowIconChanged for (&'a  QIcon) {
  fn windowIconChanged(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17windowIconChangedERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget17windowIconChangedERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn style<T: QWidget_style>(&mut self, value: T) -> i32 {
    value.style(self);
    return 1;
  }
}

pub trait QWidget_style {
  fn style(self, this: &mut QWidget) -> i32;
}

// proto: QStyle * QWidget::style();
impl<'a> /*trait*/ QWidget_style for () {
  fn style(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5styleEv()};
    unsafe {_ZNK7QWidget5styleEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn createWinId<T: QWidget_createWinId>(&mut self, value: T) -> i32 {
    value.createWinId(self);
    return 1;
  }
}

pub trait QWidget_createWinId {
  fn createWinId(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::createWinId();
impl<'a> /*trait*/ QWidget_createWinId for () {
  fn createWinId(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11createWinIdEv()};
    unsafe {_ZN7QWidget11createWinIdEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowOpacity<T: QWidget_setWindowOpacity>(&mut self, value: T) -> i32 {
    value.setWindowOpacity(self);
    return 1;
  }
}

pub trait QWidget_setWindowOpacity {
  fn setWindowOpacity(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setWindowOpacity(qreal level);
impl<'a> /*trait*/ QWidget_setWindowOpacity for (f64) {
  fn setWindowOpacity(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setWindowOpacityEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QWidget16setWindowOpacityEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isRightToLeft<T: QWidget_isRightToLeft>(&mut self, value: T) -> i32 {
    value.isRightToLeft(self);
    return 1;
  }
}

pub trait QWidget_isRightToLeft {
  fn isRightToLeft(self, this: &mut QWidget) -> i32;
}

// proto: bool QWidget::isRightToLeft();
impl<'a> /*trait*/ QWidget_isRightToLeft for () {
  fn isRightToLeft(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13isRightToLeftEv()};
    unsafe {_ZNK7QWidget13isRightToLeftEv()};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setAccessibleName<T: QWidget_setAccessibleName>(&mut self, value: T) -> i32 {
    value.setAccessibleName(self);
    return 1;
  }
}

pub trait QWidget_setAccessibleName {
  fn setAccessibleName(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::setAccessibleName(const QString & name);
impl<'a> /*trait*/ QWidget_setAccessibleName for (&'a  QString) {
  fn setAccessibleName(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setAccessibleNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget17setAccessibleNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowIconTextChanged<T: QWidget_windowIconTextChanged>(&mut self, value: T) -> i32 {
    value.windowIconTextChanged(self);
    return 1;
  }
}

pub trait QWidget_windowIconTextChanged {
  fn windowIconTextChanged(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::windowIconTextChanged(const QString & iconText);
impl<'a> /*trait*/ QWidget_windowIconTextChanged for (&'a  QString) {
  fn windowIconTextChanged(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget21windowIconTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWidget21windowIconTextChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn unsetCursor<T: QWidget_unsetCursor>(&mut self, value: T) -> i32 {
    value.unsetCursor(self);
    return 1;
  }
}

pub trait QWidget_unsetCursor {
  fn unsetCursor(self, this: &mut QWidget) -> i32;
}

// proto: void QWidget::unsetCursor();
impl<'a> /*trait*/ QWidget_unsetCursor for () {
  fn unsetCursor(self, this: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11unsetCursorEv()};
    unsafe {_ZN7QWidget11unsetCursorEv()};
    return 1;
  }
}

