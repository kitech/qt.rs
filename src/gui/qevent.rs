// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
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
use super::super::core::qurl::QUrl; // 771
use super::super::core::qfile::QFile; // 771
use super::qtouchdevice::QTouchDevice; // 773
use super::qscreen::QScreen; // 773
// use super::qevent::QDragMoveEvent; // 773
use super::super::core::qmimedata::QMimeData; // 771
// use super::qevent::QDropEvent; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qsize::QSizeF; // 771
use super::qkeysequence::QKeySequence; // 773
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QWhatsThisClickedEvent_Class_Size() -> c_int;
  // proto:  void QWhatsThisClickedEvent::~QWhatsThisClickedEvent();
  fn _ZN22QWhatsThisClickedEventD0Ev(qthis: *mut c_void);
  // proto:  void QWhatsThisClickedEvent::QWhatsThisClickedEvent(const QString & href);
  fn dector_ZN22QWhatsThisClickedEventC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN22QWhatsThisClickedEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  fn QExposeEvent_Class_Size() -> c_int;
  // proto:  void QExposeEvent::QExposeEvent(const QRegion & rgn);
  fn dector_ZN12QExposeEventC1ERK7QRegion(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QExposeEventC1ERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QExposeEvent::~QExposeEvent();
  fn _ZN12QExposeEventD0Ev(qthis: *mut c_void);
  fn QInputMethodEvent_Class_Size() -> c_int;
  // proto:  void QInputMethodEvent::QInputMethodEvent(const QInputMethodEvent & other);
  fn dector_ZN17QInputMethodEventC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN17QInputMethodEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
  fn _ZN17QInputMethodEvent15setCommitStringERK7QStringii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QInputMethodEvent::QInputMethodEvent();
  fn dector_ZN17QInputMethodEventC1Ev() -> *mut c_void;
  fn _ZN17QInputMethodEventC1Ev(qthis: *mut c_void);
  fn QHelpEvent_Class_Size() -> c_int;
  // proto:  void QHelpEvent::~QHelpEvent();
  fn _ZN10QHelpEventD0Ev(qthis: *mut c_void);
  fn QActionEvent_Class_Size() -> c_int;
  // proto:  void QActionEvent::QActionEvent(int type, QAction * action, QAction * before);
  fn dector_ZN12QActionEventC1EiP7QActionS1_(arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN12QActionEventC1EiP7QActionS1_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QActionEvent::~QActionEvent();
  fn _ZN12QActionEventD0Ev(qthis: *mut c_void);
  fn QMouseEvent_Class_Size() -> c_int;
  // proto:  void QMouseEvent::~QMouseEvent();
  fn _ZN11QMouseEventD0Ev(qthis: *mut c_void);
  fn QFileOpenEvent_Class_Size() -> c_int;
  // proto:  void QFileOpenEvent::QFileOpenEvent(const QString & file);
  fn dector_ZN14QFileOpenEventC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QFileOpenEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFileOpenEvent::QFileOpenEvent(const QUrl & url);
  fn dector_ZN14QFileOpenEventC1ERK4QUrl(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QFileOpenEventC1ERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFileOpenEvent::~QFileOpenEvent();
  fn _ZN14QFileOpenEventD0Ev(qthis: *mut c_void);
  fn QToolBarChangeEvent_Class_Size() -> c_int;
  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
  fn dector_ZN19QToolBarChangeEventC1Eb(arg0: c_char) -> *mut c_void;
  fn _ZN19QToolBarChangeEventC1Eb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
  fn _ZN19QToolBarChangeEventD0Ev(qthis: *mut c_void);
  fn QTabletEvent_Class_Size() -> c_int;
  // proto:  void QTabletEvent::~QTabletEvent();
  fn _ZN12QTabletEventD0Ev(qthis: *mut c_void);
  fn QTouchEvent_Class_Size() -> c_int;
  // proto:  void QTouchEvent::~QTouchEvent();
  fn _ZN11QTouchEventD0Ev(qthis: *mut c_void);
  fn QScreenOrientationChangeEvent_Class_Size() -> c_int;
  // proto:  QScreen * QScreenOrientationChangeEvent::screen();
  fn _ZNK29QScreenOrientationChangeEvent6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScreenOrientationChangeEvent::~QScreenOrientationChangeEvent();
  fn _ZN29QScreenOrientationChangeEventD0Ev(qthis: *mut c_void);
  fn QIconDragEvent_Class_Size() -> c_int;
  // proto:  void QIconDragEvent::~QIconDragEvent();
  fn _ZN14QIconDragEventD0Ev(qthis: *mut c_void);
  // proto:  void QIconDragEvent::QIconDragEvent();
  fn dector_ZN14QIconDragEventC1Ev() -> *mut c_void;
  fn _ZN14QIconDragEventC1Ev(qthis: *mut c_void);
  fn QCloseEvent_Class_Size() -> c_int;
  // proto:  void QCloseEvent::~QCloseEvent();
  fn _ZN11QCloseEventD0Ev(qthis: *mut c_void);
  // proto:  void QCloseEvent::QCloseEvent();
  fn dector_ZN11QCloseEventC1Ev() -> *mut c_void;
  fn _ZN11QCloseEventC1Ev(qthis: *mut c_void);
  fn QDragEnterEvent_Class_Size() -> c_int;
  // proto:  void QDragEnterEvent::~QDragEnterEvent();
  fn _ZN15QDragEnterEventD0Ev(qthis: *mut c_void);
  fn QWheelEvent_Class_Size() -> c_int;
  // proto:  void QWheelEvent::~QWheelEvent();
  fn _ZN11QWheelEventD0Ev(qthis: *mut c_void);
  fn QScrollEvent_Class_Size() -> c_int;
  // proto:  QPointF QScrollEvent::contentPos();
  fn _ZNK12QScrollEvent10contentPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QScrollEvent::overshootDistance();
  fn _ZNK12QScrollEvent17overshootDistanceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollEvent::~QScrollEvent();
  fn _ZN12QScrollEventD0Ev(qthis: *mut c_void);
  fn QHoverEvent_Class_Size() -> c_int;
  // proto:  void QHoverEvent::~QHoverEvent();
  fn _ZN11QHoverEventD0Ev(qthis: *mut c_void);
  fn QDragMoveEvent_Class_Size() -> c_int;
  // proto:  void QDragMoveEvent::~QDragMoveEvent();
  fn _ZN14QDragMoveEventD0Ev(qthis: *mut c_void);
  fn QShowEvent_Class_Size() -> c_int;
  // proto:  void QShowEvent::~QShowEvent();
  fn _ZN10QShowEventD0Ev(qthis: *mut c_void);
  // proto:  void QShowEvent::QShowEvent();
  fn dector_ZN10QShowEventC1Ev() -> *mut c_void;
  fn _ZN10QShowEventC1Ev(qthis: *mut c_void);
  fn QPlatformSurfaceEvent_Class_Size() -> c_int;
  // proto:  void QPlatformSurfaceEvent::~QPlatformSurfaceEvent();
  fn _ZN21QPlatformSurfaceEventD0Ev(qthis: *mut c_void);
  fn QPaintEvent_Class_Size() -> c_int;
  // proto:  void QPaintEvent::~QPaintEvent();
  fn _ZN11QPaintEventD0Ev(qthis: *mut c_void);
  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
  fn dector_ZN11QPaintEventC1ERK5QRect(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QPaintEventC1ERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintEvent::QPaintEvent(const QRegion & paintRegion);
  fn dector_ZN11QPaintEventC1ERK7QRegion(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QPaintEventC1ERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  fn QFocusEvent_Class_Size() -> c_int;
  // proto:  void QFocusEvent::~QFocusEvent();
  fn _ZN11QFocusEventD0Ev(qthis: *mut c_void);
  fn QNativeGestureEvent_Class_Size() -> c_int;
  fn QResizeEvent_Class_Size() -> c_int;
  // proto:  void QResizeEvent::~QResizeEvent();
  fn _ZN12QResizeEventD0Ev(qthis: *mut c_void);
  // proto:  void QResizeEvent::QResizeEvent(const QSize & size, const QSize & oldSize);
  fn dector_ZN12QResizeEventC1ERK5QSizeS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN12QResizeEventC1ERK5QSizeS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  fn QStatusTipEvent_Class_Size() -> c_int;
  // proto:  void QStatusTipEvent::~QStatusTipEvent();
  fn _ZN15QStatusTipEventD0Ev(qthis: *mut c_void);
  // proto:  void QStatusTipEvent::QStatusTipEvent(const QString & tip);
  fn dector_ZN15QStatusTipEventC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QStatusTipEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  fn QEnterEvent_Class_Size() -> c_int;
  // proto:  void QEnterEvent::~QEnterEvent();
  fn _ZN11QEnterEventD0Ev(qthis: *mut c_void);
  // proto:  void QEnterEvent::QEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
  fn dector_ZN11QEnterEventC1ERK7QPointFS2_S2_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN11QEnterEventC1ERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  fn QMoveEvent_Class_Size() -> c_int;
  // proto:  void QMoveEvent::~QMoveEvent();
  fn _ZN10QMoveEventD0Ev(qthis: *mut c_void);
  // proto:  void QMoveEvent::QMoveEvent(const QPoint & pos, const QPoint & oldPos);
  fn dector_ZN10QMoveEventC1ERK6QPointS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN10QMoveEventC1ERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  fn QHideEvent_Class_Size() -> c_int;
  // proto:  void QHideEvent::QHideEvent();
  fn dector_ZN10QHideEventC1Ev() -> *mut c_void;
  fn _ZN10QHideEventC1Ev(qthis: *mut c_void);
  // proto:  void QHideEvent::~QHideEvent();
  fn _ZN10QHideEventD0Ev(qthis: *mut c_void);
  fn QDragLeaveEvent_Class_Size() -> c_int;
  // proto:  void QDragLeaveEvent::~QDragLeaveEvent();
  fn _ZN15QDragLeaveEventD0Ev(qthis: *mut c_void);
  // proto:  void QDragLeaveEvent::QDragLeaveEvent();
  fn dector_ZN15QDragLeaveEventC1Ev() -> *mut c_void;
  fn _ZN15QDragLeaveEventC1Ev(qthis: *mut c_void);
  fn QDropEvent_Class_Size() -> c_int;
  // proto:  void QDropEvent::~QDropEvent();
  fn _ZN10QDropEventD0Ev(qthis: *mut c_void);
  // proto:  QObject * QDropEvent::source();
  fn _ZNK10QDropEvent6sourceEv(qthis: *mut c_void) -> *mut c_void;
  fn QInputEvent_Class_Size() -> c_int;
  // proto:  void QInputEvent::~QInputEvent();
  fn _ZN11QInputEventD0Ev(qthis: *mut c_void);
  fn QApplicationStateChangeEvent_Class_Size() -> c_int;
  fn QKeyEvent_Class_Size() -> c_int;
  // proto:  void QKeyEvent::~QKeyEvent();
  fn _ZN9QKeyEventD0Ev(qthis: *mut c_void);
  fn QContextMenuEvent_Class_Size() -> c_int;
  // proto:  void QContextMenuEvent::~QContextMenuEvent();
  fn _ZN17QContextMenuEventD0Ev(qthis: *mut c_void);
  fn QScrollPrepareEvent_Class_Size() -> c_int;
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
  fn dector_ZN19QScrollPrepareEventC1ERK7QPointF(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QScrollPrepareEventC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QScrollPrepareEvent::startPos();
  fn _ZNK19QScrollPrepareEvent8startPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QScrollPrepareEvent::viewportSize();
  fn _ZNK19QScrollPrepareEvent12viewportSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollPrepareEvent::~QScrollPrepareEvent();
  fn _ZN19QScrollPrepareEventD0Ev(qthis: *mut c_void);
  fn QShortcutEvent_Class_Size() -> c_int;
  // proto:  void QShortcutEvent::~QShortcutEvent();
  fn _ZN14QShortcutEventD0Ev(qthis: *mut c_void);
  // proto:  void QShortcutEvent::QShortcutEvent(const QKeySequence & key, int id, bool ambiguous);
  fn dector_ZN14QShortcutEventC1ERK12QKeySequenceib(arg0: *mut c_void, arg1: c_int, arg2: c_char) -> *mut c_void;
  fn _ZN14QShortcutEventC1ERK12QKeySequenceib(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_char);
  fn QWindowStateChangeEvent_Class_Size() -> c_int;
  // proto:  bool QWindowStateChangeEvent::isOverride();
  fn _ZNK23QWindowStateChangeEvent10isOverrideEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
  fn _ZN23QWindowStateChangeEventD0Ev(qthis: *mut c_void);
  fn QInputMethodQueryEvent_Class_Size() -> c_int;
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QWhatsThisClickedEventC1ERK7QString()};
    let ctysz: c_int = unsafe{QWhatsThisClickedEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN22QWhatsThisClickedEventC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN22QWhatsThisClickedEventC1ERK7QString(arg0)};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QExposeEventC1ERK7QRegion()};
    let ctysz: c_int = unsafe{QExposeEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QExposeEventC1ERK7QRegion(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QExposeEventC1ERK7QRegion(arg0)};
    let rsthis = QExposeEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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
  // proto:  void QInputMethodEvent::QInputMethodEvent(const QInputMethodEvent & other);
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

  // proto:  void QInputMethodEvent::QInputMethodEvent(const QInputMethodEvent & other);
impl<'a> /*trait*/ QInputMethodEvent_New for (&'a QInputMethodEvent) {
  fn New(self) -> QInputMethodEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC1ERKS_()};
    let ctysz: c_int = unsafe{QInputMethodEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN17QInputMethodEventC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN17QInputMethodEventC1ERKS_(arg0)};
    let rsthis = QInputMethodEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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

  // proto:  void QInputMethodEvent::QInputMethodEvent();
impl<'a> /*trait*/ QInputMethodEvent_New for () {
  fn New(self) -> QInputMethodEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC1Ev()};
    let ctysz: c_int = unsafe{QInputMethodEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN17QInputMethodEventC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN17QInputMethodEventC1Ev()};
    let rsthis = QInputMethodEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QActionEventC1EiP7QActionS1_()};
    let ctysz: c_int = unsafe{QActionEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN12QActionEventC1EiP7QActionS1_(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN12QActionEventC1EiP7QActionS1_(arg0, arg1, arg2)};
    let rsthis = QActionEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventC1ERK7QString()};
    let ctysz: c_int = unsafe{QFileOpenEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QFileOpenEventC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN14QFileOpenEventC1ERK7QString(arg0)};
    let rsthis = QFileOpenEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileOpenEvent::QFileOpenEvent(const QUrl & url);
impl<'a> /*trait*/ QFileOpenEvent_New for (&'a QUrl) {
  fn New(self) -> QFileOpenEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventC1ERK4QUrl()};
    let ctysz: c_int = unsafe{QFileOpenEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QFileOpenEventC1ERK4QUrl(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN14QFileOpenEventC1ERK4QUrl(arg0)};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventC1Eb()};
    let ctysz: c_int = unsafe{QToolBarChangeEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_char;
    // unsafe {_ZN19QToolBarChangeEventC1Eb(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN19QToolBarChangeEventC1Eb(arg0)};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventC1Ev()};
    let ctysz: c_int = unsafe{QIconDragEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN14QIconDragEventC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN14QIconDragEventC1Ev()};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QCloseEventC1Ev()};
    let ctysz: c_int = unsafe{QCloseEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN11QCloseEventC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN11QCloseEventC1Ev()};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventC1Ev()};
    let ctysz: c_int = unsafe{QShowEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN10QShowEventC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN10QShowEventC1Ev()};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK5QRect()};
    let ctysz: c_int = unsafe{QPaintEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QPaintEventC1ERK5QRect(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QPaintEventC1ERK5QRect(arg0)};
    let rsthis = QPaintEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPaintEvent::QPaintEvent(const QRegion & paintRegion);
impl<'a> /*trait*/ QPaintEvent_New for (&'a QRegion) {
  fn New(self) -> QPaintEvent {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK7QRegion()};
    let ctysz: c_int = unsafe{QPaintEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QPaintEventC1ERK7QRegion(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QPaintEventC1ERK7QRegion(arg0)};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventC1ERK5QSizeS2_()};
    let ctysz: c_int = unsafe{QResizeEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN12QResizeEventC1ERK5QSizeS2_(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN12QResizeEventC1ERK5QSizeS2_(arg0, arg1)};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStatusTipEventC1ERK7QString()};
    let ctysz: c_int = unsafe{QStatusTipEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QStatusTipEventC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN15QStatusTipEventC1ERK7QString(arg0)};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventC1ERK7QPointFS2_S2_()};
    let ctysz: c_int = unsafe{QEnterEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN11QEnterEventC1ERK7QPointFS2_S2_(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN11QEnterEventC1ERK7QPointFS2_S2_(arg0, arg1, arg2)};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventC1ERK6QPointS2_()};
    let ctysz: c_int = unsafe{QMoveEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN10QMoveEventC1ERK6QPointS2_(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN10QMoveEventC1ERK6QPointS2_(arg0, arg1)};
    let rsthis = QMoveEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QHideEventC1Ev()};
    let ctysz: c_int = unsafe{QHideEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN10QHideEventC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN10QHideEventC1Ev()};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragLeaveEventC1Ev()};
    let ctysz: c_int = unsafe{QDragLeaveEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN15QDragLeaveEventC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN15QDragLeaveEventC1Ev()};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZN19QScrollPrepareEventC1ERK7QPointF()};
    let ctysz: c_int = unsafe{QScrollPrepareEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QScrollPrepareEventC1ERK7QPointF(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN19QScrollPrepareEventC1ERK7QPointF(arg0)};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QShortcutEventC1ERK12QKeySequenceib()};
    let ctysz: c_int = unsafe{QShortcutEvent_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_char;
    // unsafe {_ZN14QShortcutEventC1ERK12QKeySequenceib(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN14QShortcutEventC1ERK12QKeySequenceib(arg0, arg1, arg2)};
    let rsthis = QShortcutEvent{/**/qbase: QEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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

