// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qwidget.h
// dst-file: /src/widgets/qwidget.rs
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::qgraphicseffect::*; // 773
use super::super::core::qstring::*; // 771
use super::super::core::qsize::*; // 771
use super::super::core::qrect::*; // 771
use super::super::core::qpoint::*; // 771
use super::super::gui::qregion::*; // 771
use super::qaction::*; // 773
use super::super::gui::qpalette::*; // 771
use super::qstyle::*; // 773
use super::super::gui::qpainter::*; // 771
use super::super::gui::qcursor::*; // 771
use super::super::gui::qpixmap::*; // 771
use super::super::gui::qwindow::*; // 771
use super::super::core::qmargins::*; // 771
use super::super::core::qbytearray::*; // 771
use super::super::gui::qpaintengine::*; // 771
// use super::qlist::*; // 775
use super::super::gui::qfontinfo::*; // 771
use super::super::gui::qfontmetrics::*; // 771
use super::super::gui::qkeysequence::*; // 771
use super::qsizepolicy::*; // 773
use super::super::gui::qicon::*; // 771
use super::qlayout::*; // 773
use super::super::gui::qbackingstore::*; // 771
use super::super::gui::qfont::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qlocale::*; // 771
use super::super::gui::qpaintdevice::*; // 771
use super::super::gui::qbitmap::*; // 771
use super::qgraphicsproxywidget::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QWidget_Class_Size() -> c_int;
  // proto:  void QWidget::setGraphicsEffect(QGraphicsEffect * effect);
  fn C_ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QWidget::accessibleDescription();
  fn C_ZNK7QWidget21accessibleDescriptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QGraphicsEffect * QWidget::graphicsEffect();
  fn C_ZNK7QWidget14graphicsEffectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidget::isFullScreen();
  fn C_ZNK7QWidget12isFullScreenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSize QWidget::maximumSize();
  fn C_ZNK7QWidget11maximumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidget::isEnabledToTLW();
  fn C_ZNK7QWidget14isEnabledToTLWEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::setStatusTip(const QString & );
  fn C_ZN7QWidget12setStatusTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::setSizeIncrement(const QSize & );
  fn C_ZN7QWidget16setSizeIncrementERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QWidget::focusWidget();
  fn C_ZNK7QWidget11focusWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidget::isTopLevel();
  fn C_ZNK7QWidget10isTopLevelEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QWidget::acceptDrops();
  fn C_ZNK7QWidget11acceptDropsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QWidget::isWindow();
  fn C_ZNK7QWidget8isWindowEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::setFixedSize(const QSize & );
  fn C_ZN7QWidget12setFixedSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QWidget::isVisible();
  fn C_ZNK7QWidget9isVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QWidget::minimumHeight();
  fn C_ZNK7QWidget13minimumHeightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QSize QWidget::sizeIncrement();
  fn C_ZNK7QWidget13sizeIncrementEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::repaint(const QRect & );
  fn C_ZN7QWidget7repaintERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::update(int x, int y, int w, int h);
  fn C_ZN7QWidget6updateEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  QString QWidget::windowFilePath();
  fn C_ZNK7QWidget14windowFilePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::clearMask();
  fn C_ZN7QWidget9clearMaskEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPoint QWidget::mapFromParent(const QPoint & );
  fn C_ZNK7QWidget13mapFromParentERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRect QWidget::rect();
  fn C_ZNK7QWidget4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::unsetLayoutDirection();
  fn C_ZN7QWidget20unsetLayoutDirectionEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::setMinimumSize(const QSize & );
  fn C_ZN7QWidget14setMinimumSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QWidget::isActiveWindow();
  fn C_ZNK7QWidget14isActiveWindowEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::grabKeyboard();
  fn C_ZN7QWidget12grabKeyboardEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QWidget::frameSize();
  fn C_ZNK7QWidget9frameSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setFocus();
  fn C_ZN7QWidget8setFocusEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPoint QWidget::mapToParent(const QPoint & );
  fn C_ZNK7QWidget11mapToParentERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::updateGeometry();
  fn C_ZN7QWidget14updateGeometryEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::repaint(const QRegion & );
  fn C_ZN7QWidget7repaintERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::insertAction(QAction * before, QAction * action);
  fn C_ZN7QWidget12insertActionEP7QActionS1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QWidget::setWindowRole(const QString & );
  fn C_ZN7QWidget13setWindowRoleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QWidget::toolTipDuration();
  fn C_ZNK7QWidget15toolTipDurationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWidget::setPalette(const QPalette & );
  fn C_ZN7QWidget10setPaletteERK8QPalette(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::setStyleSheet(const QString & styleSheet);
  fn C_ZN7QWidget13setStyleSheetERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QWidget::windowIconText();
  fn C_ZNK7QWidget14windowIconTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::releaseMouse();
  fn C_ZN7QWidget12releaseMouseEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QWidget::isModal();
  fn C_ZNK7QWidget7isModalEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::setStyle(QStyle * );
  fn C_ZN7QWidget8setStyleEP6QStyle(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::repaint();
  fn C_ZN7QWidget7repaintEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::setBaseSize(int basew, int baseh);
  fn C_ZN7QWidget11setBaseSizeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  bool QWidget::isWindowModified();
  fn C_ZNK7QWidget16isWindowModifiedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QRect & QWidget::geometry();
  fn C_ZNK7QWidget8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setAccessibleDescription(const QString & description);
  fn C_ZN7QWidget24setAccessibleDescriptionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QWidget::nextInFocusChain();
  fn C_ZNK7QWidget16nextInFocusChainEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static void QWidget::setTabOrder(QWidget * , QWidget * );
  fn C_ZN7QWidget11setTabOrderEPS_S0_(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QRect QWidget::frameGeometry();
  fn C_ZNK7QWidget13frameGeometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QWidget::sizeHint();
  fn C_ZNK7QWidget8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setMinimumWidth(int minw);
  fn C_ZN7QWidget15setMinimumWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QWidget::isVisibleTo(const QWidget * );
  fn C_ZNK7QWidget11isVisibleToEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QWidget::setMaximumSize(int maxw, int maxh);
  fn C_ZN7QWidget14setMaximumSizeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  bool QWidget::hasMouseTracking();
  fn C_ZNK7QWidget16hasMouseTrackingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::update(const QRect & );
  fn C_ZN7QWidget6updateERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QWidget::isHidden();
  fn C_ZNK7QWidget8isHiddenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QWidget::devType();
  fn C_ZNK7QWidget7devTypeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QWidget * QWidget::childAt(int x, int y);
  fn C_ZNK7QWidget7childAtEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QWidget::activateWindow();
  fn C_ZN7QWidget14activateWindowEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRect QWidget::normalGeometry();
  fn C_ZNK7QWidget14normalGeometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QWidget::windowTitle();
  fn C_ZNK7QWidget11windowTitleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::grabMouse(const QCursor & );
  fn C_ZN7QWidget9grabMouseERK7QCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPixmap QWidget::grab(const QRect & rectangle);
  fn C_ZN7QWidget4grabERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setVisible(bool visible);
  fn C_ZN7QWidget10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QWidget::isEnabledTo(const QWidget * );
  fn C_ZNK7QWidget11isEnabledToEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QWidget::isLeftToRight();
  fn C_ZNK7QWidget13isLeftToRightEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::setGeometry(const QRect & );
  fn C_ZN7QWidget11setGeometryERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::unsetLocale();
  fn C_ZN7QWidget11unsetLocaleEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::showNormal();
  fn C_ZN7QWidget10showNormalEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QWidget::y();
  fn C_ZNK7QWidget1yEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QWidget::width();
  fn C_ZNK7QWidget5widthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QWidget::isMaximized();
  fn C_ZNK7QWidget11isMaximizedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::resize(const QSize & );
  fn C_ZN7QWidget6resizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWindow * QWidget::windowHandle();
  fn C_ZNK7QWidget12windowHandleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QWidget::accessibleName();
  fn C_ZNK7QWidget14accessibleNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setContentsMargins(const QMargins & margins);
  fn C_ZN7QWidget18setContentsMarginsERK8QMargins(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QWidget::saveGeometry();
  fn C_ZNK7QWidget12saveGeometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidget::isEnabled();
  fn C_ZNK7QWidget9isEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::setFixedHeight(int h);
  fn C_ZN7QWidget14setFixedHeightEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRegion QWidget::mask();
  fn C_ZNK7QWidget4maskEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::stackUnder(QWidget * );
  fn C_ZN7QWidget10stackUnderEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPaintEngine * QWidget::paintEngine();
  fn C_ZNK7QWidget11paintEngineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setAcceptDrops(bool on);
  fn C_ZN7QWidget14setAcceptDropsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QWidget::move(const QPoint & );
  fn C_ZN7QWidget4moveERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QAction *> QWidget::actions();
  fn C_ZNK7QWidget7actionsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::show();
  fn C_ZN7QWidget4showEv(qthis: u64 /* *mut c_void*/);
  // proto: static QWidget * QWidget::keyboardGrabber();
  fn C_ZN7QWidget15keyboardGrabberEv() -> *mut c_void;
  // proto:  QPoint QWidget::mapTo(const QWidget * , const QPoint & );
  fn C_ZNK7QWidget5mapToEPKS_RK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QWidget::minimumWidth();
  fn C_ZNK7QWidget12minimumWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QFontInfo QWidget::fontInfo();
  fn C_ZNK7QWidget8fontInfoEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidget::autoFillBackground();
  fn C_ZNK7QWidget18autoFillBackgroundEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::scroll(int dx, int dy, const QRect & );
  fn C_ZN7QWidget6scrollEiiRK5QRect(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  QFontMetrics QWidget::fontMetrics();
  fn C_ZNK7QWidget11fontMetricsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::adjustSize();
  fn C_ZN7QWidget10adjustSizeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QWidget * QWidget::previousInFocusChain();
  fn C_ZNK7QWidget20previousInFocusChainEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidget::updatesEnabled();
  fn C_ZNK7QWidget14updatesEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::setMaximumHeight(int maxh);
  fn C_ZN7QWidget16setMaximumHeightEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWidget::showMaximized();
  fn C_ZN7QWidget13showMaximizedEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPoint QWidget::mapFrom(const QWidget * , const QPoint & );
  fn C_ZNK7QWidget7mapFromEPKS_RK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QWidget::x();
  fn C_ZNK7QWidget1xEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWidget::clearFocus();
  fn C_ZN7QWidget10clearFocusEv(qthis: u64 /* *mut c_void*/);
  // proto: static QWidget * QWidget::find(WId );
  fn C_ZN7QWidget4findEi(arg0: c_int) -> *mut c_void;
  // proto:  const QPalette & QWidget::palette();
  fn C_ZNK7QWidget7paletteEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setSizePolicy(QSizePolicy );
  fn C_ZN7QWidget13setSizePolicyE11QSizePolicy(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::setMask(const QRegion & );
  fn C_ZN7QWidget7setMaskERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::setMaximumWidth(int maxw);
  fn C_ZN7QWidget15setMaximumWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWidget::setWindowIconText(const QString & );
  fn C_ZN7QWidget17setWindowIconTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::setWindowIcon(const QIcon & icon);
  fn C_ZN7QWidget13setWindowIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::~QWidget();
  fn C_ZN7QWidgetD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::getContentsMargins(int * left, int * top, int * right, int * bottom);
  fn C_ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  QSize QWidget::minimumSizeHint();
  fn C_ZNK7QWidget15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setWindowModified(bool );
  fn C_ZN7QWidget17setWindowModifiedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QWidget::restoreGeometry(const QByteArray & geometry);
  fn C_ZN7QWidget15restoreGeometryERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QLayout * QWidget::layout();
  fn C_ZNK7QWidget6layoutEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QWidget::contentsRect();
  fn C_ZNK7QWidget12contentsRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QBackingStore * QWidget::backingStore();
  fn C_ZNK7QWidget12backingStoreEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QWidget::focusProxy();
  fn C_ZNK7QWidget10focusProxyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QWidget::styleSheet();
  fn C_ZNK7QWidget10styleSheetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QWidget::childAt(const QPoint & p);
  fn C_ZNK7QWidget7childAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::repaint(int x, int y, int w, int h);
  fn C_ZN7QWidget7repaintEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  QString QWidget::whatsThis();
  fn C_ZNK7QWidget9whatsThisEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QFont & QWidget::font();
  fn C_ZNK7QWidget4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setMinimumSize(int minw, int minh);
  fn C_ZN7QWidget14setMinimumSizeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  const QMetaObject * QWidget::metaObject();
  fn C_ZNK7QWidget10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setMinimumHeight(int minh);
  fn C_ZN7QWidget16setMinimumHeightEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWidget::update(const QRegion & );
  fn C_ZN7QWidget6updateERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QWidget::windowOpacity();
  fn C_ZNK7QWidget13windowOpacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QRegion QWidget::childrenRegion();
  fn C_ZNK7QWidget14childrenRegionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setWindowFilePath(const QString & filePath);
  fn C_ZN7QWidget17setWindowFilePathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::setShortcutEnabled(int id, bool enable);
  fn C_ZN7QWidget18setShortcutEnabledEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  void QWidget::raise();
  fn C_ZN7QWidget5raiseEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QWidget::statusTip();
  fn C_ZNK7QWidget9statusTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QWidget::childrenRect();
  fn C_ZNK7QWidget12childrenRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setParent(QWidget * parent);
  fn C_ZN7QWidget9setParentEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRegion QWidget::visibleRegion();
  fn C_ZNK7QWidget13visibleRegionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QLocale QWidget::locale();
  fn C_ZNK7QWidget6localeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::releaseKeyboard();
  fn C_ZN7QWidget15releaseKeyboardEv(qthis: u64 /* *mut c_void*/);
  // proto: static QWidget * QWidget::mouseGrabber();
  fn C_ZN7QWidget12mouseGrabberEv() -> *mut c_void;
  // proto:  void QWidget::setFixedWidth(int w);
  fn C_ZN7QWidget13setFixedWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWidget::addAction(QAction * action);
  fn C_ZN7QWidget9addActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::setDisabled(bool );
  fn C_ZN7QWidget11setDisabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QIcon QWidget::windowIcon();
  fn C_ZNK7QWidget10windowIconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setContentsMargins(int left, int top, int right, int bottom);
  fn C_ZN7QWidget18setContentsMarginsEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  QString QWidget::windowRole();
  fn C_ZNK7QWidget10windowRoleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setShortcutAutoRepeat(int id, bool enable);
  fn C_ZN7QWidget21setShortcutAutoRepeatEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  void QWidget::showFullScreen();
  fn C_ZN7QWidget14showFullScreenEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::grabMouse();
  fn C_ZN7QWidget9grabMouseEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::setMaximumSize(const QSize & );
  fn C_ZN7QWidget14setMaximumSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPoint QWidget::mapToGlobal(const QPoint & );
  fn C_ZNK7QWidget11mapToGlobalERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QWidget::toolTip();
  fn C_ZNK7QWidget7toolTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setWhatsThis(const QString & );
  fn C_ZN7QWidget12setWhatsThisERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::resize(int w, int h);
  fn C_ZN7QWidget6resizeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QWidget * QWidget::parentWidget();
  fn C_ZNK7QWidget12parentWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QWidget::pos();
  fn C_ZNK7QWidget3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setAutoFillBackground(bool enabled);
  fn C_ZN7QWidget21setAutoFillBackgroundEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QWidget::hasFocus();
  fn C_ZNK7QWidget8hasFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSize QWidget::baseSize();
  fn C_ZNK7QWidget8baseSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setMask(const QBitmap & );
  fn C_ZN7QWidget7setMaskERK7QBitmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::ensurePolished();
  fn C_ZNK7QWidget14ensurePolishedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::setWindowTitle(const QString & );
  fn C_ZN7QWidget14setWindowTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QWidget::window();
  fn C_ZNK7QWidget6windowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::scroll(int dx, int dy);
  fn C_ZN7QWidget6scrollEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QWidget::releaseShortcut(int id);
  fn C_ZN7QWidget15releaseShortcutEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWidget::setToolTipDuration(int msec);
  fn C_ZN7QWidget18setToolTipDurationEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWidget::setGeometry(int x, int y, int w, int h);
  fn C_ZN7QWidget11setGeometryEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QWidget::setSizeIncrement(int w, int h);
  fn C_ZN7QWidget16setSizeIncrementEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QWidget::setUpdatesEnabled(bool enable);
  fn C_ZN7QWidget17setUpdatesEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QWidget::lower();
  fn C_ZN7QWidget5lowerEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::setMouseTracking(bool enable);
  fn C_ZN7QWidget16setMouseTrackingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QWidget::setBaseSize(const QSize & );
  fn C_ZN7QWidget11setBaseSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::hide();
  fn C_ZN7QWidget4hideEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::removeAction(QAction * action);
  fn C_ZN7QWidget12removeActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::setFocusProxy(QWidget * );
  fn C_ZN7QWidget13setFocusProxyEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QWidget::close();
  fn C_ZN7QWidget5closeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::showMinimized();
  fn C_ZN7QWidget13showMinimizedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::setFixedSize(int w, int h);
  fn C_ZN7QWidget12setFixedSizeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QSize QWidget::minimumSize();
  fn C_ZNK7QWidget11minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setEnabled(bool );
  fn C_ZN7QWidget10setEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QWidget::maximumHeight();
  fn C_ZNK7QWidget13maximumHeightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWidget::move(int x, int y);
  fn C_ZN7QWidget4moveEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  bool QWidget::isAncestorOf(const QWidget * child);
  fn C_ZNK7QWidget12isAncestorOfEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QCursor QWidget::cursor();
  fn C_ZNK7QWidget6cursorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QWidget::mapFromGlobal(const QPoint & );
  fn C_ZNK7QWidget13mapFromGlobalERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QWidget::setToolTip(const QString & );
  fn C_ZN7QWidget10setToolTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSizePolicy QWidget::sizePolicy();
  fn C_ZNK7QWidget10sizePolicyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWidget::hasHeightForWidth();
  fn C_ZNK7QWidget17hasHeightForWidthEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QGraphicsProxyWidget * QWidget::graphicsProxyWidget();
  fn C_ZNK7QWidget19graphicsProxyWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QMargins QWidget::contentsMargins();
  fn C_ZNK7QWidget15contentsMarginsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QWidget::topLevelWidget();
  fn C_ZNK7QWidget14topLevelWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setLayout(QLayout * );
  fn C_ZN7QWidget9setLayoutEP7QLayout(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QWidget::underMouse();
  fn C_ZNK7QWidget10underMouseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QWidget::heightForWidth(int );
  fn C_ZNK7QWidget14heightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QWidget::setFont(const QFont & );
  fn C_ZN7QWidget7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QWidget::nativeParentWidget();
  fn C_ZNK7QWidget18nativeParentWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::setLocale(const QLocale & locale);
  fn C_ZN7QWidget9setLocaleERK7QLocale(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QWidget::height();
  fn C_ZNK7QWidget6heightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWidget::setHidden(bool hidden);
  fn C_ZN7QWidget9setHiddenEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QSize QWidget::size();
  fn C_ZNK7QWidget4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QWidget::maximumWidth();
  fn C_ZNK7QWidget12maximumWidthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QWidget::isMinimized();
  fn C_ZNK7QWidget11isMinimizedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::update();
  fn C_ZN7QWidget6updateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::setCursor(const QCursor & );
  fn C_ZN7QWidget9setCursorERK7QCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStyle * QWidget::style();
  fn C_ZNK7QWidget5styleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWidget::createWinId();
  fn C_ZN7QWidget11createWinIdEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWidget::setWindowOpacity(qreal level);
  fn C_ZN7QWidget16setWindowOpacityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  bool QWidget::isRightToLeft();
  fn C_ZNK7QWidget13isRightToLeftEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWidget::setAccessibleName(const QString & name);
  fn C_ZN7QWidget17setAccessibleNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWidget::unsetCursor();
  fn C_ZN7QWidget11unsetCursorEv(qthis: u64 /* *mut c_void*/);
  fn QWidgetData_Class_Size() -> c_int;
  fn QWidget_SlotProxy_connect__ZN7QWidget26customContextMenuRequestedERK6QPoint(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWidget_SlotProxy_connect__ZN7QWidget17windowIconChangedERK5QIcon(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWidget_SlotProxy_connect__ZN7QWidget21windowIconTextChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWidget_SlotProxy_connect__ZN7QWidget18windowTitleChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QWidget)=1
#[derive(Default)]
pub struct QWidget {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _windowIconChanged: QWidget_windowIconChanged_signal,
  pub _windowTitleChanged: QWidget_windowTitleChanged_signal,
  pub _customContextMenuRequested: QWidget_customContextMenuRequested_signal,
  pub _windowIconTextChanged: QWidget_windowIconTextChanged_signal,
}

// class sizeof(QWidgetData)=1
#[derive(Default)]
pub struct QWidgetData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWidget {
    return QWidget{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWidget {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QWidget {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QWidget::setGraphicsEffect(QGraphicsEffect * effect);
impl /*struct*/ QWidget {
  pub fn setGraphicsEffect<RetType, T: QWidget_setGraphicsEffect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGraphicsEffect(self);
    // return 1;
  }
}

pub trait QWidget_setGraphicsEffect<RetType> {
  fn setGraphicsEffect(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setGraphicsEffect(QGraphicsEffect * effect);
impl<'a> /*trait*/ QWidget_setGraphicsEffect<()> for (&'a QGraphicsEffect) {
  fn setGraphicsEffect(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget17setGraphicsEffectEP15QGraphicsEffect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QWidget::accessibleDescription();
impl /*struct*/ QWidget {
  pub fn accessibleDescription<RetType, T: QWidget_accessibleDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accessibleDescription(self);
    // return 1;
  }
}

pub trait QWidget_accessibleDescription<RetType> {
  fn accessibleDescription(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::accessibleDescription();
impl<'a> /*trait*/ QWidget_accessibleDescription<QString> for () {
  fn accessibleDescription(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget21accessibleDescriptionEv()};
    let mut ret = unsafe {C_ZNK7QWidget21accessibleDescriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsEffect * QWidget::graphicsEffect();
impl /*struct*/ QWidget {
  pub fn graphicsEffect<RetType, T: QWidget_graphicsEffect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.graphicsEffect(self);
    // return 1;
  }
}

pub trait QWidget_graphicsEffect<RetType> {
  fn graphicsEffect(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QGraphicsEffect * QWidget::graphicsEffect();
impl<'a> /*trait*/ QWidget_graphicsEffect<QGraphicsEffect> for () {
  fn graphicsEffect(self , rsthis: & QWidget) -> QGraphicsEffect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14graphicsEffectEv()};
    let mut ret = unsafe {C_ZNK7QWidget14graphicsEffectEv(rsthis.qclsinst)};
    let mut ret1 = QGraphicsEffect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidget::isFullScreen();
impl /*struct*/ QWidget {
  pub fn isFullScreen<RetType, T: QWidget_isFullScreen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFullScreen(self);
    // return 1;
  }
}

pub trait QWidget_isFullScreen<RetType> {
  fn isFullScreen(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isFullScreen();
impl<'a> /*trait*/ QWidget_isFullScreen<i8> for () {
  fn isFullScreen(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12isFullScreenEv()};
    let mut ret = unsafe {C_ZNK7QWidget12isFullScreenEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QSize QWidget::maximumSize();
impl /*struct*/ QWidget {
  pub fn maximumSize<RetType, T: QWidget_maximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QWidget_maximumSize<RetType> {
  fn maximumSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSize QWidget::maximumSize();
impl<'a> /*trait*/ QWidget_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: & QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11maximumSizeEv()};
    let mut ret = unsafe {C_ZNK7QWidget11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidget::isEnabledToTLW();
impl /*struct*/ QWidget {
  pub fn isEnabledToTLW<RetType, T: QWidget_isEnabledToTLW<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabledToTLW(self);
    // return 1;
  }
}

pub trait QWidget_isEnabledToTLW<RetType> {
  fn isEnabledToTLW(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isEnabledToTLW();
impl<'a> /*trait*/ QWidget_isEnabledToTLW<i8> for () {
  fn isEnabledToTLW(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14isEnabledToTLWEv()};
    let mut ret = unsafe {C_ZNK7QWidget14isEnabledToTLWEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setStatusTip(const QString & );
impl /*struct*/ QWidget {
  pub fn setStatusTip<RetType, T: QWidget_setStatusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip(self);
    // return 1;
  }
}

pub trait QWidget_setStatusTip<RetType> {
  fn setStatusTip(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setStatusTip(const QString & );
impl<'a> /*trait*/ QWidget_setStatusTip<()> for (&'a QString) {
  fn setStatusTip(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setSizeIncrement(const QSize & );
impl /*struct*/ QWidget {
  pub fn setSizeIncrement<RetType, T: QWidget_setSizeIncrement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizeIncrement(self);
    // return 1;
  }
}

pub trait QWidget_setSizeIncrement<RetType> {
  fn setSizeIncrement(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setSizeIncrement(const QSize & );
impl<'a> /*trait*/ QWidget_setSizeIncrement<()> for (&'a QSize) {
  fn setSizeIncrement(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setSizeIncrementERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget16setSizeIncrementERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QWidget::focusWidget();
impl /*struct*/ QWidget {
  pub fn focusWidget<RetType, T: QWidget_focusWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusWidget(self);
    // return 1;
  }
}

pub trait QWidget_focusWidget<RetType> {
  fn focusWidget(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::focusWidget();
impl<'a> /*trait*/ QWidget_focusWidget<QWidget> for () {
  fn focusWidget(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11focusWidgetEv()};
    let mut ret = unsafe {C_ZNK7QWidget11focusWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidget::isTopLevel();
impl /*struct*/ QWidget {
  pub fn isTopLevel<RetType, T: QWidget_isTopLevel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTopLevel(self);
    // return 1;
  }
}

pub trait QWidget_isTopLevel<RetType> {
  fn isTopLevel(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isTopLevel();
impl<'a> /*trait*/ QWidget_isTopLevel<i8> for () {
  fn isTopLevel(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10isTopLevelEv()};
    let mut ret = unsafe {C_ZNK7QWidget10isTopLevelEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QWidget::acceptDrops();
impl /*struct*/ QWidget {
  pub fn acceptDrops<RetType, T: QWidget_acceptDrops<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptDrops(self);
    // return 1;
  }
}

pub trait QWidget_acceptDrops<RetType> {
  fn acceptDrops(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::acceptDrops();
impl<'a> /*trait*/ QWidget_acceptDrops<i8> for () {
  fn acceptDrops(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11acceptDropsEv()};
    let mut ret = unsafe {C_ZNK7QWidget11acceptDropsEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QWidget::isWindow();
impl /*struct*/ QWidget {
  pub fn isWindow<RetType, T: QWidget_isWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWindow(self);
    // return 1;
  }
}

pub trait QWidget_isWindow<RetType> {
  fn isWindow(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isWindow();
impl<'a> /*trait*/ QWidget_isWindow<i8> for () {
  fn isWindow(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8isWindowEv()};
    let mut ret = unsafe {C_ZNK7QWidget8isWindowEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setFixedSize(const QSize & );
impl /*struct*/ QWidget {
  pub fn setFixedSize<RetType, T: QWidget_setFixedSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFixedSize(self);
    // return 1;
  }
}

pub trait QWidget_setFixedSize<RetType> {
  fn setFixedSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setFixedSize(const QSize & );
impl<'a> /*trait*/ QWidget_setFixedSize<()> for (&'a QSize) {
  fn setFixedSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setFixedSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget12setFixedSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::isVisible();
impl /*struct*/ QWidget {
  pub fn isVisible<RetType, T: QWidget_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QWidget_isVisible<RetType> {
  fn isVisible(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isVisible();
impl<'a> /*trait*/ QWidget_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9isVisibleEv()};
    let mut ret = unsafe {C_ZNK7QWidget9isVisibleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QWidget::minimumHeight();
impl /*struct*/ QWidget {
  pub fn minimumHeight<RetType, T: QWidget_minimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumHeight(self);
    // return 1;
  }
}

pub trait QWidget_minimumHeight<RetType> {
  fn minimumHeight(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::minimumHeight();
impl<'a> /*trait*/ QWidget_minimumHeight<i32> for () {
  fn minimumHeight(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13minimumHeightEv()};
    let mut ret = unsafe {C_ZNK7QWidget13minimumHeightEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QSize QWidget::sizeIncrement();
impl /*struct*/ QWidget {
  pub fn sizeIncrement<RetType, T: QWidget_sizeIncrement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeIncrement(self);
    // return 1;
  }
}

pub trait QWidget_sizeIncrement<RetType> {
  fn sizeIncrement(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSize QWidget::sizeIncrement();
impl<'a> /*trait*/ QWidget_sizeIncrement<QSize> for () {
  fn sizeIncrement(self , rsthis: & QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13sizeIncrementEv()};
    let mut ret = unsafe {C_ZNK7QWidget13sizeIncrementEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::repaint(const QRect & );
impl /*struct*/ QWidget {
  pub fn repaint<RetType, T: QWidget_repaint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.repaint(self);
    // return 1;
  }
}

pub trait QWidget_repaint<RetType> {
  fn repaint(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::repaint(const QRect & );
impl<'a> /*trait*/ QWidget_repaint<()> for (&'a QRect) {
  fn repaint(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget7repaintERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::update(int x, int y, int w, int h);
impl /*struct*/ QWidget {
  pub fn update<RetType, T: QWidget_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QWidget_update<RetType> {
  fn update(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::update(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_update<()> for (i32, i32, i32, i32) {
  fn update(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN7QWidget6updateEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QString QWidget::windowFilePath();
impl /*struct*/ QWidget {
  pub fn windowFilePath<RetType, T: QWidget_windowFilePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowFilePath(self);
    // return 1;
  }
}

pub trait QWidget_windowFilePath<RetType> {
  fn windowFilePath(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::windowFilePath();
impl<'a> /*trait*/ QWidget_windowFilePath<QString> for () {
  fn windowFilePath(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14windowFilePathEv()};
    let mut ret = unsafe {C_ZNK7QWidget14windowFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::clearMask();
impl /*struct*/ QWidget {
  pub fn clearMask<RetType, T: QWidget_clearMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMask(self);
    // return 1;
  }
}

pub trait QWidget_clearMask<RetType> {
  fn clearMask(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::clearMask();
impl<'a> /*trait*/ QWidget_clearMask<()> for () {
  fn clearMask(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9clearMaskEv()};
     unsafe {C_ZN7QWidget9clearMaskEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWidget::mapFromParent(const QPoint & );
impl /*struct*/ QWidget {
  pub fn mapFromParent<RetType, T: QWidget_mapFromParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent(self);
    // return 1;
  }
}

pub trait QWidget_mapFromParent<RetType> {
  fn mapFromParent(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPoint QWidget::mapFromParent(const QPoint & );
impl<'a> /*trait*/ QWidget_mapFromParent<QPoint> for (&'a QPoint) {
  fn mapFromParent(self , rsthis: & QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13mapFromParentERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget13mapFromParentERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QWidget::rect();
impl /*struct*/ QWidget {
  pub fn rect<RetType, T: QWidget_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QWidget_rect<RetType> {
  fn rect(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QRect QWidget::rect();
impl<'a> /*trait*/ QWidget_rect<QRect> for () {
  fn rect(self , rsthis: & QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4rectEv()};
    let mut ret = unsafe {C_ZNK7QWidget4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::unsetLayoutDirection();
impl /*struct*/ QWidget {
  pub fn unsetLayoutDirection<RetType, T: QWidget_unsetLayoutDirection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetLayoutDirection(self);
    // return 1;
  }
}

pub trait QWidget_unsetLayoutDirection<RetType> {
  fn unsetLayoutDirection(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::unsetLayoutDirection();
impl<'a> /*trait*/ QWidget_unsetLayoutDirection<()> for () {
  fn unsetLayoutDirection(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget20unsetLayoutDirectionEv()};
     unsafe {C_ZN7QWidget20unsetLayoutDirectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::setMinimumSize(const QSize & );
impl /*struct*/ QWidget {
  pub fn setMinimumSize<RetType, T: QWidget_setMinimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize(self);
    // return 1;
  }
}

pub trait QWidget_setMinimumSize<RetType> {
  fn setMinimumSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setMinimumSize(const QSize & );
impl<'a> /*trait*/ QWidget_setMinimumSize<()> for (&'a QSize) {
  fn setMinimumSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMinimumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget14setMinimumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::isActiveWindow();
impl /*struct*/ QWidget {
  pub fn isActiveWindow<RetType, T: QWidget_isActiveWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActiveWindow(self);
    // return 1;
  }
}

pub trait QWidget_isActiveWindow<RetType> {
  fn isActiveWindow(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isActiveWindow();
impl<'a> /*trait*/ QWidget_isActiveWindow<i8> for () {
  fn isActiveWindow(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14isActiveWindowEv()};
    let mut ret = unsafe {C_ZNK7QWidget14isActiveWindowEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::grabKeyboard();
impl /*struct*/ QWidget {
  pub fn grabKeyboard<RetType, T: QWidget_grabKeyboard<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabKeyboard(self);
    // return 1;
  }
}

pub trait QWidget_grabKeyboard<RetType> {
  fn grabKeyboard(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::grabKeyboard();
impl<'a> /*trait*/ QWidget_grabKeyboard<()> for () {
  fn grabKeyboard(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12grabKeyboardEv()};
     unsafe {C_ZN7QWidget12grabKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QWidget::frameSize();
impl /*struct*/ QWidget {
  pub fn frameSize<RetType, T: QWidget_frameSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameSize(self);
    // return 1;
  }
}

pub trait QWidget_frameSize<RetType> {
  fn frameSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSize QWidget::frameSize();
impl<'a> /*trait*/ QWidget_frameSize<QSize> for () {
  fn frameSize(self , rsthis: & QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9frameSizeEv()};
    let mut ret = unsafe {C_ZNK7QWidget9frameSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setFocus();
impl /*struct*/ QWidget {
  pub fn setFocus<RetType, T: QWidget_setFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFocus(self);
    // return 1;
  }
}

pub trait QWidget_setFocus<RetType> {
  fn setFocus(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setFocus();
impl<'a> /*trait*/ QWidget_setFocus<()> for () {
  fn setFocus(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget8setFocusEv()};
     unsafe {C_ZN7QWidget8setFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWidget::mapToParent(const QPoint & );
impl /*struct*/ QWidget {
  pub fn mapToParent<RetType, T: QWidget_mapToParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToParent(self);
    // return 1;
  }
}

pub trait QWidget_mapToParent<RetType> {
  fn mapToParent(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPoint QWidget::mapToParent(const QPoint & );
impl<'a> /*trait*/ QWidget_mapToParent<QPoint> for (&'a QPoint) {
  fn mapToParent(self , rsthis: & QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11mapToParentERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget11mapToParentERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::updateGeometry();
impl /*struct*/ QWidget {
  pub fn updateGeometry<RetType, T: QWidget_updateGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry(self);
    // return 1;
  }
}

pub trait QWidget_updateGeometry<RetType> {
  fn updateGeometry(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::updateGeometry();
impl<'a> /*trait*/ QWidget_updateGeometry<()> for () {
  fn updateGeometry(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14updateGeometryEv()};
     unsafe {C_ZN7QWidget14updateGeometryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::repaint(const QRegion & );
impl<'a> /*trait*/ QWidget_repaint<()> for (&'a QRegion) {
  fn repaint(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget7repaintERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::insertAction(QAction * before, QAction * action);
impl /*struct*/ QWidget {
  pub fn insertAction<RetType, T: QWidget_insertAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertAction(self);
    // return 1;
  }
}

pub trait QWidget_insertAction<RetType> {
  fn insertAction(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::insertAction(QAction * before, QAction * action);
impl<'a> /*trait*/ QWidget_insertAction<()> for (&'a QAction, &'a QAction) {
  fn insertAction(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12insertActionEP7QActionS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget12insertActionEP7QActionS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWidget::setWindowRole(const QString & );
impl /*struct*/ QWidget {
  pub fn setWindowRole<RetType, T: QWidget_setWindowRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowRole(self);
    // return 1;
  }
}

pub trait QWidget_setWindowRole<RetType> {
  fn setWindowRole(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setWindowRole(const QString & );
impl<'a> /*trait*/ QWidget_setWindowRole<()> for (&'a QString) {
  fn setWindowRole(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setWindowRoleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget13setWindowRoleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWidget::toolTipDuration();
impl /*struct*/ QWidget {
  pub fn toolTipDuration<RetType, T: QWidget_toolTipDuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTipDuration(self);
    // return 1;
  }
}

pub trait QWidget_toolTipDuration<RetType> {
  fn toolTipDuration(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::toolTipDuration();
impl<'a> /*trait*/ QWidget_toolTipDuration<i32> for () {
  fn toolTipDuration(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15toolTipDurationEv()};
    let mut ret = unsafe {C_ZNK7QWidget15toolTipDurationEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setPalette(const QPalette & );
impl /*struct*/ QWidget {
  pub fn setPalette<RetType, T: QWidget_setPalette<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPalette(self);
    // return 1;
  }
}

pub trait QWidget_setPalette<RetType> {
  fn setPalette(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setPalette(const QPalette & );
impl<'a> /*trait*/ QWidget_setPalette<()> for (&'a QPalette) {
  fn setPalette(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget10setPaletteERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setStyleSheet(const QString & styleSheet);
impl /*struct*/ QWidget {
  pub fn setStyleSheet<RetType, T: QWidget_setStyleSheet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStyleSheet(self);
    // return 1;
  }
}

pub trait QWidget_setStyleSheet<RetType> {
  fn setStyleSheet(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setStyleSheet(const QString & styleSheet);
impl<'a> /*trait*/ QWidget_setStyleSheet<()> for (&'a QString) {
  fn setStyleSheet(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget13setStyleSheetERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QWidget::windowIconText();
impl /*struct*/ QWidget {
  pub fn windowIconText<RetType, T: QWidget_windowIconText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowIconText(self);
    // return 1;
  }
}

pub trait QWidget_windowIconText<RetType> {
  fn windowIconText(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::windowIconText();
impl<'a> /*trait*/ QWidget_windowIconText<QString> for () {
  fn windowIconText(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14windowIconTextEv()};
    let mut ret = unsafe {C_ZNK7QWidget14windowIconTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::releaseMouse();
impl /*struct*/ QWidget {
  pub fn releaseMouse<RetType, T: QWidget_releaseMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.releaseMouse(self);
    // return 1;
  }
}

pub trait QWidget_releaseMouse<RetType> {
  fn releaseMouse(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::releaseMouse();
impl<'a> /*trait*/ QWidget_releaseMouse<()> for () {
  fn releaseMouse(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12releaseMouseEv()};
     unsafe {C_ZN7QWidget12releaseMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QWidget::isModal();
impl /*struct*/ QWidget {
  pub fn isModal<RetType, T: QWidget_isModal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isModal(self);
    // return 1;
  }
}

pub trait QWidget_isModal<RetType> {
  fn isModal(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isModal();
impl<'a> /*trait*/ QWidget_isModal<i8> for () {
  fn isModal(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7isModalEv()};
    let mut ret = unsafe {C_ZNK7QWidget7isModalEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setStyle(QStyle * );
impl /*struct*/ QWidget {
  pub fn setStyle<RetType, T: QWidget_setStyle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStyle(self);
    // return 1;
  }
}

pub trait QWidget_setStyle<RetType> {
  fn setStyle(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setStyle(QStyle * );
impl<'a> /*trait*/ QWidget_setStyle<()> for (&'a QStyle) {
  fn setStyle(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget8setStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::repaint();
impl<'a> /*trait*/ QWidget_repaint<()> for () {
  fn repaint(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintEv()};
     unsafe {C_ZN7QWidget7repaintEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::setBaseSize(int basew, int baseh);
impl /*struct*/ QWidget {
  pub fn setBaseSize<RetType, T: QWidget_setBaseSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBaseSize(self);
    // return 1;
  }
}

pub trait QWidget_setBaseSize<RetType> {
  fn setBaseSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setBaseSize(int basew, int baseh);
impl<'a> /*trait*/ QWidget_setBaseSize<()> for (i32, i32) {
  fn setBaseSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setBaseSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QWidget11setBaseSizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QWidget::isWindowModified();
impl /*struct*/ QWidget {
  pub fn isWindowModified<RetType, T: QWidget_isWindowModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWindowModified(self);
    // return 1;
  }
}

pub trait QWidget_isWindowModified<RetType> {
  fn isWindowModified(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isWindowModified();
impl<'a> /*trait*/ QWidget_isWindowModified<i8> for () {
  fn isWindowModified(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16isWindowModifiedEv()};
    let mut ret = unsafe {C_ZNK7QWidget16isWindowModifiedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QRect & QWidget::geometry();
impl /*struct*/ QWidget {
  pub fn geometry<RetType, T: QWidget_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QWidget_geometry<RetType> {
  fn geometry(self , rsthis: & QWidget) -> RetType;
}

  // proto:  const QRect & QWidget::geometry();
impl<'a> /*trait*/ QWidget_geometry<QRect> for () {
  fn geometry(self , rsthis: & QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8geometryEv()};
    let mut ret = unsafe {C_ZNK7QWidget8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setAccessibleDescription(const QString & description);
impl /*struct*/ QWidget {
  pub fn setAccessibleDescription<RetType, T: QWidget_setAccessibleDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleDescription(self);
    // return 1;
  }
}

pub trait QWidget_setAccessibleDescription<RetType> {
  fn setAccessibleDescription(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setAccessibleDescription(const QString & description);
impl<'a> /*trait*/ QWidget_setAccessibleDescription<()> for (&'a QString) {
  fn setAccessibleDescription(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget24setAccessibleDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget24setAccessibleDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QWidget::nextInFocusChain();
impl /*struct*/ QWidget {
  pub fn nextInFocusChain<RetType, T: QWidget_nextInFocusChain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextInFocusChain(self);
    // return 1;
  }
}

pub trait QWidget_nextInFocusChain<RetType> {
  fn nextInFocusChain(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::nextInFocusChain();
impl<'a> /*trait*/ QWidget_nextInFocusChain<QWidget> for () {
  fn nextInFocusChain(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16nextInFocusChainEv()};
    let mut ret = unsafe {C_ZNK7QWidget16nextInFocusChainEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QWidget::setTabOrder(QWidget * , QWidget * );
impl /*struct*/ QWidget {
  pub fn setTabOrder_s<RetType, T: QWidget_setTabOrder_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTabOrder_s();
    // return 1;
  }
}

pub trait QWidget_setTabOrder_s<RetType> {
  fn setTabOrder_s(self ) -> RetType;
}

  // proto: static void QWidget::setTabOrder(QWidget * , QWidget * );
impl<'a> /*trait*/ QWidget_setTabOrder_s<()> for (&'a QWidget, &'a QWidget) {
  fn setTabOrder_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setTabOrderEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget11setTabOrderEPS_S0_(arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRect QWidget::frameGeometry();
impl /*struct*/ QWidget {
  pub fn frameGeometry<RetType, T: QWidget_frameGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameGeometry(self);
    // return 1;
  }
}

pub trait QWidget_frameGeometry<RetType> {
  fn frameGeometry(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QRect QWidget::frameGeometry();
impl<'a> /*trait*/ QWidget_frameGeometry<QRect> for () {
  fn frameGeometry(self , rsthis: & QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13frameGeometryEv()};
    let mut ret = unsafe {C_ZNK7QWidget13frameGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QWidget::sizeHint();
impl /*struct*/ QWidget {
  pub fn sizeHint<RetType, T: QWidget_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWidget_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSize QWidget::sizeHint();
impl<'a> /*trait*/ QWidget_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8sizeHintEv()};
    let mut ret = unsafe {C_ZNK7QWidget8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setMinimumWidth(int minw);
impl /*struct*/ QWidget {
  pub fn setMinimumWidth<RetType, T: QWidget_setMinimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumWidth(self);
    // return 1;
  }
}

pub trait QWidget_setMinimumWidth<RetType> {
  fn setMinimumWidth(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setMinimumWidth(int minw);
impl<'a> /*trait*/ QWidget_setMinimumWidth<()> for (i32) {
  fn setMinimumWidth(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15setMinimumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWidget15setMinimumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::isVisibleTo(const QWidget * );
impl /*struct*/ QWidget {
  pub fn isVisibleTo<RetType, T: QWidget_isVisibleTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisibleTo(self);
    // return 1;
  }
}

pub trait QWidget_isVisibleTo<RetType> {
  fn isVisibleTo(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isVisibleTo(const QWidget * );
impl<'a> /*trait*/ QWidget_isVisibleTo<i8> for (&'a QWidget) {
  fn isVisibleTo(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isVisibleToEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget11isVisibleToEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setMaximumSize(int maxw, int maxh);
impl /*struct*/ QWidget {
  pub fn setMaximumSize<RetType, T: QWidget_setMaximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize(self);
    // return 1;
  }
}

pub trait QWidget_setMaximumSize<RetType> {
  fn setMaximumSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setMaximumSize(int maxw, int maxh);
impl<'a> /*trait*/ QWidget_setMaximumSize<()> for (i32, i32) {
  fn setMaximumSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMaximumSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QWidget14setMaximumSizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QWidget::hasMouseTracking();
impl /*struct*/ QWidget {
  pub fn hasMouseTracking<RetType, T: QWidget_hasMouseTracking<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasMouseTracking(self);
    // return 1;
  }
}

pub trait QWidget_hasMouseTracking<RetType> {
  fn hasMouseTracking(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::hasMouseTracking();
impl<'a> /*trait*/ QWidget_hasMouseTracking<i8> for () {
  fn hasMouseTracking(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget16hasMouseTrackingEv()};
    let mut ret = unsafe {C_ZNK7QWidget16hasMouseTrackingEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::update(const QRect & );
impl<'a> /*trait*/ QWidget_update<()> for (&'a QRect) {
  fn update(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget6updateERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::isHidden();
impl /*struct*/ QWidget {
  pub fn isHidden<RetType, T: QWidget_isHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isHidden(self);
    // return 1;
  }
}

pub trait QWidget_isHidden<RetType> {
  fn isHidden(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isHidden();
impl<'a> /*trait*/ QWidget_isHidden<i8> for () {
  fn isHidden(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8isHiddenEv()};
    let mut ret = unsafe {C_ZNK7QWidget8isHiddenEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QWidget::devType();
impl /*struct*/ QWidget {
  pub fn devType<RetType, T: QWidget_devType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QWidget_devType<RetType> {
  fn devType(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::devType();
impl<'a> /*trait*/ QWidget_devType<i32> for () {
  fn devType(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7devTypeEv()};
    let mut ret = unsafe {C_ZNK7QWidget7devTypeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QWidget * QWidget::childAt(int x, int y);
impl /*struct*/ QWidget {
  pub fn childAt<RetType, T: QWidget_childAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childAt(self);
    // return 1;
  }
}

pub trait QWidget_childAt<RetType> {
  fn childAt(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::childAt(int x, int y);
impl<'a> /*trait*/ QWidget_childAt<QWidget> for (i32, i32) {
  fn childAt(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QWidget7childAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::activateWindow();
impl /*struct*/ QWidget {
  pub fn activateWindow<RetType, T: QWidget_activateWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activateWindow(self);
    // return 1;
  }
}

pub trait QWidget_activateWindow<RetType> {
  fn activateWindow(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::activateWindow();
impl<'a> /*trait*/ QWidget_activateWindow<()> for () {
  fn activateWindow(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14activateWindowEv()};
     unsafe {C_ZN7QWidget14activateWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRect QWidget::normalGeometry();
impl /*struct*/ QWidget {
  pub fn normalGeometry<RetType, T: QWidget_normalGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalGeometry(self);
    // return 1;
  }
}

pub trait QWidget_normalGeometry<RetType> {
  fn normalGeometry(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QRect QWidget::normalGeometry();
impl<'a> /*trait*/ QWidget_normalGeometry<QRect> for () {
  fn normalGeometry(self , rsthis: & QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14normalGeometryEv()};
    let mut ret = unsafe {C_ZNK7QWidget14normalGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWidget::windowTitle();
impl /*struct*/ QWidget {
  pub fn windowTitle<RetType, T: QWidget_windowTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowTitle(self);
    // return 1;
  }
}

pub trait QWidget_windowTitle<RetType> {
  fn windowTitle(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::windowTitle();
impl<'a> /*trait*/ QWidget_windowTitle<QString> for () {
  fn windowTitle(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11windowTitleEv()};
    let mut ret = unsafe {C_ZNK7QWidget11windowTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::grabMouse(const QCursor & );
impl /*struct*/ QWidget {
  pub fn grabMouse<RetType, T: QWidget_grabMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabMouse(self);
    // return 1;
  }
}

pub trait QWidget_grabMouse<RetType> {
  fn grabMouse(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::grabMouse(const QCursor & );
impl<'a> /*trait*/ QWidget_grabMouse<()> for (&'a QCursor) {
  fn grabMouse(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9grabMouseERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget9grabMouseERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPixmap QWidget::grab(const QRect & rectangle);
impl /*struct*/ QWidget {
  pub fn grab<RetType, T: QWidget_grab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grab(self);
    // return 1;
  }
}

pub trait QWidget_grab<RetType> {
  fn grab(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPixmap QWidget::grab(const QRect & rectangle);
impl<'a> /*trait*/ QWidget_grab<QPixmap> for (&'a QRect) {
  fn grab(self , rsthis: & QWidget) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4grabERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QWidget4grabERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setVisible(bool visible);
impl /*struct*/ QWidget {
  pub fn setVisible<RetType, T: QWidget_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QWidget_setVisible<RetType> {
  fn setVisible(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setVisible(bool visible);
impl<'a> /*trait*/ QWidget_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::isEnabledTo(const QWidget * );
impl /*struct*/ QWidget {
  pub fn isEnabledTo<RetType, T: QWidget_isEnabledTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabledTo(self);
    // return 1;
  }
}

pub trait QWidget_isEnabledTo<RetType> {
  fn isEnabledTo(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isEnabledTo(const QWidget * );
impl<'a> /*trait*/ QWidget_isEnabledTo<i8> for (&'a QWidget) {
  fn isEnabledTo(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isEnabledToEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget11isEnabledToEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QWidget::isLeftToRight();
impl /*struct*/ QWidget {
  pub fn isLeftToRight<RetType, T: QWidget_isLeftToRight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLeftToRight(self);
    // return 1;
  }
}

pub trait QWidget_isLeftToRight<RetType> {
  fn isLeftToRight(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isLeftToRight();
impl<'a> /*trait*/ QWidget_isLeftToRight<i8> for () {
  fn isLeftToRight(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13isLeftToRightEv()};
    let mut ret = unsafe {C_ZNK7QWidget13isLeftToRightEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setGeometry(const QRect & );
impl /*struct*/ QWidget {
  pub fn setGeometry<RetType, T: QWidget_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QWidget_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setGeometry(const QRect & );
impl<'a> /*trait*/ QWidget_setGeometry<()> for (&'a QRect) {
  fn setGeometry(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::unsetLocale();
impl /*struct*/ QWidget {
  pub fn unsetLocale<RetType, T: QWidget_unsetLocale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetLocale(self);
    // return 1;
  }
}

pub trait QWidget_unsetLocale<RetType> {
  fn unsetLocale(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::unsetLocale();
impl<'a> /*trait*/ QWidget_unsetLocale<()> for () {
  fn unsetLocale(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11unsetLocaleEv()};
     unsafe {C_ZN7QWidget11unsetLocaleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::showNormal();
impl /*struct*/ QWidget {
  pub fn showNormal<RetType, T: QWidget_showNormal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showNormal(self);
    // return 1;
  }
}

pub trait QWidget_showNormal<RetType> {
  fn showNormal(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::showNormal();
impl<'a> /*trait*/ QWidget_showNormal<()> for () {
  fn showNormal(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10showNormalEv()};
     unsafe {C_ZN7QWidget10showNormalEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QWidget::y();
impl /*struct*/ QWidget {
  pub fn y<RetType, T: QWidget_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QWidget_y<RetType> {
  fn y(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::y();
impl<'a> /*trait*/ QWidget_y<i32> for () {
  fn y(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget1yEv()};
    let mut ret = unsafe {C_ZNK7QWidget1yEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QWidget::width();
impl /*struct*/ QWidget {
  pub fn width<RetType, T: QWidget_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QWidget_width<RetType> {
  fn width(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::width();
impl<'a> /*trait*/ QWidget_width<i32> for () {
  fn width(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5widthEv()};
    let mut ret = unsafe {C_ZNK7QWidget5widthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QWidget::isMaximized();
impl /*struct*/ QWidget {
  pub fn isMaximized<RetType, T: QWidget_isMaximized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMaximized(self);
    // return 1;
  }
}

pub trait QWidget_isMaximized<RetType> {
  fn isMaximized(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isMaximized();
impl<'a> /*trait*/ QWidget_isMaximized<i8> for () {
  fn isMaximized(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isMaximizedEv()};
    let mut ret = unsafe {C_ZNK7QWidget11isMaximizedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::resize(const QSize & );
impl /*struct*/ QWidget {
  pub fn resize<RetType, T: QWidget_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QWidget_resize<RetType> {
  fn resize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::resize(const QSize & );
impl<'a> /*trait*/ QWidget_resize<()> for (&'a QSize) {
  fn resize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWindow * QWidget::windowHandle();
impl /*struct*/ QWidget {
  pub fn windowHandle<RetType, T: QWidget_windowHandle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowHandle(self);
    // return 1;
  }
}

pub trait QWidget_windowHandle<RetType> {
  fn windowHandle(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWindow * QWidget::windowHandle();
impl<'a> /*trait*/ QWidget_windowHandle<QWindow> for () {
  fn windowHandle(self , rsthis: & QWidget) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12windowHandleEv()};
    let mut ret = unsafe {C_ZNK7QWidget12windowHandleEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWidget::accessibleName();
impl /*struct*/ QWidget {
  pub fn accessibleName<RetType, T: QWidget_accessibleName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accessibleName(self);
    // return 1;
  }
}

pub trait QWidget_accessibleName<RetType> {
  fn accessibleName(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::accessibleName();
impl<'a> /*trait*/ QWidget_accessibleName<QString> for () {
  fn accessibleName(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14accessibleNameEv()};
    let mut ret = unsafe {C_ZNK7QWidget14accessibleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setContentsMargins(const QMargins & margins);
impl /*struct*/ QWidget {
  pub fn setContentsMargins<RetType, T: QWidget_setContentsMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins(self);
    // return 1;
  }
}

pub trait QWidget_setContentsMargins<RetType> {
  fn setContentsMargins(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setContentsMargins(const QMargins & margins);
impl<'a> /*trait*/ QWidget_setContentsMargins<()> for (&'a QMargins) {
  fn setContentsMargins(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setContentsMarginsERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget18setContentsMarginsERK8QMargins(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QWidget::saveGeometry();
impl /*struct*/ QWidget {
  pub fn saveGeometry<RetType, T: QWidget_saveGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.saveGeometry(self);
    // return 1;
  }
}

pub trait QWidget_saveGeometry<RetType> {
  fn saveGeometry(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QByteArray QWidget::saveGeometry();
impl<'a> /*trait*/ QWidget_saveGeometry<QByteArray> for () {
  fn saveGeometry(self , rsthis: & QWidget) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12saveGeometryEv()};
    let mut ret = unsafe {C_ZNK7QWidget12saveGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidget::isEnabled();
impl /*struct*/ QWidget {
  pub fn isEnabled<RetType, T: QWidget_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QWidget_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isEnabled();
impl<'a> /*trait*/ QWidget_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9isEnabledEv()};
    let mut ret = unsafe {C_ZNK7QWidget9isEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setFixedHeight(int h);
impl /*struct*/ QWidget {
  pub fn setFixedHeight<RetType, T: QWidget_setFixedHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFixedHeight(self);
    // return 1;
  }
}

pub trait QWidget_setFixedHeight<RetType> {
  fn setFixedHeight(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setFixedHeight(int h);
impl<'a> /*trait*/ QWidget_setFixedHeight<()> for (i32) {
  fn setFixedHeight(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setFixedHeightEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWidget14setFixedHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRegion QWidget::mask();
impl /*struct*/ QWidget {
  pub fn mask<RetType, T: QWidget_mask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mask(self);
    // return 1;
  }
}

pub trait QWidget_mask<RetType> {
  fn mask(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QRegion QWidget::mask();
impl<'a> /*trait*/ QWidget_mask<QRegion> for () {
  fn mask(self , rsthis: & QWidget) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4maskEv()};
    let mut ret = unsafe {C_ZNK7QWidget4maskEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::stackUnder(QWidget * );
impl /*struct*/ QWidget {
  pub fn stackUnder<RetType, T: QWidget_stackUnder<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stackUnder(self);
    // return 1;
  }
}

pub trait QWidget_stackUnder<RetType> {
  fn stackUnder(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::stackUnder(QWidget * );
impl<'a> /*trait*/ QWidget_stackUnder<()> for (&'a QWidget) {
  fn stackUnder(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10stackUnderEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget10stackUnderEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPaintEngine * QWidget::paintEngine();
impl /*struct*/ QWidget {
  pub fn paintEngine<RetType, T: QWidget_paintEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QWidget_paintEngine<RetType> {
  fn paintEngine(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPaintEngine * QWidget::paintEngine();
impl<'a> /*trait*/ QWidget_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: & QWidget) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11paintEngineEv()};
    let mut ret = unsafe {C_ZNK7QWidget11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setAcceptDrops(bool on);
impl /*struct*/ QWidget {
  pub fn setAcceptDrops<RetType, T: QWidget_setAcceptDrops<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAcceptDrops(self);
    // return 1;
  }
}

pub trait QWidget_setAcceptDrops<RetType> {
  fn setAcceptDrops(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setAcceptDrops(bool on);
impl<'a> /*trait*/ QWidget_setAcceptDrops<()> for (i8) {
  fn setAcceptDrops(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setAcceptDropsEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget14setAcceptDropsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::move(const QPoint & );
impl /*struct*/ QWidget {
  pub fn move_<RetType, T: QWidget_move_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.move_(self);
    // return 1;
  }
}

pub trait QWidget_move_<RetType> {
  fn move_(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::move(const QPoint & );
impl<'a> /*trait*/ QWidget_move_<()> for (&'a QPoint) {
  fn move_(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4moveERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget4moveERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QAction *> QWidget::actions();
impl /*struct*/ QWidget {
  pub fn actions<RetType, T: QWidget_actions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actions(self);
    // return 1;
  }
}

pub trait QWidget_actions<RetType> {
  fn actions(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QList<QAction *> QWidget::actions();
impl<'a> /*trait*/ QWidget_actions<u64> for () {
  fn actions(self , rsthis: & QWidget) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7actionsEv()};
    let mut ret = unsafe {C_ZNK7QWidget7actionsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QWidget::show();
impl /*struct*/ QWidget {
  pub fn show<RetType, T: QWidget_show<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.show(self);
    // return 1;
  }
}

pub trait QWidget_show<RetType> {
  fn show(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::show();
impl<'a> /*trait*/ QWidget_show<()> for () {
  fn show(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4showEv()};
     unsafe {C_ZN7QWidget4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QWidget * QWidget::keyboardGrabber();
impl /*struct*/ QWidget {
  pub fn keyboardGrabber_s<RetType, T: QWidget_keyboardGrabber_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.keyboardGrabber_s();
    // return 1;
  }
}

pub trait QWidget_keyboardGrabber_s<RetType> {
  fn keyboardGrabber_s(self ) -> RetType;
}

  // proto: static QWidget * QWidget::keyboardGrabber();
impl<'a> /*trait*/ QWidget_keyboardGrabber_s<QWidget> for () {
  fn keyboardGrabber_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15keyboardGrabberEv()};
    let mut ret = unsafe {C_ZN7QWidget15keyboardGrabberEv()};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QWidget::mapTo(const QWidget * , const QPoint & );
impl /*struct*/ QWidget {
  pub fn mapTo<RetType, T: QWidget_mapTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapTo(self);
    // return 1;
  }
}

pub trait QWidget_mapTo<RetType> {
  fn mapTo(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPoint QWidget::mapTo(const QWidget * , const QPoint & );
impl<'a> /*trait*/ QWidget_mapTo<QPoint> for (&'a QWidget, &'a QPoint) {
  fn mapTo(self , rsthis: & QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5mapToEPKS_RK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget5mapToEPKS_RK6QPoint(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWidget::minimumWidth();
impl /*struct*/ QWidget {
  pub fn minimumWidth<RetType, T: QWidget_minimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth(self);
    // return 1;
  }
}

pub trait QWidget_minimumWidth<RetType> {
  fn minimumWidth(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::minimumWidth();
impl<'a> /*trait*/ QWidget_minimumWidth<i32> for () {
  fn minimumWidth(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12minimumWidthEv()};
    let mut ret = unsafe {C_ZNK7QWidget12minimumWidthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QFontInfo QWidget::fontInfo();
impl /*struct*/ QWidget {
  pub fn fontInfo<RetType, T: QWidget_fontInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontInfo(self);
    // return 1;
  }
}

pub trait QWidget_fontInfo<RetType> {
  fn fontInfo(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QFontInfo QWidget::fontInfo();
impl<'a> /*trait*/ QWidget_fontInfo<QFontInfo> for () {
  fn fontInfo(self , rsthis: & QWidget) -> QFontInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8fontInfoEv()};
    let mut ret = unsafe {C_ZNK7QWidget8fontInfoEv(rsthis.qclsinst)};
    let mut ret1 = QFontInfo::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidget::autoFillBackground();
impl /*struct*/ QWidget {
  pub fn autoFillBackground<RetType, T: QWidget_autoFillBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoFillBackground(self);
    // return 1;
  }
}

pub trait QWidget_autoFillBackground<RetType> {
  fn autoFillBackground(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::autoFillBackground();
impl<'a> /*trait*/ QWidget_autoFillBackground<i8> for () {
  fn autoFillBackground(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18autoFillBackgroundEv()};
    let mut ret = unsafe {C_ZNK7QWidget18autoFillBackgroundEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::scroll(int dx, int dy, const QRect & );
impl /*struct*/ QWidget {
  pub fn scroll<RetType, T: QWidget_scroll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scroll(self);
    // return 1;
  }
}

pub trait QWidget_scroll<RetType> {
  fn scroll(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::scroll(int dx, int dy, const QRect & );
impl<'a> /*trait*/ QWidget_scroll<()> for (i32, i32, &'a QRect) {
  fn scroll(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6scrollEiiRK5QRect()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget6scrollEiiRK5QRect(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QFontMetrics QWidget::fontMetrics();
impl /*struct*/ QWidget {
  pub fn fontMetrics<RetType, T: QWidget_fontMetrics<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontMetrics(self);
    // return 1;
  }
}

pub trait QWidget_fontMetrics<RetType> {
  fn fontMetrics(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QFontMetrics QWidget::fontMetrics();
impl<'a> /*trait*/ QWidget_fontMetrics<QFontMetrics> for () {
  fn fontMetrics(self , rsthis: & QWidget) -> QFontMetrics {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11fontMetricsEv()};
    let mut ret = unsafe {C_ZNK7QWidget11fontMetricsEv(rsthis.qclsinst)};
    let mut ret1 = QFontMetrics::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::adjustSize();
impl /*struct*/ QWidget {
  pub fn adjustSize<RetType, T: QWidget_adjustSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.adjustSize(self);
    // return 1;
  }
}

pub trait QWidget_adjustSize<RetType> {
  fn adjustSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::adjustSize();
impl<'a> /*trait*/ QWidget_adjustSize<()> for () {
  fn adjustSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10adjustSizeEv()};
     unsafe {C_ZN7QWidget10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWidget * QWidget::previousInFocusChain();
impl /*struct*/ QWidget {
  pub fn previousInFocusChain<RetType, T: QWidget_previousInFocusChain<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.previousInFocusChain(self);
    // return 1;
  }
}

pub trait QWidget_previousInFocusChain<RetType> {
  fn previousInFocusChain(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::previousInFocusChain();
impl<'a> /*trait*/ QWidget_previousInFocusChain<QWidget> for () {
  fn previousInFocusChain(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget20previousInFocusChainEv()};
    let mut ret = unsafe {C_ZNK7QWidget20previousInFocusChainEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidget::updatesEnabled();
impl /*struct*/ QWidget {
  pub fn updatesEnabled<RetType, T: QWidget_updatesEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updatesEnabled(self);
    // return 1;
  }
}

pub trait QWidget_updatesEnabled<RetType> {
  fn updatesEnabled(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::updatesEnabled();
impl<'a> /*trait*/ QWidget_updatesEnabled<i8> for () {
  fn updatesEnabled(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14updatesEnabledEv()};
    let mut ret = unsafe {C_ZNK7QWidget14updatesEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setMaximumHeight(int maxh);
impl /*struct*/ QWidget {
  pub fn setMaximumHeight<RetType, T: QWidget_setMaximumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumHeight(self);
    // return 1;
  }
}

pub trait QWidget_setMaximumHeight<RetType> {
  fn setMaximumHeight(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setMaximumHeight(int maxh);
impl<'a> /*trait*/ QWidget_setMaximumHeight<()> for (i32) {
  fn setMaximumHeight(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMaximumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWidget16setMaximumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::showMaximized();
impl /*struct*/ QWidget {
  pub fn showMaximized<RetType, T: QWidget_showMaximized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showMaximized(self);
    // return 1;
  }
}

pub trait QWidget_showMaximized<RetType> {
  fn showMaximized(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::showMaximized();
impl<'a> /*trait*/ QWidget_showMaximized<()> for () {
  fn showMaximized(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13showMaximizedEv()};
     unsafe {C_ZN7QWidget13showMaximizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWidget::mapFrom(const QWidget * , const QPoint & );
impl /*struct*/ QWidget {
  pub fn mapFrom<RetType, T: QWidget_mapFrom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFrom(self);
    // return 1;
  }
}

pub trait QWidget_mapFrom<RetType> {
  fn mapFrom(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPoint QWidget::mapFrom(const QWidget * , const QPoint & );
impl<'a> /*trait*/ QWidget_mapFrom<QPoint> for (&'a QWidget, &'a QPoint) {
  fn mapFrom(self , rsthis: & QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7mapFromEPKS_RK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget7mapFromEPKS_RK6QPoint(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWidget::x();
impl /*struct*/ QWidget {
  pub fn x<RetType, T: QWidget_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QWidget_x<RetType> {
  fn x(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::x();
impl<'a> /*trait*/ QWidget_x<i32> for () {
  fn x(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget1xEv()};
    let mut ret = unsafe {C_ZNK7QWidget1xEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWidget::clearFocus();
impl /*struct*/ QWidget {
  pub fn clearFocus<RetType, T: QWidget_clearFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearFocus(self);
    // return 1;
  }
}

pub trait QWidget_clearFocus<RetType> {
  fn clearFocus(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::clearFocus();
impl<'a> /*trait*/ QWidget_clearFocus<()> for () {
  fn clearFocus(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10clearFocusEv()};
     unsafe {C_ZN7QWidget10clearFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QWidget * QWidget::find(WId );
impl /*struct*/ QWidget {
  pub fn find_s<RetType, T: QWidget_find_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.find_s();
    // return 1;
  }
}

pub trait QWidget_find_s<RetType> {
  fn find_s(self ) -> RetType;
}

  // proto: static QWidget * QWidget::find(WId );
impl<'a> /*trait*/ QWidget_find_s<QWidget> for (*mut i32) {
  fn find_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4findEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN7QWidget4findEi(arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPalette & QWidget::palette();
impl /*struct*/ QWidget {
  pub fn palette<RetType, T: QWidget_palette<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.palette(self);
    // return 1;
  }
}

pub trait QWidget_palette<RetType> {
  fn palette(self , rsthis: & QWidget) -> RetType;
}

  // proto:  const QPalette & QWidget::palette();
impl<'a> /*trait*/ QWidget_palette<QPalette> for () {
  fn palette(self , rsthis: & QWidget) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7paletteEv()};
    let mut ret = unsafe {C_ZNK7QWidget7paletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setSizePolicy(QSizePolicy );
impl /*struct*/ QWidget {
  pub fn setSizePolicy<RetType, T: QWidget_setSizePolicy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizePolicy(self);
    // return 1;
  }
}

pub trait QWidget_setSizePolicy<RetType> {
  fn setSizePolicy(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setSizePolicy(QSizePolicy );
impl<'a> /*trait*/ QWidget_setSizePolicy<()> for (QSizePolicy) {
  fn setSizePolicy(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setSizePolicyE11QSizePolicy()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget13setSizePolicyE11QSizePolicy(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setMask(const QRegion & );
impl /*struct*/ QWidget {
  pub fn setMask<RetType, T: QWidget_setMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMask(self);
    // return 1;
  }
}

pub trait QWidget_setMask<RetType> {
  fn setMask(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setMask(const QRegion & );
impl<'a> /*trait*/ QWidget_setMask<()> for (&'a QRegion) {
  fn setMask(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setMaskERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget7setMaskERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setMaximumWidth(int maxw);
impl /*struct*/ QWidget {
  pub fn setMaximumWidth<RetType, T: QWidget_setMaximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumWidth(self);
    // return 1;
  }
}

pub trait QWidget_setMaximumWidth<RetType> {
  fn setMaximumWidth(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setMaximumWidth(int maxw);
impl<'a> /*trait*/ QWidget_setMaximumWidth<()> for (i32) {
  fn setMaximumWidth(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15setMaximumWidthEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWidget15setMaximumWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setWindowIconText(const QString & );
impl /*struct*/ QWidget {
  pub fn setWindowIconText<RetType, T: QWidget_setWindowIconText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowIconText(self);
    // return 1;
  }
}

pub trait QWidget_setWindowIconText<RetType> {
  fn setWindowIconText(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setWindowIconText(const QString & );
impl<'a> /*trait*/ QWidget_setWindowIconText<()> for (&'a QString) {
  fn setWindowIconText(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowIconTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget17setWindowIconTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setWindowIcon(const QIcon & icon);
impl /*struct*/ QWidget {
  pub fn setWindowIcon<RetType, T: QWidget_setWindowIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowIcon(self);
    // return 1;
  }
}

pub trait QWidget_setWindowIcon<RetType> {
  fn setWindowIcon(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QWidget_setWindowIcon<()> for (&'a QIcon) {
  fn setWindowIcon(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget13setWindowIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::~QWidget();
impl /*struct*/ QWidget {
  pub fn free<RetType, T: QWidget_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWidget_free<RetType> {
  fn free(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::~QWidget();
impl<'a> /*trait*/ QWidget_free<()> for () {
  fn free(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidgetD2Ev()};
     unsafe {C_ZN7QWidgetD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::getContentsMargins(int * left, int * top, int * right, int * bottom);
impl /*struct*/ QWidget {
  pub fn getContentsMargins<RetType, T: QWidget_getContentsMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins(self);
    // return 1;
  }
}

pub trait QWidget_getContentsMargins<RetType> {
  fn getContentsMargins(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::getContentsMargins(int * left, int * top, int * right, int * bottom);
impl<'a> /*trait*/ QWidget_getContentsMargins<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn getContentsMargins(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {C_ZNK7QWidget18getContentsMarginsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QSize QWidget::minimumSizeHint();
impl /*struct*/ QWidget {
  pub fn minimumSizeHint<RetType, T: QWidget_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QWidget_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSize QWidget::minimumSizeHint();
impl<'a> /*trait*/ QWidget_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK7QWidget15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setWindowModified(bool );
impl /*struct*/ QWidget {
  pub fn setWindowModified<RetType, T: QWidget_setWindowModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowModified(self);
    // return 1;
  }
}

pub trait QWidget_setWindowModified<RetType> {
  fn setWindowModified(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setWindowModified(bool );
impl<'a> /*trait*/ QWidget_setWindowModified<()> for (i8) {
  fn setWindowModified(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowModifiedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget17setWindowModifiedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::restoreGeometry(const QByteArray & geometry);
impl /*struct*/ QWidget {
  pub fn restoreGeometry<RetType, T: QWidget_restoreGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restoreGeometry(self);
    // return 1;
  }
}

pub trait QWidget_restoreGeometry<RetType> {
  fn restoreGeometry(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::restoreGeometry(const QByteArray & geometry);
impl<'a> /*trait*/ QWidget_restoreGeometry<i8> for (&'a QByteArray) {
  fn restoreGeometry(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15restoreGeometryERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QWidget15restoreGeometryERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QLayout * QWidget::layout();
impl /*struct*/ QWidget {
  pub fn layout<RetType, T: QWidget_layout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.layout(self);
    // return 1;
  }
}

pub trait QWidget_layout<RetType> {
  fn layout(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QLayout * QWidget::layout();
impl<'a> /*trait*/ QWidget_layout<QLayout> for () {
  fn layout(self , rsthis: & QWidget) -> QLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6layoutEv()};
    let mut ret = unsafe {C_ZNK7QWidget6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QLayout::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QWidget::contentsRect();
impl /*struct*/ QWidget {
  pub fn contentsRect<RetType, T: QWidget_contentsRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentsRect(self);
    // return 1;
  }
}

pub trait QWidget_contentsRect<RetType> {
  fn contentsRect(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QRect QWidget::contentsRect();
impl<'a> /*trait*/ QWidget_contentsRect<QRect> for () {
  fn contentsRect(self , rsthis: & QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12contentsRectEv()};
    let mut ret = unsafe {C_ZNK7QWidget12contentsRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QBackingStore * QWidget::backingStore();
impl /*struct*/ QWidget {
  pub fn backingStore<RetType, T: QWidget_backingStore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backingStore(self);
    // return 1;
  }
}

pub trait QWidget_backingStore<RetType> {
  fn backingStore(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QBackingStore * QWidget::backingStore();
impl<'a> /*trait*/ QWidget_backingStore<QBackingStore> for () {
  fn backingStore(self , rsthis: & QWidget) -> QBackingStore {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12backingStoreEv()};
    let mut ret = unsafe {C_ZNK7QWidget12backingStoreEv(rsthis.qclsinst)};
    let mut ret1 = QBackingStore::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QWidget::focusProxy();
impl /*struct*/ QWidget {
  pub fn focusProxy<RetType, T: QWidget_focusProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusProxy(self);
    // return 1;
  }
}

pub trait QWidget_focusProxy<RetType> {
  fn focusProxy(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::focusProxy();
impl<'a> /*trait*/ QWidget_focusProxy<QWidget> for () {
  fn focusProxy(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10focusProxyEv()};
    let mut ret = unsafe {C_ZNK7QWidget10focusProxyEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWidget::styleSheet();
impl /*struct*/ QWidget {
  pub fn styleSheet<RetType, T: QWidget_styleSheet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.styleSheet(self);
    // return 1;
  }
}

pub trait QWidget_styleSheet<RetType> {
  fn styleSheet(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::styleSheet();
impl<'a> /*trait*/ QWidget_styleSheet<QString> for () {
  fn styleSheet(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10styleSheetEv()};
    let mut ret = unsafe {C_ZNK7QWidget10styleSheetEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QWidget::childAt(const QPoint & p);
impl<'a> /*trait*/ QWidget_childAt<QWidget> for (&'a QPoint) {
  fn childAt(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7childAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget7childAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::repaint(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_repaint<()> for (i32, i32, i32, i32) {
  fn repaint(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7repaintEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN7QWidget7repaintEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QString QWidget::whatsThis();
impl /*struct*/ QWidget {
  pub fn whatsThis<RetType, T: QWidget_whatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.whatsThis(self);
    // return 1;
  }
}

pub trait QWidget_whatsThis<RetType> {
  fn whatsThis(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::whatsThis();
impl<'a> /*trait*/ QWidget_whatsThis<QString> for () {
  fn whatsThis(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9whatsThisEv()};
    let mut ret = unsafe {C_ZNK7QWidget9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QFont & QWidget::font();
impl /*struct*/ QWidget {
  pub fn font<RetType, T: QWidget_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QWidget_font<RetType> {
  fn font(self , rsthis: & QWidget) -> RetType;
}

  // proto:  const QFont & QWidget::font();
impl<'a> /*trait*/ QWidget_font<QFont> for () {
  fn font(self , rsthis: & QWidget) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4fontEv()};
    let mut ret = unsafe {C_ZNK7QWidget4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setMinimumSize(int minw, int minh);
impl<'a> /*trait*/ QWidget_setMinimumSize<()> for (i32, i32) {
  fn setMinimumSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMinimumSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QWidget14setMinimumSizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QWidget::metaObject();
impl /*struct*/ QWidget {
  pub fn metaObject<RetType, T: QWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QWidget) -> RetType;
}

  // proto:  const QMetaObject * QWidget::metaObject();
impl<'a> /*trait*/ QWidget_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QWidget) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10metaObjectEv()};
    let mut ret = unsafe {C_ZNK7QWidget10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setMinimumHeight(int minh);
impl /*struct*/ QWidget {
  pub fn setMinimumHeight<RetType, T: QWidget_setMinimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumHeight(self);
    // return 1;
  }
}

pub trait QWidget_setMinimumHeight<RetType> {
  fn setMinimumHeight(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setMinimumHeight(int minh);
impl<'a> /*trait*/ QWidget_setMinimumHeight<()> for (i32) {
  fn setMinimumHeight(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMinimumHeightEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWidget16setMinimumHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::update(const QRegion & );
impl<'a> /*trait*/ QWidget_update<()> for (&'a QRegion) {
  fn update(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget6updateERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QWidget::windowOpacity();
impl /*struct*/ QWidget {
  pub fn windowOpacity<RetType, T: QWidget_windowOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowOpacity(self);
    // return 1;
  }
}

pub trait QWidget_windowOpacity<RetType> {
  fn windowOpacity(self , rsthis: & QWidget) -> RetType;
}

  // proto:  qreal QWidget::windowOpacity();
impl<'a> /*trait*/ QWidget_windowOpacity<f64> for () {
  fn windowOpacity(self , rsthis: & QWidget) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13windowOpacityEv()};
    let mut ret = unsafe {C_ZNK7QWidget13windowOpacityEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QRegion QWidget::childrenRegion();
impl /*struct*/ QWidget {
  pub fn childrenRegion<RetType, T: QWidget_childrenRegion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childrenRegion(self);
    // return 1;
  }
}

pub trait QWidget_childrenRegion<RetType> {
  fn childrenRegion(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QRegion QWidget::childrenRegion();
impl<'a> /*trait*/ QWidget_childrenRegion<QRegion> for () {
  fn childrenRegion(self , rsthis: & QWidget) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14childrenRegionEv()};
    let mut ret = unsafe {C_ZNK7QWidget14childrenRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setWindowFilePath(const QString & filePath);
impl /*struct*/ QWidget {
  pub fn setWindowFilePath<RetType, T: QWidget_setWindowFilePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowFilePath(self);
    // return 1;
  }
}

pub trait QWidget_setWindowFilePath<RetType> {
  fn setWindowFilePath(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setWindowFilePath(const QString & filePath);
impl<'a> /*trait*/ QWidget_setWindowFilePath<()> for (&'a QString) {
  fn setWindowFilePath(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setWindowFilePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget17setWindowFilePathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setShortcutEnabled(int id, bool enable);
impl /*struct*/ QWidget {
  pub fn setShortcutEnabled<RetType, T: QWidget_setShortcutEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setShortcutEnabled(self);
    // return 1;
  }
}

pub trait QWidget_setShortcutEnabled<RetType> {
  fn setShortcutEnabled(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setShortcutEnabled(int id, bool enable);
impl<'a> /*trait*/ QWidget_setShortcutEnabled<()> for (i32, i8) {
  fn setShortcutEnabled(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setShortcutEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN7QWidget18setShortcutEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWidget::raise();
impl /*struct*/ QWidget {
  pub fn raise<RetType, T: QWidget_raise<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.raise(self);
    // return 1;
  }
}

pub trait QWidget_raise<RetType> {
  fn raise(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::raise();
impl<'a> /*trait*/ QWidget_raise<()> for () {
  fn raise(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5raiseEv()};
     unsafe {C_ZN7QWidget5raiseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QWidget::statusTip();
impl /*struct*/ QWidget {
  pub fn statusTip<RetType, T: QWidget_statusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.statusTip(self);
    // return 1;
  }
}

pub trait QWidget_statusTip<RetType> {
  fn statusTip(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::statusTip();
impl<'a> /*trait*/ QWidget_statusTip<QString> for () {
  fn statusTip(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget9statusTipEv()};
    let mut ret = unsafe {C_ZNK7QWidget9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QWidget::childrenRect();
impl /*struct*/ QWidget {
  pub fn childrenRect<RetType, T: QWidget_childrenRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childrenRect(self);
    // return 1;
  }
}

pub trait QWidget_childrenRect<RetType> {
  fn childrenRect(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QRect QWidget::childrenRect();
impl<'a> /*trait*/ QWidget_childrenRect<QRect> for () {
  fn childrenRect(self , rsthis: & QWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12childrenRectEv()};
    let mut ret = unsafe {C_ZNK7QWidget12childrenRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setParent(QWidget * parent);
impl /*struct*/ QWidget {
  pub fn setParent<RetType, T: QWidget_setParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setParent(self);
    // return 1;
  }
}

pub trait QWidget_setParent<RetType> {
  fn setParent(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setParent(QWidget * parent);
impl<'a> /*trait*/ QWidget_setParent<()> for (&'a QWidget) {
  fn setParent(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget9setParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRegion QWidget::visibleRegion();
impl /*struct*/ QWidget {
  pub fn visibleRegion<RetType, T: QWidget_visibleRegion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visibleRegion(self);
    // return 1;
  }
}

pub trait QWidget_visibleRegion<RetType> {
  fn visibleRegion(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QRegion QWidget::visibleRegion();
impl<'a> /*trait*/ QWidget_visibleRegion<QRegion> for () {
  fn visibleRegion(self , rsthis: & QWidget) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13visibleRegionEv()};
    let mut ret = unsafe {C_ZNK7QWidget13visibleRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QLocale QWidget::locale();
impl /*struct*/ QWidget {
  pub fn locale<RetType, T: QWidget_locale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.locale(self);
    // return 1;
  }
}

pub trait QWidget_locale<RetType> {
  fn locale(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QLocale QWidget::locale();
impl<'a> /*trait*/ QWidget_locale<QLocale> for () {
  fn locale(self , rsthis: & QWidget) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6localeEv()};
    let mut ret = unsafe {C_ZNK7QWidget6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::releaseKeyboard();
impl /*struct*/ QWidget {
  pub fn releaseKeyboard<RetType, T: QWidget_releaseKeyboard<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.releaseKeyboard(self);
    // return 1;
  }
}

pub trait QWidget_releaseKeyboard<RetType> {
  fn releaseKeyboard(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::releaseKeyboard();
impl<'a> /*trait*/ QWidget_releaseKeyboard<()> for () {
  fn releaseKeyboard(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15releaseKeyboardEv()};
     unsafe {C_ZN7QWidget15releaseKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QWidget * QWidget::mouseGrabber();
impl /*struct*/ QWidget {
  pub fn mouseGrabber_s<RetType, T: QWidget_mouseGrabber_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.mouseGrabber_s();
    // return 1;
  }
}

pub trait QWidget_mouseGrabber_s<RetType> {
  fn mouseGrabber_s(self ) -> RetType;
}

  // proto: static QWidget * QWidget::mouseGrabber();
impl<'a> /*trait*/ QWidget_mouseGrabber_s<QWidget> for () {
  fn mouseGrabber_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12mouseGrabberEv()};
    let mut ret = unsafe {C_ZN7QWidget12mouseGrabberEv()};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setFixedWidth(int w);
impl /*struct*/ QWidget {
  pub fn setFixedWidth<RetType, T: QWidget_setFixedWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFixedWidth(self);
    // return 1;
  }
}

pub trait QWidget_setFixedWidth<RetType> {
  fn setFixedWidth(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setFixedWidth(int w);
impl<'a> /*trait*/ QWidget_setFixedWidth<()> for (i32) {
  fn setFixedWidth(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setFixedWidthEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWidget13setFixedWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::addAction(QAction * action);
impl /*struct*/ QWidget {
  pub fn addAction<RetType, T: QWidget_addAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addAction(self);
    // return 1;
  }
}

pub trait QWidget_addAction<RetType> {
  fn addAction(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::addAction(QAction * action);
impl<'a> /*trait*/ QWidget_addAction<()> for (&'a QAction) {
  fn addAction(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget9addActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setDisabled(bool );
impl /*struct*/ QWidget {
  pub fn setDisabled<RetType, T: QWidget_setDisabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDisabled(self);
    // return 1;
  }
}

pub trait QWidget_setDisabled<RetType> {
  fn setDisabled(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setDisabled(bool );
impl<'a> /*trait*/ QWidget_setDisabled<()> for (i8) {
  fn setDisabled(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setDisabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget11setDisabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QWidget::windowIcon();
impl /*struct*/ QWidget {
  pub fn windowIcon<RetType, T: QWidget_windowIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowIcon(self);
    // return 1;
  }
}

pub trait QWidget_windowIcon<RetType> {
  fn windowIcon(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QIcon QWidget::windowIcon();
impl<'a> /*trait*/ QWidget_windowIcon<QIcon> for () {
  fn windowIcon(self , rsthis: & QWidget) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10windowIconEv()};
    let mut ret = unsafe {C_ZNK7QWidget10windowIconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setContentsMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QWidget_setContentsMargins<()> for (i32, i32, i32, i32) {
  fn setContentsMargins(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setContentsMarginsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN7QWidget18setContentsMarginsEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QString QWidget::windowRole();
impl /*struct*/ QWidget {
  pub fn windowRole<RetType, T: QWidget_windowRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowRole(self);
    // return 1;
  }
}

pub trait QWidget_windowRole<RetType> {
  fn windowRole(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::windowRole();
impl<'a> /*trait*/ QWidget_windowRole<QString> for () {
  fn windowRole(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10windowRoleEv()};
    let mut ret = unsafe {C_ZNK7QWidget10windowRoleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setShortcutAutoRepeat(int id, bool enable);
impl /*struct*/ QWidget {
  pub fn setShortcutAutoRepeat<RetType, T: QWidget_setShortcutAutoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setShortcutAutoRepeat(self);
    // return 1;
  }
}

pub trait QWidget_setShortcutAutoRepeat<RetType> {
  fn setShortcutAutoRepeat(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setShortcutAutoRepeat(int id, bool enable);
impl<'a> /*trait*/ QWidget_setShortcutAutoRepeat<()> for (i32, i8) {
  fn setShortcutAutoRepeat(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget21setShortcutAutoRepeatEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN7QWidget21setShortcutAutoRepeatEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWidget::showFullScreen();
impl /*struct*/ QWidget {
  pub fn showFullScreen<RetType, T: QWidget_showFullScreen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showFullScreen(self);
    // return 1;
  }
}

pub trait QWidget_showFullScreen<RetType> {
  fn showFullScreen(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::showFullScreen();
impl<'a> /*trait*/ QWidget_showFullScreen<()> for () {
  fn showFullScreen(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14showFullScreenEv()};
     unsafe {C_ZN7QWidget14showFullScreenEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::grabMouse();
impl<'a> /*trait*/ QWidget_grabMouse<()> for () {
  fn grabMouse(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9grabMouseEv()};
     unsafe {C_ZN7QWidget9grabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::setMaximumSize(const QSize & );
impl<'a> /*trait*/ QWidget_setMaximumSize<()> for (&'a QSize) {
  fn setMaximumSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setMaximumSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget14setMaximumSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPoint QWidget::mapToGlobal(const QPoint & );
impl /*struct*/ QWidget {
  pub fn mapToGlobal<RetType, T: QWidget_mapToGlobal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToGlobal(self);
    // return 1;
  }
}

pub trait QWidget_mapToGlobal<RetType> {
  fn mapToGlobal(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPoint QWidget::mapToGlobal(const QPoint & );
impl<'a> /*trait*/ QWidget_mapToGlobal<QPoint> for (&'a QPoint) {
  fn mapToGlobal(self , rsthis: & QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11mapToGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget11mapToGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWidget::toolTip();
impl /*struct*/ QWidget {
  pub fn toolTip<RetType, T: QWidget_toolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QWidget_toolTip<RetType> {
  fn toolTip(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QString QWidget::toolTip();
impl<'a> /*trait*/ QWidget_toolTip<QString> for () {
  fn toolTip(self , rsthis: & QWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget7toolTipEv()};
    let mut ret = unsafe {C_ZNK7QWidget7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setWhatsThis(const QString & );
impl /*struct*/ QWidget {
  pub fn setWhatsThis<RetType, T: QWidget_setWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis(self);
    // return 1;
  }
}

pub trait QWidget_setWhatsThis<RetType> {
  fn setWhatsThis(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setWhatsThis(const QString & );
impl<'a> /*trait*/ QWidget_setWhatsThis<()> for (&'a QString) {
  fn setWhatsThis(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::resize(int w, int h);
impl<'a> /*trait*/ QWidget_resize<()> for (i32, i32) {
  fn resize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QWidget6resizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QWidget * QWidget::parentWidget();
impl /*struct*/ QWidget {
  pub fn parentWidget<RetType, T: QWidget_parentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentWidget(self);
    // return 1;
  }
}

pub trait QWidget_parentWidget<RetType> {
  fn parentWidget(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::parentWidget();
impl<'a> /*trait*/ QWidget_parentWidget<QWidget> for () {
  fn parentWidget(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12parentWidgetEv()};
    let mut ret = unsafe {C_ZNK7QWidget12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QWidget::pos();
impl /*struct*/ QWidget {
  pub fn pos<RetType, T: QWidget_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QWidget_pos<RetType> {
  fn pos(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPoint QWidget::pos();
impl<'a> /*trait*/ QWidget_pos<QPoint> for () {
  fn pos(self , rsthis: & QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget3posEv()};
    let mut ret = unsafe {C_ZNK7QWidget3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setAutoFillBackground(bool enabled);
impl /*struct*/ QWidget {
  pub fn setAutoFillBackground<RetType, T: QWidget_setAutoFillBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoFillBackground(self);
    // return 1;
  }
}

pub trait QWidget_setAutoFillBackground<RetType> {
  fn setAutoFillBackground(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setAutoFillBackground(bool enabled);
impl<'a> /*trait*/ QWidget_setAutoFillBackground<()> for (i8) {
  fn setAutoFillBackground(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget21setAutoFillBackgroundEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget21setAutoFillBackgroundEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::hasFocus();
impl /*struct*/ QWidget {
  pub fn hasFocus<RetType, T: QWidget_hasFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFocus(self);
    // return 1;
  }
}

pub trait QWidget_hasFocus<RetType> {
  fn hasFocus(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::hasFocus();
impl<'a> /*trait*/ QWidget_hasFocus<i8> for () {
  fn hasFocus(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8hasFocusEv()};
    let mut ret = unsafe {C_ZNK7QWidget8hasFocusEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QSize QWidget::baseSize();
impl /*struct*/ QWidget {
  pub fn baseSize<RetType, T: QWidget_baseSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baseSize(self);
    // return 1;
  }
}

pub trait QWidget_baseSize<RetType> {
  fn baseSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSize QWidget::baseSize();
impl<'a> /*trait*/ QWidget_baseSize<QSize> for () {
  fn baseSize(self , rsthis: & QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget8baseSizeEv()};
    let mut ret = unsafe {C_ZNK7QWidget8baseSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setMask(const QBitmap & );
impl<'a> /*trait*/ QWidget_setMask<()> for (&'a QBitmap) {
  fn setMask(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setMaskERK7QBitmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget7setMaskERK7QBitmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::ensurePolished();
impl /*struct*/ QWidget {
  pub fn ensurePolished<RetType, T: QWidget_ensurePolished<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensurePolished(self);
    // return 1;
  }
}

pub trait QWidget_ensurePolished<RetType> {
  fn ensurePolished(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::ensurePolished();
impl<'a> /*trait*/ QWidget_ensurePolished<()> for () {
  fn ensurePolished(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14ensurePolishedEv()};
     unsafe {C_ZNK7QWidget14ensurePolishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::setWindowTitle(const QString & );
impl /*struct*/ QWidget {
  pub fn setWindowTitle<RetType, T: QWidget_setWindowTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowTitle(self);
    // return 1;
  }
}

pub trait QWidget_setWindowTitle<RetType> {
  fn setWindowTitle(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setWindowTitle(const QString & );
impl<'a> /*trait*/ QWidget_setWindowTitle<()> for (&'a QString) {
  fn setWindowTitle(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget14setWindowTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget14setWindowTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QWidget::window();
impl /*struct*/ QWidget {
  pub fn window<RetType, T: QWidget_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QWidget_window<RetType> {
  fn window(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::window();
impl<'a> /*trait*/ QWidget_window<QWidget> for () {
  fn window(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6windowEv()};
    let mut ret = unsafe {C_ZNK7QWidget6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::scroll(int dx, int dy);
impl<'a> /*trait*/ QWidget_scroll<()> for (i32, i32) {
  fn scroll(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6scrollEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QWidget6scrollEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWidget::releaseShortcut(int id);
impl /*struct*/ QWidget {
  pub fn releaseShortcut<RetType, T: QWidget_releaseShortcut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.releaseShortcut(self);
    // return 1;
  }
}

pub trait QWidget_releaseShortcut<RetType> {
  fn releaseShortcut(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::releaseShortcut(int id);
impl<'a> /*trait*/ QWidget_releaseShortcut<()> for (i32) {
  fn releaseShortcut(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget15releaseShortcutEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWidget15releaseShortcutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setToolTipDuration(int msec);
impl /*struct*/ QWidget {
  pub fn setToolTipDuration<RetType, T: QWidget_setToolTipDuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTipDuration(self);
    // return 1;
  }
}

pub trait QWidget_setToolTipDuration<RetType> {
  fn setToolTipDuration(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setToolTipDuration(int msec);
impl<'a> /*trait*/ QWidget_setToolTipDuration<()> for (i32) {
  fn setToolTipDuration(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget18setToolTipDurationEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWidget18setToolTipDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setGeometry(int x, int y, int w, int h);
impl<'a> /*trait*/ QWidget_setGeometry<()> for (i32, i32, i32, i32) {
  fn setGeometry(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setGeometryEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN7QWidget11setGeometryEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QWidget::setSizeIncrement(int w, int h);
impl<'a> /*trait*/ QWidget_setSizeIncrement<()> for (i32, i32) {
  fn setSizeIncrement(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setSizeIncrementEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QWidget16setSizeIncrementEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWidget::setUpdatesEnabled(bool enable);
impl /*struct*/ QWidget {
  pub fn setUpdatesEnabled<RetType, T: QWidget_setUpdatesEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUpdatesEnabled(self);
    // return 1;
  }
}

pub trait QWidget_setUpdatesEnabled<RetType> {
  fn setUpdatesEnabled(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setUpdatesEnabled(bool enable);
impl<'a> /*trait*/ QWidget_setUpdatesEnabled<()> for (i8) {
  fn setUpdatesEnabled(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setUpdatesEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget17setUpdatesEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::lower();
impl /*struct*/ QWidget {
  pub fn lower<RetType, T: QWidget_lower<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lower(self);
    // return 1;
  }
}

pub trait QWidget_lower<RetType> {
  fn lower(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::lower();
impl<'a> /*trait*/ QWidget_lower<()> for () {
  fn lower(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5lowerEv()};
     unsafe {C_ZN7QWidget5lowerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::setMouseTracking(bool enable);
impl /*struct*/ QWidget {
  pub fn setMouseTracking<RetType, T: QWidget_setMouseTracking<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMouseTracking(self);
    // return 1;
  }
}

pub trait QWidget_setMouseTracking<RetType> {
  fn setMouseTracking(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setMouseTracking(bool enable);
impl<'a> /*trait*/ QWidget_setMouseTracking<()> for (i8) {
  fn setMouseTracking(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setMouseTrackingEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget16setMouseTrackingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setBaseSize(const QSize & );
impl<'a> /*trait*/ QWidget_setBaseSize<()> for (&'a QSize) {
  fn setBaseSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11setBaseSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget11setBaseSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::hide();
impl /*struct*/ QWidget {
  pub fn hide<RetType, T: QWidget_hide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hide(self);
    // return 1;
  }
}

pub trait QWidget_hide<RetType> {
  fn hide(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::hide();
impl<'a> /*trait*/ QWidget_hide<()> for () {
  fn hide(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4hideEv()};
     unsafe {C_ZN7QWidget4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::removeAction(QAction * action);
impl /*struct*/ QWidget {
  pub fn removeAction<RetType, T: QWidget_removeAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAction(self);
    // return 1;
  }
}

pub trait QWidget_removeAction<RetType> {
  fn removeAction(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::removeAction(QAction * action);
impl<'a> /*trait*/ QWidget_removeAction<()> for (&'a QAction) {
  fn removeAction(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget12removeActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::setFocusProxy(QWidget * );
impl /*struct*/ QWidget {
  pub fn setFocusProxy<RetType, T: QWidget_setFocusProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFocusProxy(self);
    // return 1;
  }
}

pub trait QWidget_setFocusProxy<RetType> {
  fn setFocusProxy(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setFocusProxy(QWidget * );
impl<'a> /*trait*/ QWidget_setFocusProxy<()> for (&'a QWidget) {
  fn setFocusProxy(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13setFocusProxyEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget13setFocusProxyEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::close();
impl /*struct*/ QWidget {
  pub fn close<RetType, T: QWidget_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QWidget_close<RetType> {
  fn close(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::close();
impl<'a> /*trait*/ QWidget_close<i8> for () {
  fn close(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget5closeEv()};
    let mut ret = unsafe {C_ZN7QWidget5closeEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::showMinimized();
impl /*struct*/ QWidget {
  pub fn showMinimized<RetType, T: QWidget_showMinimized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showMinimized(self);
    // return 1;
  }
}

pub trait QWidget_showMinimized<RetType> {
  fn showMinimized(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::showMinimized();
impl<'a> /*trait*/ QWidget_showMinimized<()> for () {
  fn showMinimized(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget13showMinimizedEv()};
     unsafe {C_ZN7QWidget13showMinimizedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::setFixedSize(int w, int h);
impl<'a> /*trait*/ QWidget_setFixedSize<()> for (i32, i32) {
  fn setFixedSize(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget12setFixedSizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QWidget12setFixedSizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QWidget::minimumSize();
impl /*struct*/ QWidget {
  pub fn minimumSize<RetType, T: QWidget_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QWidget_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSize QWidget::minimumSize();
impl<'a> /*trait*/ QWidget_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: & QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11minimumSizeEv()};
    let mut ret = unsafe {C_ZNK7QWidget11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setEnabled(bool );
impl /*struct*/ QWidget {
  pub fn setEnabled<RetType, T: QWidget_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QWidget_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setEnabled(bool );
impl<'a> /*trait*/ QWidget_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWidget::maximumHeight();
impl /*struct*/ QWidget {
  pub fn maximumHeight<RetType, T: QWidget_maximumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumHeight(self);
    // return 1;
  }
}

pub trait QWidget_maximumHeight<RetType> {
  fn maximumHeight(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::maximumHeight();
impl<'a> /*trait*/ QWidget_maximumHeight<i32> for () {
  fn maximumHeight(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13maximumHeightEv()};
    let mut ret = unsafe {C_ZNK7QWidget13maximumHeightEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWidget::move(int x, int y);
impl<'a> /*trait*/ QWidget_move_<()> for (i32, i32) {
  fn move_(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget4moveEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QWidget4moveEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QWidget::isAncestorOf(const QWidget * child);
impl /*struct*/ QWidget {
  pub fn isAncestorOf<RetType, T: QWidget_isAncestorOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAncestorOf(self);
    // return 1;
  }
}

pub trait QWidget_isAncestorOf<RetType> {
  fn isAncestorOf(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isAncestorOf(const QWidget * child);
impl<'a> /*trait*/ QWidget_isAncestorOf<i8> for (&'a QWidget) {
  fn isAncestorOf(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12isAncestorOfEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget12isAncestorOfEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QCursor QWidget::cursor();
impl /*struct*/ QWidget {
  pub fn cursor<RetType, T: QWidget_cursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursor(self);
    // return 1;
  }
}

pub trait QWidget_cursor<RetType> {
  fn cursor(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QCursor QWidget::cursor();
impl<'a> /*trait*/ QWidget_cursor<QCursor> for () {
  fn cursor(self , rsthis: & QWidget) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6cursorEv()};
    let mut ret = unsafe {C_ZNK7QWidget6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QWidget::mapFromGlobal(const QPoint & );
impl /*struct*/ QWidget {
  pub fn mapFromGlobal<RetType, T: QWidget_mapFromGlobal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromGlobal(self);
    // return 1;
  }
}

pub trait QWidget_mapFromGlobal<RetType> {
  fn mapFromGlobal(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QPoint QWidget::mapFromGlobal(const QPoint & );
impl<'a> /*trait*/ QWidget_mapFromGlobal<QPoint> for (&'a QPoint) {
  fn mapFromGlobal(self , rsthis: & QWidget) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13mapFromGlobalERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWidget13mapFromGlobalERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setToolTip(const QString & );
impl /*struct*/ QWidget {
  pub fn setToolTip<RetType, T: QWidget_setToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QWidget_setToolTip<RetType> {
  fn setToolTip(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setToolTip(const QString & );
impl<'a> /*trait*/ QWidget_setToolTip<()> for (&'a QString) {
  fn setToolTip(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSizePolicy QWidget::sizePolicy();
impl /*struct*/ QWidget {
  pub fn sizePolicy<RetType, T: QWidget_sizePolicy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy(self);
    // return 1;
  }
}

pub trait QWidget_sizePolicy<RetType> {
  fn sizePolicy(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSizePolicy QWidget::sizePolicy();
impl<'a> /*trait*/ QWidget_sizePolicy<QSizePolicy> for () {
  fn sizePolicy(self , rsthis: & QWidget) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10sizePolicyEv()};
    let mut ret = unsafe {C_ZNK7QWidget10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWidget::hasHeightForWidth();
impl /*struct*/ QWidget {
  pub fn hasHeightForWidth<RetType, T: QWidget_hasHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QWidget_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::hasHeightForWidth();
impl<'a> /*trait*/ QWidget_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget17hasHeightForWidthEv()};
    let mut ret = unsafe {C_ZNK7QWidget17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QGraphicsProxyWidget * QWidget::graphicsProxyWidget();
impl /*struct*/ QWidget {
  pub fn graphicsProxyWidget<RetType, T: QWidget_graphicsProxyWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.graphicsProxyWidget(self);
    // return 1;
  }
}

pub trait QWidget_graphicsProxyWidget<RetType> {
  fn graphicsProxyWidget(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QGraphicsProxyWidget * QWidget::graphicsProxyWidget();
impl<'a> /*trait*/ QWidget_graphicsProxyWidget<QGraphicsProxyWidget> for () {
  fn graphicsProxyWidget(self , rsthis: & QWidget) -> QGraphicsProxyWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget19graphicsProxyWidgetEv()};
    let mut ret = unsafe {C_ZNK7QWidget19graphicsProxyWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QGraphicsProxyWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMargins QWidget::contentsMargins();
impl /*struct*/ QWidget {
  pub fn contentsMargins<RetType, T: QWidget_contentsMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentsMargins(self);
    // return 1;
  }
}

pub trait QWidget_contentsMargins<RetType> {
  fn contentsMargins(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QMargins QWidget::contentsMargins();
impl<'a> /*trait*/ QWidget_contentsMargins<QMargins> for () {
  fn contentsMargins(self , rsthis: & QWidget) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget15contentsMarginsEv()};
    let mut ret = unsafe {C_ZNK7QWidget15contentsMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QWidget::topLevelWidget();
impl /*struct*/ QWidget {
  pub fn topLevelWidget<RetType, T: QWidget_topLevelWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topLevelWidget(self);
    // return 1;
  }
}

pub trait QWidget_topLevelWidget<RetType> {
  fn topLevelWidget(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::topLevelWidget();
impl<'a> /*trait*/ QWidget_topLevelWidget<QWidget> for () {
  fn topLevelWidget(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14topLevelWidgetEv()};
    let mut ret = unsafe {C_ZNK7QWidget14topLevelWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setLayout(QLayout * );
impl /*struct*/ QWidget {
  pub fn setLayout<RetType, T: QWidget_setLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLayout(self);
    // return 1;
  }
}

pub trait QWidget_setLayout<RetType> {
  fn setLayout(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setLayout(QLayout * );
impl<'a> /*trait*/ QWidget_setLayout<()> for (&'a QLayout) {
  fn setLayout(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setLayoutEP7QLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget9setLayoutEP7QLayout(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::underMouse();
impl /*struct*/ QWidget {
  pub fn underMouse<RetType, T: QWidget_underMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.underMouse(self);
    // return 1;
  }
}

pub trait QWidget_underMouse<RetType> {
  fn underMouse(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::underMouse();
impl<'a> /*trait*/ QWidget_underMouse<i8> for () {
  fn underMouse(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget10underMouseEv()};
    let mut ret = unsafe {C_ZNK7QWidget10underMouseEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QWidget::heightForWidth(int );
impl /*struct*/ QWidget {
  pub fn heightForWidth<RetType, T: QWidget_heightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QWidget_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::heightForWidth(int );
impl<'a> /*trait*/ QWidget_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QWidget14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setFont(const QFont & );
impl /*struct*/ QWidget {
  pub fn setFont<RetType, T: QWidget_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QWidget_setFont<RetType> {
  fn setFont(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setFont(const QFont & );
impl<'a> /*trait*/ QWidget_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QWidget::nativeParentWidget();
impl /*struct*/ QWidget {
  pub fn nativeParentWidget<RetType, T: QWidget_nativeParentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeParentWidget(self);
    // return 1;
  }
}

pub trait QWidget_nativeParentWidget<RetType> {
  fn nativeParentWidget(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QWidget * QWidget::nativeParentWidget();
impl<'a> /*trait*/ QWidget_nativeParentWidget<QWidget> for () {
  fn nativeParentWidget(self , rsthis: & QWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget18nativeParentWidgetEv()};
    let mut ret = unsafe {C_ZNK7QWidget18nativeParentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::setLocale(const QLocale & locale);
impl /*struct*/ QWidget {
  pub fn setLocale<RetType, T: QWidget_setLocale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLocale(self);
    // return 1;
  }
}

pub trait QWidget_setLocale<RetType> {
  fn setLocale(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QWidget_setLocale<()> for (&'a QLocale) {
  fn setLocale(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QWidget::height();
impl /*struct*/ QWidget {
  pub fn height<RetType, T: QWidget_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QWidget_height<RetType> {
  fn height(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::height();
impl<'a> /*trait*/ QWidget_height<i32> for () {
  fn height(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget6heightEv()};
    let mut ret = unsafe {C_ZNK7QWidget6heightEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setHidden(bool hidden);
impl /*struct*/ QWidget {
  pub fn setHidden<RetType, T: QWidget_setHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHidden(self);
    // return 1;
  }
}

pub trait QWidget_setHidden<RetType> {
  fn setHidden(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setHidden(bool hidden);
impl<'a> /*trait*/ QWidget_setHidden<()> for (i8) {
  fn setHidden(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setHiddenEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWidget9setHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QWidget::size();
impl /*struct*/ QWidget {
  pub fn size<RetType, T: QWidget_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QWidget_size<RetType> {
  fn size(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QSize QWidget::size();
impl<'a> /*trait*/ QWidget_size<QSize> for () {
  fn size(self , rsthis: & QWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget4sizeEv()};
    let mut ret = unsafe {C_ZNK7QWidget4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWidget::maximumWidth();
impl /*struct*/ QWidget {
  pub fn maximumWidth<RetType, T: QWidget_maximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth(self);
    // return 1;
  }
}

pub trait QWidget_maximumWidth<RetType> {
  fn maximumWidth(self , rsthis: & QWidget) -> RetType;
}

  // proto:  int QWidget::maximumWidth();
impl<'a> /*trait*/ QWidget_maximumWidth<i32> for () {
  fn maximumWidth(self , rsthis: & QWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget12maximumWidthEv()};
    let mut ret = unsafe {C_ZNK7QWidget12maximumWidthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QWidget::isMinimized();
impl /*struct*/ QWidget {
  pub fn isMinimized<RetType, T: QWidget_isMinimized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMinimized(self);
    // return 1;
  }
}

pub trait QWidget_isMinimized<RetType> {
  fn isMinimized(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isMinimized();
impl<'a> /*trait*/ QWidget_isMinimized<i8> for () {
  fn isMinimized(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget11isMinimizedEv()};
    let mut ret = unsafe {C_ZNK7QWidget11isMinimizedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::update();
impl<'a> /*trait*/ QWidget_update<()> for () {
  fn update(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget6updateEv()};
     unsafe {C_ZN7QWidget6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::setCursor(const QCursor & );
impl /*struct*/ QWidget {
  pub fn setCursor<RetType, T: QWidget_setCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursor(self);
    // return 1;
  }
}

pub trait QWidget_setCursor<RetType> {
  fn setCursor(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setCursor(const QCursor & );
impl<'a> /*trait*/ QWidget_setCursor<()> for (&'a QCursor) {
  fn setCursor(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStyle * QWidget::style();
impl /*struct*/ QWidget {
  pub fn style<RetType, T: QWidget_style<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.style(self);
    // return 1;
  }
}

pub trait QWidget_style<RetType> {
  fn style(self , rsthis: & QWidget) -> RetType;
}

  // proto:  QStyle * QWidget::style();
impl<'a> /*trait*/ QWidget_style<QStyle> for () {
  fn style(self , rsthis: & QWidget) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget5styleEv()};
    let mut ret = unsafe {C_ZNK7QWidget5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWidget::createWinId();
impl /*struct*/ QWidget {
  pub fn createWinId<RetType, T: QWidget_createWinId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createWinId(self);
    // return 1;
  }
}

pub trait QWidget_createWinId<RetType> {
  fn createWinId(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::createWinId();
impl<'a> /*trait*/ QWidget_createWinId<()> for () {
  fn createWinId(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11createWinIdEv()};
     unsafe {C_ZN7QWidget11createWinIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWidget::setWindowOpacity(qreal level);
impl /*struct*/ QWidget {
  pub fn setWindowOpacity<RetType, T: QWidget_setWindowOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowOpacity(self);
    // return 1;
  }
}

pub trait QWidget_setWindowOpacity<RetType> {
  fn setWindowOpacity(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setWindowOpacity(qreal level);
impl<'a> /*trait*/ QWidget_setWindowOpacity<()> for (f64) {
  fn setWindowOpacity(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget16setWindowOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN7QWidget16setWindowOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWidget::isRightToLeft();
impl /*struct*/ QWidget {
  pub fn isRightToLeft<RetType, T: QWidget_isRightToLeft<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft(self);
    // return 1;
  }
}

pub trait QWidget_isRightToLeft<RetType> {
  fn isRightToLeft(self , rsthis: & QWidget) -> RetType;
}

  // proto:  bool QWidget::isRightToLeft();
impl<'a> /*trait*/ QWidget_isRightToLeft<i8> for () {
  fn isRightToLeft(self , rsthis: & QWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWidget13isRightToLeftEv()};
    let mut ret = unsafe {C_ZNK7QWidget13isRightToLeftEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWidget::setAccessibleName(const QString & name);
impl /*struct*/ QWidget {
  pub fn setAccessibleName<RetType, T: QWidget_setAccessibleName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleName(self);
    // return 1;
  }
}

pub trait QWidget_setAccessibleName<RetType> {
  fn setAccessibleName(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::setAccessibleName(const QString & name);
impl<'a> /*trait*/ QWidget_setAccessibleName<()> for (&'a QString) {
  fn setAccessibleName(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget17setAccessibleNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWidget17setAccessibleNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWidget::unsetCursor();
impl /*struct*/ QWidget {
  pub fn unsetCursor<RetType, T: QWidget_unsetCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor(self);
    // return 1;
  }
}

pub trait QWidget_unsetCursor<RetType> {
  fn unsetCursor(self , rsthis: & QWidget) -> RetType;
}

  // proto:  void QWidget::unsetCursor();
impl<'a> /*trait*/ QWidget_unsetCursor<()> for () {
  fn unsetCursor(self , rsthis: & QWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWidget11unsetCursorEv()};
     unsafe {C_ZN7QWidget11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWidgetData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWidgetData {
    return QWidgetData{qclsinst: qthis, ..Default::default()};
  }
}
#[derive(Default)] // for QWidget_windowIconChanged
pub struct QWidget_windowIconChanged_signal{poi:u64}
impl /* struct */ QWidget {
  pub fn windowIconChanged(&self) -> QWidget_windowIconChanged_signal {
     return QWidget_windowIconChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWidget_windowIconChanged_signal {
  pub fn connect<T: QWidget_windowIconChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWidget_windowIconChanged_signal_connect {
  fn connect(self, sigthis: QWidget_windowIconChanged_signal);
}

#[derive(Default)] // for QWidget_windowTitleChanged
pub struct QWidget_windowTitleChanged_signal{poi:u64}
impl /* struct */ QWidget {
  pub fn windowTitleChanged(&self) -> QWidget_windowTitleChanged_signal {
     return QWidget_windowTitleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWidget_windowTitleChanged_signal {
  pub fn connect<T: QWidget_windowTitleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWidget_windowTitleChanged_signal_connect {
  fn connect(self, sigthis: QWidget_windowTitleChanged_signal);
}

#[derive(Default)] // for QWidget_customContextMenuRequested
pub struct QWidget_customContextMenuRequested_signal{poi:u64}
impl /* struct */ QWidget {
  pub fn customContextMenuRequested(&self) -> QWidget_customContextMenuRequested_signal {
     return QWidget_customContextMenuRequested_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWidget_customContextMenuRequested_signal {
  pub fn connect<T: QWidget_customContextMenuRequested_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWidget_customContextMenuRequested_signal_connect {
  fn connect(self, sigthis: QWidget_customContextMenuRequested_signal);
}

#[derive(Default)] // for QWidget_windowIconTextChanged
pub struct QWidget_windowIconTextChanged_signal{poi:u64}
impl /* struct */ QWidget {
  pub fn windowIconTextChanged(&self) -> QWidget_windowIconTextChanged_signal {
     return QWidget_windowIconTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWidget_windowIconTextChanged_signal {
  pub fn connect<T: QWidget_windowIconTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWidget_windowIconTextChanged_signal_connect {
  fn connect(self, sigthis: QWidget_windowIconTextChanged_signal);
}

// customContextMenuRequested(const class QPoint &)
extern fn QWidget_customContextMenuRequested_signal_connect_cb_0(rsfptr:fn(QPoint), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QPoint::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QWidget_customContextMenuRequested_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QPoint)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QPoint::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QWidget_customContextMenuRequested_signal_connect for fn(QPoint) {
  fn connect(self, sigthis: QWidget_customContextMenuRequested_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWidget_customContextMenuRequested_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWidget_SlotProxy_connect__ZN7QWidget26customContextMenuRequestedERK6QPoint(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWidget_customContextMenuRequested_signal_connect for Box<Fn(QPoint)> {
  fn connect(self, sigthis: QWidget_customContextMenuRequested_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWidget_customContextMenuRequested_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWidget_SlotProxy_connect__ZN7QWidget26customContextMenuRequestedERK6QPoint(arg0, arg1, arg2)};
  }
}
// windowIconChanged(const class QIcon &)
extern fn QWidget_windowIconChanged_signal_connect_cb_1(rsfptr:fn(QIcon), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QIcon::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QWidget_windowIconChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QIcon)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QIcon::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QWidget_windowIconChanged_signal_connect for fn(QIcon) {
  fn connect(self, sigthis: QWidget_windowIconChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWidget_windowIconChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWidget_SlotProxy_connect__ZN7QWidget17windowIconChangedERK5QIcon(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWidget_windowIconChanged_signal_connect for Box<Fn(QIcon)> {
  fn connect(self, sigthis: QWidget_windowIconChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWidget_windowIconChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWidget_SlotProxy_connect__ZN7QWidget17windowIconChangedERK5QIcon(arg0, arg1, arg2)};
  }
}
// windowIconTextChanged(const class QString &)
extern fn QWidget_windowIconTextChanged_signal_connect_cb_2(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QWidget_windowIconTextChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QWidget_windowIconTextChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QWidget_windowIconTextChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWidget_windowIconTextChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWidget_SlotProxy_connect__ZN7QWidget21windowIconTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWidget_windowIconTextChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QWidget_windowIconTextChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWidget_windowIconTextChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWidget_SlotProxy_connect__ZN7QWidget21windowIconTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
// windowTitleChanged(const class QString &)
extern fn QWidget_windowTitleChanged_signal_connect_cb_3(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QWidget_windowTitleChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QWidget_windowTitleChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QWidget_windowTitleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWidget_windowTitleChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWidget_SlotProxy_connect__ZN7QWidget18windowTitleChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWidget_windowTitleChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QWidget_windowTitleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWidget_windowTitleChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWidget_SlotProxy_connect__ZN7QWidget18windowTitleChangedERK7QString(arg0, arg1, arg2)};
  }
}
// <= body block end

