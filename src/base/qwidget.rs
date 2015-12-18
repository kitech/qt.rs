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
use super::qpixmap::QPixmap;
use super::qwindow::QWindow;
use super::qmargins::QMargins;
use super::qbytearray::QByteArray;
use super::qpaintengine::QPaintEngine;
use super::qfontinfo::QFontInfo;
use super::qfontmetrics::QFontMetrics;
use super::qsizepolicy::QSizePolicy;
use super::qicon::QIcon;
use super::qlayout::QLayout;
use super::qbackingstore::QBackingStore;
use super::qfont::QFont;
use super::qlocale::QLocale;
use super::qbitmap::QBitmap;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QWidget::setGraphicsEffect(QGraphicsEffect * effect);
  fn _ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QWidget::accessibleDescription();
  fn _ZNK7QWidget21accessibleDescriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsEffect * QWidget::graphicsEffect();
  fn _ZNK7QWidget14graphicsEffectEv(qthis: *mut c_void) ;
  // proto:  bool QWidget::isFullScreen();
  fn _ZNK7QWidget12isFullScreenEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QWidget::maximumSize();
  fn _ZNK7QWidget11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidget::isEnabledToTLW();
  fn _ZNK7QWidget14isEnabledToTLWEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::setStatusTip(const QString & );
  fn _ZN7QWidget12setStatusTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::setSizeIncrement(const QSize & );
  fn _ZN7QWidget16setSizeIncrementERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::customContextMenuRequested(const QPoint & pos);
  fn _ZN7QWidget26customContextMenuRequestedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QWidget::focusWidget();
  fn _ZNK7QWidget11focusWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidget::isTopLevel();
  fn _ZNK7QWidget10isTopLevelEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QWidget::acceptDrops();
  fn _ZNK7QWidget11acceptDropsEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QWidget::isWindow();
  fn _ZNK7QWidget8isWindowEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::setFixedSize(const QSize & );
  fn _ZN7QWidget12setFixedSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QWidget::isVisible();
  fn _ZNK7QWidget9isVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QWidget::minimumHeight();
  fn _ZNK7QWidget13minimumHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QWidget::sizeIncrement();
  fn _ZNK7QWidget13sizeIncrementEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::repaint(const QRect & );
  fn _ZN7QWidget7repaintERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::update(int x, int y, int w, int h);
  fn _ZN7QWidget6updateEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  QString QWidget::windowFilePath();
  fn _ZNK7QWidget14windowFilePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::clearMask();
  fn _ZN7QWidget9clearMaskEv(qthis: *mut c_void) ;
  // proto:  QRect QWidget::rect();
  fn _ZNK7QWidget4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::unsetLayoutDirection();
  fn _ZN7QWidget20unsetLayoutDirectionEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setMinimumSize(const QSize & );
  fn _ZN7QWidget14setMinimumSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QWidget::isActiveWindow();
  fn _ZNK7QWidget14isActiveWindowEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::grabKeyboard();
  fn _ZN7QWidget12grabKeyboardEv(qthis: *mut c_void) ;
  // proto:  QSize QWidget::frameSize();
  fn _ZNK7QWidget9frameSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setFocus();
  fn _ZN7QWidget8setFocusEv(qthis: *mut c_void) ;
  // proto:  void QWidget::updateGeometry();
  fn _ZN7QWidget14updateGeometryEv(qthis: *mut c_void) ;
  // proto:  void QWidget::repaint(const QRegion & );
  fn _ZN7QWidget7repaintERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::insertAction(QAction * before, QAction * action);
  fn _ZN7QWidget12insertActionEP7QActionS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QWidget::setWindowRole(const QString & );
  fn _ZN7QWidget13setWindowRoleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QWidget::toolTipDuration();
  fn _ZNK7QWidget15toolTipDurationEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWidget::setPalette(const QPalette & );
  fn _ZN7QWidget10setPaletteERK8QPalette(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::setStyleSheet(const QString & styleSheet);
  fn _ZN7QWidget13setStyleSheetERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QWidget::windowIconText();
  fn _ZNK7QWidget14windowIconTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::releaseMouse();
  fn _ZN7QWidget12releaseMouseEv(qthis: *mut c_void) ;
  // proto:  bool QWidget::isModal();
  fn _ZNK7QWidget7isModalEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::setStyle(QStyle * );
  fn _ZN7QWidget8setStyleEP6QStyle(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::repaint();
  fn _ZN7QWidget7repaintEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setBaseSize(int basew, int baseh);
  fn _ZN7QWidget11setBaseSizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QWidget::isWindowModified();
  fn _ZNK7QWidget16isWindowModifiedEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QRect & QWidget::geometry();
  fn _ZNK7QWidget8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setAccessibleDescription(const QString & description);
  fn _ZN7QWidget24setAccessibleDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::windowTitleChanged(const QString & title);
  fn _ZN7QWidget18windowTitleChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QWidget::nextInFocusChain();
  fn _ZNK7QWidget16nextInFocusChainEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QWidget::setTabOrder(QWidget * , QWidget * );
  fn _ZN7QWidget11setTabOrderEPS_S0_(arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QRect QWidget::frameGeometry();
  fn _ZNK7QWidget13frameGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QWidget::sizeHint();
  fn _ZNK7QWidget8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setMinimumWidth(int minw);
  fn _ZN7QWidget15setMinimumWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QWidget::isVisibleTo(const QWidget * );
  fn _ZNK7QWidget11isVisibleToEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QWidget::setMaximumSize(int maxw, int maxh);
  fn _ZN7QWidget14setMaximumSizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QWidget::hasMouseTracking();
  fn _ZNK7QWidget16hasMouseTrackingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::update(const QRect & );
  fn _ZN7QWidget6updateERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QWidget::isHidden();
  fn _ZNK7QWidget8isHiddenEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QWidget::devType();
  fn _ZNK7QWidget7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  QWidget * QWidget::childAt(int x, int y);
  fn _ZNK7QWidget7childAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QWidget::activateWindow();
  fn _ZN7QWidget14activateWindowEv(qthis: *mut c_void) ;
  // proto:  QRect QWidget::normalGeometry();
  fn _ZNK7QWidget14normalGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QWidget::windowTitle();
  fn _ZNK7QWidget11windowTitleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::grabMouse(const QCursor & );
  fn _ZN7QWidget9grabMouseERK7QCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPixmap QWidget::grab(const QRect & rectangle);
  fn _ZN7QWidget4grabERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setVisible(bool visible);
  fn _ZN7QWidget10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QWidget::isEnabledTo(const QWidget * );
  fn _ZNK7QWidget11isEnabledToEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QWidget::isLeftToRight();
  fn _ZNK7QWidget13isLeftToRightEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::setGeometry(const QRect & );
  fn _ZN7QWidget11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::unsetLocale();
  fn _ZN7QWidget11unsetLocaleEv(qthis: *mut c_void) ;
  // proto:  void QWidget::showNormal();
  fn _ZN7QWidget10showNormalEv(qthis: *mut c_void) ;
  // proto:  int QWidget::y();
  fn _ZNK7QWidget1yEv(qthis: *mut c_void) ;
  // proto:  int QWidget::width();
  fn _ZNK7QWidget5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QWidget::isMaximized();
  fn _ZNK7QWidget11isMaximizedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::resize(const QSize & );
  fn _ZN7QWidget6resizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWindow * QWidget::windowHandle();
  fn _ZNK7QWidget12windowHandleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QWidget::accessibleName();
  fn _ZNK7QWidget14accessibleNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setContentsMargins(const QMargins & margins);
  fn _ZN7QWidget18setContentsMarginsERK8QMargins(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray QWidget::saveGeometry();
  fn _ZNK7QWidget12saveGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidget::isEnabled();
  fn _ZNK7QWidget9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::setFixedHeight(int h);
  fn _ZN7QWidget14setFixedHeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRegion QWidget::mask();
  fn _ZNK7QWidget4maskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::stackUnder(QWidget * );
  fn _ZN7QWidget10stackUnderEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPaintEngine * QWidget::paintEngine();
  fn _ZNK7QWidget11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setAcceptDrops(bool on);
  fn _ZN7QWidget14setAcceptDropsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QWidget::move_(const QPoint & );
  fn _ZN7QWidget4moveERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QAction *> QWidget::actions();
  fn _ZNK7QWidget7actionsEv(qthis: *mut c_void) ;
  // proto:  void QWidget::show();
  fn _ZN7QWidget4showEv(qthis: *mut c_void) ;
  // proto: static QWidget * QWidget::keyboardGrabber();
  fn _ZN7QWidget15keyboardGrabberEv() -> *mut c_void;
  // proto:  QPoint QWidget::mapTo(const QWidget * , const QPoint & );
  fn _ZNK7QWidget5mapToEPKS_RK6QPoint(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QWidget::minimumWidth();
  fn _ZNK7QWidget12minimumWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  QFontInfo QWidget::fontInfo();
  fn _ZNK7QWidget8fontInfoEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidget::autoFillBackground();
  fn _ZNK7QWidget18autoFillBackgroundEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::scroll(int dx, int dy, const QRect & );
  fn _ZN7QWidget6scrollEiiRK5QRect(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QFontMetrics QWidget::fontMetrics();
  fn _ZNK7QWidget11fontMetricsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::adjustSize();
  fn _ZN7QWidget10adjustSizeEv(qthis: *mut c_void) ;
  // proto:  QWidget * QWidget::previousInFocusChain();
  fn _ZNK7QWidget20previousInFocusChainEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidget::updatesEnabled();
  fn _ZNK7QWidget14updatesEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::setMaximumHeight(int maxh);
  fn _ZN7QWidget16setMaximumHeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWidget::showMaximized();
  fn _ZN7QWidget13showMaximizedEv(qthis: *mut c_void) ;
  // proto:  QPoint QWidget::mapFrom(const QWidget * , const QPoint & );
  fn _ZNK7QWidget7mapFromEPKS_RK6QPoint(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QWidget::x();
  fn _ZNK7QWidget1xEv(qthis: *mut c_void) ;
  // proto:  void QWidget::clearFocus();
  fn _ZN7QWidget10clearFocusEv(qthis: *mut c_void) ;
  // proto: static QWidget * QWidget::find(WId );
  fn _ZN7QWidget4findEi(arg0: *mut c_uint) -> *mut c_void;
  // proto:  const QPalette & QWidget::palette();
  fn _ZNK7QWidget7paletteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setSizePolicy(QSizePolicy );
  fn _ZN7QWidget13setSizePolicyE11QSizePolicy(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::setMask(const QRegion & );
  fn _ZN7QWidget7setMaskERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::setMaximumWidth(int maxw);
  fn _ZN7QWidget15setMaximumWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWidget::setWindowIconText(const QString & );
  fn _ZN7QWidget17setWindowIconTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::setWindowIcon(const QIcon & icon);
  fn _ZN7QWidget13setWindowIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::FreeQWidget();
  fn _ZN7QWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  void QWidget::getContentsMargins(int * left, int * top, int * right, int * bottom);
  fn _ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  QSize QWidget::minimumSizeHint();
  fn _ZNK7QWidget15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setWindowModified(bool );
  fn _ZN7QWidget17setWindowModifiedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QWidget::restoreGeometry(const QByteArray & geometry);
  fn _ZN7QWidget15restoreGeometryERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QLayout * QWidget::layout();
  fn _ZNK7QWidget6layoutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QWidget::contentsRect();
  fn _ZNK7QWidget12contentsRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBackingStore * QWidget::backingStore();
  fn _ZNK7QWidget12backingStoreEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QWidget::focusProxy();
  fn _ZNK7QWidget10focusProxyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QWidget::styleSheet();
  fn _ZNK7QWidget10styleSheetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QWidget::childAt(const QPoint & p);
  fn _ZNK7QWidget7childAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::repaint(int x, int y, int w, int h);
  fn _ZN7QWidget7repaintEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  QString QWidget::whatsThis();
  fn _ZNK7QWidget9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QFont & QWidget::font();
  fn _ZNK7QWidget4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setMinimumSize(int minw, int minh);
  fn _ZN7QWidget14setMinimumSizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  const QMetaObject * QWidget::metaObject();
  fn _ZNK7QWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setMinimumHeight(int minh);
  fn _ZN7QWidget16setMinimumHeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWidget::update(const QRegion & );
  fn _ZN7QWidget6updateERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QWidget::windowOpacity();
  fn _ZNK7QWidget13windowOpacityEv(qthis: *mut c_void) -> c_double;
  // proto:  QRegion QWidget::childrenRegion();
  fn _ZNK7QWidget14childrenRegionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setWindowFilePath(const QString & filePath);
  fn _ZN7QWidget17setWindowFilePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::setShortcutEnabled(int id, bool enable);
  fn _ZN7QWidget18setShortcutEnabledEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QWidget::raise();
  fn _ZN7QWidget5raiseEv(qthis: *mut c_void) ;
  // proto:  QString QWidget::statusTip();
  fn _ZNK7QWidget9statusTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QWidget::childrenRect();
  fn _ZNK7QWidget12childrenRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setParent(QWidget * parent);
  fn _ZN7QWidget9setParentEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRegion QWidget::visibleRegion();
  fn _ZNK7QWidget13visibleRegionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLocale QWidget::locale();
  fn _ZNK7QWidget6localeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::releaseKeyboard();
  fn _ZN7QWidget15releaseKeyboardEv(qthis: *mut c_void) ;
  // proto: static QWidget * QWidget::mouseGrabber();
  fn _ZN7QWidget12mouseGrabberEv() -> *mut c_void;
  // proto:  void QWidget::setFixedWidth(int w);
  fn _ZN7QWidget13setFixedWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWidget::addAction(QAction * action);
  fn _ZN7QWidget9addActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::setDisabled(bool );
  fn _ZN7QWidget11setDisabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QIcon QWidget::windowIcon();
  fn _ZNK7QWidget10windowIconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setContentsMargins(int left, int top, int right, int bottom);
  fn _ZN7QWidget18setContentsMarginsEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  QString QWidget::windowRole();
  fn _ZNK7QWidget10windowRoleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setShortcutAutoRepeat(int id, bool enable);
  fn _ZN7QWidget21setShortcutAutoRepeatEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  void QWidget::showFullScreen();
  fn _ZN7QWidget14showFullScreenEv(qthis: *mut c_void) ;
  // proto:  void QWidget::grabMouse();
  fn _ZN7QWidget9grabMouseEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setMaximumSize(const QSize & );
  fn _ZN7QWidget14setMaximumSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPoint QWidget::mapToGlobal(const QPoint & );
  fn _ZNK7QWidget11mapToGlobalERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QWidget::toolTip();
  fn _ZNK7QWidget7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setWhatsThis(const QString & );
  fn _ZN7QWidget12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::resize(int w, int h);
  fn _ZN7QWidget6resizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QWidget * QWidget::parentWidget();
  fn _ZNK7QWidget12parentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QWidget::pos();
  fn _ZNK7QWidget3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setAutoFillBackground(bool enabled);
  fn _ZN7QWidget21setAutoFillBackgroundEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QWidget::hasFocus();
  fn _ZNK7QWidget8hasFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QWidget::baseSize();
  fn _ZNK7QWidget8baseSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setMask(const QBitmap & );
  fn _ZN7QWidget7setMaskERK7QBitmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::ensurePolished();
  fn _ZNK7QWidget14ensurePolishedEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setWindowTitle(const QString & );
  fn _ZN7QWidget14setWindowTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QWidget::window();
  fn _ZNK7QWidget6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::scroll(int dx, int dy);
  fn _ZN7QWidget6scrollEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QWidget::releaseShortcut(int id);
  fn _ZN7QWidget15releaseShortcutEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWidget::setToolTipDuration(int msec);
  fn _ZN7QWidget18setToolTipDurationEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWidget::setGeometry(int x, int y, int w, int h);
  fn _ZN7QWidget11setGeometryEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QWidget::setSizeIncrement(int w, int h);
  fn _ZN7QWidget16setSizeIncrementEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QWidget::setUpdatesEnabled(bool enable);
  fn _ZN7QWidget17setUpdatesEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QWidget::lower();
  fn _ZN7QWidget5lowerEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setMouseTracking(bool enable);
  fn _ZN7QWidget16setMouseTrackingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QWidget::setBaseSize(const QSize & );
  fn _ZN7QWidget11setBaseSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::hide();
  fn _ZN7QWidget4hideEv(qthis: *mut c_void) ;
  // proto:  void QWidget::removeAction(QAction * action);
  fn _ZN7QWidget12removeActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::setFocusProxy(QWidget * );
  fn _ZN7QWidget13setFocusProxyEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QWidget::close();
  fn _ZN7QWidget5closeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::showMinimized();
  fn _ZN7QWidget13showMinimizedEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setFixedSize(int w, int h);
  fn _ZN7QWidget12setFixedSizeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QSize QWidget::minimumSize();
  fn _ZNK7QWidget11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setEnabled(bool );
  fn _ZN7QWidget10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QWidget::maximumHeight();
  fn _ZNK7QWidget13maximumHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWidget::move_(int x, int y);
  fn _ZN7QWidget4moveEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  bool QWidget::isAncestorOf(const QWidget * child);
  fn _ZNK7QWidget12isAncestorOfEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QWidget::NewQWidget(const QWidget & );
  fn _ZN7QWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QCursor QWidget::cursor();
  fn _ZNK7QWidget6cursorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QWidget::mapFromGlobal(const QPoint & );
  fn _ZNK7QWidget13mapFromGlobalERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setToolTip(const QString & );
  fn _ZN7QWidget10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSizePolicy QWidget::sizePolicy();
  fn _ZNK7QWidget10sizePolicyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWidget::hasHeightForWidth();
  fn _ZNK7QWidget17hasHeightForWidthEv(qthis: *mut c_void) -> int8_t;
  // proto:  QGraphicsProxyWidget * QWidget::graphicsProxyWidget();
  fn _ZNK7QWidget19graphicsProxyWidgetEv(qthis: *mut c_void) ;
  // proto:  QMargins QWidget::contentsMargins();
  fn _ZNK7QWidget15contentsMarginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QWidget::topLevelWidget();
  fn _ZNK7QWidget14topLevelWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setLayout(QLayout * );
  fn _ZN7QWidget9setLayoutEP7QLayout(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QWidget::underMouse();
  fn _ZNK7QWidget10underMouseEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QWidget::heightForWidth(int );
  fn _ZNK7QWidget14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QWidget::setFont(const QFont & );
  fn _ZN7QWidget7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QWidget::nativeParentWidget();
  fn _ZNK7QWidget18nativeParentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setLocale(const QLocale & locale);
  fn _ZN7QWidget9setLocaleERK7QLocale(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QWidget::height();
  fn _ZNK7QWidget6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWidget::setHidden(bool hidden);
  fn _ZN7QWidget9setHiddenEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QWidget::size();
  fn _ZNK7QWidget4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWidget::maximumWidth();
  fn _ZNK7QWidget12maximumWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QWidget::isMinimized();
  fn _ZNK7QWidget11isMinimizedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::update();
  fn _ZN7QWidget6updateEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setCursor(const QCursor & );
  fn _ZN7QWidget9setCursorERK7QCursor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::windowIconChanged(const QIcon & icon);
  fn _ZN7QWidget17windowIconChangedERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStyle * QWidget::style();
  fn _ZNK7QWidget5styleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::createWinId();
  fn _ZN7QWidget11createWinIdEv(qthis: *mut c_void) ;
  // proto:  void QWidget::setWindowOpacity(qreal level);
  fn _ZN7QWidget16setWindowOpacityEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QWidget::isRightToLeft();
  fn _ZNK7QWidget13isRightToLeftEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWidget::setAccessibleName(const QString & name);
  fn _ZN7QWidget17setAccessibleNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::windowIconTextChanged(const QString & iconText);
  fn _ZN7QWidget21windowIconTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWidget::unsetCursor();
  fn _ZN7QWidget11unsetCursorEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QWidget)=1
pub struct QWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWidget {
  pub fn setGraphicsEffect<RetType, T: QWidget_setGraphicsEffect<RetType>>(&mut self, value: T) -> RetType {
    return value.setGraphicsEffect(self);
    // return 1;
  }
}

pub trait QWidget_setGraphicsEffect<RetType> {
  fn setGraphicsEffect(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setGraphicsEffect(QGraphicsEffect * effect);
impl<'a> /*trait*/ QWidget_setGraphicsEffect<()> for (&'a mut QGraphicsEffect) {
  fn setGraphicsEffect(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn accessibleDescription<RetType, T: QWidget_accessibleDescription<RetType>>(&mut self, value: T) -> RetType {
    return value.accessibleDescription(self);
    // return 1;
  }
}

pub trait QWidget_accessibleDescription<RetType> {
  fn accessibleDescription(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::accessibleDescription();
impl<'a> /*trait*/ QWidget_accessibleDescription<QString> for () {
  fn accessibleDescription(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget21accessibleDescriptionEv()};
    let mut ret = unsafe {_ZNK7QWidget21accessibleDescriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn graphicsEffect<RetType, T: QWidget_graphicsEffect<RetType>>(&mut self, value: T) -> RetType {
    return value.graphicsEffect(self);
    // return 1;
  }
}

pub trait QWidget_graphicsEffect<RetType> {
  fn graphicsEffect(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QGraphicsEffect * QWidget::graphicsEffect();
impl<'a> /*trait*/ QWidget_graphicsEffect<()> for () {
  fn graphicsEffect(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14graphicsEffectEv()};
     unsafe {_ZNK7QWidget14graphicsEffectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isFullScreen<RetType, T: QWidget_isFullScreen<RetType>>(&mut self, value: T) -> RetType {
    return value.isFullScreen(self);
    // return 1;
  }
}

pub trait QWidget_isFullScreen<RetType> {
  fn isFullScreen(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isFullScreen();
impl<'a> /*trait*/ QWidget_isFullScreen<i8> for () {
  fn isFullScreen(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12isFullScreenEv()};
    let mut ret = unsafe {_ZNK7QWidget12isFullScreenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn maximumSize<RetType, T: QWidget_maximumSize<RetType>>(&mut self, value: T) -> RetType {
    return value.maximumSize(self);
    // return 1;
  }
}

pub trait QWidget_maximumSize<RetType> {
  fn maximumSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSize QWidget::maximumSize();
impl<'a> /*trait*/ QWidget_maximumSize<QSize> for () {
  fn maximumSize(self, rsthis: &mut QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11maximumSizeEv()};
    let mut ret = unsafe {_ZNK7QWidget11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isEnabledToTLW<RetType, T: QWidget_isEnabledToTLW<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabledToTLW(self);
    // return 1;
  }
}

pub trait QWidget_isEnabledToTLW<RetType> {
  fn isEnabledToTLW(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isEnabledToTLW();
impl<'a> /*trait*/ QWidget_isEnabledToTLW<i8> for () {
  fn isEnabledToTLW(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14isEnabledToTLWEv()};
    let mut ret = unsafe {_ZNK7QWidget14isEnabledToTLWEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setStatusTip<RetType, T: QWidget_setStatusTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setStatusTip(self);
    // return 1;
  }
}

pub trait QWidget_setStatusTip<RetType> {
  fn setStatusTip(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setStatusTip(const QString & );
impl<'a> /*trait*/ QWidget_setStatusTip<()> for (&'a  QString) {
  fn setStatusTip(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setSizeIncrement<RetType, T: QWidget_setSizeIncrement<RetType>>(&mut self, value: T) -> RetType {
    return value.setSizeIncrement(self);
    // return 1;
  }
}

pub trait QWidget_setSizeIncrement<RetType> {
  fn setSizeIncrement(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setSizeIncrement(const QSize & );
impl<'a> /*trait*/ QWidget_setSizeIncrement<()> for (&'a  QSize) {
  fn setSizeIncrement(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setSizeIncrementERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget16setSizeIncrementERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn customContextMenuRequested<RetType, T: QWidget_customContextMenuRequested<RetType>>(&mut self, value: T) -> RetType {
    return value.customContextMenuRequested(self);
    // return 1;
  }
}

pub trait QWidget_customContextMenuRequested<RetType> {
  fn customContextMenuRequested(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::customContextMenuRequested(const QPoint & pos);
impl<'a> /*trait*/ QWidget_customContextMenuRequested<()> for (&'a  QPoint) {
  fn customContextMenuRequested(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget26customContextMenuRequestedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget26customContextMenuRequestedERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn focusWidget<RetType, T: QWidget_focusWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.focusWidget(self);
    // return 1;
  }
}

pub trait QWidget_focusWidget<RetType> {
  fn focusWidget(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::focusWidget();
impl<'a> /*trait*/ QWidget_focusWidget<QWidget> for () {
  fn focusWidget(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11focusWidgetEv()};
    let mut ret = unsafe {_ZNK7QWidget11focusWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isTopLevel<RetType, T: QWidget_isTopLevel<RetType>>(&mut self, value: T) -> RetType {
    return value.isTopLevel(self);
    // return 1;
  }
}

pub trait QWidget_isTopLevel<RetType> {
  fn isTopLevel(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isTopLevel();
impl<'a> /*trait*/ QWidget_isTopLevel<i8> for () {
  fn isTopLevel(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10isTopLevelEv()};
    let mut ret = unsafe {_ZNK7QWidget10isTopLevelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn acceptDrops<RetType, T: QWidget_acceptDrops<RetType>>(&mut self, value: T) -> RetType {
    return value.acceptDrops(self);
    // return 1;
  }
}

pub trait QWidget_acceptDrops<RetType> {
  fn acceptDrops(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::acceptDrops();
impl<'a> /*trait*/ QWidget_acceptDrops<i8> for () {
  fn acceptDrops(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11acceptDropsEv()};
    let mut ret = unsafe {_ZNK7QWidget11acceptDropsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isWindow<RetType, T: QWidget_isWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.isWindow(self);
    // return 1;
  }
}

pub trait QWidget_isWindow<RetType> {
  fn isWindow(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isWindow();
impl<'a> /*trait*/ QWidget_isWindow<i8> for () {
  fn isWindow(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8isWindowEv()};
    let mut ret = unsafe {_ZNK7QWidget8isWindowEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFixedSize<RetType, T: QWidget_setFixedSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setFixedSize(self);
    // return 1;
  }
}

pub trait QWidget_setFixedSize<RetType> {
  fn setFixedSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setFixedSize(const QSize & );
impl<'a> /*trait*/ QWidget_setFixedSize<()> for (&'a  QSize) {
  fn setFixedSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setFixedSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget12setFixedSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isVisible<RetType, T: QWidget_isVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.isVisible(self);
    // return 1;
  }
}

pub trait QWidget_isVisible<RetType> {
  fn isVisible(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isVisible();
impl<'a> /*trait*/ QWidget_isVisible<i8> for () {
  fn isVisible(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9isVisibleEv()};
    let mut ret = unsafe {_ZNK7QWidget9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn minimumHeight<RetType, T: QWidget_minimumHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumHeight(self);
    // return 1;
  }
}

pub trait QWidget_minimumHeight<RetType> {
  fn minimumHeight(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::minimumHeight();
impl<'a> /*trait*/ QWidget_minimumHeight<i32> for () {
  fn minimumHeight(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13minimumHeightEv()};
    let mut ret = unsafe {_ZNK7QWidget13minimumHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn sizeIncrement<RetType, T: QWidget_sizeIncrement<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeIncrement(self);
    // return 1;
  }
}

pub trait QWidget_sizeIncrement<RetType> {
  fn sizeIncrement(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSize QWidget::sizeIncrement();
impl<'a> /*trait*/ QWidget_sizeIncrement<QSize> for () {
  fn sizeIncrement(self, rsthis: &mut QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13sizeIncrementEv()};
    let mut ret = unsafe {_ZNK7QWidget13sizeIncrementEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn repaint<RetType, T: QWidget_repaint<RetType>>(&mut self, value: T) -> RetType {
    return value.repaint(self);
    // return 1;
  }
}

pub trait QWidget_repaint<RetType> {
  fn repaint(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::repaint(const QRect & );
impl<'a> /*trait*/ QWidget_repaint<()> for (&'a  QRect) {
  fn repaint(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget7repaintERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn update<RetType, T: QWidget_update<RetType>>(&mut self, value: T) -> RetType {
    return value.update(self);
    // return 1;
  }
}

pub trait QWidget_update<RetType> {
  fn update(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::update(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_update<()> for (i32, i32, i32, i32) {
  fn update(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN7QWidget6updateEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowFilePath<RetType, T: QWidget_windowFilePath<RetType>>(&mut self, value: T) -> RetType {
    return value.windowFilePath(self);
    // return 1;
  }
}

pub trait QWidget_windowFilePath<RetType> {
  fn windowFilePath(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::windowFilePath();
impl<'a> /*trait*/ QWidget_windowFilePath<QString> for () {
  fn windowFilePath(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14windowFilePathEv()};
    let mut ret = unsafe {_ZNK7QWidget14windowFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn clearMask<RetType, T: QWidget_clearMask<RetType>>(&mut self, value: T) -> RetType {
    return value.clearMask(self);
    // return 1;
  }
}

pub trait QWidget_clearMask<RetType> {
  fn clearMask(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::clearMask();
impl<'a> /*trait*/ QWidget_clearMask<()> for () {
  fn clearMask(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9clearMaskEv()};
     unsafe {_ZN7QWidget9clearMaskEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn rect<RetType, T: QWidget_rect<RetType>>(&mut self, value: T) -> RetType {
    return value.rect(self);
    // return 1;
  }
}

pub trait QWidget_rect<RetType> {
  fn rect(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QRect QWidget::rect();
impl<'a> /*trait*/ QWidget_rect<QRect> for () {
  fn rect(self, rsthis: &mut QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4rectEv()};
    let mut ret = unsafe {_ZNK7QWidget4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn unsetLayoutDirection<RetType, T: QWidget_unsetLayoutDirection<RetType>>(&mut self, value: T) -> RetType {
    return value.unsetLayoutDirection(self);
    // return 1;
  }
}

pub trait QWidget_unsetLayoutDirection<RetType> {
  fn unsetLayoutDirection(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::unsetLayoutDirection();
impl<'a> /*trait*/ QWidget_unsetLayoutDirection<()> for () {
  fn unsetLayoutDirection(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget20unsetLayoutDirectionEv()};
     unsafe {_ZN7QWidget20unsetLayoutDirectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMinimumSize<RetType, T: QWidget_setMinimumSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setMinimumSize(self);
    // return 1;
  }
}

pub trait QWidget_setMinimumSize<RetType> {
  fn setMinimumSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setMinimumSize(const QSize & );
impl<'a> /*trait*/ QWidget_setMinimumSize<()> for (&'a  QSize) {
  fn setMinimumSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMinimumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget14setMinimumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isActiveWindow<RetType, T: QWidget_isActiveWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.isActiveWindow(self);
    // return 1;
  }
}

pub trait QWidget_isActiveWindow<RetType> {
  fn isActiveWindow(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isActiveWindow();
impl<'a> /*trait*/ QWidget_isActiveWindow<i8> for () {
  fn isActiveWindow(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14isActiveWindowEv()};
    let mut ret = unsafe {_ZNK7QWidget14isActiveWindowEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn grabKeyboard<RetType, T: QWidget_grabKeyboard<RetType>>(&mut self, value: T) -> RetType {
    return value.grabKeyboard(self);
    // return 1;
  }
}

pub trait QWidget_grabKeyboard<RetType> {
  fn grabKeyboard(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::grabKeyboard();
impl<'a> /*trait*/ QWidget_grabKeyboard<()> for () {
  fn grabKeyboard(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12grabKeyboardEv()};
     unsafe {_ZN7QWidget12grabKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn frameSize<RetType, T: QWidget_frameSize<RetType>>(&mut self, value: T) -> RetType {
    return value.frameSize(self);
    // return 1;
  }
}

pub trait QWidget_frameSize<RetType> {
  fn frameSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSize QWidget::frameSize();
impl<'a> /*trait*/ QWidget_frameSize<QSize> for () {
  fn frameSize(self, rsthis: &mut QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9frameSizeEv()};
    let mut ret = unsafe {_ZNK7QWidget9frameSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFocus<RetType, T: QWidget_setFocus<RetType>>(&mut self, value: T) -> RetType {
    return value.setFocus(self);
    // return 1;
  }
}

pub trait QWidget_setFocus<RetType> {
  fn setFocus(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setFocus();
impl<'a> /*trait*/ QWidget_setFocus<()> for () {
  fn setFocus(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget8setFocusEv()};
     unsafe {_ZN7QWidget8setFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn updateGeometry<RetType, T: QWidget_updateGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.updateGeometry(self);
    // return 1;
  }
}

pub trait QWidget_updateGeometry<RetType> {
  fn updateGeometry(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::updateGeometry();
impl<'a> /*trait*/ QWidget_updateGeometry<()> for () {
  fn updateGeometry(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14updateGeometryEv()};
     unsafe {_ZN7QWidget14updateGeometryEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QWidget::repaint(const QRegion & );
impl<'a> /*trait*/ QWidget_repaint<()> for (&'a  QRegion) {
  fn repaint(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget7repaintERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn insertAction<RetType, T: QWidget_insertAction<RetType>>(&mut self, value: T) -> RetType {
    return value.insertAction(self);
    // return 1;
  }
}

pub trait QWidget_insertAction<RetType> {
  fn insertAction(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::insertAction(QAction * before, QAction * action);
impl<'a> /*trait*/ QWidget_insertAction<()> for (&'a mut QAction, &'a mut QAction) {
  fn insertAction(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12insertActionEP7QActionS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget12insertActionEP7QActionS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowRole<RetType, T: QWidget_setWindowRole<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowRole(self);
    // return 1;
  }
}

pub trait QWidget_setWindowRole<RetType> {
  fn setWindowRole(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setWindowRole(const QString & );
impl<'a> /*trait*/ QWidget_setWindowRole<()> for (&'a  QString) {
  fn setWindowRole(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setWindowRoleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget13setWindowRoleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn toolTipDuration<RetType, T: QWidget_toolTipDuration<RetType>>(&mut self, value: T) -> RetType {
    return value.toolTipDuration(self);
    // return 1;
  }
}

pub trait QWidget_toolTipDuration<RetType> {
  fn toolTipDuration(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::toolTipDuration();
impl<'a> /*trait*/ QWidget_toolTipDuration<i32> for () {
  fn toolTipDuration(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15toolTipDurationEv()};
    let mut ret = unsafe {_ZNK7QWidget15toolTipDurationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setPalette<RetType, T: QWidget_setPalette<RetType>>(&mut self, value: T) -> RetType {
    return value.setPalette(self);
    // return 1;
  }
}

pub trait QWidget_setPalette<RetType> {
  fn setPalette(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setPalette(const QPalette & );
impl<'a> /*trait*/ QWidget_setPalette<()> for (&'a  QPalette) {
  fn setPalette(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget10setPaletteERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setStyleSheet<RetType, T: QWidget_setStyleSheet<RetType>>(&mut self, value: T) -> RetType {
    return value.setStyleSheet(self);
    // return 1;
  }
}

pub trait QWidget_setStyleSheet<RetType> {
  fn setStyleSheet(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setStyleSheet(const QString & styleSheet);
impl<'a> /*trait*/ QWidget_setStyleSheet<()> for (&'a  QString) {
  fn setStyleSheet(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget13setStyleSheetERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowIconText<RetType, T: QWidget_windowIconText<RetType>>(&mut self, value: T) -> RetType {
    return value.windowIconText(self);
    // return 1;
  }
}

pub trait QWidget_windowIconText<RetType> {
  fn windowIconText(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::windowIconText();
impl<'a> /*trait*/ QWidget_windowIconText<QString> for () {
  fn windowIconText(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14windowIconTextEv()};
    let mut ret = unsafe {_ZNK7QWidget14windowIconTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn releaseMouse<RetType, T: QWidget_releaseMouse<RetType>>(&mut self, value: T) -> RetType {
    return value.releaseMouse(self);
    // return 1;
  }
}

pub trait QWidget_releaseMouse<RetType> {
  fn releaseMouse(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::releaseMouse();
impl<'a> /*trait*/ QWidget_releaseMouse<()> for () {
  fn releaseMouse(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12releaseMouseEv()};
     unsafe {_ZN7QWidget12releaseMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isModal<RetType, T: QWidget_isModal<RetType>>(&mut self, value: T) -> RetType {
    return value.isModal(self);
    // return 1;
  }
}

pub trait QWidget_isModal<RetType> {
  fn isModal(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isModal();
impl<'a> /*trait*/ QWidget_isModal<i8> for () {
  fn isModal(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7isModalEv()};
    let mut ret = unsafe {_ZNK7QWidget7isModalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setStyle<RetType, T: QWidget_setStyle<RetType>>(&mut self, value: T) -> RetType {
    return value.setStyle(self);
    // return 1;
  }
}

pub trait QWidget_setStyle<RetType> {
  fn setStyle(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setStyle(QStyle * );
impl<'a> /*trait*/ QWidget_setStyle<()> for (&'a mut QStyle) {
  fn setStyle(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget8setStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QWidget::repaint();
impl<'a> /*trait*/ QWidget_repaint<()> for () {
  fn repaint(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintEv()};
     unsafe {_ZN7QWidget7repaintEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setBaseSize<RetType, T: QWidget_setBaseSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setBaseSize(self);
    // return 1;
  }
}

pub trait QWidget_setBaseSize<RetType> {
  fn setBaseSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setBaseSize(int basew, int baseh);
impl<'a> /*trait*/ QWidget_setBaseSize<()> for (i32, i32) {
  fn setBaseSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setBaseSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWidget11setBaseSizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isWindowModified<RetType, T: QWidget_isWindowModified<RetType>>(&mut self, value: T) -> RetType {
    return value.isWindowModified(self);
    // return 1;
  }
}

pub trait QWidget_isWindowModified<RetType> {
  fn isWindowModified(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isWindowModified();
impl<'a> /*trait*/ QWidget_isWindowModified<i8> for () {
  fn isWindowModified(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16isWindowModifiedEv()};
    let mut ret = unsafe {_ZNK7QWidget16isWindowModifiedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn geometry<RetType, T: QWidget_geometry<RetType>>(&mut self, value: T) -> RetType {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QWidget_geometry<RetType> {
  fn geometry(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  const QRect & QWidget::geometry();
impl<'a> /*trait*/ QWidget_geometry<QRect> for () {
  fn geometry(self, rsthis: &mut QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8geometryEv()};
    let mut ret = unsafe {_ZNK7QWidget8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setAccessibleDescription<RetType, T: QWidget_setAccessibleDescription<RetType>>(&mut self, value: T) -> RetType {
    return value.setAccessibleDescription(self);
    // return 1;
  }
}

pub trait QWidget_setAccessibleDescription<RetType> {
  fn setAccessibleDescription(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setAccessibleDescription(const QString & description);
impl<'a> /*trait*/ QWidget_setAccessibleDescription<()> for (&'a  QString) {
  fn setAccessibleDescription(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget24setAccessibleDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget24setAccessibleDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowTitleChanged<RetType, T: QWidget_windowTitleChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.windowTitleChanged(self);
    // return 1;
  }
}

pub trait QWidget_windowTitleChanged<RetType> {
  fn windowTitleChanged(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::windowTitleChanged(const QString & title);
impl<'a> /*trait*/ QWidget_windowTitleChanged<()> for (&'a  QString) {
  fn windowTitleChanged(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18windowTitleChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget18windowTitleChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn nextInFocusChain<RetType, T: QWidget_nextInFocusChain<RetType>>(&mut self, value: T) -> RetType {
    return value.nextInFocusChain(self);
    // return 1;
  }
}

pub trait QWidget_nextInFocusChain<RetType> {
  fn nextInFocusChain(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::nextInFocusChain();
impl<'a> /*trait*/ QWidget_nextInFocusChain<QWidget> for () {
  fn nextInFocusChain(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16nextInFocusChainEv()};
    let mut ret = unsafe {_ZNK7QWidget16nextInFocusChainEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setTabOrder<RetType, T: QWidget_setTabOrder<RetType>>(&mut self, value: T) -> RetType {
    return value.setTabOrder(self);
    // return 1;
  }
}

pub trait QWidget_setTabOrder<RetType> {
  fn setTabOrder(self, rsthis: &mut QWidget) -> RetType;
}

// proto: static void QWidget::setTabOrder(QWidget * , QWidget * );
impl<'a> /*trait*/ QWidget_setTabOrder<()> for (&'a mut QWidget, &'a mut QWidget) {
  fn setTabOrder(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setTabOrderEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget11setTabOrderEPS_S0_(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn frameGeometry<RetType, T: QWidget_frameGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.frameGeometry(self);
    // return 1;
  }
}

pub trait QWidget_frameGeometry<RetType> {
  fn frameGeometry(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QRect QWidget::frameGeometry();
impl<'a> /*trait*/ QWidget_frameGeometry<QRect> for () {
  fn frameGeometry(self, rsthis: &mut QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13frameGeometryEv()};
    let mut ret = unsafe {_ZNK7QWidget13frameGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn sizeHint<RetType, T: QWidget_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QWidget_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSize QWidget::sizeHint();
impl<'a> /*trait*/ QWidget_sizeHint<QSize> for () {
  fn sizeHint(self, rsthis: &mut QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8sizeHintEv()};
    let mut ret = unsafe {_ZNK7QWidget8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMinimumWidth<RetType, T: QWidget_setMinimumWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setMinimumWidth(self);
    // return 1;
  }
}

pub trait QWidget_setMinimumWidth<RetType> {
  fn setMinimumWidth(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setMinimumWidth(int minw);
impl<'a> /*trait*/ QWidget_setMinimumWidth<()> for (i32) {
  fn setMinimumWidth(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15setMinimumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWidget15setMinimumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isVisibleTo<RetType, T: QWidget_isVisibleTo<RetType>>(&mut self, value: T) -> RetType {
    return value.isVisibleTo(self);
    // return 1;
  }
}

pub trait QWidget_isVisibleTo<RetType> {
  fn isVisibleTo(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isVisibleTo(const QWidget * );
impl<'a> /*trait*/ QWidget_isVisibleTo<i8> for (&'a  QWidget) {
  fn isVisibleTo(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isVisibleToEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWidget11isVisibleToEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMaximumSize<RetType, T: QWidget_setMaximumSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setMaximumSize(self);
    // return 1;
  }
}

pub trait QWidget_setMaximumSize<RetType> {
  fn setMaximumSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setMaximumSize(int maxw, int maxh);
impl<'a> /*trait*/ QWidget_setMaximumSize<()> for (i32, i32) {
  fn setMaximumSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMaximumSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWidget14setMaximumSizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn hasMouseTracking<RetType, T: QWidget_hasMouseTracking<RetType>>(&mut self, value: T) -> RetType {
    return value.hasMouseTracking(self);
    // return 1;
  }
}

pub trait QWidget_hasMouseTracking<RetType> {
  fn hasMouseTracking(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::hasMouseTracking();
impl<'a> /*trait*/ QWidget_hasMouseTracking<i8> for () {
  fn hasMouseTracking(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16hasMouseTrackingEv()};
    let mut ret = unsafe {_ZNK7QWidget16hasMouseTrackingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QWidget::update(const QRect & );
impl<'a> /*trait*/ QWidget_update<()> for (&'a  QRect) {
  fn update(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget6updateERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isHidden<RetType, T: QWidget_isHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.isHidden(self);
    // return 1;
  }
}

pub trait QWidget_isHidden<RetType> {
  fn isHidden(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isHidden();
impl<'a> /*trait*/ QWidget_isHidden<i8> for () {
  fn isHidden(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8isHiddenEv()};
    let mut ret = unsafe {_ZNK7QWidget8isHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn devType<RetType, T: QWidget_devType<RetType>>(&mut self, value: T) -> RetType {
    return value.devType(self);
    // return 1;
  }
}

pub trait QWidget_devType<RetType> {
  fn devType(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::devType();
impl<'a> /*trait*/ QWidget_devType<i32> for () {
  fn devType(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7devTypeEv()};
    let mut ret = unsafe {_ZNK7QWidget7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn childAt<RetType, T: QWidget_childAt<RetType>>(&mut self, value: T) -> RetType {
    return value.childAt(self);
    // return 1;
  }
}

pub trait QWidget_childAt<RetType> {
  fn childAt(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::childAt(int x, int y);
impl<'a> /*trait*/ QWidget_childAt<QWidget> for (i32, i32) {
  fn childAt(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QWidget7childAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn activateWindow<RetType, T: QWidget_activateWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.activateWindow(self);
    // return 1;
  }
}

pub trait QWidget_activateWindow<RetType> {
  fn activateWindow(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::activateWindow();
impl<'a> /*trait*/ QWidget_activateWindow<()> for () {
  fn activateWindow(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14activateWindowEv()};
     unsafe {_ZN7QWidget14activateWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn normalGeometry<RetType, T: QWidget_normalGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.normalGeometry(self);
    // return 1;
  }
}

pub trait QWidget_normalGeometry<RetType> {
  fn normalGeometry(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QRect QWidget::normalGeometry();
impl<'a> /*trait*/ QWidget_normalGeometry<QRect> for () {
  fn normalGeometry(self, rsthis: &mut QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14normalGeometryEv()};
    let mut ret = unsafe {_ZNK7QWidget14normalGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowTitle<RetType, T: QWidget_windowTitle<RetType>>(&mut self, value: T) -> RetType {
    return value.windowTitle(self);
    // return 1;
  }
}

pub trait QWidget_windowTitle<RetType> {
  fn windowTitle(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::windowTitle();
impl<'a> /*trait*/ QWidget_windowTitle<QString> for () {
  fn windowTitle(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11windowTitleEv()};
    let mut ret = unsafe {_ZNK7QWidget11windowTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn grabMouse<RetType, T: QWidget_grabMouse<RetType>>(&mut self, value: T) -> RetType {
    return value.grabMouse(self);
    // return 1;
  }
}

pub trait QWidget_grabMouse<RetType> {
  fn grabMouse(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::grabMouse(const QCursor & );
impl<'a> /*trait*/ QWidget_grabMouse<()> for (&'a  QCursor) {
  fn grabMouse(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9grabMouseERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget9grabMouseERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn grab<RetType, T: QWidget_grab<RetType>>(&mut self, value: T) -> RetType {
    return value.grab(self);
    // return 1;
  }
}

pub trait QWidget_grab<RetType> {
  fn grab(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QPixmap QWidget::grab(const QRect & rectangle);
impl<'a> /*trait*/ QWidget_grab<QPixmap> for (&'a  QRect) {
  fn grab(self, rsthis: &mut QWidget) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4grabERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QWidget4grabERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setVisible<RetType, T: QWidget_setVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setVisible(self);
    // return 1;
  }
}

pub trait QWidget_setVisible<RetType> {
  fn setVisible(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setVisible(bool visible);
impl<'a> /*trait*/ QWidget_setVisible<()> for (i8) {
  fn setVisible(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isEnabledTo<RetType, T: QWidget_isEnabledTo<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabledTo(self);
    // return 1;
  }
}

pub trait QWidget_isEnabledTo<RetType> {
  fn isEnabledTo(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isEnabledTo(const QWidget * );
impl<'a> /*trait*/ QWidget_isEnabledTo<i8> for (&'a  QWidget) {
  fn isEnabledTo(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isEnabledToEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWidget11isEnabledToEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isLeftToRight<RetType, T: QWidget_isLeftToRight<RetType>>(&mut self, value: T) -> RetType {
    return value.isLeftToRight(self);
    // return 1;
  }
}

pub trait QWidget_isLeftToRight<RetType> {
  fn isLeftToRight(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isLeftToRight();
impl<'a> /*trait*/ QWidget_isLeftToRight<i8> for () {
  fn isLeftToRight(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13isLeftToRightEv()};
    let mut ret = unsafe {_ZNK7QWidget13isLeftToRightEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setGeometry<RetType, T: QWidget_setGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.setGeometry(self);
    // return 1;
  }
}

pub trait QWidget_setGeometry<RetType> {
  fn setGeometry(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setGeometry(const QRect & );
impl<'a> /*trait*/ QWidget_setGeometry<()> for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn unsetLocale<RetType, T: QWidget_unsetLocale<RetType>>(&mut self, value: T) -> RetType {
    return value.unsetLocale(self);
    // return 1;
  }
}

pub trait QWidget_unsetLocale<RetType> {
  fn unsetLocale(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::unsetLocale();
impl<'a> /*trait*/ QWidget_unsetLocale<()> for () {
  fn unsetLocale(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11unsetLocaleEv()};
     unsafe {_ZN7QWidget11unsetLocaleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn showNormal<RetType, T: QWidget_showNormal<RetType>>(&mut self, value: T) -> RetType {
    return value.showNormal(self);
    // return 1;
  }
}

pub trait QWidget_showNormal<RetType> {
  fn showNormal(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::showNormal();
impl<'a> /*trait*/ QWidget_showNormal<()> for () {
  fn showNormal(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10showNormalEv()};
     unsafe {_ZN7QWidget10showNormalEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn y<RetType, T: QWidget_y<RetType>>(&mut self, value: T) -> RetType {
    return value.y(self);
    // return 1;
  }
}

pub trait QWidget_y<RetType> {
  fn y(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::y();
impl<'a> /*trait*/ QWidget_y<()> for () {
  fn y(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget1yEv()};
     unsafe {_ZNK7QWidget1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn width<RetType, T: QWidget_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QWidget_width<RetType> {
  fn width(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::width();
impl<'a> /*trait*/ QWidget_width<i32> for () {
  fn width(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5widthEv()};
    let mut ret = unsafe {_ZNK7QWidget5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isMaximized<RetType, T: QWidget_isMaximized<RetType>>(&mut self, value: T) -> RetType {
    return value.isMaximized(self);
    // return 1;
  }
}

pub trait QWidget_isMaximized<RetType> {
  fn isMaximized(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isMaximized();
impl<'a> /*trait*/ QWidget_isMaximized<i8> for () {
  fn isMaximized(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isMaximizedEv()};
    let mut ret = unsafe {_ZNK7QWidget11isMaximizedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn resize<RetType, T: QWidget_resize<RetType>>(&mut self, value: T) -> RetType {
    return value.resize(self);
    // return 1;
  }
}

pub trait QWidget_resize<RetType> {
  fn resize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::resize(const QSize & );
impl<'a> /*trait*/ QWidget_resize<()> for (&'a  QSize) {
  fn resize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowHandle<RetType, T: QWidget_windowHandle<RetType>>(&mut self, value: T) -> RetType {
    return value.windowHandle(self);
    // return 1;
  }
}

pub trait QWidget_windowHandle<RetType> {
  fn windowHandle(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWindow * QWidget::windowHandle();
impl<'a> /*trait*/ QWidget_windowHandle<QWindow> for () {
  fn windowHandle(self, rsthis: &mut QWidget) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12windowHandleEv()};
    let mut ret = unsafe {_ZNK7QWidget12windowHandleEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn accessibleName<RetType, T: QWidget_accessibleName<RetType>>(&mut self, value: T) -> RetType {
    return value.accessibleName(self);
    // return 1;
  }
}

pub trait QWidget_accessibleName<RetType> {
  fn accessibleName(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::accessibleName();
impl<'a> /*trait*/ QWidget_accessibleName<QString> for () {
  fn accessibleName(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14accessibleNameEv()};
    let mut ret = unsafe {_ZNK7QWidget14accessibleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setContentsMargins<RetType, T: QWidget_setContentsMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.setContentsMargins(self);
    // return 1;
  }
}

pub trait QWidget_setContentsMargins<RetType> {
  fn setContentsMargins(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setContentsMargins(const QMargins & margins);
impl<'a> /*trait*/ QWidget_setContentsMargins<()> for (&'a  QMargins) {
  fn setContentsMargins(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setContentsMarginsERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget18setContentsMarginsERK8QMargins(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn saveGeometry<RetType, T: QWidget_saveGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.saveGeometry(self);
    // return 1;
  }
}

pub trait QWidget_saveGeometry<RetType> {
  fn saveGeometry(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QByteArray QWidget::saveGeometry();
impl<'a> /*trait*/ QWidget_saveGeometry<QByteArray> for () {
  fn saveGeometry(self, rsthis: &mut QWidget) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12saveGeometryEv()};
    let mut ret = unsafe {_ZNK7QWidget12saveGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isEnabled<RetType, T: QWidget_isEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QWidget_isEnabled<RetType> {
  fn isEnabled(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isEnabled();
impl<'a> /*trait*/ QWidget_isEnabled<i8> for () {
  fn isEnabled(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9isEnabledEv()};
    let mut ret = unsafe {_ZNK7QWidget9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFixedHeight<RetType, T: QWidget_setFixedHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.setFixedHeight(self);
    // return 1;
  }
}

pub trait QWidget_setFixedHeight<RetType> {
  fn setFixedHeight(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setFixedHeight(int h);
impl<'a> /*trait*/ QWidget_setFixedHeight<()> for (i32) {
  fn setFixedHeight(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setFixedHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWidget14setFixedHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mask<RetType, T: QWidget_mask<RetType>>(&mut self, value: T) -> RetType {
    return value.mask(self);
    // return 1;
  }
}

pub trait QWidget_mask<RetType> {
  fn mask(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QRegion QWidget::mask();
impl<'a> /*trait*/ QWidget_mask<QRegion> for () {
  fn mask(self, rsthis: &mut QWidget) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4maskEv()};
    let mut ret = unsafe {_ZNK7QWidget4maskEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn stackUnder<RetType, T: QWidget_stackUnder<RetType>>(&mut self, value: T) -> RetType {
    return value.stackUnder(self);
    // return 1;
  }
}

pub trait QWidget_stackUnder<RetType> {
  fn stackUnder(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::stackUnder(QWidget * );
impl<'a> /*trait*/ QWidget_stackUnder<()> for (&'a mut QWidget) {
  fn stackUnder(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10stackUnderEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget10stackUnderEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn paintEngine<RetType, T: QWidget_paintEngine<RetType>>(&mut self, value: T) -> RetType {
    return value.paintEngine(self);
    // return 1;
  }
}

pub trait QWidget_paintEngine<RetType> {
  fn paintEngine(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QPaintEngine * QWidget::paintEngine();
impl<'a> /*trait*/ QWidget_paintEngine<QPaintEngine> for () {
  fn paintEngine(self, rsthis: &mut QWidget) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11paintEngineEv()};
    let mut ret = unsafe {_ZNK7QWidget11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setAcceptDrops<RetType, T: QWidget_setAcceptDrops<RetType>>(&mut self, value: T) -> RetType {
    return value.setAcceptDrops(self);
    // return 1;
  }
}

pub trait QWidget_setAcceptDrops<RetType> {
  fn setAcceptDrops(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setAcceptDrops(bool on);
impl<'a> /*trait*/ QWidget_setAcceptDrops<()> for (i8) {
  fn setAcceptDrops(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setAcceptDropsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget14setAcceptDropsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn move_<RetType, T: QWidget_move_<RetType>>(&mut self, value: T) -> RetType {
    return value.move_(self);
    // return 1;
  }
}

pub trait QWidget_move_<RetType> {
  fn move_(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::move_(const QPoint & );
impl<'a> /*trait*/ QWidget_move_<()> for (&'a  QPoint) {
  fn move_(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4moveERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget4moveERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn actions<RetType, T: QWidget_actions<RetType>>(&mut self, value: T) -> RetType {
    return value.actions(self);
    // return 1;
  }
}

pub trait QWidget_actions<RetType> {
  fn actions(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QList<QAction *> QWidget::actions();
impl<'a> /*trait*/ QWidget_actions<()> for () {
  fn actions(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7actionsEv()};
     unsafe {_ZNK7QWidget7actionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn show<RetType, T: QWidget_show<RetType>>(&mut self, value: T) -> RetType {
    return value.show(self);
    // return 1;
  }
}

pub trait QWidget_show<RetType> {
  fn show(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::show();
impl<'a> /*trait*/ QWidget_show<()> for () {
  fn show(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4showEv()};
     unsafe {_ZN7QWidget4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn keyboardGrabber<RetType, T: QWidget_keyboardGrabber<RetType>>(&mut self, value: T) -> RetType {
    return value.keyboardGrabber(self);
    // return 1;
  }
}

pub trait QWidget_keyboardGrabber<RetType> {
  fn keyboardGrabber(self, rsthis: &mut QWidget) -> RetType;
}

// proto: static QWidget * QWidget::keyboardGrabber();
impl<'a> /*trait*/ QWidget_keyboardGrabber<QWidget> for () {
  fn keyboardGrabber(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15keyboardGrabberEv()};
    let mut ret = unsafe {_ZN7QWidget15keyboardGrabberEv()};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapTo<RetType, T: QWidget_mapTo<RetType>>(&mut self, value: T) -> RetType {
    return value.mapTo(self);
    // return 1;
  }
}

pub trait QWidget_mapTo<RetType> {
  fn mapTo(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QPoint QWidget::mapTo(const QWidget * , const QPoint & );
impl<'a> /*trait*/ QWidget_mapTo<QPoint> for (&'a  QWidget, &'a  QPoint) {
  fn mapTo(self, rsthis: &mut QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5mapToEPKS_RK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWidget5mapToEPKS_RK6QPoint(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn minimumWidth<RetType, T: QWidget_minimumWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumWidth(self);
    // return 1;
  }
}

pub trait QWidget_minimumWidth<RetType> {
  fn minimumWidth(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::minimumWidth();
impl<'a> /*trait*/ QWidget_minimumWidth<i32> for () {
  fn minimumWidth(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12minimumWidthEv()};
    let mut ret = unsafe {_ZNK7QWidget12minimumWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn fontInfo<RetType, T: QWidget_fontInfo<RetType>>(&mut self, value: T) -> RetType {
    return value.fontInfo(self);
    // return 1;
  }
}

pub trait QWidget_fontInfo<RetType> {
  fn fontInfo(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QFontInfo QWidget::fontInfo();
impl<'a> /*trait*/ QWidget_fontInfo<QFontInfo> for () {
  fn fontInfo(self, rsthis: &mut QWidget) -> QFontInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8fontInfoEv()};
    let mut ret = unsafe {_ZNK7QWidget8fontInfoEv(rsthis.qclsinst)};
    let mut ret1 = QFontInfo{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn autoFillBackground<RetType, T: QWidget_autoFillBackground<RetType>>(&mut self, value: T) -> RetType {
    return value.autoFillBackground(self);
    // return 1;
  }
}

pub trait QWidget_autoFillBackground<RetType> {
  fn autoFillBackground(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::autoFillBackground();
impl<'a> /*trait*/ QWidget_autoFillBackground<i8> for () {
  fn autoFillBackground(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18autoFillBackgroundEv()};
    let mut ret = unsafe {_ZNK7QWidget18autoFillBackgroundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn scroll<RetType, T: QWidget_scroll<RetType>>(&mut self, value: T) -> RetType {
    return value.scroll(self);
    // return 1;
  }
}

pub trait QWidget_scroll<RetType> {
  fn scroll(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::scroll(int dx, int dy, const QRect & );
impl<'a> /*trait*/ QWidget_scroll<()> for (i32, i32, &'a  QRect) {
  fn scroll(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6scrollEiiRK5QRect()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget6scrollEiiRK5QRect(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn fontMetrics<RetType, T: QWidget_fontMetrics<RetType>>(&mut self, value: T) -> RetType {
    return value.fontMetrics(self);
    // return 1;
  }
}

pub trait QWidget_fontMetrics<RetType> {
  fn fontMetrics(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QFontMetrics QWidget::fontMetrics();
impl<'a> /*trait*/ QWidget_fontMetrics<QFontMetrics> for () {
  fn fontMetrics(self, rsthis: &mut QWidget) -> QFontMetrics {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11fontMetricsEv()};
    let mut ret = unsafe {_ZNK7QWidget11fontMetricsEv(rsthis.qclsinst)};
    let mut ret1 = QFontMetrics{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn adjustSize<RetType, T: QWidget_adjustSize<RetType>>(&mut self, value: T) -> RetType {
    return value.adjustSize(self);
    // return 1;
  }
}

pub trait QWidget_adjustSize<RetType> {
  fn adjustSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::adjustSize();
impl<'a> /*trait*/ QWidget_adjustSize<()> for () {
  fn adjustSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10adjustSizeEv()};
     unsafe {_ZN7QWidget10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn previousInFocusChain<RetType, T: QWidget_previousInFocusChain<RetType>>(&mut self, value: T) -> RetType {
    return value.previousInFocusChain(self);
    // return 1;
  }
}

pub trait QWidget_previousInFocusChain<RetType> {
  fn previousInFocusChain(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::previousInFocusChain();
impl<'a> /*trait*/ QWidget_previousInFocusChain<QWidget> for () {
  fn previousInFocusChain(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget20previousInFocusChainEv()};
    let mut ret = unsafe {_ZNK7QWidget20previousInFocusChainEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn updatesEnabled<RetType, T: QWidget_updatesEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.updatesEnabled(self);
    // return 1;
  }
}

pub trait QWidget_updatesEnabled<RetType> {
  fn updatesEnabled(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::updatesEnabled();
impl<'a> /*trait*/ QWidget_updatesEnabled<i8> for () {
  fn updatesEnabled(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14updatesEnabledEv()};
    let mut ret = unsafe {_ZNK7QWidget14updatesEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMaximumHeight<RetType, T: QWidget_setMaximumHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.setMaximumHeight(self);
    // return 1;
  }
}

pub trait QWidget_setMaximumHeight<RetType> {
  fn setMaximumHeight(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setMaximumHeight(int maxh);
impl<'a> /*trait*/ QWidget_setMaximumHeight<()> for (i32) {
  fn setMaximumHeight(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMaximumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWidget16setMaximumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn showMaximized<RetType, T: QWidget_showMaximized<RetType>>(&mut self, value: T) -> RetType {
    return value.showMaximized(self);
    // return 1;
  }
}

pub trait QWidget_showMaximized<RetType> {
  fn showMaximized(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::showMaximized();
impl<'a> /*trait*/ QWidget_showMaximized<()> for () {
  fn showMaximized(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13showMaximizedEv()};
     unsafe {_ZN7QWidget13showMaximizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapFrom<RetType, T: QWidget_mapFrom<RetType>>(&mut self, value: T) -> RetType {
    return value.mapFrom(self);
    // return 1;
  }
}

pub trait QWidget_mapFrom<RetType> {
  fn mapFrom(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QPoint QWidget::mapFrom(const QWidget * , const QPoint & );
impl<'a> /*trait*/ QWidget_mapFrom<QPoint> for (&'a  QWidget, &'a  QPoint) {
  fn mapFrom(self, rsthis: &mut QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7mapFromEPKS_RK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWidget7mapFromEPKS_RK6QPoint(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn x<RetType, T: QWidget_x<RetType>>(&mut self, value: T) -> RetType {
    return value.x(self);
    // return 1;
  }
}

pub trait QWidget_x<RetType> {
  fn x(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::x();
impl<'a> /*trait*/ QWidget_x<()> for () {
  fn x(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget1xEv()};
     unsafe {_ZNK7QWidget1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn clearFocus<RetType, T: QWidget_clearFocus<RetType>>(&mut self, value: T) -> RetType {
    return value.clearFocus(self);
    // return 1;
  }
}

pub trait QWidget_clearFocus<RetType> {
  fn clearFocus(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::clearFocus();
impl<'a> /*trait*/ QWidget_clearFocus<()> for () {
  fn clearFocus(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10clearFocusEv()};
     unsafe {_ZN7QWidget10clearFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn find<RetType, T: QWidget_find<RetType>>(&mut self, value: T) -> RetType {
    return value.find(self);
    // return 1;
  }
}

pub trait QWidget_find<RetType> {
  fn find(self, rsthis: &mut QWidget) -> RetType;
}

// proto: static QWidget * QWidget::find(WId );
impl<'a> /*trait*/ QWidget_find<QWidget> for (*mut i32) {
  fn find(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4findEi()};
    let arg0 = self  as *mut c_uint;
    let mut ret = unsafe {_ZN7QWidget4findEi(arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn palette<RetType, T: QWidget_palette<RetType>>(&mut self, value: T) -> RetType {
    return value.palette(self);
    // return 1;
  }
}

pub trait QWidget_palette<RetType> {
  fn palette(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  const QPalette & QWidget::palette();
impl<'a> /*trait*/ QWidget_palette<QPalette> for () {
  fn palette(self, rsthis: &mut QWidget) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7paletteEv()};
    let mut ret = unsafe {_ZNK7QWidget7paletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setSizePolicy<RetType, T: QWidget_setSizePolicy<RetType>>(&mut self, value: T) -> RetType {
    return value.setSizePolicy(self);
    // return 1;
  }
}

pub trait QWidget_setSizePolicy<RetType> {
  fn setSizePolicy(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setSizePolicy(QSizePolicy );
impl<'a> /*trait*/ QWidget_setSizePolicy<()> for (QSizePolicy) {
  fn setSizePolicy(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setSizePolicyE11QSizePolicy()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget13setSizePolicyE11QSizePolicy(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMask<RetType, T: QWidget_setMask<RetType>>(&mut self, value: T) -> RetType {
    return value.setMask(self);
    // return 1;
  }
}

pub trait QWidget_setMask<RetType> {
  fn setMask(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setMask(const QRegion & );
impl<'a> /*trait*/ QWidget_setMask<()> for (&'a  QRegion) {
  fn setMask(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setMaskERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget7setMaskERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMaximumWidth<RetType, T: QWidget_setMaximumWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setMaximumWidth(self);
    // return 1;
  }
}

pub trait QWidget_setMaximumWidth<RetType> {
  fn setMaximumWidth(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setMaximumWidth(int maxw);
impl<'a> /*trait*/ QWidget_setMaximumWidth<()> for (i32) {
  fn setMaximumWidth(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15setMaximumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWidget15setMaximumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowIconText<RetType, T: QWidget_setWindowIconText<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowIconText(self);
    // return 1;
  }
}

pub trait QWidget_setWindowIconText<RetType> {
  fn setWindowIconText(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setWindowIconText(const QString & );
impl<'a> /*trait*/ QWidget_setWindowIconText<()> for (&'a  QString) {
  fn setWindowIconText(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowIconTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget17setWindowIconTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowIcon<RetType, T: QWidget_setWindowIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowIcon(self);
    // return 1;
  }
}

pub trait QWidget_setWindowIcon<RetType> {
  fn setWindowIcon(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QWidget_setWindowIcon<()> for (&'a  QIcon) {
  fn setWindowIcon(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget13setWindowIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn FreeQWidget<RetType, T: QWidget_FreeQWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQWidget(self);
    // return 1;
  }
}

pub trait QWidget_FreeQWidget<RetType> {
  fn FreeQWidget(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::FreeQWidget();
impl<'a> /*trait*/ QWidget_FreeQWidget<()> for () {
  fn FreeQWidget(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidgetD0Ev()};
     unsafe {_ZN7QWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn getContentsMargins<RetType, T: QWidget_getContentsMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.getContentsMargins(self);
    // return 1;
  }
}

pub trait QWidget_getContentsMargins<RetType> {
  fn getContentsMargins(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::getContentsMargins(int * left, int * top, int * right, int * bottom);
impl<'a> /*trait*/ QWidget_getContentsMargins<()> for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getContentsMargins(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn minimumSizeHint<RetType, T: QWidget_minimumSizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QWidget_minimumSizeHint<RetType> {
  fn minimumSizeHint(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSize QWidget::minimumSizeHint();
impl<'a> /*trait*/ QWidget_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self, rsthis: &mut QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK7QWidget15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowModified<RetType, T: QWidget_setWindowModified<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowModified(self);
    // return 1;
  }
}

pub trait QWidget_setWindowModified<RetType> {
  fn setWindowModified(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setWindowModified(bool );
impl<'a> /*trait*/ QWidget_setWindowModified<()> for (i8) {
  fn setWindowModified(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowModifiedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget17setWindowModifiedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn restoreGeometry<RetType, T: QWidget_restoreGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.restoreGeometry(self);
    // return 1;
  }
}

pub trait QWidget_restoreGeometry<RetType> {
  fn restoreGeometry(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::restoreGeometry(const QByteArray & geometry);
impl<'a> /*trait*/ QWidget_restoreGeometry<i8> for (&'a  QByteArray) {
  fn restoreGeometry(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15restoreGeometryERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QWidget15restoreGeometryERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn layout<RetType, T: QWidget_layout<RetType>>(&mut self, value: T) -> RetType {
    return value.layout(self);
    // return 1;
  }
}

pub trait QWidget_layout<RetType> {
  fn layout(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QLayout * QWidget::layout();
impl<'a> /*trait*/ QWidget_layout<QLayout> for () {
  fn layout(self, rsthis: &mut QWidget) -> QLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6layoutEv()};
    let mut ret = unsafe {_ZNK7QWidget6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QLayout{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn contentsRect<RetType, T: QWidget_contentsRect<RetType>>(&mut self, value: T) -> RetType {
    return value.contentsRect(self);
    // return 1;
  }
}

pub trait QWidget_contentsRect<RetType> {
  fn contentsRect(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QRect QWidget::contentsRect();
impl<'a> /*trait*/ QWidget_contentsRect<QRect> for () {
  fn contentsRect(self, rsthis: &mut QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12contentsRectEv()};
    let mut ret = unsafe {_ZNK7QWidget12contentsRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn backingStore<RetType, T: QWidget_backingStore<RetType>>(&mut self, value: T) -> RetType {
    return value.backingStore(self);
    // return 1;
  }
}

pub trait QWidget_backingStore<RetType> {
  fn backingStore(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QBackingStore * QWidget::backingStore();
impl<'a> /*trait*/ QWidget_backingStore<QBackingStore> for () {
  fn backingStore(self, rsthis: &mut QWidget) -> QBackingStore {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12backingStoreEv()};
    let mut ret = unsafe {_ZNK7QWidget12backingStoreEv(rsthis.qclsinst)};
    let mut ret1 = QBackingStore{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn focusProxy<RetType, T: QWidget_focusProxy<RetType>>(&mut self, value: T) -> RetType {
    return value.focusProxy(self);
    // return 1;
  }
}

pub trait QWidget_focusProxy<RetType> {
  fn focusProxy(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::focusProxy();
impl<'a> /*trait*/ QWidget_focusProxy<QWidget> for () {
  fn focusProxy(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10focusProxyEv()};
    let mut ret = unsafe {_ZNK7QWidget10focusProxyEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn styleSheet<RetType, T: QWidget_styleSheet<RetType>>(&mut self, value: T) -> RetType {
    return value.styleSheet(self);
    // return 1;
  }
}

pub trait QWidget_styleSheet<RetType> {
  fn styleSheet(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::styleSheet();
impl<'a> /*trait*/ QWidget_styleSheet<QString> for () {
  fn styleSheet(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10styleSheetEv()};
    let mut ret = unsafe {_ZNK7QWidget10styleSheetEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QWidget * QWidget::childAt(const QPoint & p);
impl<'a> /*trait*/ QWidget_childAt<QWidget> for (&'a  QPoint) {
  fn childAt(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7childAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWidget7childAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QWidget::repaint(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_repaint<()> for (i32, i32, i32, i32) {
  fn repaint(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN7QWidget7repaintEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn whatsThis<RetType, T: QWidget_whatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QWidget_whatsThis<RetType> {
  fn whatsThis(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::whatsThis();
impl<'a> /*trait*/ QWidget_whatsThis<QString> for () {
  fn whatsThis(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9whatsThisEv()};
    let mut ret = unsafe {_ZNK7QWidget9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn font<RetType, T: QWidget_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QWidget_font<RetType> {
  fn font(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  const QFont & QWidget::font();
impl<'a> /*trait*/ QWidget_font<QFont> for () {
  fn font(self, rsthis: &mut QWidget) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4fontEv()};
    let mut ret = unsafe {_ZNK7QWidget4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QWidget::setMinimumSize(int minw, int minh);
impl<'a> /*trait*/ QWidget_setMinimumSize<()> for (i32, i32) {
  fn setMinimumSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMinimumSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWidget14setMinimumSizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn metaObject<RetType, T: QWidget_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QWidget_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  const QMetaObject * QWidget::metaObject();
impl<'a> /*trait*/ QWidget_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10metaObjectEv()};
     unsafe {_ZNK7QWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMinimumHeight<RetType, T: QWidget_setMinimumHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.setMinimumHeight(self);
    // return 1;
  }
}

pub trait QWidget_setMinimumHeight<RetType> {
  fn setMinimumHeight(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setMinimumHeight(int minh);
impl<'a> /*trait*/ QWidget_setMinimumHeight<()> for (i32) {
  fn setMinimumHeight(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMinimumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWidget16setMinimumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QWidget::update(const QRegion & );
impl<'a> /*trait*/ QWidget_update<()> for (&'a  QRegion) {
  fn update(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget6updateERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowOpacity<RetType, T: QWidget_windowOpacity<RetType>>(&mut self, value: T) -> RetType {
    return value.windowOpacity(self);
    // return 1;
  }
}

pub trait QWidget_windowOpacity<RetType> {
  fn windowOpacity(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  double QWidget::windowOpacity();
impl<'a> /*trait*/ QWidget_windowOpacity<f64> for () {
  fn windowOpacity(self, rsthis: &mut QWidget) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13windowOpacityEv()};
    let mut ret = unsafe {_ZNK7QWidget13windowOpacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn childrenRegion<RetType, T: QWidget_childrenRegion<RetType>>(&mut self, value: T) -> RetType {
    return value.childrenRegion(self);
    // return 1;
  }
}

pub trait QWidget_childrenRegion<RetType> {
  fn childrenRegion(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QRegion QWidget::childrenRegion();
impl<'a> /*trait*/ QWidget_childrenRegion<QRegion> for () {
  fn childrenRegion(self, rsthis: &mut QWidget) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14childrenRegionEv()};
    let mut ret = unsafe {_ZNK7QWidget14childrenRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowFilePath<RetType, T: QWidget_setWindowFilePath<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowFilePath(self);
    // return 1;
  }
}

pub trait QWidget_setWindowFilePath<RetType> {
  fn setWindowFilePath(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setWindowFilePath(const QString & filePath);
impl<'a> /*trait*/ QWidget_setWindowFilePath<()> for (&'a  QString) {
  fn setWindowFilePath(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget17setWindowFilePathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setShortcutEnabled<RetType, T: QWidget_setShortcutEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setShortcutEnabled(self);
    // return 1;
  }
}

pub trait QWidget_setShortcutEnabled<RetType> {
  fn setShortcutEnabled(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setShortcutEnabled(int id, bool enable);
impl<'a> /*trait*/ QWidget_setShortcutEnabled<()> for (i32, i8) {
  fn setShortcutEnabled(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setShortcutEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN7QWidget18setShortcutEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn raise<RetType, T: QWidget_raise<RetType>>(&mut self, value: T) -> RetType {
    return value.raise(self);
    // return 1;
  }
}

pub trait QWidget_raise<RetType> {
  fn raise(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::raise();
impl<'a> /*trait*/ QWidget_raise<()> for () {
  fn raise(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5raiseEv()};
     unsafe {_ZN7QWidget5raiseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn statusTip<RetType, T: QWidget_statusTip<RetType>>(&mut self, value: T) -> RetType {
    return value.statusTip(self);
    // return 1;
  }
}

pub trait QWidget_statusTip<RetType> {
  fn statusTip(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::statusTip();
impl<'a> /*trait*/ QWidget_statusTip<QString> for () {
  fn statusTip(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9statusTipEv()};
    let mut ret = unsafe {_ZNK7QWidget9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn childrenRect<RetType, T: QWidget_childrenRect<RetType>>(&mut self, value: T) -> RetType {
    return value.childrenRect(self);
    // return 1;
  }
}

pub trait QWidget_childrenRect<RetType> {
  fn childrenRect(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QRect QWidget::childrenRect();
impl<'a> /*trait*/ QWidget_childrenRect<QRect> for () {
  fn childrenRect(self, rsthis: &mut QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12childrenRectEv()};
    let mut ret = unsafe {_ZNK7QWidget12childrenRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setParent<RetType, T: QWidget_setParent<RetType>>(&mut self, value: T) -> RetType {
    return value.setParent(self);
    // return 1;
  }
}

pub trait QWidget_setParent<RetType> {
  fn setParent(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setParent(QWidget * parent);
impl<'a> /*trait*/ QWidget_setParent<()> for (&'a mut QWidget) {
  fn setParent(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget9setParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn visibleRegion<RetType, T: QWidget_visibleRegion<RetType>>(&mut self, value: T) -> RetType {
    return value.visibleRegion(self);
    // return 1;
  }
}

pub trait QWidget_visibleRegion<RetType> {
  fn visibleRegion(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QRegion QWidget::visibleRegion();
impl<'a> /*trait*/ QWidget_visibleRegion<QRegion> for () {
  fn visibleRegion(self, rsthis: &mut QWidget) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13visibleRegionEv()};
    let mut ret = unsafe {_ZNK7QWidget13visibleRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn locale<RetType, T: QWidget_locale<RetType>>(&mut self, value: T) -> RetType {
    return value.locale(self);
    // return 1;
  }
}

pub trait QWidget_locale<RetType> {
  fn locale(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QLocale QWidget::locale();
impl<'a> /*trait*/ QWidget_locale<QLocale> for () {
  fn locale(self, rsthis: &mut QWidget) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6localeEv()};
    let mut ret = unsafe {_ZNK7QWidget6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn releaseKeyboard<RetType, T: QWidget_releaseKeyboard<RetType>>(&mut self, value: T) -> RetType {
    return value.releaseKeyboard(self);
    // return 1;
  }
}

pub trait QWidget_releaseKeyboard<RetType> {
  fn releaseKeyboard(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::releaseKeyboard();
impl<'a> /*trait*/ QWidget_releaseKeyboard<()> for () {
  fn releaseKeyboard(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15releaseKeyboardEv()};
     unsafe {_ZN7QWidget15releaseKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mouseGrabber<RetType, T: QWidget_mouseGrabber<RetType>>(&mut self, value: T) -> RetType {
    return value.mouseGrabber(self);
    // return 1;
  }
}

pub trait QWidget_mouseGrabber<RetType> {
  fn mouseGrabber(self, rsthis: &mut QWidget) -> RetType;
}

// proto: static QWidget * QWidget::mouseGrabber();
impl<'a> /*trait*/ QWidget_mouseGrabber<QWidget> for () {
  fn mouseGrabber(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12mouseGrabberEv()};
    let mut ret = unsafe {_ZN7QWidget12mouseGrabberEv()};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFixedWidth<RetType, T: QWidget_setFixedWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setFixedWidth(self);
    // return 1;
  }
}

pub trait QWidget_setFixedWidth<RetType> {
  fn setFixedWidth(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setFixedWidth(int w);
impl<'a> /*trait*/ QWidget_setFixedWidth<()> for (i32) {
  fn setFixedWidth(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setFixedWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWidget13setFixedWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn addAction<RetType, T: QWidget_addAction<RetType>>(&mut self, value: T) -> RetType {
    return value.addAction(self);
    // return 1;
  }
}

pub trait QWidget_addAction<RetType> {
  fn addAction(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::addAction(QAction * action);
impl<'a> /*trait*/ QWidget_addAction<()> for (&'a mut QAction) {
  fn addAction(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget9addActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setDisabled<RetType, T: QWidget_setDisabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setDisabled(self);
    // return 1;
  }
}

pub trait QWidget_setDisabled<RetType> {
  fn setDisabled(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setDisabled(bool );
impl<'a> /*trait*/ QWidget_setDisabled<()> for (i8) {
  fn setDisabled(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setDisabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget11setDisabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowIcon<RetType, T: QWidget_windowIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.windowIcon(self);
    // return 1;
  }
}

pub trait QWidget_windowIcon<RetType> {
  fn windowIcon(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QIcon QWidget::windowIcon();
impl<'a> /*trait*/ QWidget_windowIcon<QIcon> for () {
  fn windowIcon(self, rsthis: &mut QWidget) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10windowIconEv()};
    let mut ret = unsafe {_ZNK7QWidget10windowIconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QWidget::setContentsMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QWidget_setContentsMargins<()> for (i32, i32, i32, i32) {
  fn setContentsMargins(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setContentsMarginsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN7QWidget18setContentsMarginsEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowRole<RetType, T: QWidget_windowRole<RetType>>(&mut self, value: T) -> RetType {
    return value.windowRole(self);
    // return 1;
  }
}

pub trait QWidget_windowRole<RetType> {
  fn windowRole(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::windowRole();
impl<'a> /*trait*/ QWidget_windowRole<QString> for () {
  fn windowRole(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10windowRoleEv()};
    let mut ret = unsafe {_ZNK7QWidget10windowRoleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setShortcutAutoRepeat<RetType, T: QWidget_setShortcutAutoRepeat<RetType>>(&mut self, value: T) -> RetType {
    return value.setShortcutAutoRepeat(self);
    // return 1;
  }
}

pub trait QWidget_setShortcutAutoRepeat<RetType> {
  fn setShortcutAutoRepeat(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setShortcutAutoRepeat(int id, bool enable);
impl<'a> /*trait*/ QWidget_setShortcutAutoRepeat<()> for (i32, i8) {
  fn setShortcutAutoRepeat(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget21setShortcutAutoRepeatEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN7QWidget21setShortcutAutoRepeatEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn showFullScreen<RetType, T: QWidget_showFullScreen<RetType>>(&mut self, value: T) -> RetType {
    return value.showFullScreen(self);
    // return 1;
  }
}

pub trait QWidget_showFullScreen<RetType> {
  fn showFullScreen(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::showFullScreen();
impl<'a> /*trait*/ QWidget_showFullScreen<()> for () {
  fn showFullScreen(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14showFullScreenEv()};
     unsafe {_ZN7QWidget14showFullScreenEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QWidget::grabMouse();
impl<'a> /*trait*/ QWidget_grabMouse<()> for () {
  fn grabMouse(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9grabMouseEv()};
     unsafe {_ZN7QWidget9grabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QWidget::setMaximumSize(const QSize & );
impl<'a> /*trait*/ QWidget_setMaximumSize<()> for (&'a  QSize) {
  fn setMaximumSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMaximumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget14setMaximumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapToGlobal<RetType, T: QWidget_mapToGlobal<RetType>>(&mut self, value: T) -> RetType {
    return value.mapToGlobal(self);
    // return 1;
  }
}

pub trait QWidget_mapToGlobal<RetType> {
  fn mapToGlobal(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QPoint QWidget::mapToGlobal(const QPoint & );
impl<'a> /*trait*/ QWidget_mapToGlobal<QPoint> for (&'a  QPoint) {
  fn mapToGlobal(self, rsthis: &mut QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11mapToGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWidget11mapToGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn toolTip<RetType, T: QWidget_toolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QWidget_toolTip<RetType> {
  fn toolTip(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QString QWidget::toolTip();
impl<'a> /*trait*/ QWidget_toolTip<QString> for () {
  fn toolTip(self, rsthis: &mut QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7toolTipEv()};
    let mut ret = unsafe {_ZNK7QWidget7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWhatsThis<RetType, T: QWidget_setWhatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QWidget_setWhatsThis<RetType> {
  fn setWhatsThis(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setWhatsThis(const QString & );
impl<'a> /*trait*/ QWidget_setWhatsThis<()> for (&'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QWidget::resize(int w, int h);
impl<'a> /*trait*/ QWidget_resize<()> for (i32, i32) {
  fn resize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWidget6resizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn parentWidget<RetType, T: QWidget_parentWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.parentWidget(self);
    // return 1;
  }
}

pub trait QWidget_parentWidget<RetType> {
  fn parentWidget(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::parentWidget();
impl<'a> /*trait*/ QWidget_parentWidget<QWidget> for () {
  fn parentWidget(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12parentWidgetEv()};
    let mut ret = unsafe {_ZNK7QWidget12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn pos<RetType, T: QWidget_pos<RetType>>(&mut self, value: T) -> RetType {
    return value.pos(self);
    // return 1;
  }
}

pub trait QWidget_pos<RetType> {
  fn pos(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QPoint QWidget::pos();
impl<'a> /*trait*/ QWidget_pos<QPoint> for () {
  fn pos(self, rsthis: &mut QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget3posEv()};
    let mut ret = unsafe {_ZNK7QWidget3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setAutoFillBackground<RetType, T: QWidget_setAutoFillBackground<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoFillBackground(self);
    // return 1;
  }
}

pub trait QWidget_setAutoFillBackground<RetType> {
  fn setAutoFillBackground(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setAutoFillBackground(bool enabled);
impl<'a> /*trait*/ QWidget_setAutoFillBackground<()> for (i8) {
  fn setAutoFillBackground(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget21setAutoFillBackgroundEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget21setAutoFillBackgroundEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn hasFocus<RetType, T: QWidget_hasFocus<RetType>>(&mut self, value: T) -> RetType {
    return value.hasFocus(self);
    // return 1;
  }
}

pub trait QWidget_hasFocus<RetType> {
  fn hasFocus(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::hasFocus();
impl<'a> /*trait*/ QWidget_hasFocus<i8> for () {
  fn hasFocus(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8hasFocusEv()};
    let mut ret = unsafe {_ZNK7QWidget8hasFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn baseSize<RetType, T: QWidget_baseSize<RetType>>(&mut self, value: T) -> RetType {
    return value.baseSize(self);
    // return 1;
  }
}

pub trait QWidget_baseSize<RetType> {
  fn baseSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSize QWidget::baseSize();
impl<'a> /*trait*/ QWidget_baseSize<QSize> for () {
  fn baseSize(self, rsthis: &mut QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8baseSizeEv()};
    let mut ret = unsafe {_ZNK7QWidget8baseSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QWidget::setMask(const QBitmap & );
impl<'a> /*trait*/ QWidget_setMask<()> for (&'a  QBitmap) {
  fn setMask(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setMaskERK7QBitmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget7setMaskERK7QBitmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn ensurePolished<RetType, T: QWidget_ensurePolished<RetType>>(&mut self, value: T) -> RetType {
    return value.ensurePolished(self);
    // return 1;
  }
}

pub trait QWidget_ensurePolished<RetType> {
  fn ensurePolished(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::ensurePolished();
impl<'a> /*trait*/ QWidget_ensurePolished<()> for () {
  fn ensurePolished(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14ensurePolishedEv()};
     unsafe {_ZNK7QWidget14ensurePolishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowTitle<RetType, T: QWidget_setWindowTitle<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowTitle(self);
    // return 1;
  }
}

pub trait QWidget_setWindowTitle<RetType> {
  fn setWindowTitle(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setWindowTitle(const QString & );
impl<'a> /*trait*/ QWidget_setWindowTitle<()> for (&'a  QString) {
  fn setWindowTitle(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setWindowTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget14setWindowTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn window<RetType, T: QWidget_window<RetType>>(&mut self, value: T) -> RetType {
    return value.window(self);
    // return 1;
  }
}

pub trait QWidget_window<RetType> {
  fn window(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::window();
impl<'a> /*trait*/ QWidget_window<QWidget> for () {
  fn window(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6windowEv()};
    let mut ret = unsafe {_ZNK7QWidget6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QWidget::scroll(int dx, int dy);
impl<'a> /*trait*/ QWidget_scroll<()> for (i32, i32) {
  fn scroll(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6scrollEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWidget6scrollEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn releaseShortcut<RetType, T: QWidget_releaseShortcut<RetType>>(&mut self, value: T) -> RetType {
    return value.releaseShortcut(self);
    // return 1;
  }
}

pub trait QWidget_releaseShortcut<RetType> {
  fn releaseShortcut(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::releaseShortcut(int id);
impl<'a> /*trait*/ QWidget_releaseShortcut<()> for (i32) {
  fn releaseShortcut(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15releaseShortcutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWidget15releaseShortcutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setToolTipDuration<RetType, T: QWidget_setToolTipDuration<RetType>>(&mut self, value: T) -> RetType {
    return value.setToolTipDuration(self);
    // return 1;
  }
}

pub trait QWidget_setToolTipDuration<RetType> {
  fn setToolTipDuration(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setToolTipDuration(int msec);
impl<'a> /*trait*/ QWidget_setToolTipDuration<()> for (i32) {
  fn setToolTipDuration(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setToolTipDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWidget18setToolTipDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QWidget::setGeometry(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_setGeometry<()> for (i32, i32, i32, i32) {
  fn setGeometry(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setGeometryEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN7QWidget11setGeometryEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QWidget::setSizeIncrement(int w, int h);
impl<'a> /*trait*/ QWidget_setSizeIncrement<()> for (i32, i32) {
  fn setSizeIncrement(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setSizeIncrementEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWidget16setSizeIncrementEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setUpdatesEnabled<RetType, T: QWidget_setUpdatesEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setUpdatesEnabled(self);
    // return 1;
  }
}

pub trait QWidget_setUpdatesEnabled<RetType> {
  fn setUpdatesEnabled(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setUpdatesEnabled(bool enable);
impl<'a> /*trait*/ QWidget_setUpdatesEnabled<()> for (i8) {
  fn setUpdatesEnabled(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setUpdatesEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget17setUpdatesEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn lower<RetType, T: QWidget_lower<RetType>>(&mut self, value: T) -> RetType {
    return value.lower(self);
    // return 1;
  }
}

pub trait QWidget_lower<RetType> {
  fn lower(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::lower();
impl<'a> /*trait*/ QWidget_lower<()> for () {
  fn lower(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5lowerEv()};
     unsafe {_ZN7QWidget5lowerEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setMouseTracking<RetType, T: QWidget_setMouseTracking<RetType>>(&mut self, value: T) -> RetType {
    return value.setMouseTracking(self);
    // return 1;
  }
}

pub trait QWidget_setMouseTracking<RetType> {
  fn setMouseTracking(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setMouseTracking(bool enable);
impl<'a> /*trait*/ QWidget_setMouseTracking<()> for (i8) {
  fn setMouseTracking(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMouseTrackingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget16setMouseTrackingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QWidget::setBaseSize(const QSize & );
impl<'a> /*trait*/ QWidget_setBaseSize<()> for (&'a  QSize) {
  fn setBaseSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setBaseSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget11setBaseSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn hide<RetType, T: QWidget_hide<RetType>>(&mut self, value: T) -> RetType {
    return value.hide(self);
    // return 1;
  }
}

pub trait QWidget_hide<RetType> {
  fn hide(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::hide();
impl<'a> /*trait*/ QWidget_hide<()> for () {
  fn hide(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4hideEv()};
     unsafe {_ZN7QWidget4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn removeAction<RetType, T: QWidget_removeAction<RetType>>(&mut self, value: T) -> RetType {
    return value.removeAction(self);
    // return 1;
  }
}

pub trait QWidget_removeAction<RetType> {
  fn removeAction(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::removeAction(QAction * action);
impl<'a> /*trait*/ QWidget_removeAction<()> for (&'a mut QAction) {
  fn removeAction(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget12removeActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFocusProxy<RetType, T: QWidget_setFocusProxy<RetType>>(&mut self, value: T) -> RetType {
    return value.setFocusProxy(self);
    // return 1;
  }
}

pub trait QWidget_setFocusProxy<RetType> {
  fn setFocusProxy(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setFocusProxy(QWidget * );
impl<'a> /*trait*/ QWidget_setFocusProxy<()> for (&'a mut QWidget) {
  fn setFocusProxy(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setFocusProxyEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget13setFocusProxyEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn close<RetType, T: QWidget_close<RetType>>(&mut self, value: T) -> RetType {
    return value.close(self);
    // return 1;
  }
}

pub trait QWidget_close<RetType> {
  fn close(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::close();
impl<'a> /*trait*/ QWidget_close<i8> for () {
  fn close(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5closeEv()};
    let mut ret = unsafe {_ZN7QWidget5closeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn showMinimized<RetType, T: QWidget_showMinimized<RetType>>(&mut self, value: T) -> RetType {
    return value.showMinimized(self);
    // return 1;
  }
}

pub trait QWidget_showMinimized<RetType> {
  fn showMinimized(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::showMinimized();
impl<'a> /*trait*/ QWidget_showMinimized<()> for () {
  fn showMinimized(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13showMinimizedEv()};
     unsafe {_ZN7QWidget13showMinimizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QWidget::setFixedSize(int w, int h);
impl<'a> /*trait*/ QWidget_setFixedSize<()> for (i32, i32) {
  fn setFixedSize(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setFixedSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWidget12setFixedSizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn minimumSize<RetType, T: QWidget_minimumSize<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QWidget_minimumSize<RetType> {
  fn minimumSize(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSize QWidget::minimumSize();
impl<'a> /*trait*/ QWidget_minimumSize<QSize> for () {
  fn minimumSize(self, rsthis: &mut QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11minimumSizeEv()};
    let mut ret = unsafe {_ZNK7QWidget11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setEnabled<RetType, T: QWidget_setEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnabled(self);
    // return 1;
  }
}

pub trait QWidget_setEnabled<RetType> {
  fn setEnabled(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setEnabled(bool );
impl<'a> /*trait*/ QWidget_setEnabled<()> for (i8) {
  fn setEnabled(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn maximumHeight<RetType, T: QWidget_maximumHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.maximumHeight(self);
    // return 1;
  }
}

pub trait QWidget_maximumHeight<RetType> {
  fn maximumHeight(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::maximumHeight();
impl<'a> /*trait*/ QWidget_maximumHeight<i32> for () {
  fn maximumHeight(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13maximumHeightEv()};
    let mut ret = unsafe {_ZNK7QWidget13maximumHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QWidget::move_(int x, int y);
impl<'a> /*trait*/ QWidget_move_<()> for (i32, i32) {
  fn move_(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4moveEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QWidget4moveEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isAncestorOf<RetType, T: QWidget_isAncestorOf<RetType>>(&mut self, value: T) -> RetType {
    return value.isAncestorOf(self);
    // return 1;
  }
}

pub trait QWidget_isAncestorOf<RetType> {
  fn isAncestorOf(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isAncestorOf(const QWidget * child);
impl<'a> /*trait*/ QWidget_isAncestorOf<i8> for (&'a  QWidget) {
  fn isAncestorOf(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12isAncestorOfEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWidget12isAncestorOfEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn cursor<RetType, T: QWidget_cursor<RetType>>(&mut self, value: T) -> RetType {
    return value.cursor(self);
    // return 1;
  }
}

pub trait QWidget_cursor<RetType> {
  fn cursor(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QCursor QWidget::cursor();
impl<'a> /*trait*/ QWidget_cursor<QCursor> for () {
  fn cursor(self, rsthis: &mut QWidget) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6cursorEv()};
    let mut ret = unsafe {_ZNK7QWidget6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn mapFromGlobal<RetType, T: QWidget_mapFromGlobal<RetType>>(&mut self, value: T) -> RetType {
    return value.mapFromGlobal(self);
    // return 1;
  }
}

pub trait QWidget_mapFromGlobal<RetType> {
  fn mapFromGlobal(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QPoint QWidget::mapFromGlobal(const QPoint & );
impl<'a> /*trait*/ QWidget_mapFromGlobal<QPoint> for (&'a  QPoint) {
  fn mapFromGlobal(self, rsthis: &mut QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13mapFromGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWidget13mapFromGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setToolTip<RetType, T: QWidget_setToolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setToolTip(self);
    // return 1;
  }
}

pub trait QWidget_setToolTip<RetType> {
  fn setToolTip(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setToolTip(const QString & );
impl<'a> /*trait*/ QWidget_setToolTip<()> for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn sizePolicy<RetType, T: QWidget_sizePolicy<RetType>>(&mut self, value: T) -> RetType {
    return value.sizePolicy(self);
    // return 1;
  }
}

pub trait QWidget_sizePolicy<RetType> {
  fn sizePolicy(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSizePolicy QWidget::sizePolicy();
impl<'a> /*trait*/ QWidget_sizePolicy<QSizePolicy> for () {
  fn sizePolicy(self, rsthis: &mut QWidget) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10sizePolicyEv()};
    let mut ret = unsafe {_ZNK7QWidget10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn hasHeightForWidth<RetType, T: QWidget_hasHeightForWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QWidget_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::hasHeightForWidth();
impl<'a> /*trait*/ QWidget_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK7QWidget17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn graphicsProxyWidget<RetType, T: QWidget_graphicsProxyWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.graphicsProxyWidget(self);
    // return 1;
  }
}

pub trait QWidget_graphicsProxyWidget<RetType> {
  fn graphicsProxyWidget(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QGraphicsProxyWidget * QWidget::graphicsProxyWidget();
impl<'a> /*trait*/ QWidget_graphicsProxyWidget<()> for () {
  fn graphicsProxyWidget(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget19graphicsProxyWidgetEv()};
     unsafe {_ZNK7QWidget19graphicsProxyWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn contentsMargins<RetType, T: QWidget_contentsMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.contentsMargins(self);
    // return 1;
  }
}

pub trait QWidget_contentsMargins<RetType> {
  fn contentsMargins(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QMargins QWidget::contentsMargins();
impl<'a> /*trait*/ QWidget_contentsMargins<QMargins> for () {
  fn contentsMargins(self, rsthis: &mut QWidget) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15contentsMarginsEv()};
    let mut ret = unsafe {_ZNK7QWidget15contentsMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn topLevelWidget<RetType, T: QWidget_topLevelWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.topLevelWidget(self);
    // return 1;
  }
}

pub trait QWidget_topLevelWidget<RetType> {
  fn topLevelWidget(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::topLevelWidget();
impl<'a> /*trait*/ QWidget_topLevelWidget<QWidget> for () {
  fn topLevelWidget(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14topLevelWidgetEv()};
    let mut ret = unsafe {_ZNK7QWidget14topLevelWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setLayout<RetType, T: QWidget_setLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.setLayout(self);
    // return 1;
  }
}

pub trait QWidget_setLayout<RetType> {
  fn setLayout(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setLayout(QLayout * );
impl<'a> /*trait*/ QWidget_setLayout<()> for (&'a mut QLayout) {
  fn setLayout(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setLayoutEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget9setLayoutEP7QLayout(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn underMouse<RetType, T: QWidget_underMouse<RetType>>(&mut self, value: T) -> RetType {
    return value.underMouse(self);
    // return 1;
  }
}

pub trait QWidget_underMouse<RetType> {
  fn underMouse(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::underMouse();
impl<'a> /*trait*/ QWidget_underMouse<i8> for () {
  fn underMouse(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10underMouseEv()};
    let mut ret = unsafe {_ZNK7QWidget10underMouseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn heightForWidth<RetType, T: QWidget_heightForWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidget_heightForWidth<RetType> {
  fn heightForWidth(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::heightForWidth(int );
impl<'a> /*trait*/ QWidget_heightForWidth<i32> for (i32) {
  fn heightForWidth(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QWidget14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setFont<RetType, T: QWidget_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QWidget_setFont<RetType> {
  fn setFont(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setFont(const QFont & );
impl<'a> /*trait*/ QWidget_setFont<()> for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn nativeParentWidget<RetType, T: QWidget_nativeParentWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.nativeParentWidget(self);
    // return 1;
  }
}

pub trait QWidget_nativeParentWidget<RetType> {
  fn nativeParentWidget(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QWidget * QWidget::nativeParentWidget();
impl<'a> /*trait*/ QWidget_nativeParentWidget<QWidget> for () {
  fn nativeParentWidget(self, rsthis: &mut QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18nativeParentWidgetEv()};
    let mut ret = unsafe {_ZNK7QWidget18nativeParentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setLocale<RetType, T: QWidget_setLocale<RetType>>(&mut self, value: T) -> RetType {
    return value.setLocale(self);
    // return 1;
  }
}

pub trait QWidget_setLocale<RetType> {
  fn setLocale(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QWidget_setLocale<()> for (&'a  QLocale) {
  fn setLocale(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn height<RetType, T: QWidget_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QWidget_height<RetType> {
  fn height(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::height();
impl<'a> /*trait*/ QWidget_height<i32> for () {
  fn height(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6heightEv()};
    let mut ret = unsafe {_ZNK7QWidget6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setHidden<RetType, T: QWidget_setHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.setHidden(self);
    // return 1;
  }
}

pub trait QWidget_setHidden<RetType> {
  fn setHidden(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setHidden(bool hidden);
impl<'a> /*trait*/ QWidget_setHidden<()> for (i8) {
  fn setHidden(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setHiddenEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWidget9setHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn size<RetType, T: QWidget_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QWidget_size<RetType> {
  fn size(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QSize QWidget::size();
impl<'a> /*trait*/ QWidget_size<QSize> for () {
  fn size(self, rsthis: &mut QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4sizeEv()};
    let mut ret = unsafe {_ZNK7QWidget4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn maximumWidth<RetType, T: QWidget_maximumWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.maximumWidth(self);
    // return 1;
  }
}

pub trait QWidget_maximumWidth<RetType> {
  fn maximumWidth(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  int QWidget::maximumWidth();
impl<'a> /*trait*/ QWidget_maximumWidth<i32> for () {
  fn maximumWidth(self, rsthis: &mut QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12maximumWidthEv()};
    let mut ret = unsafe {_ZNK7QWidget12maximumWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isMinimized<RetType, T: QWidget_isMinimized<RetType>>(&mut self, value: T) -> RetType {
    return value.isMinimized(self);
    // return 1;
  }
}

pub trait QWidget_isMinimized<RetType> {
  fn isMinimized(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isMinimized();
impl<'a> /*trait*/ QWidget_isMinimized<i8> for () {
  fn isMinimized(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isMinimizedEv()};
    let mut ret = unsafe {_ZNK7QWidget11isMinimizedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QWidget::update();
impl<'a> /*trait*/ QWidget_update<()> for () {
  fn update(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateEv()};
     unsafe {_ZN7QWidget6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setCursor<RetType, T: QWidget_setCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.setCursor(self);
    // return 1;
  }
}

pub trait QWidget_setCursor<RetType> {
  fn setCursor(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setCursor(const QCursor & );
impl<'a> /*trait*/ QWidget_setCursor<()> for (&'a  QCursor) {
  fn setCursor(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowIconChanged<RetType, T: QWidget_windowIconChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.windowIconChanged(self);
    // return 1;
  }
}

pub trait QWidget_windowIconChanged<RetType> {
  fn windowIconChanged(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::windowIconChanged(const QIcon & icon);
impl<'a> /*trait*/ QWidget_windowIconChanged<()> for (&'a  QIcon) {
  fn windowIconChanged(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17windowIconChangedERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget17windowIconChangedERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn style<RetType, T: QWidget_style<RetType>>(&mut self, value: T) -> RetType {
    return value.style(self);
    // return 1;
  }
}

pub trait QWidget_style<RetType> {
  fn style(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  QStyle * QWidget::style();
impl<'a> /*trait*/ QWidget_style<QStyle> for () {
  fn style(self, rsthis: &mut QWidget) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5styleEv()};
    let mut ret = unsafe {_ZNK7QWidget5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn createWinId<RetType, T: QWidget_createWinId<RetType>>(&mut self, value: T) -> RetType {
    return value.createWinId(self);
    // return 1;
  }
}

pub trait QWidget_createWinId<RetType> {
  fn createWinId(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::createWinId();
impl<'a> /*trait*/ QWidget_createWinId<()> for () {
  fn createWinId(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11createWinIdEv()};
     unsafe {_ZN7QWidget11createWinIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setWindowOpacity<RetType, T: QWidget_setWindowOpacity<RetType>>(&mut self, value: T) -> RetType {
    return value.setWindowOpacity(self);
    // return 1;
  }
}

pub trait QWidget_setWindowOpacity<RetType> {
  fn setWindowOpacity(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setWindowOpacity(qreal level);
impl<'a> /*trait*/ QWidget_setWindowOpacity<()> for (f64) {
  fn setWindowOpacity(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setWindowOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QWidget16setWindowOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn isRightToLeft<RetType, T: QWidget_isRightToLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.isRightToLeft(self);
    // return 1;
  }
}

pub trait QWidget_isRightToLeft<RetType> {
  fn isRightToLeft(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  bool QWidget::isRightToLeft();
impl<'a> /*trait*/ QWidget_isRightToLeft<i8> for () {
  fn isRightToLeft(self, rsthis: &mut QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13isRightToLeftEv()};
    let mut ret = unsafe {_ZNK7QWidget13isRightToLeftEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn setAccessibleName<RetType, T: QWidget_setAccessibleName<RetType>>(&mut self, value: T) -> RetType {
    return value.setAccessibleName(self);
    // return 1;
  }
}

pub trait QWidget_setAccessibleName<RetType> {
  fn setAccessibleName(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::setAccessibleName(const QString & name);
impl<'a> /*trait*/ QWidget_setAccessibleName<()> for (&'a  QString) {
  fn setAccessibleName(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setAccessibleNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget17setAccessibleNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn windowIconTextChanged<RetType, T: QWidget_windowIconTextChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.windowIconTextChanged(self);
    // return 1;
  }
}

pub trait QWidget_windowIconTextChanged<RetType> {
  fn windowIconTextChanged(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::windowIconTextChanged(const QString & iconText);
impl<'a> /*trait*/ QWidget_windowIconTextChanged<()> for (&'a  QString) {
  fn windowIconTextChanged(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget21windowIconTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWidget21windowIconTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWidget {
  pub fn unsetCursor<RetType, T: QWidget_unsetCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.unsetCursor(self);
    // return 1;
  }
}

pub trait QWidget_unsetCursor<RetType> {
  fn unsetCursor(self, rsthis: &mut QWidget) -> RetType;
}

// proto:  void QWidget::unsetCursor();
impl<'a> /*trait*/ QWidget_unsetCursor<()> for () {
  fn unsetCursor(self, rsthis: &mut QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11unsetCursorEv()};
     unsafe {_ZN7QWidget11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

