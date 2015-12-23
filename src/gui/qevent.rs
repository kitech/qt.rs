// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtGui/qevent.h
// dst-file: /src/gui/qevent.rs
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
use super::super::core::qcoreevent::QEvent; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qregion::QRegion; // 773
use super::super::core::qpoint::QPoint; // 771
use super::super::widgets::qaction::QAction; // 771
// use super::qevent::QInputEvent; // 773
use super::super::core::qpoint::QPointF; // 771
use super::super::core::qfile::QFile; // 771
use super::super::core::qurl::QUrl; // 771
use super::qtouchdevice::QTouchDevice; // 773
use super::qwindow::QWindow; // 773
use super::super::core::qobject::QObject; // 771
use super::qscreen::QScreen; // 773
// use super::qevent::QDragMoveEvent; // 773
use super::super::core::qmimedata::QMimeData; // 771
// use super::qevent::QDropEvent; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qsize::QSizeF; // 771
use super::qkeysequence::QKeySequence; // 773
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QString QWhatsThisClickedEvent::href();
  fn _ZNK22QWhatsThisClickedEvent4hrefEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWhatsThisClickedEvent::~QWhatsThisClickedEvent();
  fn _ZN22QWhatsThisClickedEventD0Ev(qthis: *mut c_void);
  // proto:  void QWhatsThisClickedEvent::QWhatsThisClickedEvent(const QString & href);
  fn _ZN22QWhatsThisClickedEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QExposeEvent::QExposeEvent(const QRegion & rgn);
  fn _ZN12QExposeEventC1ERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QRegion & QExposeEvent::region();
  fn _ZNK12QExposeEvent6regionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QExposeEvent::~QExposeEvent();
  fn _ZN12QExposeEventD0Ev(qthis: *mut c_void);
  // proto:  const QString & QInputMethodEvent::preeditString();
  fn _ZNK17QInputMethodEvent13preeditStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethodEvent::QInputMethodEvent();
  fn _ZN17QInputMethodEventC1Ev(qthis: *mut c_void);
  // proto:  int QInputMethodEvent::replacementStart();
  fn _ZNK17QInputMethodEvent16replacementStartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputMethodEvent::QInputMethodEvent(const QInputMethodEvent & other);
  fn _ZN17QInputMethodEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QString & QInputMethodEvent::commitString();
  fn _ZNK17QInputMethodEvent12commitStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
  fn _ZN17QInputMethodEvent15setCommitStringERK7QStringii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  int QInputMethodEvent::replacementLength();
  fn _ZNK17QInputMethodEvent17replacementLengthEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPoint & QHelpEvent::globalPos();
  fn _ZNK10QHelpEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QHelpEvent::globalX();
  fn _ZNK10QHelpEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPoint & QHelpEvent::pos();
  fn _ZNK10QHelpEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QHelpEvent::y();
  fn _ZNK10QHelpEvent1yEv(qthis: *mut c_void);
  // proto:  int QHelpEvent::globalY();
  fn _ZNK10QHelpEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  int QHelpEvent::x();
  fn _ZNK10QHelpEvent1xEv(qthis: *mut c_void);
  // proto:  void QHelpEvent::~QHelpEvent();
  fn _ZN10QHelpEventD0Ev(qthis: *mut c_void);
  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
  fn _ZN12QActionEventC1EiP7QActionS1_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QAction * QActionEvent::before();
  fn _ZNK12QActionEvent6beforeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAction * QActionEvent::action();
  fn _ZNK12QActionEvent6actionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QActionEvent::~QActionEvent();
  fn _ZN12QActionEventD0Ev(qthis: *mut c_void);
  // proto:  QPoint QMouseEvent::globalPos();
  fn _ZNK11QMouseEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMouseEvent::y();
  fn _ZNK11QMouseEvent1yEv(qthis: *mut c_void);
  // proto:  const QPointF & QMouseEvent::screenPos();
  fn _ZNK11QMouseEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMouseEvent::x();
  fn _ZNK11QMouseEvent1xEv(qthis: *mut c_void);
  // proto:  const QPointF & QMouseEvent::localPos();
  fn _ZNK11QMouseEvent8localPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMouseEvent::globalX();
  fn _ZNK11QMouseEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QMouseEvent::windowPos();
  fn _ZNK11QMouseEvent9windowPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMouseEvent::~QMouseEvent();
  fn _ZN11QMouseEventD0Ev(qthis: *mut c_void);
  // proto:  int QMouseEvent::globalY();
  fn _ZNK11QMouseEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QMouseEvent::pos();
  fn _ZNK11QMouseEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileOpenEvent::QFileOpenEvent(const QString & file);
  fn _ZN14QFileOpenEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFileOpenEvent::~QFileOpenEvent();
  fn _ZN14QFileOpenEventD0Ev(qthis: *mut c_void);
  // proto:  void QFileOpenEvent::QFileOpenEvent(const QUrl & url);
  fn _ZN14QFileOpenEventC1ERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QFileOpenEvent::file();
  fn _ZNK14QFileOpenEvent4fileEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QUrl QFileOpenEvent::url();
  fn _ZNK14QFileOpenEvent3urlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
  fn _ZN19QToolBarChangeEventC1Eb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
  fn _ZN19QToolBarChangeEventD0Ev(qthis: *mut c_void);
  // proto:  bool QToolBarChangeEvent::toggle();
  fn _ZNK19QToolBarChangeEvent6toggleEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTabletEvent::x();
  fn _ZNK12QTabletEvent1xEv(qthis: *mut c_void);
  // proto:  int QTabletEvent::xTilt();
  fn _ZNK12QTabletEvent5xTiltEv(qthis: *mut c_void) -> c_int;
  // proto:  qint64 QTabletEvent::uniqueId();
  fn _ZNK12QTabletEvent8uniqueIdEv(qthis: *mut c_void) -> c_longlong;
  // proto:  const QPointF & QTabletEvent::globalPosF();
  fn _ZNK12QTabletEvent10globalPosFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTabletEvent::z();
  fn _ZNK12QTabletEvent1zEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTabletEvent::y();
  fn _ZNK12QTabletEvent1yEv(qthis: *mut c_void);
  // proto:  QPoint QTabletEvent::pos();
  fn _ZNK12QTabletEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTabletEvent::rotation();
  fn _ZNK12QTabletEvent8rotationEv(qthis: *mut c_void) -> c_double;
  // proto:  QPoint QTabletEvent::globalPos();
  fn _ZNK12QTabletEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabletEvent::~QTabletEvent();
  fn _ZN12QTabletEventD0Ev(qthis: *mut c_void);
  // proto:  qreal QTabletEvent::tangentialPressure();
  fn _ZNK12QTabletEvent18tangentialPressureEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTabletEvent::hiResGlobalX();
  fn _ZNK12QTabletEvent12hiResGlobalXEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::globalY();
  fn _ZNK12QTabletEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  qreal QTabletEvent::hiResGlobalY();
  fn _ZNK12QTabletEvent12hiResGlobalYEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::globalX();
  fn _ZNK12QTabletEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QTabletEvent::posF();
  fn _ZNK12QTabletEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTabletEvent::pressure();
  fn _ZNK12QTabletEvent8pressureEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::yTilt();
  fn _ZNK12QTabletEvent5yTiltEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
  fn _ZN11QTouchEvent9setDeviceEP12QTouchDevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWindow * QTouchEvent::window();
  fn _ZNK11QTouchEvent6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTouchDevice * QTouchEvent::device();
  fn _ZNK11QTouchEvent6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QObject * QTouchEvent::target();
  fn _ZNK11QTouchEvent6targetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTouchEvent::~QTouchEvent();
  fn _ZN11QTouchEventD0Ev(qthis: *mut c_void);
  // proto:  void QTouchEvent::setWindow(QWindow * awindow);
  fn _ZN11QTouchEvent9setWindowEP7QWindow(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTouchEvent::setTarget(QObject * atarget);
  fn _ZN11QTouchEvent9setTargetEP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QScreen * QScreenOrientationChangeEvent::screen();
  fn _ZNK29QScreenOrientationChangeEvent6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScreenOrientationChangeEvent::~QScreenOrientationChangeEvent();
  fn _ZN29QScreenOrientationChangeEventD0Ev(qthis: *mut c_void);
  // proto:  void QIconDragEvent::~QIconDragEvent();
  fn _ZN14QIconDragEventD0Ev(qthis: *mut c_void);
  // proto:  void QIconDragEvent::QIconDragEvent();
  fn _ZN14QIconDragEventC1Ev(qthis: *mut c_void);
  // proto:  void QCloseEvent::~QCloseEvent();
  fn _ZN11QCloseEventD0Ev(qthis: *mut c_void);
  // proto:  void QCloseEvent::QCloseEvent();
  fn _ZN11QCloseEventC1Ev(qthis: *mut c_void);
  // proto:  void QDragEnterEvent::~QDragEnterEvent();
  fn _ZN15QDragEnterEventD0Ev(qthis: *mut c_void);
  // proto:  int QWheelEvent::x();
  fn _ZNK11QWheelEvent1xEv(qthis: *mut c_void);
  // proto:  QPoint QWheelEvent::angleDelta();
  fn _ZNK11QWheelEvent10angleDeltaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QWheelEvent::pos();
  fn _ZNK11QWheelEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWheelEvent::globalY();
  fn _ZNK11QWheelEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QWheelEvent::posF();
  fn _ZNK11QWheelEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWheelEvent::globalX();
  fn _ZNK11QWheelEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  int QWheelEvent::y();
  fn _ZNK11QWheelEvent1yEv(qthis: *mut c_void);
  // proto:  void QWheelEvent::~QWheelEvent();
  fn _ZN11QWheelEventD0Ev(qthis: *mut c_void);
  // proto:  QPoint QWheelEvent::pixelDelta();
  fn _ZNK11QWheelEvent10pixelDeltaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWheelEvent::delta();
  fn _ZNK11QWheelEvent5deltaEv(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QWheelEvent::globalPos();
  fn _ZNK11QWheelEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QWheelEvent::globalPosF();
  fn _ZNK11QWheelEvent10globalPosFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QScrollEvent::contentPos();
  fn _ZNK12QScrollEvent10contentPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QScrollEvent::overshootDistance();
  fn _ZNK12QScrollEvent17overshootDistanceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollEvent::~QScrollEvent();
  fn _ZN12QScrollEventD0Ev(qthis: *mut c_void);
  // proto:  void QHoverEvent::~QHoverEvent();
  fn _ZN11QHoverEventD0Ev(qthis: *mut c_void);
  // proto:  const QPointF & QHoverEvent::posF();
  fn _ZNK11QHoverEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QHoverEvent::oldPos();
  fn _ZNK11QHoverEvent6oldPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QHoverEvent::oldPosF();
  fn _ZNK11QHoverEvent7oldPosFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QHoverEvent::pos();
  fn _ZNK11QHoverEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDragMoveEvent::accept(const QRect & r);
  fn _ZN14QDragMoveEvent6acceptERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QDragMoveEvent::answerRect();
  fn _ZNK14QDragMoveEvent10answerRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDragMoveEvent::ignore(const QRect & r);
  fn _ZN14QDragMoveEvent6ignoreERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDragMoveEvent::ignore();
  fn _ZN14QDragMoveEvent6ignoreEv(qthis: *mut c_void);
  // proto:  void QDragMoveEvent::~QDragMoveEvent();
  fn _ZN14QDragMoveEventD0Ev(qthis: *mut c_void);
  // proto:  void QDragMoveEvent::accept();
  fn _ZN14QDragMoveEvent6acceptEv(qthis: *mut c_void);
  // proto:  void QShowEvent::~QShowEvent();
  fn _ZN10QShowEventD0Ev(qthis: *mut c_void);
  // proto:  void QShowEvent::QShowEvent();
  fn _ZN10QShowEventC1Ev(qthis: *mut c_void);
  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
  fn _ZN21QPlatformSurfaceEventD0Ev(qthis: *mut c_void);
  // proto:  void QPaintEvent::~QPaintEvent();
  fn _ZN11QPaintEventD0Ev(qthis: *mut c_void);
  // proto:  const QRect & QPaintEvent::rect();
  fn _ZNK11QPaintEvent4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
  fn _ZN11QPaintEventC1ERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QRegion & QPaintEvent::region();
  fn _ZNK11QPaintEvent6regionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEvent::QPaintEvent(const QRegion & paintRegion);
  fn _ZN11QPaintEventC1ERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QFocusEvent::lostFocus();
  fn _ZNK11QFocusEvent9lostFocusEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QFocusEvent::gotFocus();
  fn _ZNK11QFocusEvent8gotFocusEv(qthis: *mut c_void) -> c_char;
  // proto:  void QFocusEvent::~QFocusEvent();
  fn _ZN11QFocusEventD0Ev(qthis: *mut c_void);
  // proto:  const QPointF & QNativeGestureEvent::localPos();
  fn _ZNK19QNativeGestureEvent8localPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QNativeGestureEvent::screenPos();
  fn _ZNK19QNativeGestureEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPoint QNativeGestureEvent::pos();
  fn _ZNK19QNativeGestureEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPoint QNativeGestureEvent::globalPos();
  fn _ZNK19QNativeGestureEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QNativeGestureEvent::value();
  fn _ZNK19QNativeGestureEvent5valueEv(qthis: *mut c_void) -> c_double;
  // proto:  const QPointF & QNativeGestureEvent::windowPos();
  fn _ZNK19QNativeGestureEvent9windowPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QSize & QResizeEvent::oldSize();
  fn _ZNK12QResizeEvent7oldSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QSize & QResizeEvent::size();
  fn _ZNK12QResizeEvent4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QResizeEvent::~QResizeEvent();
  fn _ZN12QResizeEventD0Ev(qthis: *mut c_void);
  // proto:  void QResizeEvent::QResizeEvent(const QSize & size, const QSize & oldSize);
  fn _ZN12QResizeEventC1ERK5QSizeS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QStatusTipEvent::~QStatusTipEvent();
  fn _ZN15QStatusTipEventD0Ev(qthis: *mut c_void);
  // proto:  QString QStatusTipEvent::tip();
  fn _ZNK15QStatusTipEvent3tipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStatusTipEvent::QStatusTipEvent(const QString & tip);
  fn _ZN15QStatusTipEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QEnterEvent::y();
  fn _ZNK11QEnterEvent1yEv(qthis: *mut c_void);
  // proto:  QPoint QEnterEvent::pos();
  fn _ZNK11QEnterEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QEnterEvent::~QEnterEvent();
  fn _ZN11QEnterEventD0Ev(qthis: *mut c_void);
  // proto:  const QPointF & QEnterEvent::screenPos();
  fn _ZNK11QEnterEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QEnterEvent::localPos();
  fn _ZNK11QEnterEvent8localPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QEnterEvent::windowPos();
  fn _ZNK11QEnterEvent9windowPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QEnterEvent::globalX();
  fn _ZNK11QEnterEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  int QEnterEvent::x();
  fn _ZNK11QEnterEvent1xEv(qthis: *mut c_void);
  // proto:  QPoint QEnterEvent::globalPos();
  fn _ZNK11QEnterEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QEnterEvent::globalY();
  fn _ZNK11QEnterEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  void QEnterEvent::QEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
  fn _ZN11QEnterEventC1ERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QMoveEvent::~QMoveEvent();
  fn _ZN10QMoveEventD0Ev(qthis: *mut c_void);
  // proto:  const QPoint & QMoveEvent::oldPos();
  fn _ZNK10QMoveEvent6oldPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMoveEvent::QMoveEvent(const QPoint & pos, const QPoint & oldPos);
  fn _ZN10QMoveEventC1ERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QPoint & QMoveEvent::pos();
  fn _ZNK10QMoveEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QHideEvent::QHideEvent();
  fn _ZN10QHideEventC1Ev(qthis: *mut c_void);
  // proto:  void QHideEvent::~QHideEvent();
  fn _ZN10QHideEventD0Ev(qthis: *mut c_void);
  // proto:  void QDragLeaveEvent::~QDragLeaveEvent();
  fn _ZN15QDragLeaveEventD0Ev(qthis: *mut c_void);
  // proto:  void QDragLeaveEvent::QDragLeaveEvent();
  fn _ZN15QDragLeaveEventC1Ev(qthis: *mut c_void);
  // proto:  void QDropEvent::~QDropEvent();
  fn _ZN10QDropEventD0Ev(qthis: *mut c_void);
  // proto:  QPoint QDropEvent::pos();
  fn _ZNK10QDropEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QObject * QDropEvent::source();
  fn _ZNK10QDropEvent6sourceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMimeData * QDropEvent::mimeData();
  fn _ZNK10QDropEvent8mimeDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDropEvent::acceptProposedAction();
  fn _ZN10QDropEvent20acceptProposedActionEv(qthis: *mut c_void);
  // proto:  const QPointF & QDropEvent::posF();
  fn _ZNK10QDropEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputEvent::setTimestamp(ulong atimestamp);
  fn _ZN11QInputEvent12setTimestampEm(qthis: *mut c_void, arg0: c_ulong);
  // proto:  ulong QInputEvent::timestamp();
  fn _ZNK11QInputEvent9timestampEv(qthis: *mut c_void) -> c_ulong;
  // proto:  void QInputEvent::~QInputEvent();
  fn _ZN11QInputEventD0Ev(qthis: *mut c_void);
  // proto:  int QKeyEvent::count();
  fn _ZNK9QKeyEvent5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QKeyEvent::~QKeyEvent();
  fn _ZN9QKeyEventD0Ev(qthis: *mut c_void);
  // proto:  QString QKeyEvent::text();
  fn _ZNK9QKeyEvent4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  quint32 QKeyEvent::nativeVirtualKey();
  fn _ZNK9QKeyEvent16nativeVirtualKeyEv(qthis: *mut c_void) -> c_uint;
  // proto:  bool QKeyEvent::isAutoRepeat();
  fn _ZNK9QKeyEvent12isAutoRepeatEv(qthis: *mut c_void) -> c_char;
  // proto:  int QKeyEvent::key();
  fn _ZNK9QKeyEvent3keyEv(qthis: *mut c_void) -> c_int;
  // proto:  quint32 QKeyEvent::nativeModifiers();
  fn _ZNK9QKeyEvent15nativeModifiersEv(qthis: *mut c_void) -> c_uint;
  // proto:  quint32 QKeyEvent::nativeScanCode();
  fn _ZNK9QKeyEvent14nativeScanCodeEv(qthis: *mut c_void) -> c_uint;
  // proto:  const QPoint & QContextMenuEvent::globalPos();
  fn _ZNK17QContextMenuEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QContextMenuEvent::globalY();
  fn _ZNK17QContextMenuEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  int QContextMenuEvent::globalX();
  fn _ZNK17QContextMenuEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPoint & QContextMenuEvent::pos();
  fn _ZNK17QContextMenuEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QContextMenuEvent::y();
  fn _ZNK17QContextMenuEvent1yEv(qthis: *mut c_void);
  // proto:  int QContextMenuEvent::x();
  fn _ZNK17QContextMenuEvent1xEv(qthis: *mut c_void);
  // proto:  void QContextMenuEvent::~QContextMenuEvent();
  fn _ZN17QContextMenuEventD0Ev(qthis: *mut c_void);
  // proto:  void QScrollPrepareEvent::setContentPosRange(const QRectF & rect);
  fn _ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QScrollPrepareEvent::setContentPos(const QPointF & pos);
  fn _ZN19QScrollPrepareEvent13setContentPosERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QScrollPrepareEvent::contentPosRange();
  fn _ZNK19QScrollPrepareEvent15contentPosRangeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QScrollPrepareEvent::contentPos();
  fn _ZNK19QScrollPrepareEvent10contentPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollPrepareEvent::setViewportSize(const QSizeF & size);
  fn _ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QScrollPrepareEvent::QScrollPrepareEvent(const QPointF & startPos);
  fn _ZN19QScrollPrepareEventC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QScrollPrepareEvent::startPos();
  fn _ZNK19QScrollPrepareEvent8startPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QScrollPrepareEvent::viewportSize();
  fn _ZNK19QScrollPrepareEvent12viewportSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollPrepareEvent::~QScrollPrepareEvent();
  fn _ZN19QScrollPrepareEventD0Ev(qthis: *mut c_void);
  // proto:  const QKeySequence & QShortcutEvent::key();
  fn _ZNK14QShortcutEvent3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcutEvent::~QShortcutEvent();
  fn _ZN14QShortcutEventD0Ev(qthis: *mut c_void);
  // proto:  bool QShortcutEvent::isAmbiguous();
  fn _ZNK14QShortcutEvent11isAmbiguousEv(qthis: *mut c_void) -> c_char;
  // proto:  void QShortcutEvent::QShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
  fn _ZN14QShortcutEventC1ERK12QKeySequenceib(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_char);
  // proto:  int QShortcutEvent::shortcutId();
  fn _ZNK14QShortcutEvent10shortcutIdEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QWindowStateChangeEvent::isOverride();
  fn _ZNK23QWindowStateChangeEvent10isOverrideEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
  fn _ZN23QWindowStateChangeEventD0Ev(qthis: *mut c_void);
  // proto:  void QInputMethodQueryEvent::~QInputMethodQueryEvent();
  fn _ZN22QInputMethodQueryEventD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QWhatsThisClickedEvent)=32
pub struct QWhatsThisClickedEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QExposeEvent)=32
pub struct QExposeEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QInputMethodEvent)=1
pub struct QInputMethodEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QHelpEvent)=40
pub struct QHelpEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QActionEvent)=40
pub struct QActionEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QMouseEvent)=1
pub struct QMouseEvent {
  qbase: QInputEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QFileOpenEvent)=40
