// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtWidgets/qgraphicswidget.h
// dst-file: /src/widgets/qgraphicswidget.rs
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
use super::qgraphicsitem::QGraphicsObject; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qgraphicslayout::QGraphicsLayout; // 773
use super::super::core::qrect::QRectF; // 771
use super::super::core::qsize::QSizeF; // 771
use super::qgraphicsitem::QGraphicsItem; // 773
use super::super::gui::qpainter::QPainter; // 771
use super::qstyleoption::QStyleOptionGraphicsItem; // 773
use super::qwidget::QWidget; // 773
use super::super::gui::qpalette::QPalette; // 771
use super::qstyle::QStyle; // 773
use super::super::gui::qpainterpath::QPainterPath; // 771
use super::qaction::QAction; // 773
use super::super::gui::qfont::QFont; // 771
use super::super::gui::qkeysequence::QKeySequence; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsWidget_Class_Size() -> c_int;
  // proto:  void QGraphicsWidget::setAutoFillBackground(bool enabled);
  fn _ZN15QGraphicsWidget21setAutoFillBackgroundEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QGraphicsWidget::setWindowTitle(const QString & title);
  fn _ZN15QGraphicsWidget14setWindowTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsWidget::setLayout(QGraphicsLayout * layout);
  fn _ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsWidget::setGeometry(const QRectF & rect);
  fn _ZN15QGraphicsWidget11setGeometryERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRectF QGraphicsWidget::rect();
  fn demth_ZNK15QGraphicsWidget4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSizeF QGraphicsWidget::size();
  fn _ZNK15QGraphicsWidget4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsWidget::releaseShortcut(int id);
  fn _ZN15QGraphicsWidget15releaseShortcutEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QGraphicsWidget::setWindowFrameMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsWidget21setWindowFrameMarginsEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  int QGraphicsWidget::type();
  fn _ZNK15QGraphicsWidget4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGraphicsWidget::unsetLayoutDirection();
  fn _ZN15QGraphicsWidget20unsetLayoutDirectionEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsWidget::QGraphicsWidget(const QGraphicsWidget & );
  fn dector_ZN15QGraphicsWidgetC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QGraphicsWidgetC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRectF QGraphicsWidget::windowFrameGeometry();
  fn _ZNK15QGraphicsWidget19windowFrameGeometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsWidget::resize(qreal w, qreal h);
  fn demth_ZN15QGraphicsWidget6resizeEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  QRectF QGraphicsWidget::windowFrameRect();
  fn _ZNK15QGraphicsWidget15windowFrameRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QGraphicsWidget::adjustSize();
  fn _ZN15QGraphicsWidget10adjustSizeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsWidget::paintWindowFrame(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QPalette QGraphicsWidget::palette();
  fn _ZNK15QGraphicsWidget7paletteEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsWidget::unsetWindowFrameMargins();
  fn _ZN15QGraphicsWidget23unsetWindowFrameMarginsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsWidget::resize(const QSizeF & size);
  fn _ZN15QGraphicsWidget6resizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsWidget::setPalette(const QPalette & palette);
  fn _ZN15QGraphicsWidget10setPaletteERK8QPalette(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QGraphicsWidget::autoFillBackground();
  fn _ZNK15QGraphicsWidget18autoFillBackgroundEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QStyle * QGraphicsWidget::style();
  fn _ZNK15QGraphicsWidget5styleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPainterPath QGraphicsWidget::shape();
  fn _ZNK15QGraphicsWidget5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsWidget::setShortcutEnabled(int id, bool enabled);
  fn _ZN15QGraphicsWidget18setShortcutEnabledEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  void QGraphicsWidget::removeAction(QAction * action);
  fn _ZN15QGraphicsWidget12removeActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsWidget::insertAction(QAction * before, QAction * action);
  fn _ZN15QGraphicsWidget12insertActionEP7QActionS1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QGraphicsWidget::close();
  fn _ZN15QGraphicsWidget5closeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QGraphicsWidget::metaObject();
  fn _ZNK15QGraphicsWidget10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QGraphicsWidget::boundingRect();
  fn _ZNK15QGraphicsWidget12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsWidget::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
  fn _ZN15QGraphicsWidget18setContentsMarginsEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsWidget::setFont(const QFont & font);
  fn _ZN15QGraphicsWidget7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsWidget::geometryChanged();
  fn _ZN15QGraphicsWidget15geometryChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QGraphicsWidget::windowTitle();
  fn _ZNK15QGraphicsWidget11windowTitleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QGraphicsLayout * QGraphicsWidget::layout();
  fn _ZNK15QGraphicsWidget6layoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsWidget::~QGraphicsWidget();
  fn _ZN15QGraphicsWidgetD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QGraphicsWidget * QGraphicsWidget::focusWidget();
  fn _ZNK15QGraphicsWidget11focusWidgetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsWidget::addAction(QAction * action);
  fn _ZN15QGraphicsWidget9addActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QFont QGraphicsWidget::font();
  fn _ZNK15QGraphicsWidget4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QAction *> QGraphicsWidget::actions();
  fn _ZNK15QGraphicsWidget7actionsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsWidget::layoutChanged();
  fn _ZN15QGraphicsWidget13layoutChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsWidget::setShortcutAutoRepeat(int id, bool enabled);
  fn _ZN15QGraphicsWidget21setShortcutAutoRepeatEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto: static void QGraphicsWidget::setTabOrder(QGraphicsWidget * first, QGraphicsWidget * second);
  fn _ZN15QGraphicsWidget11setTabOrderEPS_S0_(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QGraphicsWidget::getWindowFrameMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  void QGraphicsWidget::setStyle(QStyle * style);
  fn _ZN15QGraphicsWidget8setStyleEP6QStyle(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsWidget::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  bool QGraphicsWidget::isActiveWindow();
  fn _ZNK15QGraphicsWidget14isActiveWindowEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsWidget::setGeometry(qreal x, qreal y, qreal w, qreal h);
  fn demth_ZN15QGraphicsWidget11setGeometryEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  fn QGraphicsWidget_SlotProxy_connect__ZN15QGraphicsWidget13layoutChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsWidget_SlotProxy_connect__ZN15QGraphicsWidget15geometryChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsWidget)=1
#[derive(Default)]
pub struct QGraphicsWidget {
  qbase: QGraphicsObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _layoutChanged_1: QGraphicsWidget_layoutChanged_signal,
  pub _geometryChanged_1: QGraphicsWidget_geometryChanged_signal,
}

impl /*struct*/ QGraphicsWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsWidget {
    return QGraphicsWidget{qbase: QGraphicsObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsWidget {
  type Target = QGraphicsObject;

  fn deref(&self) -> &QGraphicsObject {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsObject> for QGraphicsWidget {
  fn as_ref(& self) -> & QGraphicsObject {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsWidget::setAutoFillBackground(bool enabled);
impl /*struct*/ QGraphicsWidget {
  pub fn setAutoFillBackground<RetType, T: QGraphicsWidget_setAutoFillBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoFillBackground(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setAutoFillBackground<RetType> {
  fn setAutoFillBackground(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setAutoFillBackground(bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setAutoFillBackground<()> for (i8) {
  fn setAutoFillBackground(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setAutoFillBackgroundEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QGraphicsWidget21setAutoFillBackgroundEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setWindowTitle(const QString & title);
impl /*struct*/ QGraphicsWidget {
  pub fn setWindowTitle<RetType, T: QGraphicsWidget_setWindowTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowTitle(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setWindowTitle<RetType> {
  fn setWindowTitle(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setWindowTitle(const QString & title);
impl<'a> /*trait*/ QGraphicsWidget_setWindowTitle<()> for (&'a QString) {
  fn setWindowTitle(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget14setWindowTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget14setWindowTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setLayout(QGraphicsLayout * layout);
impl /*struct*/ QGraphicsWidget {
  pub fn setLayout<RetType, T: QGraphicsWidget_setLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLayout(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setLayout<RetType> {
  fn setLayout(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setLayout(QGraphicsLayout * layout);
impl<'a> /*trait*/ QGraphicsWidget_setLayout<()> for (&'a QGraphicsLayout) {
  fn setLayout(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget9setLayoutEP15QGraphicsLayout(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setGeometry(const QRectF & rect);
impl /*struct*/ QGraphicsWidget {
  pub fn setGeometry<RetType, T: QGraphicsWidget_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsWidget_setGeometry<()> for (&'a QRectF) {
  fn setGeometry(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsWidget::rect();
impl /*struct*/ QGraphicsWidget {
  pub fn rect<RetType, T: QGraphicsWidget_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_rect<RetType> {
  fn rect(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QRectF QGraphicsWidget::rect();
impl<'a> /*trait*/ QGraphicsWidget_rect<QRectF> for () {
  fn rect(self , rsthis: & QGraphicsWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4rectEv()};
    let mut ret = unsafe {demth_ZNK15QGraphicsWidget4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizeF QGraphicsWidget::size();
impl /*struct*/ QGraphicsWidget {
  pub fn size<RetType, T: QGraphicsWidget_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_size<RetType> {
  fn size(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QSizeF QGraphicsWidget::size();
impl<'a> /*trait*/ QGraphicsWidget_size<QSizeF> for () {
  fn size(self , rsthis: & QGraphicsWidget) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4sizeEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::releaseShortcut(int id);
impl /*struct*/ QGraphicsWidget {
  pub fn releaseShortcut<RetType, T: QGraphicsWidget_releaseShortcut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.releaseShortcut(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_releaseShortcut<RetType> {
  fn releaseShortcut(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::releaseShortcut(int id);
impl<'a> /*trait*/ QGraphicsWidget_releaseShortcut<()> for (i32) {
  fn releaseShortcut(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget15releaseShortcutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QGraphicsWidget15releaseShortcutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setWindowFrameMargins(qreal left, qreal top, qreal right, qreal bottom);
impl /*struct*/ QGraphicsWidget {
  pub fn setWindowFrameMargins<RetType, T: QGraphicsWidget_setWindowFrameMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowFrameMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setWindowFrameMargins<RetType> {
  fn setWindowFrameMargins(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setWindowFrameMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsWidget_setWindowFrameMargins<()> for (f64, f64, f64, f64) {
  fn setWindowFrameMargins(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setWindowFrameMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN15QGraphicsWidget21setWindowFrameMarginsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  int QGraphicsWidget::type();
impl /*struct*/ QGraphicsWidget {
  pub fn type_<RetType, T: QGraphicsWidget_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  int QGraphicsWidget::type();
impl<'a> /*trait*/ QGraphicsWidget_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4typeEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::unsetLayoutDirection();
impl /*struct*/ QGraphicsWidget {
  pub fn unsetLayoutDirection<RetType, T: QGraphicsWidget_unsetLayoutDirection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetLayoutDirection(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_unsetLayoutDirection<RetType> {
  fn unsetLayoutDirection(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::unsetLayoutDirection();
impl<'a> /*trait*/ QGraphicsWidget_unsetLayoutDirection<()> for () {
  fn unsetLayoutDirection(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget20unsetLayoutDirectionEv()};
     unsafe {_ZN15QGraphicsWidget20unsetLayoutDirectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::QGraphicsWidget(const QGraphicsWidget & );
impl /*struct*/ QGraphicsWidget {
  pub fn New<T: QGraphicsWidget_New>(value: T) -> QGraphicsWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsWidget_New {
  fn New(self) -> QGraphicsWidget;
}

  // proto:  void QGraphicsWidget::QGraphicsWidget(const QGraphicsWidget & );
impl<'a> /*trait*/ QGraphicsWidget_New for (&'a QGraphicsWidget) {
  fn New(self) -> QGraphicsWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidgetC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QGraphicsWidgetC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QGraphicsWidgetC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsWidget{qbase: QGraphicsObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsWidget::windowFrameGeometry();
impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameGeometry<RetType, T: QGraphicsWidget_windowFrameGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowFrameGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_windowFrameGeometry<RetType> {
  fn windowFrameGeometry(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QRectF QGraphicsWidget::windowFrameGeometry();
impl<'a> /*trait*/ QGraphicsWidget_windowFrameGeometry<QRectF> for () {
  fn windowFrameGeometry(self , rsthis: & QGraphicsWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget19windowFrameGeometryEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget19windowFrameGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::resize(qreal w, qreal h);
impl /*struct*/ QGraphicsWidget {
  pub fn resize<RetType, T: QGraphicsWidget_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_resize<RetType> {
  fn resize(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::resize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsWidget_resize<()> for (f64, f64) {
  fn resize(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget6resizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {demth_ZN15QGraphicsWidget6resizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsWidget::windowFrameRect();
impl /*struct*/ QGraphicsWidget {
  pub fn windowFrameRect<RetType, T: QGraphicsWidget_windowFrameRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowFrameRect(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_windowFrameRect<RetType> {
  fn windowFrameRect(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QRectF QGraphicsWidget::windowFrameRect();
impl<'a> /*trait*/ QGraphicsWidget_windowFrameRect<QRectF> for () {
  fn windowFrameRect(self , rsthis: & QGraphicsWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget15windowFrameRectEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget15windowFrameRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsWidget {
  pub fn paint<RetType, T: QGraphicsWidget_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsWidget_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::adjustSize();
impl /*struct*/ QGraphicsWidget {
  pub fn adjustSize<RetType, T: QGraphicsWidget_adjustSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.adjustSize(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_adjustSize<RetType> {
  fn adjustSize(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::adjustSize();
impl<'a> /*trait*/ QGraphicsWidget_adjustSize<()> for () {
  fn adjustSize(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget10adjustSizeEv()};
     unsafe {_ZN15QGraphicsWidget10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::paintWindowFrame(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsWidget {
  pub fn paintWindowFrame<RetType, T: QGraphicsWidget_paintWindowFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintWindowFrame(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_paintWindowFrame<RetType> {
  fn paintWindowFrame(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::paintWindowFrame(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsWidget_paintWindowFrame<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paintWindowFrame(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget16paintWindowFrameEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QPalette QGraphicsWidget::palette();
impl /*struct*/ QGraphicsWidget {
  pub fn palette<RetType, T: QGraphicsWidget_palette<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.palette(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_palette<RetType> {
  fn palette(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QPalette QGraphicsWidget::palette();
impl<'a> /*trait*/ QGraphicsWidget_palette<QPalette> for () {
  fn palette(self , rsthis: & QGraphicsWidget) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget7paletteEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget7paletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::unsetWindowFrameMargins();
impl /*struct*/ QGraphicsWidget {
  pub fn unsetWindowFrameMargins<RetType, T: QGraphicsWidget_unsetWindowFrameMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetWindowFrameMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_unsetWindowFrameMargins<RetType> {
  fn unsetWindowFrameMargins(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::unsetWindowFrameMargins();
impl<'a> /*trait*/ QGraphicsWidget_unsetWindowFrameMargins<()> for () {
  fn unsetWindowFrameMargins(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget23unsetWindowFrameMarginsEv()};
     unsafe {_ZN15QGraphicsWidget23unsetWindowFrameMarginsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::resize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsWidget_resize<()> for (&'a QSizeF) {
  fn resize(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget6resizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget6resizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setPalette(const QPalette & palette);
impl /*struct*/ QGraphicsWidget {
  pub fn setPalette<RetType, T: QGraphicsWidget_setPalette<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPalette(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setPalette<RetType> {
  fn setPalette(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setPalette(const QPalette & palette);
impl<'a> /*trait*/ QGraphicsWidget_setPalette<()> for (&'a QPalette) {
  fn setPalette(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget10setPaletteERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsWidget::autoFillBackground();
impl /*struct*/ QGraphicsWidget {
  pub fn autoFillBackground<RetType, T: QGraphicsWidget_autoFillBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoFillBackground(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_autoFillBackground<RetType> {
  fn autoFillBackground(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  bool QGraphicsWidget::autoFillBackground();
impl<'a> /*trait*/ QGraphicsWidget_autoFillBackground<i8> for () {
  fn autoFillBackground(self , rsthis: & QGraphicsWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget18autoFillBackgroundEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget18autoFillBackgroundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStyle * QGraphicsWidget::style();
impl /*struct*/ QGraphicsWidget {
  pub fn style<RetType, T: QGraphicsWidget_style<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.style(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_style<RetType> {
  fn style(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QStyle * QGraphicsWidget::style();
impl<'a> /*trait*/ QGraphicsWidget_style<QStyle> for () {
  fn style(self , rsthis: & QGraphicsWidget) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget5styleEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsWidget::shape();
impl /*struct*/ QGraphicsWidget {
  pub fn shape<RetType, T: QGraphicsWidget_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QPainterPath QGraphicsWidget::shape();
impl<'a> /*trait*/ QGraphicsWidget_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsWidget) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget5shapeEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setShortcutEnabled(int id, bool enabled);
impl /*struct*/ QGraphicsWidget {
  pub fn setShortcutEnabled<RetType, T: QGraphicsWidget_setShortcutEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setShortcutEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setShortcutEnabled<RetType> {
  fn setShortcutEnabled(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setShortcutEnabled(int id, bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setShortcutEnabled<()> for (i32, i8) {
  fn setShortcutEnabled(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget18setShortcutEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN15QGraphicsWidget18setShortcutEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::removeAction(QAction * action);
impl /*struct*/ QGraphicsWidget {
  pub fn removeAction<RetType, T: QGraphicsWidget_removeAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAction(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_removeAction<RetType> {
  fn removeAction(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::removeAction(QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_removeAction<()> for (&'a QAction) {
  fn removeAction(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget12removeActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::insertAction(QAction * before, QAction * action);
impl /*struct*/ QGraphicsWidget {
  pub fn insertAction<RetType, T: QGraphicsWidget_insertAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertAction(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_insertAction<RetType> {
  fn insertAction(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::insertAction(QAction * before, QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_insertAction<()> for (&'a QAction, &'a QAction) {
  fn insertAction(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget12insertActionEP7QActionS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget12insertActionEP7QActionS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QGraphicsWidget::close();
impl /*struct*/ QGraphicsWidget {
  pub fn close<RetType, T: QGraphicsWidget_close<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.close(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_close<RetType> {
  fn close(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  bool QGraphicsWidget::close();
impl<'a> /*trait*/ QGraphicsWidget_close<i8> for () {
  fn close(self , rsthis: & QGraphicsWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget5closeEv()};
    let mut ret = unsafe {_ZN15QGraphicsWidget5closeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsWidget::metaObject();
impl /*struct*/ QGraphicsWidget {
  pub fn metaObject<RetType, T: QGraphicsWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsWidget::metaObject();
impl<'a> /*trait*/ QGraphicsWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget10metaObjectEv()};
     unsafe {_ZNK15QGraphicsWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsWidget::boundingRect();
impl /*struct*/ QGraphicsWidget {
  pub fn boundingRect<RetType, T: QGraphicsWidget_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QRectF QGraphicsWidget::boundingRect();
impl<'a> /*trait*/ QGraphicsWidget_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsWidget) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget12boundingRectEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl /*struct*/ QGraphicsWidget {
  pub fn setContentsMargins<RetType, T: QGraphicsWidget_setContentsMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setContentsMargins<RetType> {
  fn setContentsMargins(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setContentsMargins(qreal left, qreal top, qreal right, qreal bottom);
impl<'a> /*trait*/ QGraphicsWidget_setContentsMargins<()> for (f64, f64, f64, f64) {
  fn setContentsMargins(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget18setContentsMarginsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN15QGraphicsWidget18setContentsMarginsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setFont(const QFont & font);
impl /*struct*/ QGraphicsWidget {
  pub fn setFont<RetType, T: QGraphicsWidget_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setFont<RetType> {
  fn setFont(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsWidget_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::geometryChanged();
impl /*struct*/ QGraphicsWidget {
  pub fn geometryChanged<RetType, T: QGraphicsWidget_geometryChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometryChanged(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_geometryChanged<RetType> {
  fn geometryChanged(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::geometryChanged();
impl<'a> /*trait*/ QGraphicsWidget_geometryChanged<()> for () {
  fn geometryChanged(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget15geometryChangedEv()};
     unsafe {_ZN15QGraphicsWidget15geometryChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QGraphicsWidget::windowTitle();
impl /*struct*/ QGraphicsWidget {
  pub fn windowTitle<RetType, T: QGraphicsWidget_windowTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.windowTitle(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_windowTitle<RetType> {
  fn windowTitle(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QString QGraphicsWidget::windowTitle();
impl<'a> /*trait*/ QGraphicsWidget_windowTitle<QString> for () {
  fn windowTitle(self , rsthis: & QGraphicsWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget11windowTitleEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget11windowTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsLayout * QGraphicsWidget::layout();
impl /*struct*/ QGraphicsWidget {
  pub fn layout<RetType, T: QGraphicsWidget_layout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.layout(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_layout<RetType> {
  fn layout(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QGraphicsLayout * QGraphicsWidget::layout();
impl<'a> /*trait*/ QGraphicsWidget_layout<()> for () {
  fn layout(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget6layoutEv()};
     unsafe {_ZNK15QGraphicsWidget6layoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::~QGraphicsWidget();
impl /*struct*/ QGraphicsWidget {
  pub fn Free<RetType, T: QGraphicsWidget_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::~QGraphicsWidget();
impl<'a> /*trait*/ QGraphicsWidget_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidgetD0Ev()};
     unsafe {_ZN15QGraphicsWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsWidget * QGraphicsWidget::focusWidget();
impl /*struct*/ QGraphicsWidget {
  pub fn focusWidget<RetType, T: QGraphicsWidget_focusWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusWidget(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_focusWidget<RetType> {
  fn focusWidget(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QGraphicsWidget * QGraphicsWidget::focusWidget();
impl<'a> /*trait*/ QGraphicsWidget_focusWidget<()> for () {
  fn focusWidget(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget11focusWidgetEv()};
     unsafe {_ZNK15QGraphicsWidget11focusWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::addAction(QAction * action);
impl /*struct*/ QGraphicsWidget {
  pub fn addAction<RetType, T: QGraphicsWidget_addAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addAction(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_addAction<RetType> {
  fn addAction(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::addAction(QAction * action);
impl<'a> /*trait*/ QGraphicsWidget_addAction<()> for (&'a QAction) {
  fn addAction(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget9addActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFont QGraphicsWidget::font();
impl /*struct*/ QGraphicsWidget {
  pub fn font<RetType, T: QGraphicsWidget_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_font<RetType> {
  fn font(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QFont QGraphicsWidget::font();
impl<'a> /*trait*/ QGraphicsWidget_font<QFont> for () {
  fn font(self , rsthis: & QGraphicsWidget) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget4fontEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QAction *> QGraphicsWidget::actions();
impl /*struct*/ QGraphicsWidget {
  pub fn actions<RetType, T: QGraphicsWidget_actions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actions(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_actions<RetType> {
  fn actions(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  QList<QAction *> QGraphicsWidget::actions();
impl<'a> /*trait*/ QGraphicsWidget_actions<()> for () {
  fn actions(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget7actionsEv()};
     unsafe {_ZNK15QGraphicsWidget7actionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::layoutChanged();
impl /*struct*/ QGraphicsWidget {
  pub fn layoutChanged<RetType, T: QGraphicsWidget_layoutChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.layoutChanged(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_layoutChanged<RetType> {
  fn layoutChanged(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::layoutChanged();
impl<'a> /*trait*/ QGraphicsWidget_layoutChanged<()> for () {
  fn layoutChanged(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget13layoutChangedEv()};
     unsafe {_ZN15QGraphicsWidget13layoutChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setShortcutAutoRepeat(int id, bool enabled);
impl /*struct*/ QGraphicsWidget {
  pub fn setShortcutAutoRepeat<RetType, T: QGraphicsWidget_setShortcutAutoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setShortcutAutoRepeat(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setShortcutAutoRepeat<RetType> {
  fn setShortcutAutoRepeat(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setShortcutAutoRepeat(int id, bool enabled);
impl<'a> /*trait*/ QGraphicsWidget_setShortcutAutoRepeat<()> for (i32, i8) {
  fn setShortcutAutoRepeat(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget21setShortcutAutoRepeatEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN15QGraphicsWidget21setShortcutAutoRepeatEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QGraphicsWidget::setTabOrder(QGraphicsWidget * first, QGraphicsWidget * second);
impl /*struct*/ QGraphicsWidget {
  pub fn setTabOrder_s<RetType, T: QGraphicsWidget_setTabOrder_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTabOrder_s();
    // return 1;
  }
}

pub trait QGraphicsWidget_setTabOrder_s<RetType> {
  fn setTabOrder_s(self ) -> RetType;
}

  // proto: static void QGraphicsWidget::setTabOrder(QGraphicsWidget * first, QGraphicsWidget * second);
impl<'a> /*trait*/ QGraphicsWidget_setTabOrder_s<()> for (&'a QGraphicsWidget, &'a QGraphicsWidget) {
  fn setTabOrder_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setTabOrderEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget11setTabOrderEPS_S0_(arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::getWindowFrameMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl /*struct*/ QGraphicsWidget {
  pub fn getWindowFrameMargins<RetType, T: QGraphicsWidget_getWindowFrameMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getWindowFrameMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_getWindowFrameMargins<RetType> {
  fn getWindowFrameMargins(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::getWindowFrameMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsWidget_getWindowFrameMargins<()> for (&'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn getWindowFrameMargins(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_double;
    let arg1 = self.1.as_ptr()  as *mut c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {_ZNK15QGraphicsWidget21getWindowFrameMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setStyle(QStyle * style);
impl /*struct*/ QGraphicsWidget {
  pub fn setStyle<RetType, T: QGraphicsWidget_setStyle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStyle(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_setStyle<RetType> {
  fn setStyle(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::setStyle(QStyle * style);
impl<'a> /*trait*/ QGraphicsWidget_setStyle<()> for (&'a QStyle) {
  fn setStyle(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QGraphicsWidget8setStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl /*struct*/ QGraphicsWidget {
  pub fn getContentsMargins<RetType, T: QGraphicsWidget_getContentsMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_getContentsMargins<RetType> {
  fn getContentsMargins(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  void QGraphicsWidget::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsWidget_getContentsMargins<()> for (&'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn getContentsMargins(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_double;
    let arg1 = self.1.as_ptr()  as *mut c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {_ZNK15QGraphicsWidget18getContentsMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QGraphicsWidget::isActiveWindow();
impl /*struct*/ QGraphicsWidget {
  pub fn isActiveWindow<RetType, T: QGraphicsWidget_isActiveWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActiveWindow(self);
    // return 1;
  }
}

pub trait QGraphicsWidget_isActiveWindow<RetType> {
  fn isActiveWindow(self , rsthis: & QGraphicsWidget) -> RetType;
}

  // proto:  bool QGraphicsWidget::isActiveWindow();
impl<'a> /*trait*/ QGraphicsWidget_isActiveWindow<i8> for () {
  fn isActiveWindow(self , rsthis: & QGraphicsWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsWidget14isActiveWindowEv()};
    let mut ret = unsafe {_ZNK15QGraphicsWidget14isActiveWindowEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsWidget::setGeometry(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsWidget_setGeometry<()> for (f64, f64, f64, f64) {
  fn setGeometry(self , rsthis: & QGraphicsWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsWidget11setGeometryEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {demth_ZN15QGraphicsWidget11setGeometryEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

#[derive(Default)] // for QGraphicsWidget_layoutChanged
pub struct QGraphicsWidget_layoutChanged_signal{poi:u64}
impl /* struct */ QGraphicsWidget {
  pub fn layoutChanged_1(&self) -> QGraphicsWidget_layoutChanged_signal {
     return QGraphicsWidget_layoutChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsWidget_layoutChanged_signal {
  pub fn connect<T: QGraphicsWidget_layoutChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsWidget_layoutChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsWidget_layoutChanged_signal);
}

#[derive(Default)] // for QGraphicsWidget_geometryChanged
pub struct QGraphicsWidget_geometryChanged_signal{poi:u64}
impl /* struct */ QGraphicsWidget {
  pub fn geometryChanged_1(&self) -> QGraphicsWidget_geometryChanged_signal {
     return QGraphicsWidget_geometryChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsWidget_geometryChanged_signal {
  pub fn connect<T: QGraphicsWidget_geometryChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsWidget_geometryChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsWidget_geometryChanged_signal);
}

// layoutChanged()
extern fn QGraphicsWidget_layoutChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsWidget_layoutChanged_signal_connect_cb_box_0(rsfptr_raw:*mut fn(), ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  rsfptr();
}
impl /* trait */ QGraphicsWidget_layoutChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsWidget_layoutChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsWidget_layoutChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsWidget_SlotProxy_connect__ZN15QGraphicsWidget13layoutChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsWidget_layoutChanged_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QGraphicsWidget_layoutChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsWidget_layoutChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QGraphicsWidget_SlotProxy_connect__ZN15QGraphicsWidget13layoutChangedEv(arg0, arg1, arg2)};
  }
}
// geometryChanged()
extern fn QGraphicsWidget_geometryChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsWidget_geometryChanged_signal_connect_cb_box_1(rsfptr_raw:*mut fn(), ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  rsfptr();
}
impl /* trait */ QGraphicsWidget_geometryChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsWidget_geometryChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsWidget_geometryChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsWidget_SlotProxy_connect__ZN15QGraphicsWidget15geometryChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsWidget_geometryChanged_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QGraphicsWidget_geometryChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsWidget_geometryChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QGraphicsWidget_SlotProxy_connect__ZN15QGraphicsWidget15geometryChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