pub struct QFileOpenEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QToolBarChangeEvent)=24
pub struct QToolBarChangeEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTabletEvent)=1
pub struct QTabletEvent {
  qbase: QInputEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTouchEvent)=1
pub struct QTouchEvent {
  qbase: QInputEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QScreenOrientationChangeEvent)=40
pub struct QScreenOrientationChangeEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QIconDragEvent)=24
pub struct QIconDragEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QCloseEvent)=24
pub struct QCloseEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDragEnterEvent)=1
pub struct QDragEnterEvent {
  qbase: QDragMoveEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QWheelEvent)=1
pub struct QWheelEvent {
  qbase: QInputEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QScrollEvent)=64
pub struct QScrollEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QHoverEvent)=1
pub struct QHoverEvent {
  qbase: QInputEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDragMoveEvent)=1
pub struct QDragMoveEvent {
  qbase: QDropEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QShowEvent)=24
pub struct QShowEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QPlatformSurfaceEvent)=24
pub struct QPlatformSurfaceEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QPaintEvent)=56
pub struct QPaintEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QFocusEvent)=24
pub struct QFocusEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QNativeGestureEvent)=1
pub struct QNativeGestureEvent {
  qbase: QInputEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QResizeEvent)=40
pub struct QResizeEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QStatusTipEvent)=32
pub struct QStatusTipEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QEnterEvent)=72
pub struct QEnterEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QMoveEvent)=40
pub struct QMoveEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QHideEvent)=24
pub struct QHideEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDragLeaveEvent)=24
pub struct QDragLeaveEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QDropEvent)=1
pub struct QDropEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QInputEvent)=1
pub struct QInputEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QApplicationStateChangeEvent)=24
pub struct QApplicationStateChangeEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QKeyEvent)=1
pub struct QKeyEvent {
  qbase: QInputEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QContextMenuEvent)=1
pub struct QContextMenuEvent {
  qbase: QInputEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QScrollPrepareEvent)=112
pub struct QScrollPrepareEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QShortcutEvent)=40
pub struct QShortcutEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QWindowStateChangeEvent)=1
pub struct QWindowStateChangeEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QInputMethodQueryEvent)=1
pub struct QInputMethodQueryEvent {
  qbase: QEvent,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWhatsThisClickedEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QWhatsThisClickedEvent {
    return QWhatsThisClickedEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QWhatsThisClickedEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QWhatsThisClickedEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  QString QWhatsThisClickedEvent::href();
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn href<RetType, T: QWhatsThisClickedEvent_href<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.href(self);
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_href<RetType> {
  fn href(self , rsthis: & QWhatsThisClickedEvent) -> RetType;
}

  // proto:  QString QWhatsThisClickedEvent::href();
impl<'a> /*trait*/ QWhatsThisClickedEvent_href<QString> for () {
  fn href(self , rsthis: & QWhatsThisClickedEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QWhatsThisClickedEvent4hrefEv()};
    let mut ret = unsafe {_ZNK22QWhatsThisClickedEvent4hrefEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWhatsThisClickedEvent::~QWhatsThisClickedEvent();
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn Free<RetType, T: QWhatsThisClickedEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_Free<RetType> {
  fn Free(self , rsthis: & QWhatsThisClickedEvent) -> RetType;
}

  // proto:  void QWhatsThisClickedEvent::~QWhatsThisClickedEvent();
impl<'a> /*trait*/ QWhatsThisClickedEvent_Free<()> for () {
  fn Free(self , rsthis: & QWhatsThisClickedEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QWhatsThisClickedEventD0Ev()};
     unsafe {_ZN22QWhatsThisClickedEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWhatsThisClickedEvent::QWhatsThisClickedEvent(const QString & href);
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn New<T: QWhatsThisClickedEvent_New>(value: T) -> QWhatsThisClickedEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_New {
  fn New(self) -> QWhatsThisClickedEvent;
}

  // proto:  void QWhatsThisClickedEvent::QWhatsThisClickedEvent(const QString & href);
impl<'a> /*trait*/ QWhatsThisClickedEvent_New for (&'a QString) {
  fn New(self) -> QWhatsThisClickedEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QWhatsThisClickedEventC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QWhatsThisClickedEventC1ERK7QString(qthis, arg0)};
    let rsthis = QWhatsThisClickedEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QExposeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QExposeEvent {
    return QExposeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QExposeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QExposeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QExposeEvent::QExposeEvent(const QRegion & rgn);
impl /*struct*/ QExposeEvent {
  pub fn New<T: QExposeEvent_New>(value: T) -> QExposeEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QExposeEvent_New {
  fn New(self) -> QExposeEvent;
}

  // proto:  void QExposeEvent::QExposeEvent(const QRegion & rgn);
impl<'a> /*trait*/ QExposeEvent_New for (&'a QRegion) {
  fn New(self) -> QExposeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QExposeEventC1ERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QExposeEventC1ERK7QRegion(qthis, arg0)};
    let rsthis = QExposeEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QRegion & QExposeEvent::region();
impl /*struct*/ QExposeEvent {
  pub fn region<RetType, T: QExposeEvent_region<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.region(self);
    // return 1;
  }
}

pub trait QExposeEvent_region<RetType> {
  fn region(self , rsthis: & QExposeEvent) -> RetType;
}

  // proto:  const QRegion & QExposeEvent::region();
impl<'a> /*trait*/ QExposeEvent_region<QRegion> for () {
  fn region(self , rsthis: & QExposeEvent) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QExposeEvent6regionEv()};
    let mut ret = unsafe {_ZNK12QExposeEvent6regionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QExposeEvent::~QExposeEvent();
impl /*struct*/ QExposeEvent {
  pub fn Free<RetType, T: QExposeEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QExposeEvent_Free<RetType> {
  fn Free(self , rsthis: & QExposeEvent) -> RetType;
}

  // proto:  void QExposeEvent::~QExposeEvent();
impl<'a> /*trait*/ QExposeEvent_Free<()> for () {
  fn Free(self , rsthis: & QExposeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QExposeEventD0Ev()};
     unsafe {_ZN12QExposeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethodEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QInputMethodEvent {
    return QInputMethodEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QInputMethodEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QInputMethodEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  const QString & QInputMethodEvent::preeditString();
impl /*struct*/ QInputMethodEvent {
  pub fn preeditString<RetType, T: QInputMethodEvent_preeditString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preeditString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_preeditString<RetType> {
  fn preeditString(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  const QString & QInputMethodEvent::preeditString();
impl<'a> /*trait*/ QInputMethodEvent_preeditString<QString> for () {
  fn preeditString(self , rsthis: & QInputMethodEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent13preeditStringEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent13preeditStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputMethodEvent::QInputMethodEvent();
impl /*struct*/ QInputMethodEvent {
  pub fn New<T: QInputMethodEvent_New>(value: T) -> QInputMethodEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QInputMethodEvent_New {
  fn New(self) -> QInputMethodEvent;
}

  // proto:  void QInputMethodEvent::QInputMethodEvent();
impl<'a> /*trait*/ QInputMethodEvent_New for () {
  fn New(self) -> QInputMethodEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC1Ev()};
    unsafe {_ZN17QInputMethodEventC1Ev(qthis)};
    let rsthis = QInputMethodEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QInputMethodEvent::replacementStart();
impl /*struct*/ QInputMethodEvent {
  pub fn replacementStart<RetType, T: QInputMethodEvent_replacementStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.replacementStart(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_replacementStart<RetType> {
  fn replacementStart(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  int QInputMethodEvent::replacementStart();
impl<'a> /*trait*/ QInputMethodEvent_replacementStart<i32> for () {
  fn replacementStart(self , rsthis: & QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent16replacementStartEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent16replacementStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QInputMethodEvent::QInputMethodEvent(const QInputMethodEvent & other);
impl<'a> /*trait*/ QInputMethodEvent_New for (&'a QInputMethodEvent) {
  fn New(self) -> QInputMethodEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QInputMethodEventC1ERKS_(qthis, arg0)};
    let rsthis = QInputMethodEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QString & QInputMethodEvent::commitString();
impl /*struct*/ QInputMethodEvent {
  pub fn commitString<RetType, T: QInputMethodEvent_commitString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.commitString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_commitString<RetType> {
  fn commitString(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  const QString & QInputMethodEvent::commitString();
impl<'a> /*trait*/ QInputMethodEvent_commitString<QString> for () {
  fn commitString(self , rsthis: & QInputMethodEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent12commitStringEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent12commitStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
impl /*struct*/ QInputMethodEvent {
  pub fn setCommitString<RetType, T: QInputMethodEvent_setCommitString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCommitString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_setCommitString<RetType> {
  fn setCommitString(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
impl<'a> /*trait*/ QInputMethodEvent_setCommitString<()> for (&'a QString, i32, i32) {
  fn setCommitString(self , rsthis: & QInputMethodEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEvent15setCommitStringERK7QStringii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN17QInputMethodEvent15setCommitStringERK7QStringii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  int QInputMethodEvent::replacementLength();
impl /*struct*/ QInputMethodEvent {
  pub fn replacementLength<RetType, T: QInputMethodEvent_replacementLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.replacementLength(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_replacementLength<RetType> {
  fn replacementLength(self , rsthis: & QInputMethodEvent) -> RetType;
}

  // proto:  int QInputMethodEvent::replacementLength();
impl<'a> /*trait*/ QInputMethodEvent_replacementLength<i32> for () {
  fn replacementLength(self , rsthis: & QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent17replacementLengthEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent17replacementLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QHelpEvent {
    return QHelpEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QHelpEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QHelpEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  const QPoint & QHelpEvent::globalPos();
impl /*struct*/ QHelpEvent {
  pub fn globalPos<RetType, T: QHelpEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  const QPoint & QHelpEvent::globalPos();
impl<'a> /*trait*/ QHelpEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QHelpEvent::globalX();
impl /*struct*/ QHelpEvent {
  pub fn globalX<RetType, T: QHelpEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  int QHelpEvent::globalX();
impl<'a> /*trait*/ QHelpEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalXEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QPoint & QHelpEvent::pos();
impl /*struct*/ QHelpEvent {
  pub fn pos<RetType, T: QHelpEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QHelpEvent_pos<RetType> {
  fn pos(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  const QPoint & QHelpEvent::pos();
impl<'a> /*trait*/ QHelpEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent3posEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QHelpEvent::y();
impl /*struct*/ QHelpEvent {
  pub fn y<RetType, T: QHelpEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QHelpEvent_y<RetType> {
  fn y(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  int QHelpEvent::y();
impl<'a> /*trait*/ QHelpEvent_y<()> for () {
  fn y(self , rsthis: & QHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1yEv()};
     unsafe {_ZNK10QHelpEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QHelpEvent::globalY();
impl /*struct*/ QHelpEvent {
  pub fn globalY<RetType, T: QHelpEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  int QHelpEvent::globalY();
impl<'a> /*trait*/ QHelpEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalYEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QHelpEvent::x();
impl /*struct*/ QHelpEvent {
  pub fn x<RetType, T: QHelpEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QHelpEvent_x<RetType> {
  fn x(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  int QHelpEvent::x();
impl<'a> /*trait*/ QHelpEvent_x<()> for () {
  fn x(self , rsthis: & QHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1xEv()};
     unsafe {_ZNK10QHelpEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHelpEvent::~QHelpEvent();
impl /*struct*/ QHelpEvent {
  pub fn Free<RetType, T: QHelpEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QHelpEvent_Free<RetType> {
  fn Free(self , rsthis: & QHelpEvent) -> RetType;
}

  // proto:  void QHelpEvent::~QHelpEvent();
impl<'a> /*trait*/ QHelpEvent_Free<()> for () {
  fn Free(self , rsthis: & QHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QHelpEventD0Ev()};
     unsafe {_ZN10QHelpEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QActionEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QActionEvent {
    return QActionEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QActionEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QActionEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
impl /*struct*/ QActionEvent {
  pub fn New<T: QActionEvent_New>(value: T) -> QActionEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QActionEvent_New {
  fn New(self) -> QActionEvent;
}

  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
impl<'a> /*trait*/ QActionEvent_New for (i32, &'a QAction, &'a QAction) {
  fn New(self) -> QActionEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QActionEventC1EiP7QActionS1_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionEventC1EiP7QActionS1_(qthis, arg0, arg1, arg2)};
    let rsthis = QActionEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAction * QActionEvent::before();
impl /*struct*/ QActionEvent {
  pub fn before<RetType, T: QActionEvent_before<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.before(self);
    // return 1;
  }
}

pub trait QActionEvent_before<RetType> {
  fn before(self , rsthis: & QActionEvent) -> RetType;
}

  // proto:  QAction * QActionEvent::before();
impl<'a> /*trait*/ QActionEvent_before<QAction> for () {
  fn before(self , rsthis: & QActionEvent) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QActionEvent6beforeEv()};
    let mut ret = unsafe {_ZNK12QActionEvent6beforeEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QActionEvent::action();
impl /*struct*/ QActionEvent {
  pub fn action<RetType, T: QActionEvent_action<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.action(self);
    // return 1;
  }
}

pub trait QActionEvent_action<RetType> {
  fn action(self , rsthis: & QActionEvent) -> RetType;
}

  // proto:  QAction * QActionEvent::action();
impl<'a> /*trait*/ QActionEvent_action<QAction> for () {
  fn action(self , rsthis: & QActionEvent) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QActionEvent6actionEv()};
    let mut ret = unsafe {_ZNK12QActionEvent6actionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QActionEvent::~QActionEvent();
impl /*struct*/ QActionEvent {
  pub fn Free<RetType, T: QActionEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QActionEvent_Free<RetType> {
  fn Free(self , rsthis: & QActionEvent) -> RetType;
}

  // proto:  void QActionEvent::~QActionEvent();
impl<'a> /*trait*/ QActionEvent_Free<()> for () {
  fn Free(self , rsthis: & QActionEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QActionEventD0Ev()};
     unsafe {_ZN12QActionEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QMouseEvent {
    return QMouseEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QMouseEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QMouseEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  QPoint QMouseEvent::globalPos();
impl /*struct*/ QMouseEvent {
  pub fn globalPos<RetType, T: QMouseEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  QPoint QMouseEvent::globalPos();
impl<'a> /*trait*/ QMouseEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::y();
impl /*struct*/ QMouseEvent {
  pub fn y<RetType, T: QMouseEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QMouseEvent_y<RetType> {
  fn y(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::y();
impl<'a> /*trait*/ QMouseEvent_y<()> for () {
  fn y(self , rsthis: & QMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1yEv()};
     unsafe {_ZNK11QMouseEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::screenPos();
impl /*struct*/ QMouseEvent {
  pub fn screenPos<RetType, T: QMouseEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::screenPos();
impl<'a> /*trait*/ QMouseEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: & QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::x();
impl /*struct*/ QMouseEvent {
  pub fn x<RetType, T: QMouseEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QMouseEvent_x<RetType> {
  fn x(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::x();
impl<'a> /*trait*/ QMouseEvent_x<()> for () {
  fn x(self , rsthis: & QMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1xEv()};
     unsafe {_ZNK11QMouseEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::localPos();
impl /*struct*/ QMouseEvent {
  pub fn localPos<RetType, T: QMouseEvent_localPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_localPos<RetType> {
  fn localPos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::localPos();
impl<'a> /*trait*/ QMouseEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: & QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent8localPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::globalX();
impl /*struct*/ QMouseEvent {
  pub fn globalX<RetType, T: QMouseEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::globalX();
impl<'a> /*trait*/ QMouseEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::windowPos();
impl /*struct*/ QMouseEvent {
  pub fn windowPos<RetType, T: QMouseEvent_windowPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::windowPos();
impl<'a> /*trait*/ QMouseEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: & QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMouseEvent::~QMouseEvent();
impl /*struct*/ QMouseEvent {
  pub fn Free<RetType, T: QMouseEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QMouseEvent_Free<RetType> {
  fn Free(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  void QMouseEvent::~QMouseEvent();
impl<'a> /*trait*/ QMouseEvent_Free<()> for () {
  fn Free(self , rsthis: & QMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMouseEventD0Ev()};
     unsafe {_ZN11QMouseEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QMouseEvent::globalY();
impl /*struct*/ QMouseEvent {
  pub fn globalY<RetType, T: QMouseEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::globalY();
impl<'a> /*trait*/ QMouseEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalYEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPoint QMouseEvent::pos();
impl /*struct*/ QMouseEvent {
  pub fn pos<RetType, T: QMouseEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QMouseEvent_pos<RetType> {
  fn pos(self , rsthis: & QMouseEvent) -> RetType;
}

  // proto:  QPoint QMouseEvent::pos();
impl<'a> /*trait*/ QMouseEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent3posEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileOpenEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QFileOpenEvent {
    return QFileOpenEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QFileOpenEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QFileOpenEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QFileOpenEvent::QFileOpenEvent(const QString & file);
impl /*struct*/ QFileOpenEvent {
  pub fn New<T: QFileOpenEvent_New>(value: T) -> QFileOpenEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFileOpenEvent_New {
  fn New(self) -> QFileOpenEvent;
}

  // proto:  void QFileOpenEvent::QFileOpenEvent(const QString & file);
impl<'a> /*trait*/ QFileOpenEvent_New for (&'a QString) {
  fn New(self) -> QFileOpenEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QFileOpenEventC1ERK7QString(qthis, arg0)};
    let rsthis = QFileOpenEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileOpenEvent::~QFileOpenEvent();
impl /*struct*/ QFileOpenEvent {
  pub fn Free<RetType, T: QFileOpenEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_Free<RetType> {
  fn Free(self , rsthis: & QFileOpenEvent) -> RetType;
}

  // proto:  void QFileOpenEvent::~QFileOpenEvent();
impl<'a> /*trait*/ QFileOpenEvent_Free<()> for () {
  fn Free(self , rsthis: & QFileOpenEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventD0Ev()};
     unsafe {_ZN14QFileOpenEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFileOpenEvent::QFileOpenEvent(const QUrl & url);
impl<'a> /*trait*/ QFileOpenEvent_New for (&'a QUrl) {
  fn New(self) -> QFileOpenEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventC1ERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QFileOpenEventC1ERK4QUrl(qthis, arg0)};
    let rsthis = QFileOpenEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QFileOpenEvent::file();
impl /*struct*/ QFileOpenEvent {
  pub fn file<RetType, T: QFileOpenEvent_file<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.file(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_file<RetType> {
  fn file(self , rsthis: & QFileOpenEvent) -> RetType;
}

  // proto:  QString QFileOpenEvent::file();
impl<'a> /*trait*/ QFileOpenEvent_file<QString> for () {
  fn file(self , rsthis: & QFileOpenEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QFileOpenEvent4fileEv()};
    let mut ret = unsafe {_ZNK14QFileOpenEvent4fileEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QFileOpenEvent::url();
impl /*struct*/ QFileOpenEvent {
  pub fn url<RetType, T: QFileOpenEvent_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_url<RetType> {
  fn url(self , rsthis: & QFileOpenEvent) -> RetType;
}

  // proto:  QUrl QFileOpenEvent::url();
impl<'a> /*trait*/ QFileOpenEvent_url<QUrl> for () {
  fn url(self , rsthis: & QFileOpenEvent) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QFileOpenEvent3urlEv()};
    let mut ret = unsafe {_ZNK14QFileOpenEvent3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QToolBarChangeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QToolBarChangeEvent {
    return QToolBarChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QToolBarChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QToolBarChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
impl /*struct*/ QToolBarChangeEvent {
  pub fn New<T: QToolBarChangeEvent_New>(value: T) -> QToolBarChangeEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBarChangeEvent_New {
  fn New(self) -> QToolBarChangeEvent;
}

  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
impl<'a> /*trait*/ QToolBarChangeEvent_New for (i8) {
  fn New(self) -> QToolBarChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventC1Eb()};
    let arg0 = self  as c_char;
    unsafe {_ZN19QToolBarChangeEventC1Eb(qthis, arg0)};
    let rsthis = QToolBarChangeEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
impl /*struct*/ QToolBarChangeEvent {
  pub fn Free<RetType, T: QToolBarChangeEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QToolBarChangeEvent_Free<RetType> {
  fn Free(self , rsthis: & QToolBarChangeEvent) -> RetType;
}

  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
impl<'a> /*trait*/ QToolBarChangeEvent_Free<()> for () {
  fn Free(self , rsthis: & QToolBarChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventD0Ev()};
     unsafe {_ZN19QToolBarChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QToolBarChangeEvent::toggle();
impl /*struct*/ QToolBarChangeEvent {
  pub fn toggle<RetType, T: QToolBarChangeEvent_toggle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggle(self);
    // return 1;
  }
}

pub trait QToolBarChangeEvent_toggle<RetType> {
  fn toggle(self , rsthis: & QToolBarChangeEvent) -> RetType;
}

  // proto:  bool QToolBarChangeEvent::toggle();
impl<'a> /*trait*/ QToolBarChangeEvent_toggle<i8> for () {
  fn toggle(self , rsthis: & QToolBarChangeEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QToolBarChangeEvent6toggleEv()};
    let mut ret = unsafe {_ZNK19QToolBarChangeEvent6toggleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QTabletEvent {
    return QTabletEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTabletEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QTabletEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  int QTabletEvent::x();
impl /*struct*/ QTabletEvent {
  pub fn x<RetType, T: QTabletEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QTabletEvent_x<RetType> {
  fn x(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::x();
impl<'a> /*trait*/ QTabletEvent_x<()> for () {
  fn x(self , rsthis: & QTabletEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1xEv()};
     unsafe {_ZNK12QTabletEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTabletEvent::xTilt();
impl /*struct*/ QTabletEvent {
  pub fn xTilt<RetType, T: QTabletEvent_xTilt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xTilt(self);
    // return 1;
  }
}

pub trait QTabletEvent_xTilt<RetType> {
  fn xTilt(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::xTilt();
impl<'a> /*trait*/ QTabletEvent_xTilt<i32> for () {
  fn xTilt(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5xTiltEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent5xTiltEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qint64 QTabletEvent::uniqueId();
impl /*struct*/ QTabletEvent {
  pub fn uniqueId<RetType, T: QTabletEvent_uniqueId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.uniqueId(self);
    // return 1;
  }
}

pub trait QTabletEvent_uniqueId<RetType> {
  fn uniqueId(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qint64 QTabletEvent::uniqueId();
impl<'a> /*trait*/ QTabletEvent_uniqueId<i64> for () {
  fn uniqueId(self , rsthis: & QTabletEvent) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8uniqueIdEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8uniqueIdEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  const QPointF & QTabletEvent::globalPosF();
impl /*struct*/ QTabletEvent {
  pub fn globalPosF<RetType, T: QTabletEvent_globalPosF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPosF(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalPosF<RetType> {
  fn globalPosF(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  const QPointF & QTabletEvent::globalPosF();
impl<'a> /*trait*/ QTabletEvent_globalPosF<QPointF> for () {
  fn globalPosF(self , rsthis: & QTabletEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent10globalPosFEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent10globalPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTabletEvent::z();
impl /*struct*/ QTabletEvent {
  pub fn z<RetType, T: QTabletEvent_z<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.z(self);
    // return 1;
  }
}

pub trait QTabletEvent_z<RetType> {
  fn z(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::z();
impl<'a> /*trait*/ QTabletEvent_z<i32> for () {
  fn z(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1zEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent1zEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTabletEvent::y();
impl /*struct*/ QTabletEvent {
  pub fn y<RetType, T: QTabletEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QTabletEvent_y<RetType> {
  fn y(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::y();
impl<'a> /*trait*/ QTabletEvent_y<()> for () {
  fn y(self , rsthis: & QTabletEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1yEv()};
     unsafe {_ZNK12QTabletEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QTabletEvent::pos();
impl /*struct*/ QTabletEvent {
  pub fn pos<RetType, T: QTabletEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QTabletEvent_pos<RetType> {
  fn pos(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  QPoint QTabletEvent::pos();
impl<'a> /*trait*/ QTabletEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QTabletEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent3posEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::rotation();
impl /*struct*/ QTabletEvent {
  pub fn rotation<RetType, T: QTabletEvent_rotation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotation(self);
    // return 1;
  }
}

pub trait QTabletEvent_rotation<RetType> {
  fn rotation(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::rotation();
impl<'a> /*trait*/ QTabletEvent_rotation<f64> for () {
  fn rotation(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8rotationEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8rotationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPoint QTabletEvent::globalPos();
impl /*struct*/ QTabletEvent {
  pub fn globalPos<RetType, T: QTabletEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  QPoint QTabletEvent::globalPos();
impl<'a> /*trait*/ QTabletEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QTabletEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabletEvent::~QTabletEvent();
impl /*struct*/ QTabletEvent {
  pub fn Free<RetType, T: QTabletEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTabletEvent_Free<RetType> {
  fn Free(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  void QTabletEvent::~QTabletEvent();
impl<'a> /*trait*/ QTabletEvent_Free<()> for () {
  fn Free(self , rsthis: & QTabletEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTabletEventD0Ev()};
     unsafe {_ZN12QTabletEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::tangentialPressure();
impl /*struct*/ QTabletEvent {
  pub fn tangentialPressure<RetType, T: QTabletEvent_tangentialPressure<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tangentialPressure(self);
    // return 1;
  }
}

pub trait QTabletEvent_tangentialPressure<RetType> {
  fn tangentialPressure(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::tangentialPressure();
impl<'a> /*trait*/ QTabletEvent_tangentialPressure<f64> for () {
  fn tangentialPressure(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent18tangentialPressureEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent18tangentialPressureEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::hiResGlobalX();
impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalX<RetType, T: QTabletEvent_hiResGlobalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hiResGlobalX(self);
    // return 1;
  }
}

pub trait QTabletEvent_hiResGlobalX<RetType> {
  fn hiResGlobalX(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::hiResGlobalX();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalX<f64> for () {
  fn hiResGlobalX(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalXEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent12hiResGlobalXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QTabletEvent::globalY();
impl /*struct*/ QTabletEvent {
  pub fn globalY<RetType, T: QTabletEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::globalY();
impl<'a> /*trait*/ QTabletEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalYEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::hiResGlobalY();
impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalY<RetType, T: QTabletEvent_hiResGlobalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hiResGlobalY(self);
    // return 1;
  }
}

pub trait QTabletEvent_hiResGlobalY<RetType> {
  fn hiResGlobalY(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::hiResGlobalY();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalY<f64> for () {
  fn hiResGlobalY(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalYEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent12hiResGlobalYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QTabletEvent::globalX();
impl /*struct*/ QTabletEvent {
  pub fn globalX<RetType, T: QTabletEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::globalX();
impl<'a> /*trait*/ QTabletEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalXEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QPointF & QTabletEvent::posF();
impl /*struct*/ QTabletEvent {
  pub fn posF<RetType, T: QTabletEvent_posF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QTabletEvent_posF<RetType> {
  fn posF(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  const QPointF & QTabletEvent::posF();
impl<'a> /*trait*/ QTabletEvent_posF<QPointF> for () {
  fn posF(self , rsthis: & QTabletEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent4posFEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::pressure();
impl /*struct*/ QTabletEvent {
  pub fn pressure<RetType, T: QTabletEvent_pressure<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pressure(self);
    // return 1;
  }
}

pub trait QTabletEvent_pressure<RetType> {
  fn pressure(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::pressure();
impl<'a> /*trait*/ QTabletEvent_pressure<f64> for () {
  fn pressure(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8pressureEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8pressureEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QTabletEvent::yTilt();
impl /*struct*/ QTabletEvent {
  pub fn yTilt<RetType, T: QTabletEvent_yTilt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.yTilt(self);
    // return 1;
  }
}

pub trait QTabletEvent_yTilt<RetType> {
  fn yTilt(self , rsthis: & QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::yTilt();
impl<'a> /*trait*/ QTabletEvent_yTilt<i32> for () {
  fn yTilt(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5yTiltEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent5yTiltEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTouchEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QTouchEvent {
    return QTouchEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTouchEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QTouchEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
impl /*struct*/ QTouchEvent {
  pub fn setDevice<RetType, T: QTouchEvent_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QTouchEvent_setDevice<RetType> {
  fn setDevice(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
impl<'a> /*trait*/ QTouchEvent_setDevice<()> for (&'a QTouchDevice) {
  fn setDevice(self , rsthis: & QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setDeviceEP12QTouchDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTouchEvent9setDeviceEP12QTouchDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWindow * QTouchEvent::window();
impl /*struct*/ QTouchEvent {
  pub fn window<RetType, T: QTouchEvent_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QTouchEvent_window<RetType> {
  fn window(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  QWindow * QTouchEvent::window();
impl<'a> /*trait*/ QTouchEvent_window<QWindow> for () {
  fn window(self , rsthis: & QTouchEvent) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6windowEv()};
    let mut ret = unsafe {_ZNK11QTouchEvent6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QTouchDevice * QTouchEvent::device();
impl /*struct*/ QTouchEvent {
  pub fn device<RetType, T: QTouchEvent_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QTouchEvent_device<RetType> {
  fn device(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  QTouchDevice * QTouchEvent::device();
impl<'a> /*trait*/ QTouchEvent_device<QTouchDevice> for () {
  fn device(self , rsthis: & QTouchEvent) -> QTouchDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6deviceEv()};
    let mut ret = unsafe {_ZNK11QTouchEvent6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QTouchDevice::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QTouchEvent::target();
impl /*struct*/ QTouchEvent {
  pub fn target<RetType, T: QTouchEvent_target<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.target(self);
    // return 1;
  }
}

pub trait QTouchEvent_target<RetType> {
  fn target(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  QObject * QTouchEvent::target();
impl<'a> /*trait*/ QTouchEvent_target<QObject> for () {
  fn target(self , rsthis: & QTouchEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6targetEv()};
    let mut ret = unsafe {_ZNK11QTouchEvent6targetEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTouchEvent::~QTouchEvent();
impl /*struct*/ QTouchEvent {
  pub fn Free<RetType, T: QTouchEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTouchEvent_Free<RetType> {
  fn Free(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  void QTouchEvent::~QTouchEvent();
impl<'a> /*trait*/ QTouchEvent_Free<()> for () {
  fn Free(self , rsthis: & QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEventD0Ev()};
     unsafe {_ZN11QTouchEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTouchEvent::setWindow(QWindow * awindow);
impl /*struct*/ QTouchEvent {
  pub fn setWindow<RetType, T: QTouchEvent_setWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindow(self);
    // return 1;
  }
}

pub trait QTouchEvent_setWindow<RetType> {
  fn setWindow(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  void QTouchEvent::setWindow(QWindow * awindow);
impl<'a> /*trait*/ QTouchEvent_setWindow<()> for (&'a QWindow) {
  fn setWindow(self , rsthis: & QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setWindowEP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTouchEvent9setWindowEP7QWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTouchEvent::setTarget(QObject * atarget);
impl /*struct*/ QTouchEvent {
  pub fn setTarget<RetType, T: QTouchEvent_setTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTarget(self);
    // return 1;
  }
}

pub trait QTouchEvent_setTarget<RetType> {
  fn setTarget(self , rsthis: & QTouchEvent) -> RetType;
}

  // proto:  void QTouchEvent::setTarget(QObject * atarget);
impl<'a> /*trait*/ QTouchEvent_setTarget<()> for (&'a QObject) {
  fn setTarget(self , rsthis: & QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setTargetEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTouchEvent9setTargetEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QScreenOrientationChangeEvent {
    return QScreenOrientationChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QScreenOrientationChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QScreenOrientationChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  QScreen * QScreenOrientationChangeEvent::screen();
impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn screen<RetType, T: QScreenOrientationChangeEvent_screen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QScreenOrientationChangeEvent_screen<RetType> {
  fn screen(self , rsthis: & QScreenOrientationChangeEvent) -> RetType;
}

  // proto:  QScreen * QScreenOrientationChangeEvent::screen();
impl<'a> /*trait*/ QScreenOrientationChangeEvent_screen<QScreen> for () {
  fn screen(self , rsthis: & QScreenOrientationChangeEvent) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QScreenOrientationChangeEvent6screenEv()};
    let mut ret = unsafe {_ZNK29QScreenOrientationChangeEvent6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScreenOrientationChangeEvent::~QScreenOrientationChangeEvent();
impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn Free<RetType, T: QScreenOrientationChangeEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QScreenOrientationChangeEvent_Free<RetType> {
  fn Free(self , rsthis: & QScreenOrientationChangeEvent) -> RetType;
}

  // proto:  void QScreenOrientationChangeEvent::~QScreenOrientationChangeEvent();
impl<'a> /*trait*/ QScreenOrientationChangeEvent_Free<()> for () {
  fn Free(self , rsthis: & QScreenOrientationChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QScreenOrientationChangeEventD0Ev()};
     unsafe {_ZN29QScreenOrientationChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIconDragEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QIconDragEvent {
    return QIconDragEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QIconDragEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QIconDragEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QIconDragEvent::~QIconDragEvent();
impl /*struct*/ QIconDragEvent {
  pub fn Free<RetType, T: QIconDragEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QIconDragEvent_Free<RetType> {
  fn Free(self , rsthis: & QIconDragEvent) -> RetType;
}

  // proto:  void QIconDragEvent::~QIconDragEvent();
impl<'a> /*trait*/ QIconDragEvent_Free<()> for () {
  fn Free(self , rsthis: & QIconDragEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventD0Ev()};
     unsafe {_ZN14QIconDragEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QIconDragEvent::QIconDragEvent();
impl /*struct*/ QIconDragEvent {
  pub fn New<T: QIconDragEvent_New>(value: T) -> QIconDragEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QIconDragEvent_New {
  fn New(self) -> QIconDragEvent;
}

  // proto:  void QIconDragEvent::QIconDragEvent();
impl<'a> /*trait*/ QIconDragEvent_New for () {
  fn New(self) -> QIconDragEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventC1Ev()};
    unsafe {_ZN14QIconDragEventC1Ev(qthis)};
    let rsthis = QIconDragEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCloseEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QCloseEvent {
    return QCloseEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QCloseEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QCloseEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QCloseEvent::~QCloseEvent();
impl /*struct*/ QCloseEvent {
  pub fn Free<RetType, T: QCloseEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QCloseEvent_Free<RetType> {
  fn Free(self , rsthis: & QCloseEvent) -> RetType;
}

  // proto:  void QCloseEvent::~QCloseEvent();
impl<'a> /*trait*/ QCloseEvent_Free<()> for () {
  fn Free(self , rsthis: & QCloseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QCloseEventD0Ev()};
     unsafe {_ZN11QCloseEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCloseEvent::QCloseEvent();
impl /*struct*/ QCloseEvent {
  pub fn New<T: QCloseEvent_New>(value: T) -> QCloseEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QCloseEvent_New {
  fn New(self) -> QCloseEvent;
}

  // proto:  void QCloseEvent::QCloseEvent();
impl<'a> /*trait*/ QCloseEvent_New for () {
  fn New(self) -> QCloseEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QCloseEventC1Ev()};
    unsafe {_ZN11QCloseEventC1Ev(qthis)};
    let rsthis = QCloseEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDragEnterEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QDragEnterEvent {
    return QDragEnterEvent{qbase: QDragMoveEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDragEnterEvent {
  type Target = QDragMoveEvent;

  fn deref(&self) -> &QDragMoveEvent {
    return & self.qbase;
  }
}
impl AsRef<QDragMoveEvent> for QDragEnterEvent {
  fn as_ref(& self) -> & QDragMoveEvent {
    return & self.qbase;
  }
}
  // proto:  void QDragEnterEvent::~QDragEnterEvent();
impl /*struct*/ QDragEnterEvent {
  pub fn Free<RetType, T: QDragEnterEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDragEnterEvent_Free<RetType> {
  fn Free(self , rsthis: & QDragEnterEvent) -> RetType;
}

  // proto:  void QDragEnterEvent::~QDragEnterEvent();
impl<'a> /*trait*/ QDragEnterEvent_Free<()> for () {
  fn Free(self , rsthis: & QDragEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragEnterEventD0Ev()};
     unsafe {_ZN15QDragEnterEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QWheelEvent {
    return QWheelEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QWheelEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QWheelEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  int QWheelEvent::x();
impl /*struct*/ QWheelEvent {
  pub fn x<RetType, T: QWheelEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QWheelEvent_x<RetType> {
  fn x(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::x();
impl<'a> /*trait*/ QWheelEvent_x<()> for () {
  fn x(self , rsthis: & QWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1xEv()};
     unsafe {_ZNK11QWheelEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWheelEvent::angleDelta();
impl /*struct*/ QWheelEvent {
  pub fn angleDelta<RetType, T: QWheelEvent_angleDelta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.angleDelta(self);
    // return 1;
  }
}

pub trait QWheelEvent_angleDelta<RetType> {
  fn angleDelta(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  QPoint QWheelEvent::angleDelta();
impl<'a> /*trait*/ QWheelEvent_angleDelta<QPoint> for () {
  fn angleDelta(self , rsthis: & QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10angleDeltaEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent10angleDeltaEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QWheelEvent::pos();
impl /*struct*/ QWheelEvent {
  pub fn pos<RetType, T: QWheelEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QWheelEvent_pos<RetType> {
  fn pos(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  QPoint QWheelEvent::pos();
impl<'a> /*trait*/ QWheelEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent3posEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWheelEvent::globalY();
impl /*struct*/ QWheelEvent {
  pub fn globalY<RetType, T: QWheelEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::globalY();
impl<'a> /*trait*/ QWheelEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalYEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QPointF & QWheelEvent::posF();
impl /*struct*/ QWheelEvent {
  pub fn posF<RetType, T: QWheelEvent_posF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QWheelEvent_posF<RetType> {
  fn posF(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  const QPointF & QWheelEvent::posF();
impl<'a> /*trait*/ QWheelEvent_posF<QPointF> for () {
  fn posF(self , rsthis: & QWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent4posFEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWheelEvent::globalX();
impl /*struct*/ QWheelEvent {
  pub fn globalX<RetType, T: QWheelEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::globalX();
impl<'a> /*trait*/ QWheelEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QWheelEvent::y();
impl /*struct*/ QWheelEvent {
  pub fn y<RetType, T: QWheelEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QWheelEvent_y<RetType> {
  fn y(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::y();
impl<'a> /*trait*/ QWheelEvent_y<()> for () {
  fn y(self , rsthis: & QWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1yEv()};
     unsafe {_ZNK11QWheelEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWheelEvent::~QWheelEvent();
impl /*struct*/ QWheelEvent {
  pub fn Free<RetType, T: QWheelEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QWheelEvent_Free<RetType> {
  fn Free(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  void QWheelEvent::~QWheelEvent();
impl<'a> /*trait*/ QWheelEvent_Free<()> for () {
  fn Free(self , rsthis: & QWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWheelEventD0Ev()};
     unsafe {_ZN11QWheelEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QWheelEvent::pixelDelta();
impl /*struct*/ QWheelEvent {
  pub fn pixelDelta<RetType, T: QWheelEvent_pixelDelta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixelDelta(self);
    // return 1;
  }
}

pub trait QWheelEvent_pixelDelta<RetType> {
  fn pixelDelta(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  QPoint QWheelEvent::pixelDelta();
impl<'a> /*trait*/ QWheelEvent_pixelDelta<QPoint> for () {
  fn pixelDelta(self , rsthis: & QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10pixelDeltaEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent10pixelDeltaEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QWheelEvent::delta();
impl /*struct*/ QWheelEvent {
  pub fn delta<RetType, T: QWheelEvent_delta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.delta(self);
    // return 1;
  }
}

pub trait QWheelEvent_delta<RetType> {
  fn delta(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  int QWheelEvent::delta();
impl<'a> /*trait*/ QWheelEvent_delta<i32> for () {
  fn delta(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent5deltaEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent5deltaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPoint QWheelEvent::globalPos();
impl /*struct*/ QWheelEvent {
  pub fn globalPos<RetType, T: QWheelEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  QPoint QWheelEvent::globalPos();
impl<'a> /*trait*/ QWheelEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QWheelEvent::globalPosF();
impl /*struct*/ QWheelEvent {
  pub fn globalPosF<RetType, T: QWheelEvent_globalPosF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPosF(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalPosF<RetType> {
  fn globalPosF(self , rsthis: & QWheelEvent) -> RetType;
}

  // proto:  const QPointF & QWheelEvent::globalPosF();
impl<'a> /*trait*/ QWheelEvent_globalPosF<QPointF> for () {
  fn globalPosF(self , rsthis: & QWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10globalPosFEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent10globalPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QScrollEvent {
    return QScrollEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QScrollEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QScrollEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  QPointF QScrollEvent::contentPos();
impl /*struct*/ QScrollEvent {
  pub fn contentPos<RetType, T: QScrollEvent_contentPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentPos(self);
    // return 1;
  }
}

pub trait QScrollEvent_contentPos<RetType> {
  fn contentPos(self , rsthis: & QScrollEvent) -> RetType;
}

  // proto:  QPointF QScrollEvent::contentPos();
impl<'a> /*trait*/ QScrollEvent_contentPos<QPointF> for () {
  fn contentPos(self , rsthis: & QScrollEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent10contentPosEv()};
    let mut ret = unsafe {_ZNK12QScrollEvent10contentPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QScrollEvent::overshootDistance();
impl /*struct*/ QScrollEvent {
  pub fn overshootDistance<RetType, T: QScrollEvent_overshootDistance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.overshootDistance(self);
    // return 1;
  }
}

pub trait QScrollEvent_overshootDistance<RetType> {
  fn overshootDistance(self , rsthis: & QScrollEvent) -> RetType;
}

  // proto:  QPointF QScrollEvent::overshootDistance();
impl<'a> /*trait*/ QScrollEvent_overshootDistance<QPointF> for () {
  fn overshootDistance(self , rsthis: & QScrollEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent17overshootDistanceEv()};
    let mut ret = unsafe {_ZNK12QScrollEvent17overshootDistanceEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollEvent::~QScrollEvent();
impl /*struct*/ QScrollEvent {
  pub fn Free<RetType, T: QScrollEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QScrollEvent_Free<RetType> {
  fn Free(self , rsthis: & QScrollEvent) -> RetType;
}

  // proto:  void QScrollEvent::~QScrollEvent();
impl<'a> /*trait*/ QScrollEvent_Free<()> for () {
  fn Free(self , rsthis: & QScrollEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZN12QScrollEventD0Ev()};
     unsafe {_ZN12QScrollEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QHoverEvent {
    return QHoverEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QHoverEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QHoverEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  void QHoverEvent::~QHoverEvent();
impl /*struct*/ QHoverEvent {
  pub fn Free<RetType, T: QHoverEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QHoverEvent_Free<RetType> {
  fn Free(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  void QHoverEvent::~QHoverEvent();
impl<'a> /*trait*/ QHoverEvent_Free<()> for () {
  fn Free(self , rsthis: & QHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHoverEventD0Ev()};
     unsafe {_ZN11QHoverEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QHoverEvent::posF();
impl /*struct*/ QHoverEvent {
  pub fn posF<RetType, T: QHoverEvent_posF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QHoverEvent_posF<RetType> {
  fn posF(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  const QPointF & QHoverEvent::posF();
impl<'a> /*trait*/ QHoverEvent_posF<QPointF> for () {
  fn posF(self , rsthis: & QHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent4posFEv()};
    let mut ret = unsafe {_ZNK11QHoverEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QHoverEvent::oldPos();
impl /*struct*/ QHoverEvent {
  pub fn oldPos<RetType, T: QHoverEvent_oldPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldPos(self);
    // return 1;
  }
}

pub trait QHoverEvent_oldPos<RetType> {
  fn oldPos(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  QPoint QHoverEvent::oldPos();
impl<'a> /*trait*/ QHoverEvent_oldPos<QPoint> for () {
  fn oldPos(self , rsthis: & QHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent6oldPosEv()};
    let mut ret = unsafe {_ZNK11QHoverEvent6oldPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QHoverEvent::oldPosF();
impl /*struct*/ QHoverEvent {
  pub fn oldPosF<RetType, T: QHoverEvent_oldPosF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldPosF(self);
    // return 1;
  }
}

pub trait QHoverEvent_oldPosF<RetType> {
  fn oldPosF(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  const QPointF & QHoverEvent::oldPosF();
impl<'a> /*trait*/ QHoverEvent_oldPosF<QPointF> for () {
  fn oldPosF(self , rsthis: & QHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent7oldPosFEv()};
    let mut ret = unsafe {_ZNK11QHoverEvent7oldPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QHoverEvent::pos();
impl /*struct*/ QHoverEvent {
  pub fn pos<RetType, T: QHoverEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QHoverEvent_pos<RetType> {
  fn pos(self , rsthis: & QHoverEvent) -> RetType;
}

  // proto:  QPoint QHoverEvent::pos();
impl<'a> /*trait*/ QHoverEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent3posEv()};
    let mut ret = unsafe {_ZNK11QHoverEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDragMoveEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QDragMoveEvent {
    return QDragMoveEvent{qbase: QDropEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDragMoveEvent {
  type Target = QDropEvent;

  fn deref(&self) -> &QDropEvent {
    return & self.qbase;
  }
}
impl AsRef<QDropEvent> for QDragMoveEvent {
  fn as_ref(& self) -> & QDropEvent {
    return & self.qbase;
  }
}
  // proto:  void QDragMoveEvent::accept(const QRect & r);
impl /*struct*/ QDragMoveEvent {
  pub fn accept<RetType, T: QDragMoveEvent_accept<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_accept<RetType> {
  fn accept(self , rsthis: & QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::accept(const QRect & r);
impl<'a> /*trait*/ QDragMoveEvent_accept<()> for (&'a QRect) {
  fn accept(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6acceptERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDragMoveEvent6acceptERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QDragMoveEvent::answerRect();
impl /*struct*/ QDragMoveEvent {
  pub fn answerRect<RetType, T: QDragMoveEvent_answerRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.answerRect(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_answerRect<RetType> {
  fn answerRect(self , rsthis: & QDragMoveEvent) -> RetType;
}

  // proto:  QRect QDragMoveEvent::answerRect();
impl<'a> /*trait*/ QDragMoveEvent_answerRect<QRect> for () {
  fn answerRect(self , rsthis: & QDragMoveEvent) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDragMoveEvent10answerRectEv()};
    let mut ret = unsafe {_ZNK14QDragMoveEvent10answerRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::ignore(const QRect & r);
impl /*struct*/ QDragMoveEvent {
  pub fn ignore<RetType, T: QDragMoveEvent_ignore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ignore(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_ignore<RetType> {
  fn ignore(self , rsthis: & QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::ignore(const QRect & r);
impl<'a> /*trait*/ QDragMoveEvent_ignore<()> for (&'a QRect) {
  fn ignore(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6ignoreERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDragMoveEvent6ignoreERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::ignore();
impl<'a> /*trait*/ QDragMoveEvent_ignore<()> for () {
  fn ignore(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6ignoreEv()};
     unsafe {_ZN14QDragMoveEvent6ignoreEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::~QDragMoveEvent();
impl /*struct*/ QDragMoveEvent {
  pub fn Free<RetType, T: QDragMoveEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_Free<RetType> {
  fn Free(self , rsthis: & QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::~QDragMoveEvent();
impl<'a> /*trait*/ QDragMoveEvent_Free<()> for () {
  fn Free(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEventD0Ev()};
     unsafe {_ZN14QDragMoveEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::accept();
impl<'a> /*trait*/ QDragMoveEvent_accept<()> for () {
  fn accept(self , rsthis: & QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6acceptEv()};
     unsafe {_ZN14QDragMoveEvent6acceptEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShowEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QShowEvent {
    return QShowEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QShowEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QShowEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QShowEvent::~QShowEvent();
impl /*struct*/ QShowEvent {
  pub fn Free<RetType, T: QShowEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QShowEvent_Free<RetType> {
  fn Free(self , rsthis: & QShowEvent) -> RetType;
}

  // proto:  void QShowEvent::~QShowEvent();
impl<'a> /*trait*/ QShowEvent_Free<()> for () {
  fn Free(self , rsthis: & QShowEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventD0Ev()};
     unsafe {_ZN10QShowEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QShowEvent::QShowEvent();
impl /*struct*/ QShowEvent {
  pub fn New<T: QShowEvent_New>(value: T) -> QShowEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QShowEvent_New {
  fn New(self) -> QShowEvent;
}

  // proto:  void QShowEvent::QShowEvent();
impl<'a> /*trait*/ QShowEvent_New for () {
  fn New(self) -> QShowEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventC1Ev()};
    unsafe {_ZN10QShowEventC1Ev(qthis)};
    let rsthis = QShowEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPlatformSurfaceEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QPlatformSurfaceEvent {
    return QPlatformSurfaceEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QPlatformSurfaceEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QPlatformSurfaceEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
impl /*struct*/ QPlatformSurfaceEvent {
  pub fn Free<RetType, T: QPlatformSurfaceEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPlatformSurfaceEvent_Free<RetType> {
  fn Free(self , rsthis: & QPlatformSurfaceEvent) -> RetType;
}

  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
impl<'a> /*trait*/ QPlatformSurfaceEvent_Free<()> for () {
  fn Free(self , rsthis: & QPlatformSurfaceEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPlatformSurfaceEventD0Ev()};
     unsafe {_ZN21QPlatformSurfaceEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPaintEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QPaintEvent {
    return QPaintEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QPaintEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QPaintEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QPaintEvent::~QPaintEvent();
impl /*struct*/ QPaintEvent {
  pub fn Free<RetType, T: QPaintEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPaintEvent_Free<RetType> {
  fn Free(self , rsthis: & QPaintEvent) -> RetType;
}

  // proto:  void QPaintEvent::~QPaintEvent();
impl<'a> /*trait*/ QPaintEvent_Free<()> for () {
  fn Free(self , rsthis: & QPaintEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventD0Ev()};
     unsafe {_ZN11QPaintEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QRect & QPaintEvent::rect();
impl /*struct*/ QPaintEvent {
  pub fn rect<RetType, T: QPaintEvent_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QPaintEvent_rect<RetType> {
  fn rect(self , rsthis: & QPaintEvent) -> RetType;
}

  // proto:  const QRect & QPaintEvent::rect();
impl<'a> /*trait*/ QPaintEvent_rect<QRect> for () {
  fn rect(self , rsthis: & QPaintEvent) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent4rectEv()};
    let mut ret = unsafe {_ZNK11QPaintEvent4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
impl /*struct*/ QPaintEvent {
  pub fn New<T: QPaintEvent_New>(value: T) -> QPaintEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEvent_New {
  fn New(self) -> QPaintEvent;
}

  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
impl<'a> /*trait*/ QPaintEvent_New for (&'a QRect) {
  fn New(self) -> QPaintEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPaintEventC1ERK5QRect(qthis, arg0)};
    let rsthis = QPaintEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QRegion & QPaintEvent::region();
impl /*struct*/ QPaintEvent {
  pub fn region<RetType, T: QPaintEvent_region<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.region(self);
    // return 1;
  }
}

pub trait QPaintEvent_region<RetType> {
  fn region(self , rsthis: & QPaintEvent) -> RetType;
}

  // proto:  const QRegion & QPaintEvent::region();
impl<'a> /*trait*/ QPaintEvent_region<QRegion> for () {
  fn region(self , rsthis: & QPaintEvent) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent6regionEv()};
    let mut ret = unsafe {_ZNK11QPaintEvent6regionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEvent::QPaintEvent(const QRegion & paintRegion);
impl<'a> /*trait*/ QPaintEvent_New for (&'a QRegion) {
  fn New(self) -> QPaintEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPaintEventC1ERK7QRegion(qthis, arg0)};
    let rsthis = QPaintEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFocusEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QFocusEvent {
    return QFocusEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QFocusEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QFocusEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  bool QFocusEvent::lostFocus();
impl /*struct*/ QFocusEvent {
  pub fn lostFocus<RetType, T: QFocusEvent_lostFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lostFocus(self);
    // return 1;
  }
}

pub trait QFocusEvent_lostFocus<RetType> {
  fn lostFocus(self , rsthis: & QFocusEvent) -> RetType;
}

  // proto:  bool QFocusEvent::lostFocus();
impl<'a> /*trait*/ QFocusEvent_lostFocus<i8> for () {
  fn lostFocus(self , rsthis: & QFocusEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent9lostFocusEv()};
    let mut ret = unsafe {_ZNK11QFocusEvent9lostFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QFocusEvent::gotFocus();
impl /*struct*/ QFocusEvent {
  pub fn gotFocus<RetType, T: QFocusEvent_gotFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.gotFocus(self);
    // return 1;
  }
}

pub trait QFocusEvent_gotFocus<RetType> {
  fn gotFocus(self , rsthis: & QFocusEvent) -> RetType;
}

  // proto:  bool QFocusEvent::gotFocus();
impl<'a> /*trait*/ QFocusEvent_gotFocus<i8> for () {
  fn gotFocus(self , rsthis: & QFocusEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent8gotFocusEv()};
    let mut ret = unsafe {_ZNK11QFocusEvent8gotFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QFocusEvent::~QFocusEvent();
impl /*struct*/ QFocusEvent {
  pub fn Free<RetType, T: QFocusEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFocusEvent_Free<RetType> {
  fn Free(self , rsthis: & QFocusEvent) -> RetType;
}

  // proto:  void QFocusEvent::~QFocusEvent();
impl<'a> /*trait*/ QFocusEvent_Free<()> for () {
  fn Free(self , rsthis: & QFocusEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusEventD0Ev()};
     unsafe {_ZN11QFocusEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QNativeGestureEvent {
    return QNativeGestureEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QNativeGestureEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QNativeGestureEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  const QPointF & QNativeGestureEvent::localPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn localPos<RetType, T: QNativeGestureEvent_localPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_localPos<RetType> {
  fn localPos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPointF & QNativeGestureEvent::localPos();
impl<'a> /*trait*/ QNativeGestureEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: & QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent8localPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QNativeGestureEvent::screenPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn screenPos<RetType, T: QNativeGestureEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPointF & QNativeGestureEvent::screenPos();
impl<'a> /*trait*/ QNativeGestureEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: & QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPoint QNativeGestureEvent::pos();
impl /*struct*/ QNativeGestureEvent {
  pub fn pos<RetType, T: QNativeGestureEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_pos<RetType> {
  fn pos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPoint QNativeGestureEvent::pos();
impl<'a> /*trait*/ QNativeGestureEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QNativeGestureEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent3posEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPoint QNativeGestureEvent::globalPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn globalPos<RetType, T: QNativeGestureEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPoint QNativeGestureEvent::globalPos();
impl<'a> /*trait*/ QNativeGestureEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QNativeGestureEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QNativeGestureEvent::value();
impl /*struct*/ QNativeGestureEvent {
  pub fn value<RetType, T: QNativeGestureEvent_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_value<RetType> {
  fn value(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  qreal QNativeGestureEvent::value();
impl<'a> /*trait*/ QNativeGestureEvent_value<f64> for () {
  fn value(self , rsthis: & QNativeGestureEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent5valueEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent5valueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  const QPointF & QNativeGestureEvent::windowPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn windowPos<RetType, T: QNativeGestureEvent_windowPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: & QNativeGestureEvent) -> RetType;
}

  // proto:  const QPointF & QNativeGestureEvent::windowPos();
impl<'a> /*trait*/ QNativeGestureEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: & QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QResizeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QResizeEvent {
    return QResizeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QResizeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QResizeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  const QSize & QResizeEvent::oldSize();
impl /*struct*/ QResizeEvent {
  pub fn oldSize<RetType, T: QResizeEvent_oldSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldSize(self);
    // return 1;
  }
}

pub trait QResizeEvent_oldSize<RetType> {
  fn oldSize(self , rsthis: & QResizeEvent) -> RetType;
}

  // proto:  const QSize & QResizeEvent::oldSize();
impl<'a> /*trait*/ QResizeEvent_oldSize<QSize> for () {
  fn oldSize(self , rsthis: & QResizeEvent) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QResizeEvent7oldSizeEv()};
    let mut ret = unsafe {_ZNK12QResizeEvent7oldSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QSize & QResizeEvent::size();
impl /*struct*/ QResizeEvent {
  pub fn size<RetType, T: QResizeEvent_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QResizeEvent_size<RetType> {
  fn size(self , rsthis: & QResizeEvent) -> RetType;
}

  // proto:  const QSize & QResizeEvent::size();
impl<'a> /*trait*/ QResizeEvent_size<QSize> for () {
  fn size(self , rsthis: & QResizeEvent) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QResizeEvent4sizeEv()};
    let mut ret = unsafe {_ZNK12QResizeEvent4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QResizeEvent::~QResizeEvent();
impl /*struct*/ QResizeEvent {
  pub fn Free<RetType, T: QResizeEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QResizeEvent_Free<RetType> {
  fn Free(self , rsthis: & QResizeEvent) -> RetType;
}

  // proto:  void QResizeEvent::~QResizeEvent();
impl<'a> /*trait*/ QResizeEvent_Free<()> for () {
  fn Free(self , rsthis: & QResizeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventD0Ev()};
     unsafe {_ZN12QResizeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QResizeEvent::QResizeEvent(const QSize & size, const QSize & oldSize);
impl /*struct*/ QResizeEvent {
  pub fn New<T: QResizeEvent_New>(value: T) -> QResizeEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QResizeEvent_New {
  fn New(self) -> QResizeEvent;
}

  // proto:  void QResizeEvent::QResizeEvent(const QSize & size, const QSize & oldSize);
impl<'a> /*trait*/ QResizeEvent_New for (&'a QSize, &'a QSize) {
  fn New(self) -> QResizeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventC1ERK5QSizeS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QResizeEventC1ERK5QSizeS2_(qthis, arg0, arg1)};
    let rsthis = QResizeEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStatusTipEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QStatusTipEvent {
    return QStatusTipEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStatusTipEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QStatusTipEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QStatusTipEvent::~QStatusTipEvent();
impl /*struct*/ QStatusTipEvent {
  pub fn Free<RetType, T: QStatusTipEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QStatusTipEvent_Free<RetType> {
  fn Free(self , rsthis: & QStatusTipEvent) -> RetType;
}

  // proto:  void QStatusTipEvent::~QStatusTipEvent();
impl<'a> /*trait*/ QStatusTipEvent_Free<()> for () {
  fn Free(self , rsthis: & QStatusTipEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStatusTipEventD0Ev()};
     unsafe {_ZN15QStatusTipEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QStatusTipEvent::tip();
impl /*struct*/ QStatusTipEvent {
  pub fn tip<RetType, T: QStatusTipEvent_tip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tip(self);
    // return 1;
  }
}

pub trait QStatusTipEvent_tip<RetType> {
  fn tip(self , rsthis: & QStatusTipEvent) -> RetType;
}

  // proto:  QString QStatusTipEvent::tip();
impl<'a> /*trait*/ QStatusTipEvent_tip<QString> for () {
  fn tip(self , rsthis: & QStatusTipEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QStatusTipEvent3tipEv()};
    let mut ret = unsafe {_ZNK15QStatusTipEvent3tipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStatusTipEvent::QStatusTipEvent(const QString & tip);
impl /*struct*/ QStatusTipEvent {
  pub fn New<T: QStatusTipEvent_New>(value: T) -> QStatusTipEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStatusTipEvent_New {
  fn New(self) -> QStatusTipEvent;
}

  // proto:  void QStatusTipEvent::QStatusTipEvent(const QString & tip);
impl<'a> /*trait*/ QStatusTipEvent_New for (&'a QString) {
  fn New(self) -> QStatusTipEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStatusTipEventC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QStatusTipEventC1ERK7QString(qthis, arg0)};
    let rsthis = QStatusTipEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QEnterEvent {
    return QEnterEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QEnterEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QEnterEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  int QEnterEvent::y();
impl /*struct*/ QEnterEvent {
  pub fn y<RetType, T: QEnterEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QEnterEvent_y<RetType> {
  fn y(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  int QEnterEvent::y();
impl<'a> /*trait*/ QEnterEvent_y<()> for () {
  fn y(self , rsthis: & QEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1yEv()};
     unsafe {_ZNK11QEnterEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QEnterEvent::pos();
impl /*struct*/ QEnterEvent {
  pub fn pos<RetType, T: QEnterEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QEnterEvent_pos<RetType> {
  fn pos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  QPoint QEnterEvent::pos();
impl<'a> /*trait*/ QEnterEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QEnterEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent3posEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QEnterEvent::~QEnterEvent();
impl /*struct*/ QEnterEvent {
  pub fn Free<RetType, T: QEnterEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QEnterEvent_Free<RetType> {
  fn Free(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  void QEnterEvent::~QEnterEvent();
impl<'a> /*trait*/ QEnterEvent_Free<()> for () {
  fn Free(self , rsthis: & QEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventD0Ev()};
     unsafe {_ZN11QEnterEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QEnterEvent::screenPos();
impl /*struct*/ QEnterEvent {
  pub fn screenPos<RetType, T: QEnterEvent_screenPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  const QPointF & QEnterEvent::screenPos();
impl<'a> /*trait*/ QEnterEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: & QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QEnterEvent::localPos();
impl /*struct*/ QEnterEvent {
  pub fn localPos<RetType, T: QEnterEvent_localPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_localPos<RetType> {
  fn localPos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  const QPointF & QEnterEvent::localPos();
impl<'a> /*trait*/ QEnterEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: & QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent8localPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QPointF & QEnterEvent::windowPos();
impl /*struct*/ QEnterEvent {
  pub fn windowPos<RetType, T: QEnterEvent_windowPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  const QPointF & QEnterEvent::windowPos();
impl<'a> /*trait*/ QEnterEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: & QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QEnterEvent::globalX();
impl /*struct*/ QEnterEvent {
  pub fn globalX<RetType, T: QEnterEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  int QEnterEvent::globalX();
impl<'a> /*trait*/ QEnterEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QEnterEvent::x();
impl /*struct*/ QEnterEvent {
  pub fn x<RetType, T: QEnterEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QEnterEvent_x<RetType> {
  fn x(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  int QEnterEvent::x();
impl<'a> /*trait*/ QEnterEvent_x<()> for () {
  fn x(self , rsthis: & QEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1xEv()};
     unsafe {_ZNK11QEnterEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QEnterEvent::globalPos();
impl /*struct*/ QEnterEvent {
  pub fn globalPos<RetType, T: QEnterEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  QPoint QEnterEvent::globalPos();
impl<'a> /*trait*/ QEnterEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QEnterEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QEnterEvent::globalY();
impl /*struct*/ QEnterEvent {
  pub fn globalY<RetType, T: QEnterEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QEnterEvent) -> RetType;
}

  // proto:  int QEnterEvent::globalY();
impl<'a> /*trait*/ QEnterEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalYEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QEnterEvent::QEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
impl /*struct*/ QEnterEvent {
  pub fn New<T: QEnterEvent_New>(value: T) -> QEnterEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QEnterEvent_New {
  fn New(self) -> QEnterEvent;
}

  // proto:  void QEnterEvent::QEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
impl<'a> /*trait*/ QEnterEvent_New for (&'a QPointF, &'a QPointF, &'a QPointF) {
  fn New(self) -> QEnterEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventC1ERK7QPointFS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QEnterEventC1ERK7QPointFS2_S2_(qthis, arg0, arg1, arg2)};
    let rsthis = QEnterEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMoveEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QMoveEvent {
    return QMoveEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QMoveEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QMoveEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QMoveEvent::~QMoveEvent();
impl /*struct*/ QMoveEvent {
  pub fn Free<RetType, T: QMoveEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QMoveEvent_Free<RetType> {
  fn Free(self , rsthis: & QMoveEvent) -> RetType;
}

  // proto:  void QMoveEvent::~QMoveEvent();
impl<'a> /*trait*/ QMoveEvent_Free<()> for () {
  fn Free(self , rsthis: & QMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventD0Ev()};
     unsafe {_ZN10QMoveEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPoint & QMoveEvent::oldPos();
impl /*struct*/ QMoveEvent {
  pub fn oldPos<RetType, T: QMoveEvent_oldPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.oldPos(self);
    // return 1;
  }
}

pub trait QMoveEvent_oldPos<RetType> {
  fn oldPos(self , rsthis: & QMoveEvent) -> RetType;
}

  // proto:  const QPoint & QMoveEvent::oldPos();
impl<'a> /*trait*/ QMoveEvent_oldPos<QPoint> for () {
  fn oldPos(self , rsthis: & QMoveEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QMoveEvent6oldPosEv()};
    let mut ret = unsafe {_ZNK10QMoveEvent6oldPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMoveEvent::QMoveEvent(const QPoint & pos, const QPoint & oldPos);
impl /*struct*/ QMoveEvent {
  pub fn New<T: QMoveEvent_New>(value: T) -> QMoveEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMoveEvent_New {
  fn New(self) -> QMoveEvent;
}

  // proto:  void QMoveEvent::QMoveEvent(const QPoint & pos, const QPoint & oldPos);
impl<'a> /*trait*/ QMoveEvent_New for (&'a QPoint, &'a QPoint) {
  fn New(self) -> QMoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventC1ERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QMoveEventC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QMoveEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QPoint & QMoveEvent::pos();
impl /*struct*/ QMoveEvent {
  pub fn pos<RetType, T: QMoveEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QMoveEvent_pos<RetType> {
  fn pos(self , rsthis: & QMoveEvent) -> RetType;
}

  // proto:  const QPoint & QMoveEvent::pos();
impl<'a> /*trait*/ QMoveEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QMoveEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QMoveEvent3posEv()};
    let mut ret = unsafe {_ZNK10QMoveEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHideEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QHideEvent {
    return QHideEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QHideEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QHideEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QHideEvent::QHideEvent();
impl /*struct*/ QHideEvent {
  pub fn New<T: QHideEvent_New>(value: T) -> QHideEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QHideEvent_New {
  fn New(self) -> QHideEvent;
}

  // proto:  void QHideEvent::QHideEvent();
impl<'a> /*trait*/ QHideEvent_New for () {
  fn New(self) -> QHideEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QHideEventC1Ev()};
    unsafe {_ZN10QHideEventC1Ev(qthis)};
    let rsthis = QHideEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHideEvent::~QHideEvent();
impl /*struct*/ QHideEvent {
  pub fn Free<RetType, T: QHideEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QHideEvent_Free<RetType> {
  fn Free(self , rsthis: & QHideEvent) -> RetType;
}

  // proto:  void QHideEvent::~QHideEvent();
impl<'a> /*trait*/ QHideEvent_Free<()> for () {
  fn Free(self , rsthis: & QHideEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QHideEventD0Ev()};
     unsafe {_ZN10QHideEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDragLeaveEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QDragLeaveEvent {
    return QDragLeaveEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDragLeaveEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QDragLeaveEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QDragLeaveEvent::~QDragLeaveEvent();
impl /*struct*/ QDragLeaveEvent {
  pub fn Free<RetType, T: QDragLeaveEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDragLeaveEvent_Free<RetType> {
  fn Free(self , rsthis: & QDragLeaveEvent) -> RetType;
}

  // proto:  void QDragLeaveEvent::~QDragLeaveEvent();
impl<'a> /*trait*/ QDragLeaveEvent_Free<()> for () {
  fn Free(self , rsthis: & QDragLeaveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragLeaveEventD0Ev()};
     unsafe {_ZN15QDragLeaveEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDragLeaveEvent::QDragLeaveEvent();
impl /*struct*/ QDragLeaveEvent {
  pub fn New<T: QDragLeaveEvent_New>(value: T) -> QDragLeaveEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDragLeaveEvent_New {
  fn New(self) -> QDragLeaveEvent;
}

  // proto:  void QDragLeaveEvent::QDragLeaveEvent();
impl<'a> /*trait*/ QDragLeaveEvent_New for () {
  fn New(self) -> QDragLeaveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragLeaveEventC1Ev()};
    unsafe {_ZN15QDragLeaveEventC1Ev(qthis)};
    let rsthis = QDragLeaveEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QDropEvent {
    return QDropEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDropEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QDropEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QDropEvent::~QDropEvent();
impl /*struct*/ QDropEvent {
  pub fn Free<RetType, T: QDropEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDropEvent_Free<RetType> {
  fn Free(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  void QDropEvent::~QDropEvent();
impl<'a> /*trait*/ QDropEvent_Free<()> for () {
  fn Free(self , rsthis: & QDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDropEventD0Ev()};
     unsafe {_ZN10QDropEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QDropEvent::pos();
impl /*struct*/ QDropEvent {
  pub fn pos<RetType, T: QDropEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QDropEvent_pos<RetType> {
  fn pos(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  QPoint QDropEvent::pos();
impl<'a> /*trait*/ QDropEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QDropEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent3posEv()};
    let mut ret = unsafe {_ZNK10QDropEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QDropEvent::source();
impl /*struct*/ QDropEvent {
  pub fn source<RetType, T: QDropEvent_source<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QDropEvent_source<RetType> {
  fn source(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  QObject * QDropEvent::source();
impl<'a> /*trait*/ QDropEvent_source<QObject> for () {
  fn source(self , rsthis: & QDropEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent6sourceEv()};
    let mut ret = unsafe {_ZNK10QDropEvent6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMimeData * QDropEvent::mimeData();
impl /*struct*/ QDropEvent {
  pub fn mimeData<RetType, T: QDropEvent_mimeData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeData(self);
    // return 1;
  }
}

pub trait QDropEvent_mimeData<RetType> {
  fn mimeData(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  const QMimeData * QDropEvent::mimeData();
impl<'a> /*trait*/ QDropEvent_mimeData<QMimeData> for () {
  fn mimeData(self , rsthis: & QDropEvent) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent8mimeDataEv()};
    let mut ret = unsafe {_ZNK10QDropEvent8mimeDataEv(rsthis.qclsinst)};
    let mut ret1 = QMimeData::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDropEvent::acceptProposedAction();
impl /*struct*/ QDropEvent {
  pub fn acceptProposedAction<RetType, T: QDropEvent_acceptProposedAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptProposedAction(self);
    // return 1;
  }
}

pub trait QDropEvent_acceptProposedAction<RetType> {
  fn acceptProposedAction(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  void QDropEvent::acceptProposedAction();
impl<'a> /*trait*/ QDropEvent_acceptProposedAction<()> for () {
  fn acceptProposedAction(self , rsthis: & QDropEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDropEvent20acceptProposedActionEv()};
     unsafe {_ZN10QDropEvent20acceptProposedActionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QDropEvent::posF();
impl /*struct*/ QDropEvent {
  pub fn posF<RetType, T: QDropEvent_posF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QDropEvent_posF<RetType> {
  fn posF(self , rsthis: & QDropEvent) -> RetType;
}

  // proto:  const QPointF & QDropEvent::posF();
impl<'a> /*trait*/ QDropEvent_posF<QPointF> for () {
  fn posF(self , rsthis: & QDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent4posFEv()};
    let mut ret = unsafe {_ZNK10QDropEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QInputEvent {
    return QInputEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QInputEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QInputEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QInputEvent::setTimestamp(ulong atimestamp);
impl /*struct*/ QInputEvent {
  pub fn setTimestamp<RetType, T: QInputEvent_setTimestamp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTimestamp(self);
    // return 1;
  }
}

pub trait QInputEvent_setTimestamp<RetType> {
  fn setTimestamp(self , rsthis: & QInputEvent) -> RetType;
}

  // proto:  void QInputEvent::setTimestamp(ulong atimestamp);
impl<'a> /*trait*/ QInputEvent_setTimestamp<()> for (u64) {
  fn setTimestamp(self , rsthis: & QInputEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QInputEvent12setTimestampEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN11QInputEvent12setTimestampEm(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  ulong QInputEvent::timestamp();
impl /*struct*/ QInputEvent {
  pub fn timestamp<RetType, T: QInputEvent_timestamp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timestamp(self);
    // return 1;
  }
}

pub trait QInputEvent_timestamp<RetType> {
  fn timestamp(self , rsthis: & QInputEvent) -> RetType;
}

  // proto:  ulong QInputEvent::timestamp();
impl<'a> /*trait*/ QInputEvent_timestamp<u64> for () {
  fn timestamp(self , rsthis: & QInputEvent) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QInputEvent9timestampEv()};
    let mut ret = unsafe {_ZNK11QInputEvent9timestampEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  void QInputEvent::~QInputEvent();
impl /*struct*/ QInputEvent {
  pub fn Free<RetType, T: QInputEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QInputEvent_Free<RetType> {
  fn Free(self , rsthis: & QInputEvent) -> RetType;
}

  // proto:  void QInputEvent::~QInputEvent();
impl<'a> /*trait*/ QInputEvent_Free<()> for () {
  fn Free(self , rsthis: & QInputEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QInputEventD0Ev()};
     unsafe {_ZN11QInputEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QApplicationStateChangeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QApplicationStateChangeEvent {
    return QApplicationStateChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QApplicationStateChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QApplicationStateChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
impl /*struct*/ QKeyEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QKeyEvent {
    return QKeyEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QKeyEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QKeyEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  int QKeyEvent::count();
impl /*struct*/ QKeyEvent {
  pub fn count<RetType, T: QKeyEvent_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QKeyEvent_count<RetType> {
  fn count(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  int QKeyEvent::count();
impl<'a> /*trait*/ QKeyEvent_count<i32> for () {
  fn count(self , rsthis: & QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent5countEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QKeyEvent::~QKeyEvent();
impl /*struct*/ QKeyEvent {
  pub fn Free<RetType, T: QKeyEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QKeyEvent_Free<RetType> {
  fn Free(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  void QKeyEvent::~QKeyEvent();
impl<'a> /*trait*/ QKeyEvent_Free<()> for () {
  fn Free(self , rsthis: & QKeyEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QKeyEventD0Ev()};
     unsafe {_ZN9QKeyEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QKeyEvent::text();
impl /*struct*/ QKeyEvent {
  pub fn text<RetType, T: QKeyEvent_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QKeyEvent_text<RetType> {
  fn text(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  QString QKeyEvent::text();
impl<'a> /*trait*/ QKeyEvent_text<QString> for () {
  fn text(self , rsthis: & QKeyEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent4textEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  quint32 QKeyEvent::nativeVirtualKey();
impl /*struct*/ QKeyEvent {
  pub fn nativeVirtualKey<RetType, T: QKeyEvent_nativeVirtualKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeVirtualKey(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeVirtualKey<RetType> {
  fn nativeVirtualKey(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  quint32 QKeyEvent::nativeVirtualKey();
impl<'a> /*trait*/ QKeyEvent_nativeVirtualKey<u32> for () {
  fn nativeVirtualKey(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent16nativeVirtualKeyEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent16nativeVirtualKeyEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  bool QKeyEvent::isAutoRepeat();
impl /*struct*/ QKeyEvent {
  pub fn isAutoRepeat<RetType, T: QKeyEvent_isAutoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAutoRepeat(self);
    // return 1;
  }
}

pub trait QKeyEvent_isAutoRepeat<RetType> {
  fn isAutoRepeat(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  bool QKeyEvent::isAutoRepeat();
impl<'a> /*trait*/ QKeyEvent_isAutoRepeat<i8> for () {
  fn isAutoRepeat(self , rsthis: & QKeyEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent12isAutoRepeatEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent12isAutoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QKeyEvent::key();
impl /*struct*/ QKeyEvent {
  pub fn key<RetType, T: QKeyEvent_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QKeyEvent_key<RetType> {
  fn key(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  int QKeyEvent::key();
impl<'a> /*trait*/ QKeyEvent_key<i32> for () {
  fn key(self , rsthis: & QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent3keyEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent3keyEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  quint32 QKeyEvent::nativeModifiers();
impl /*struct*/ QKeyEvent {
  pub fn nativeModifiers<RetType, T: QKeyEvent_nativeModifiers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeModifiers(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeModifiers<RetType> {
  fn nativeModifiers(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  quint32 QKeyEvent::nativeModifiers();
impl<'a> /*trait*/ QKeyEvent_nativeModifiers<u32> for () {
  fn nativeModifiers(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent15nativeModifiersEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent15nativeModifiersEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  quint32 QKeyEvent::nativeScanCode();
impl /*struct*/ QKeyEvent {
  pub fn nativeScanCode<RetType, T: QKeyEvent_nativeScanCode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeScanCode(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeScanCode<RetType> {
  fn nativeScanCode(self , rsthis: & QKeyEvent) -> RetType;
}

  // proto:  quint32 QKeyEvent::nativeScanCode();
impl<'a> /*trait*/ QKeyEvent_nativeScanCode<u32> for () {
  fn nativeScanCode(self , rsthis: & QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent14nativeScanCodeEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent14nativeScanCodeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QContextMenuEvent {
    return QContextMenuEvent{qbase: QInputEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QContextMenuEvent {
  type Target = QInputEvent;

  fn deref(&self) -> &QInputEvent {
    return & self.qbase;
  }
}
impl AsRef<QInputEvent> for QContextMenuEvent {
  fn as_ref(& self) -> & QInputEvent {
    return & self.qbase;
  }
}
  // proto:  const QPoint & QContextMenuEvent::globalPos();
impl /*struct*/ QContextMenuEvent {
  pub fn globalPos<RetType, T: QContextMenuEvent_globalPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  const QPoint & QContextMenuEvent::globalPos();
impl<'a> /*trait*/ QContextMenuEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: & QContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QContextMenuEvent::globalY();
impl /*struct*/ QContextMenuEvent {
  pub fn globalY<RetType, T: QContextMenuEvent_globalY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalY<RetType> {
  fn globalY(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  int QContextMenuEvent::globalY();
impl<'a> /*trait*/ QContextMenuEvent_globalY<i32> for () {
  fn globalY(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalYEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QContextMenuEvent::globalX();
impl /*struct*/ QContextMenuEvent {
  pub fn globalX<RetType, T: QContextMenuEvent_globalX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalX<RetType> {
  fn globalX(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  int QContextMenuEvent::globalX();
impl<'a> /*trait*/ QContextMenuEvent_globalX<i32> for () {
  fn globalX(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalXEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QPoint & QContextMenuEvent::pos();
impl /*struct*/ QContextMenuEvent {
  pub fn pos<RetType, T: QContextMenuEvent_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_pos<RetType> {
  fn pos(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  const QPoint & QContextMenuEvent::pos();
impl<'a> /*trait*/ QContextMenuEvent_pos<QPoint> for () {
  fn pos(self , rsthis: & QContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent3posEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QContextMenuEvent::y();
impl /*struct*/ QContextMenuEvent {
  pub fn y<RetType, T: QContextMenuEvent_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_y<RetType> {
  fn y(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  int QContextMenuEvent::y();
impl<'a> /*trait*/ QContextMenuEvent_y<()> for () {
  fn y(self , rsthis: & QContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1yEv()};
     unsafe {_ZNK17QContextMenuEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QContextMenuEvent::x();
impl /*struct*/ QContextMenuEvent {
  pub fn x<RetType, T: QContextMenuEvent_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_x<RetType> {
  fn x(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  int QContextMenuEvent::x();
impl<'a> /*trait*/ QContextMenuEvent_x<()> for () {
  fn x(self , rsthis: & QContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1xEv()};
     unsafe {_ZNK17QContextMenuEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QContextMenuEvent::~QContextMenuEvent();
impl /*struct*/ QContextMenuEvent {
  pub fn Free<RetType, T: QContextMenuEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_Free<RetType> {
  fn Free(self , rsthis: & QContextMenuEvent) -> RetType;
}

  // proto:  void QContextMenuEvent::~QContextMenuEvent();
impl<'a> /*trait*/ QContextMenuEvent_Free<()> for () {
  fn Free(self , rsthis: & QContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QContextMenuEventD0Ev()};
     unsafe {_ZN17QContextMenuEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QScrollPrepareEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QScrollPrepareEvent {
    return QScrollPrepareEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QScrollPrepareEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QScrollPrepareEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QScrollPrepareEvent::setContentPosRange(const QRectF & rect);
impl /*struct*/ QScrollPrepareEvent {
  pub fn setContentPosRange<RetType, T: QScrollPrepareEvent_setContentPosRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentPosRange(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setContentPosRange<RetType> {
  fn setContentPosRange(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  void QScrollPrepareEvent::setContentPosRange(const QRectF & rect);
impl<'a> /*trait*/ QScrollPrepareEvent_setContentPosRange<()> for (&'a QRectF) {
  fn setContentPosRange(self , rsthis: & QScrollPrepareEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QScrollPrepareEvent::setContentPos(const QPointF & pos);
impl /*struct*/ QScrollPrepareEvent {
  pub fn setContentPos<RetType, T: QScrollPrepareEvent_setContentPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setContentPos<RetType> {
  fn setContentPos(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  void QScrollPrepareEvent::setContentPos(const QPointF & pos);
impl<'a> /*trait*/ QScrollPrepareEvent_setContentPos<()> for (&'a QPointF) {
  fn setContentPos(self , rsthis: & QScrollPrepareEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent13setContentPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QScrollPrepareEvent13setContentPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QScrollPrepareEvent::contentPosRange();
impl /*struct*/ QScrollPrepareEvent {
  pub fn contentPosRange<RetType, T: QScrollPrepareEvent_contentPosRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentPosRange(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_contentPosRange<RetType> {
  fn contentPosRange(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  QRectF QScrollPrepareEvent::contentPosRange();
impl<'a> /*trait*/ QScrollPrepareEvent_contentPosRange<QRectF> for () {
  fn contentPosRange(self , rsthis: & QScrollPrepareEvent) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent15contentPosRangeEv()};
    let mut ret = unsafe {_ZNK19QScrollPrepareEvent15contentPosRangeEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QScrollPrepareEvent::contentPos();
impl /*struct*/ QScrollPrepareEvent {
  pub fn contentPos<RetType, T: QScrollPrepareEvent_contentPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_contentPos<RetType> {
  fn contentPos(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  QPointF QScrollPrepareEvent::contentPos();
impl<'a> /*trait*/ QScrollPrepareEvent_contentPos<QPointF> for () {
  fn contentPos(self , rsthis: & QScrollPrepareEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent10contentPosEv()};
    let mut ret = unsafe {_ZNK19QScrollPrepareEvent10contentPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollPrepareEvent::setViewportSize(const QSizeF & size);
impl /*struct*/ QScrollPrepareEvent {
  pub fn setViewportSize<RetType, T: QScrollPrepareEvent_setViewportSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setViewportSize(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_setViewportSize<RetType> {
  fn setViewportSize(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  void QScrollPrepareEvent::setViewportSize(const QSizeF & size);
impl<'a> /*trait*/ QScrollPrepareEvent_setViewportSize<()> for (&'a QSizeF) {
  fn setViewportSize(self , rsthis: & QScrollPrepareEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QScrollPrepareEvent::QScrollPrepareEvent(const QPointF & startPos);
impl /*struct*/ QScrollPrepareEvent {
  pub fn New<T: QScrollPrepareEvent_New>(value: T) -> QScrollPrepareEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollPrepareEvent_New {
  fn New(self) -> QScrollPrepareEvent;
}

  // proto:  void QScrollPrepareEvent::QScrollPrepareEvent(const QPointF & startPos);
impl<'a> /*trait*/ QScrollPrepareEvent_New for (&'a QPointF) {
  fn New(self) -> QScrollPrepareEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEventC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QScrollPrepareEventC1ERK7QPointF(qthis, arg0)};
    let rsthis = QScrollPrepareEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QScrollPrepareEvent::startPos();
impl /*struct*/ QScrollPrepareEvent {
  pub fn startPos<RetType, T: QScrollPrepareEvent_startPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startPos(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_startPos<RetType> {
  fn startPos(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  QPointF QScrollPrepareEvent::startPos();
impl<'a> /*trait*/ QScrollPrepareEvent_startPos<QPointF> for () {
  fn startPos(self , rsthis: & QScrollPrepareEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent8startPosEv()};
    let mut ret = unsafe {_ZNK19QScrollPrepareEvent8startPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizeF QScrollPrepareEvent::viewportSize();
impl /*struct*/ QScrollPrepareEvent {
  pub fn viewportSize<RetType, T: QScrollPrepareEvent_viewportSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.viewportSize(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_viewportSize<RetType> {
  fn viewportSize(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  QSizeF QScrollPrepareEvent::viewportSize();
impl<'a> /*trait*/ QScrollPrepareEvent_viewportSize<QSizeF> for () {
  fn viewportSize(self , rsthis: & QScrollPrepareEvent) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK19QScrollPrepareEvent12viewportSizeEv()};
    let mut ret = unsafe {_ZNK19QScrollPrepareEvent12viewportSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollPrepareEvent::~QScrollPrepareEvent();
impl /*struct*/ QScrollPrepareEvent {
  pub fn Free<RetType, T: QScrollPrepareEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QScrollPrepareEvent_Free<RetType> {
  fn Free(self , rsthis: & QScrollPrepareEvent) -> RetType;
}

  // proto:  void QScrollPrepareEvent::~QScrollPrepareEvent();
impl<'a> /*trait*/ QScrollPrepareEvent_Free<()> for () {
  fn Free(self , rsthis: & QScrollPrepareEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEventD0Ev()};
     unsafe {_ZN19QScrollPrepareEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcutEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QShortcutEvent {
    return QShortcutEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QShortcutEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QShortcutEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  const QKeySequence & QShortcutEvent::key();
impl /*struct*/ QShortcutEvent {
  pub fn key<RetType, T: QShortcutEvent_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QShortcutEvent_key<RetType> {
  fn key(self , rsthis: & QShortcutEvent) -> RetType;
}

  // proto:  const QKeySequence & QShortcutEvent::key();
impl<'a> /*trait*/ QShortcutEvent_key<QKeySequence> for () {
  fn key(self , rsthis: & QShortcutEvent) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent3keyEv()};
    let mut ret = unsafe {_ZNK14QShortcutEvent3keyEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QShortcutEvent::~QShortcutEvent();
impl /*struct*/ QShortcutEvent {
  pub fn Free<RetType, T: QShortcutEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QShortcutEvent_Free<RetType> {
  fn Free(self , rsthis: & QShortcutEvent) -> RetType;
}

  // proto:  void QShortcutEvent::~QShortcutEvent();
impl<'a> /*trait*/ QShortcutEvent_Free<()> for () {
  fn Free(self , rsthis: & QShortcutEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QShortcutEventD0Ev()};
     unsafe {_ZN14QShortcutEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QShortcutEvent::isAmbiguous();
impl /*struct*/ QShortcutEvent {
  pub fn isAmbiguous<RetType, T: QShortcutEvent_isAmbiguous<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAmbiguous(self);
    // return 1;
  }
}

pub trait QShortcutEvent_isAmbiguous<RetType> {
  fn isAmbiguous(self , rsthis: & QShortcutEvent) -> RetType;
}

  // proto:  bool QShortcutEvent::isAmbiguous();
impl<'a> /*trait*/ QShortcutEvent_isAmbiguous<i8> for () {
  fn isAmbiguous(self , rsthis: & QShortcutEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent11isAmbiguousEv()};
    let mut ret = unsafe {_ZNK14QShortcutEvent11isAmbiguousEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QShortcutEvent::QShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
impl /*struct*/ QShortcutEvent {
  pub fn New<T: QShortcutEvent_New>(value: T) -> QShortcutEvent {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcutEvent_New {
  fn New(self) -> QShortcutEvent;
}

  // proto:  void QShortcutEvent::QShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
impl<'a> /*trait*/ QShortcutEvent_New for (&'a QKeySequence, i32, i8) {
  fn New(self) -> QShortcutEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QShortcutEventC1ERK12QKeySequenceib()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_char;
    unsafe {_ZN14QShortcutEventC1ERK12QKeySequenceib(qthis, arg0, arg1, arg2)};
    let rsthis = QShortcutEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QShortcutEvent::shortcutId();
impl /*struct*/ QShortcutEvent {
  pub fn shortcutId<RetType, T: QShortcutEvent_shortcutId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shortcutId(self);
    // return 1;
  }
}

pub trait QShortcutEvent_shortcutId<RetType> {
  fn shortcutId(self , rsthis: & QShortcutEvent) -> RetType;
}

  // proto:  int QShortcutEvent::shortcutId();
impl<'a> /*trait*/ QShortcutEvent_shortcutId<i32> for () {
  fn shortcutId(self , rsthis: & QShortcutEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QShortcutEvent10shortcutIdEv()};
    let mut ret = unsafe {_ZNK14QShortcutEvent10shortcutIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWindowStateChangeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QWindowStateChangeEvent {
    return QWindowStateChangeEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QWindowStateChangeEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QWindowStateChangeEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  bool QWindowStateChangeEvent::isOverride();
impl /*struct*/ QWindowStateChangeEvent {
  pub fn isOverride<RetType, T: QWindowStateChangeEvent_isOverride<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isOverride(self);
    // return 1;
  }
}

pub trait QWindowStateChangeEvent_isOverride<RetType> {
  fn isOverride(self , rsthis: & QWindowStateChangeEvent) -> RetType;
}

  // proto:  bool QWindowStateChangeEvent::isOverride();
impl<'a> /*trait*/ QWindowStateChangeEvent_isOverride<i8> for () {
  fn isOverride(self , rsthis: & QWindowStateChangeEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QWindowStateChangeEvent10isOverrideEv()};
    let mut ret = unsafe {_ZNK23QWindowStateChangeEvent10isOverrideEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
impl /*struct*/ QWindowStateChangeEvent {
  pub fn Free<RetType, T: QWindowStateChangeEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QWindowStateChangeEvent_Free<RetType> {
  fn Free(self , rsthis: & QWindowStateChangeEvent) -> RetType;
}

  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
impl<'a> /*trait*/ QWindowStateChangeEvent_Free<()> for () {
  fn Free(self , rsthis: & QWindowStateChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QWindowStateChangeEventD0Ev()};
     unsafe {_ZN23QWindowStateChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QInputMethodQueryEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QInputMethodQueryEvent {
    return QInputMethodQueryEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QInputMethodQueryEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QInputMethodQueryEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  void QInputMethodQueryEvent::~QInputMethodQueryEvent();
impl /*struct*/ QInputMethodQueryEvent {
  pub fn Free<RetType, T: QInputMethodQueryEvent_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QInputMethodQueryEvent_Free<RetType> {
  fn Free(self , rsthis: & QInputMethodQueryEvent) -> RetType;
}

  // proto:  void QInputMethodQueryEvent::~QInputMethodQueryEvent();
impl<'a> /*trait*/ QInputMethodQueryEvent_Free<()> for () {
  fn Free(self , rsthis: & QInputMethodQueryEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QInputMethodQueryEventD0Ev()};
     unsafe {_ZN22QInputMethodQueryEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

