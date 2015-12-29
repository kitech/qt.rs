// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtWidgets/qgraphicsscene.h
// dst-file: /src/widgets/qgraphicsscene.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::gui::qbrush::QBrush; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::gui::qpainterpath::QPainterPath; // 771
use super::super::gui::qtransform::QTransform; // 771
use super::qgraphicsitem::QGraphicsItem; // 773
use super::super::core::qcoreevent::QEvent; // 771
use super::super::core::qpoint::QPointF; // 771
use super::super::gui::qpolygon::QPolygonF; // 771
use super::super::gui::qpen::QPen; // 771
use super::super::core::qline::QLineF; // 771
use super::super::gui::qpalette::QPalette; // 771
use super::super::gui::qpainter::QPainter; // 771
use super::super::core::qstring::QString; // 771
use super::super::gui::qfont::QFont; // 771
use super::super::gui::qpixmap::QPixmap; // 771
use super::qstyle::QStyle; // 773
use super::qgraphicswidget::QGraphicsWidget; // 773
use super::qgraphicsitem::QGraphicsItemGroup; // 773
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsScene_Class_Size() -> c_int;
  // proto:  void QGraphicsScene::setForegroundBrush(const QBrush & brush);
  fn _ZN14QGraphicsScene18setForegroundBrushERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsScene::setSceneRect(const QRectF & rect);
  fn _ZN14QGraphicsScene12setSceneRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QGraphicsScene::isActive();
  fn _ZNK14QGraphicsScene8isActiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QGraphicsScene::hasFocus();
  fn _ZNK14QGraphicsScene8hasFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRectF QGraphicsScene::itemsBoundingRect();
  fn _ZNK14QGraphicsScene17itemsBoundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsScene::sendEvent(QGraphicsItem * item, QEvent * event);
  fn _ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  qreal QGraphicsScene::minimumRenderSize();
  fn _ZNK14QGraphicsScene17minimumRenderSizeEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPainterPath QGraphicsScene::selectionArea();
  fn _ZNK14QGraphicsScene13selectionAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsScene::update(const QRectF & rect);
  fn _ZN14QGraphicsScene6updateERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsPolygonItem * QGraphicsScene::addPolygon(const QPolygonF & polygon, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QGraphicsLineItem * QGraphicsScene::addLine(const QLineF & line, const QPen & pen);
  fn _ZN14QGraphicsScene7addLineERK6QLineFRK4QPen(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QPalette QGraphicsScene::palette();
  fn _ZNK14QGraphicsScene7paletteEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsScene::isSortCacheEnabled();
  fn _ZNK14QGraphicsScene18isSortCacheEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsScene::QGraphicsScene(const QRectF & sceneRect, QObject * parent);
  fn dector_ZN14QGraphicsSceneC1ERK6QRectFP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN14QGraphicsSceneC1ERK6QRectFP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QGraphicsScene::QGraphicsScene(QObject * parent);
  fn dector_ZN14QGraphicsSceneC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QGraphicsSceneC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsScene::clearFocus();
  fn _ZN14QGraphicsScene10clearFocusEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QGraphicsScene::metaObject();
  fn _ZNK14QGraphicsScene10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QGraphicsSimpleTextItem * QGraphicsScene::addSimpleText(const QString & text, const QFont & font);
  fn _ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QGraphicsLineItem * QGraphicsScene::addLine(qreal x1, qreal y1, qreal x2, qreal y2, const QPen & pen);
  fn demth_ZN14QGraphicsScene7addLineEddddRK4QPen(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void);
  // proto:  void QGraphicsScene::setBspTreeDepth(int depth);
  fn _ZN14QGraphicsScene15setBspTreeDepthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRectF QGraphicsScene::sceneRect();
  fn _ZNK14QGraphicsScene9sceneRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QGraphicsWidget * QGraphicsScene::activeWindow();
  fn _ZNK14QGraphicsScene12activeWindowEv(qthis: u64 /* *mut c_void*/);
  // proto:  QBrush QGraphicsScene::backgroundBrush();
  fn _ZNK14QGraphicsScene15backgroundBrushEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsScene::itemAt(qreal x, qreal y, const QTransform & deviceTransform);
  fn demth_ZNK14QGraphicsScene6itemAtEddRK10QTransform(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: *mut c_void);
  // proto:  void QGraphicsScene::advance();
  fn _ZN14QGraphicsScene7advanceEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsScene::setStickyFocus(bool enabled);
  fn _ZN14QGraphicsScene14setStickyFocusEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QList<QGraphicsItem *> QGraphicsScene::selectedItems();
  fn _ZNK14QGraphicsScene13selectedItemsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsScene::clear();
  fn _ZN14QGraphicsScene5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsScene::setActivePanel(QGraphicsItem * item);
  fn _ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsPixmapItem * QGraphicsScene::addPixmap(const QPixmap & pixmap);
  fn _ZN14QGraphicsScene9addPixmapERK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QBrush QGraphicsScene::foregroundBrush();
  fn _ZNK14QGraphicsScene15foregroundBrushEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsScene::selectionChanged();
  fn _ZN14QGraphicsScene16selectionChangedEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QGraphicsView *> QGraphicsScene::views();
  fn _ZNK14QGraphicsScene5viewsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsScene::~QGraphicsScene();
  fn _ZN14QGraphicsSceneD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QGraphicsRectItem * QGraphicsScene::addRect(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
  fn demth_ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void, arg5: *mut c_void);
  // proto:  int QGraphicsScene::bspTreeDepth();
  fn _ZNK14QGraphicsScene12bspTreeDepthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGraphicsScene::setSceneRect(qreal x, qreal y, qreal w, qreal h);
  fn demth_ZN14QGraphicsScene12setSceneRectEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsScene::setStyle(QStyle * style);
  fn _ZN14QGraphicsScene8setStyleEP6QStyle(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsScene::setPalette(const QPalette & palette);
  fn _ZN14QGraphicsScene10setPaletteERK8QPalette(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsScene::setMinimumRenderSize(qreal minSize);
  fn _ZN14QGraphicsScene20setMinimumRenderSizeEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsScene::QGraphicsScene(qreal x, qreal y, qreal width, qreal height, QObject * parent);
  fn dector_ZN14QGraphicsSceneC1EddddP7QObject(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) -> *mut c_void;
  fn _ZN14QGraphicsSceneC1EddddP7QObject(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsScene::mouseGrabberItem();
  fn _ZNK14QGraphicsScene16mouseGrabberItemEv(qthis: u64 /* *mut c_void*/);
  // proto:  QGraphicsRectItem * QGraphicsScene::addRect(const QRectF & rect, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(const QRectF & rect, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  qreal QGraphicsScene::height();
  fn demth_ZNK14QGraphicsScene6heightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsScene::setSelectionArea(const QPainterPath & path, const QTransform & deviceTransform);
  fn _ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QFont QGraphicsScene::font();
  fn _ZNK14QGraphicsScene4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsScene::clearSelection();
  fn _ZN14QGraphicsScene14clearSelectionEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsScene::QGraphicsScene(const QGraphicsScene & );
  fn dector_ZN14QGraphicsSceneC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QGraphicsSceneC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsScene::removeItem(QGraphicsItem * item);
  fn _ZN14QGraphicsScene10removeItemEP13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
  fn demth_ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void, arg5: *mut c_void);
  // proto:  void QGraphicsScene::setActiveWindow(QGraphicsWidget * widget);
  fn _ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsScene::focusItem();
  fn _ZNK14QGraphicsScene9focusItemEv(qthis: u64 /* *mut c_void*/);
  // proto:  QGraphicsTextItem * QGraphicsScene::addText(const QString & text, const QFont & font);
  fn _ZN14QGraphicsScene7addTextERK7QStringRK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QGraphicsScene::setSortCacheEnabled(bool enabled);
  fn _ZN14QGraphicsScene19setSortCacheEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QGraphicsItem * QGraphicsScene::itemAt(const QPointF & pos, const QTransform & deviceTransform);
  fn _ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QGraphicsScene::destroyItemGroup(QGraphicsItemGroup * group);
  fn _ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QGraphicsScene::width();
  fn demth_ZNK14QGraphicsScene5widthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsScene::update(qreal x, qreal y, qreal w, qreal h);
  fn demth_ZN14QGraphicsScene6updateEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsScene::addItem(QGraphicsItem * item);
  fn _ZN14QGraphicsScene7addItemEP13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsScene::setBackgroundBrush(const QBrush & brush);
  fn _ZN14QGraphicsScene18setBackgroundBrushERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsScene::activePanel();
  fn _ZNK14QGraphicsScene11activePanelEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsScene::sceneRectChanged(const QRectF & rect);
  fn _ZN14QGraphicsScene16sceneRectChangedERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStyle * QGraphicsScene::style();
  fn _ZNK14QGraphicsScene5styleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsScene::setFont(const QFont & font);
  fn _ZN14QGraphicsScene7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsPathItem * QGraphicsScene::addPath(const QPainterPath & path, const QPen & pen, const QBrush & brush);
  fn _ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QGraphicsScene::stickyFocus();
  fn _ZNK14QGraphicsScene11stickyFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16selectionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScene_SlotProxy_connect_box__ZN14QGraphicsScene16selectionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16focusItemChangedEP13QGraphicsItemS1_N2Qt11FocusReasonE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScene_SlotProxy_connect_box__ZN14QGraphicsScene16focusItemChangedEP13QGraphicsItemS1_N2Qt11FocusReasonE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16sceneRectChangedERK6QRectF(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScene_SlotProxy_connect_box__ZN14QGraphicsScene16sceneRectChangedERK6QRectF(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsScene)=1
#[derive(Default)]
pub struct QGraphicsScene {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _changed_1: QGraphicsScene_changed_signal,
  pub _sceneRectChanged_1: QGraphicsScene_sceneRectChanged_signal,
  pub _selectionChanged_1: QGraphicsScene_selectionChanged_signal,
  pub _focusItemChanged_1: QGraphicsScene_focusItemChanged_signal,
}

impl /*struct*/ QGraphicsScene {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsScene {
    return QGraphicsScene{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsScene {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QGraphicsScene {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsScene::setForegroundBrush(const QBrush & brush);
impl /*struct*/ QGraphicsScene {
  pub fn setForegroundBrush<RetType, T: QGraphicsScene_setForegroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setForegroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setForegroundBrush<RetType> {
  fn setForegroundBrush(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setForegroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_setForegroundBrush<()> for (&'a QBrush) {
  fn setForegroundBrush(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene18setForegroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene18setForegroundBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setSceneRect(const QRectF & rect);
impl /*struct*/ QGraphicsScene {
  pub fn setSceneRect<RetType, T: QGraphicsScene_setSceneRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setSceneRect<RetType> {
  fn setSceneRect(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_setSceneRect<()> for (&'a QRectF) {
  fn setSceneRect(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene12setSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene12setSceneRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsScene::isActive();
impl /*struct*/ QGraphicsScene {
  pub fn isActive<RetType, T: QGraphicsScene_isActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QGraphicsScene_isActive<RetType> {
  fn isActive(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  bool QGraphicsScene::isActive();
impl<'a> /*trait*/ QGraphicsScene_isActive<i8> for () {
  fn isActive(self , rsthis: & QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene8isActiveEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QGraphicsScene::hasFocus();
impl /*struct*/ QGraphicsScene {
  pub fn hasFocus<RetType, T: QGraphicsScene_hasFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFocus(self);
    // return 1;
  }
}

pub trait QGraphicsScene_hasFocus<RetType> {
  fn hasFocus(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  bool QGraphicsScene::hasFocus();
impl<'a> /*trait*/ QGraphicsScene_hasFocus<i8> for () {
  fn hasFocus(self , rsthis: & QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene8hasFocusEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene8hasFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsScene::itemsBoundingRect();
impl /*struct*/ QGraphicsScene {
  pub fn itemsBoundingRect<RetType, T: QGraphicsScene_itemsBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemsBoundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsScene_itemsBoundingRect<RetType> {
  fn itemsBoundingRect(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QRectF QGraphicsScene::itemsBoundingRect();
impl<'a> /*trait*/ QGraphicsScene_itemsBoundingRect<QRectF> for () {
  fn itemsBoundingRect(self , rsthis: & QGraphicsScene) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene17itemsBoundingRectEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene17itemsBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsScene::sendEvent(QGraphicsItem * item, QEvent * event);
impl /*struct*/ QGraphicsScene {
  pub fn sendEvent<RetType, T: QGraphicsScene_sendEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sendEvent(self);
    // return 1;
  }
}

pub trait QGraphicsScene_sendEvent<RetType> {
  fn sendEvent(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  bool QGraphicsScene::sendEvent(QGraphicsItem * item, QEvent * event);
impl<'a> /*trait*/ QGraphicsScene_sendEvent<i8> for (&'a QGraphicsItem, &'a QEvent) {
  fn sendEvent(self , rsthis: & QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QGraphicsScene::minimumRenderSize();
impl /*struct*/ QGraphicsScene {
  pub fn minimumRenderSize<RetType, T: QGraphicsScene_minimumRenderSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumRenderSize(self);
    // return 1;
  }
}

pub trait QGraphicsScene_minimumRenderSize<RetType> {
  fn minimumRenderSize(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  qreal QGraphicsScene::minimumRenderSize();
impl<'a> /*trait*/ QGraphicsScene_minimumRenderSize<f64> for () {
  fn minimumRenderSize(self , rsthis: & QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene17minimumRenderSizeEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene17minimumRenderSizeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsScene::selectionArea();
impl /*struct*/ QGraphicsScene {
  pub fn selectionArea<RetType, T: QGraphicsScene_selectionArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionArea(self);
    // return 1;
  }
}

pub trait QGraphicsScene_selectionArea<RetType> {
  fn selectionArea(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QPainterPath QGraphicsScene::selectionArea();
impl<'a> /*trait*/ QGraphicsScene_selectionArea<QPainterPath> for () {
  fn selectionArea(self , rsthis: & QGraphicsScene) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene13selectionAreaEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene13selectionAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::update(const QRectF & rect);
impl /*struct*/ QGraphicsScene {
  pub fn update<RetType, T: QGraphicsScene_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QGraphicsScene_update<RetType> {
  fn update(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::update(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_update<()> for (&'a QRectF) {
  fn update(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene6updateERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene6updateERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsPolygonItem * QGraphicsScene::addPolygon(const QPolygonF & polygon, const QPen & pen, const QBrush & brush);
impl /*struct*/ QGraphicsScene {
  pub fn addPolygon<RetType, T: QGraphicsScene_addPolygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPolygon(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addPolygon<RetType> {
  fn addPolygon(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsPolygonItem * QGraphicsScene::addPolygon(const QPolygonF & polygon, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addPolygon<()> for (&'a QPolygonF, &'a QPen, &'a QBrush) {
  fn addPolygon(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QGraphicsLineItem * QGraphicsScene::addLine(const QLineF & line, const QPen & pen);
impl /*struct*/ QGraphicsScene {
  pub fn addLine<RetType, T: QGraphicsScene_addLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addLine(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addLine<RetType> {
  fn addLine(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsLineItem * QGraphicsScene::addLine(const QLineF & line, const QPen & pen);
impl<'a> /*trait*/ QGraphicsScene_addLine<()> for (&'a QLineF, &'a QPen) {
  fn addLine(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addLineERK6QLineFRK4QPen()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addLineERK6QLineFRK4QPen(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPalette QGraphicsScene::palette();
impl /*struct*/ QGraphicsScene {
  pub fn palette<RetType, T: QGraphicsScene_palette<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.palette(self);
    // return 1;
  }
}

pub trait QGraphicsScene_palette<RetType> {
  fn palette(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QPalette QGraphicsScene::palette();
impl<'a> /*trait*/ QGraphicsScene_palette<QPalette> for () {
  fn palette(self , rsthis: & QGraphicsScene) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene7paletteEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene7paletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsScene::isSortCacheEnabled();
impl /*struct*/ QGraphicsScene {
  pub fn isSortCacheEnabled<RetType, T: QGraphicsScene_isSortCacheEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSortCacheEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsScene_isSortCacheEnabled<RetType> {
  fn isSortCacheEnabled(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  bool QGraphicsScene::isSortCacheEnabled();
impl<'a> /*trait*/ QGraphicsScene_isSortCacheEnabled<i8> for () {
  fn isSortCacheEnabled(self , rsthis: & QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene18isSortCacheEnabledEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene18isSortCacheEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::QGraphicsScene(const QRectF & sceneRect, QObject * parent);
impl /*struct*/ QGraphicsScene {
  pub fn New<T: QGraphicsScene_New>(value: T) -> QGraphicsScene {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScene_New {
  fn New(self) -> QGraphicsScene;
}

  // proto:  void QGraphicsScene::QGraphicsScene(const QRectF & sceneRect, QObject * parent);
impl<'a> /*trait*/ QGraphicsScene_New for (&'a QRectF, &'a QObject) {
  fn New(self) -> QGraphicsScene {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1ERK6QRectFP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsScene_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN14QGraphicsSceneC1ERK6QRectFP7QObject(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN14QGraphicsSceneC1ERK6QRectFP7QObject(arg0, arg1)} as u64;
    let rsthis = QGraphicsScene{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::QGraphicsScene(QObject * parent);
impl<'a> /*trait*/ QGraphicsScene_New for (&'a QObject) {
  fn New(self) -> QGraphicsScene {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsScene_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QGraphicsSceneC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QGraphicsSceneC1EP7QObject(arg0)} as u64;
    let rsthis = QGraphicsScene{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::clearFocus();
impl /*struct*/ QGraphicsScene {
  pub fn clearFocus<RetType, T: QGraphicsScene_clearFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearFocus(self);
    // return 1;
  }
}

pub trait QGraphicsScene_clearFocus<RetType> {
  fn clearFocus(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::clearFocus();
impl<'a> /*trait*/ QGraphicsScene_clearFocus<()> for () {
  fn clearFocus(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10clearFocusEv()};
     unsafe {_ZN14QGraphicsScene10clearFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsScene::metaObject();
impl /*struct*/ QGraphicsScene {
  pub fn metaObject<RetType, T: QGraphicsScene_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsScene_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsScene::metaObject();
impl<'a> /*trait*/ QGraphicsScene_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene10metaObjectEv()};
     unsafe {_ZNK14QGraphicsScene10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsSimpleTextItem * QGraphicsScene::addSimpleText(const QString & text, const QFont & font);
impl /*struct*/ QGraphicsScene {
  pub fn addSimpleText<RetType, T: QGraphicsScene_addSimpleText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addSimpleText(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addSimpleText<RetType> {
  fn addSimpleText(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsSimpleTextItem * QGraphicsScene::addSimpleText(const QString & text, const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_addSimpleText<()> for (&'a QString, &'a QFont) {
  fn addSimpleText(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QGraphicsLineItem * QGraphicsScene::addLine(qreal x1, qreal y1, qreal x2, qreal y2, const QPen & pen);
impl<'a> /*trait*/ QGraphicsScene_addLine<()> for (f64, f64, f64, f64, &'a QPen) {
  fn addLine(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addLineEddddRK4QPen()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {demth_ZN14QGraphicsScene7addLineEddddRK4QPen(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setBspTreeDepth(int depth);
impl /*struct*/ QGraphicsScene {
  pub fn setBspTreeDepth<RetType, T: QGraphicsScene_setBspTreeDepth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBspTreeDepth(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setBspTreeDepth<RetType> {
  fn setBspTreeDepth(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setBspTreeDepth(int depth);
impl<'a> /*trait*/ QGraphicsScene_setBspTreeDepth<()> for (i32) {
  fn setBspTreeDepth(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene15setBspTreeDepthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QGraphicsScene15setBspTreeDepthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsScene::sceneRect();
impl /*struct*/ QGraphicsScene {
  pub fn sceneRect<RetType, T: QGraphicsScene_sceneRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsScene_sceneRect<RetType> {
  fn sceneRect(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QRectF QGraphicsScene::sceneRect();
impl<'a> /*trait*/ QGraphicsScene_sceneRect<QRectF> for () {
  fn sceneRect(self , rsthis: & QGraphicsScene) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene9sceneRectEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene9sceneRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsWidget * QGraphicsScene::activeWindow();
impl /*struct*/ QGraphicsScene {
  pub fn activeWindow<RetType, T: QGraphicsScene_activeWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeWindow(self);
    // return 1;
  }
}

pub trait QGraphicsScene_activeWindow<RetType> {
  fn activeWindow(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsWidget * QGraphicsScene::activeWindow();
impl<'a> /*trait*/ QGraphicsScene_activeWindow<()> for () {
  fn activeWindow(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene12activeWindowEv()};
     unsafe {_ZNK14QGraphicsScene12activeWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QBrush QGraphicsScene::backgroundBrush();
impl /*struct*/ QGraphicsScene {
  pub fn backgroundBrush<RetType, T: QGraphicsScene_backgroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backgroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsScene_backgroundBrush<RetType> {
  fn backgroundBrush(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QBrush QGraphicsScene::backgroundBrush();
impl<'a> /*trait*/ QGraphicsScene_backgroundBrush<QBrush> for () {
  fn backgroundBrush(self , rsthis: & QGraphicsScene) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene15backgroundBrushEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene15backgroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsScene::itemAt(qreal x, qreal y, const QTransform & deviceTransform);
impl /*struct*/ QGraphicsScene {
  pub fn itemAt<RetType, T: QGraphicsScene_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsScene_itemAt<RetType> {
  fn itemAt(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsScene::itemAt(qreal x, qreal y, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_itemAt<()> for (f64, f64, &'a QTransform) {
  fn itemAt(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6itemAtEddRK10QTransform()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {demth_ZNK14QGraphicsScene6itemAtEddRK10QTransform(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::advance();
impl /*struct*/ QGraphicsScene {
  pub fn advance<RetType, T: QGraphicsScene_advance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.advance(self);
    // return 1;
  }
}

pub trait QGraphicsScene_advance<RetType> {
  fn advance(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::advance();
impl<'a> /*trait*/ QGraphicsScene_advance<()> for () {
  fn advance(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7advanceEv()};
     unsafe {_ZN14QGraphicsScene7advanceEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setStickyFocus(bool enabled);
impl /*struct*/ QGraphicsScene {
  pub fn setStickyFocus<RetType, T: QGraphicsScene_setStickyFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStickyFocus(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setStickyFocus<RetType> {
  fn setStickyFocus(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setStickyFocus(bool enabled);
impl<'a> /*trait*/ QGraphicsScene_setStickyFocus<()> for (i8) {
  fn setStickyFocus(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14setStickyFocusEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QGraphicsScene14setStickyFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsItem *> QGraphicsScene::selectedItems();
impl /*struct*/ QGraphicsScene {
  pub fn selectedItems<RetType, T: QGraphicsScene_selectedItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedItems(self);
    // return 1;
  }
}

pub trait QGraphicsScene_selectedItems<RetType> {
  fn selectedItems(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QList<QGraphicsItem *> QGraphicsScene::selectedItems();
impl<'a> /*trait*/ QGraphicsScene_selectedItems<()> for () {
  fn selectedItems(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene13selectedItemsEv()};
     unsafe {_ZNK14QGraphicsScene13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::clear();
impl /*struct*/ QGraphicsScene {
  pub fn clear<RetType, T: QGraphicsScene_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QGraphicsScene_clear<RetType> {
  fn clear(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::clear();
impl<'a> /*trait*/ QGraphicsScene_clear<()> for () {
  fn clear(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene5clearEv()};
     unsafe {_ZN14QGraphicsScene5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setActivePanel(QGraphicsItem * item);
impl /*struct*/ QGraphicsScene {
  pub fn setActivePanel<RetType, T: QGraphicsScene_setActivePanel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActivePanel(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setActivePanel<RetType> {
  fn setActivePanel(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setActivePanel(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_setActivePanel<()> for (&'a QGraphicsItem) {
  fn setActivePanel(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsPixmapItem * QGraphicsScene::addPixmap(const QPixmap & pixmap);
impl /*struct*/ QGraphicsScene {
  pub fn addPixmap<RetType, T: QGraphicsScene_addPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPixmap(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addPixmap<RetType> {
  fn addPixmap(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsPixmapItem * QGraphicsScene::addPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QGraphicsScene_addPixmap<()> for (&'a QPixmap) {
  fn addPixmap(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene9addPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene9addPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QBrush QGraphicsScene::foregroundBrush();
impl /*struct*/ QGraphicsScene {
  pub fn foregroundBrush<RetType, T: QGraphicsScene_foregroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.foregroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsScene_foregroundBrush<RetType> {
  fn foregroundBrush(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QBrush QGraphicsScene::foregroundBrush();
impl<'a> /*trait*/ QGraphicsScene_foregroundBrush<QBrush> for () {
  fn foregroundBrush(self , rsthis: & QGraphicsScene) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene15foregroundBrushEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene15foregroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::selectionChanged();
impl /*struct*/ QGraphicsScene {
  pub fn selectionChanged<RetType, T: QGraphicsScene_selectionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScene_selectionChanged<RetType> {
  fn selectionChanged(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::selectionChanged();
impl<'a> /*trait*/ QGraphicsScene_selectionChanged<()> for () {
  fn selectionChanged(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16selectionChangedEv()};
     unsafe {_ZN14QGraphicsScene16selectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsView *> QGraphicsScene::views();
impl /*struct*/ QGraphicsScene {
  pub fn views<RetType, T: QGraphicsScene_views<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.views(self);
    // return 1;
  }
}

pub trait QGraphicsScene_views<RetType> {
  fn views(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QList<QGraphicsView *> QGraphicsScene::views();
impl<'a> /*trait*/ QGraphicsScene_views<()> for () {
  fn views(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5viewsEv()};
     unsafe {_ZNK14QGraphicsScene5viewsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::~QGraphicsScene();
impl /*struct*/ QGraphicsScene {
  pub fn Free<RetType, T: QGraphicsScene_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsScene_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::~QGraphicsScene();
impl<'a> /*trait*/ QGraphicsScene_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneD0Ev()};
     unsafe {_ZN14QGraphicsSceneD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsRectItem * QGraphicsScene::addRect(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
impl /*struct*/ QGraphicsScene {
  pub fn addRect<RetType, T: QGraphicsScene_addRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addRect(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addRect<RetType> {
  fn addRect(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsRectItem * QGraphicsScene::addRect(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addRect<()> for (f64, f64, f64, f64, &'a QPen, &'a QBrush) {
  fn addRect(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
     unsafe {demth_ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  int QGraphicsScene::bspTreeDepth();
impl /*struct*/ QGraphicsScene {
  pub fn bspTreeDepth<RetType, T: QGraphicsScene_bspTreeDepth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bspTreeDepth(self);
    // return 1;
  }
}

pub trait QGraphicsScene_bspTreeDepth<RetType> {
  fn bspTreeDepth(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  int QGraphicsScene::bspTreeDepth();
impl<'a> /*trait*/ QGraphicsScene_bspTreeDepth<i32> for () {
  fn bspTreeDepth(self , rsthis: & QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene12bspTreeDepthEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene12bspTreeDepthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setSceneRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsScene_setSceneRect<()> for (f64, f64, f64, f64) {
  fn setSceneRect(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene12setSceneRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {demth_ZN14QGraphicsScene12setSceneRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setStyle(QStyle * style);
impl /*struct*/ QGraphicsScene {
  pub fn setStyle<RetType, T: QGraphicsScene_setStyle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStyle(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setStyle<RetType> {
  fn setStyle(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setStyle(QStyle * style);
impl<'a> /*trait*/ QGraphicsScene_setStyle<()> for (&'a QStyle) {
  fn setStyle(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene8setStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setPalette(const QPalette & palette);
impl /*struct*/ QGraphicsScene {
  pub fn setPalette<RetType, T: QGraphicsScene_setPalette<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPalette(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setPalette<RetType> {
  fn setPalette(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setPalette(const QPalette & palette);
impl<'a> /*trait*/ QGraphicsScene_setPalette<()> for (&'a QPalette) {
  fn setPalette(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10setPaletteERK8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setMinimumRenderSize(qreal minSize);
impl /*struct*/ QGraphicsScene {
  pub fn setMinimumRenderSize<RetType, T: QGraphicsScene_setMinimumRenderSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumRenderSize(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setMinimumRenderSize<RetType> {
  fn setMinimumRenderSize(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setMinimumRenderSize(qreal minSize);
impl<'a> /*trait*/ QGraphicsScene_setMinimumRenderSize<()> for (f64) {
  fn setMinimumRenderSize(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene20setMinimumRenderSizeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QGraphicsScene20setMinimumRenderSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::QGraphicsScene(qreal x, qreal y, qreal width, qreal height, QObject * parent);
impl<'a> /*trait*/ QGraphicsScene_New for (f64, f64, f64, f64, &'a QObject) {
  fn New(self) -> QGraphicsScene {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1EddddP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsScene_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    // unsafe {_ZN14QGraphicsSceneC1EddddP7QObject(qthis, arg0, arg1, arg2, arg3, arg4)};
    let qthis: u64 = unsafe {dector_ZN14QGraphicsSceneC1EddddP7QObject(arg0, arg1, arg2, arg3, arg4)} as u64;
    let rsthis = QGraphicsScene{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsScene::mouseGrabberItem();
impl /*struct*/ QGraphicsScene {
  pub fn mouseGrabberItem<RetType, T: QGraphicsScene_mouseGrabberItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mouseGrabberItem(self);
    // return 1;
  }
}

pub trait QGraphicsScene_mouseGrabberItem<RetType> {
  fn mouseGrabberItem(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsScene::mouseGrabberItem();
impl<'a> /*trait*/ QGraphicsScene_mouseGrabberItem<()> for () {
  fn mouseGrabberItem(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene16mouseGrabberItemEv()};
     unsafe {_ZNK14QGraphicsScene16mouseGrabberItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsRectItem * QGraphicsScene::addRect(const QRectF & rect, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addRect<()> for (&'a QRectF, &'a QPen, &'a QBrush) {
  fn addRect(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(const QRectF & rect, const QPen & pen, const QBrush & brush);
impl /*struct*/ QGraphicsScene {
  pub fn addEllipse<RetType, T: QGraphicsScene_addEllipse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addEllipse(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addEllipse<RetType> {
  fn addEllipse(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(const QRectF & rect, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addEllipse<()> for (&'a QRectF, &'a QPen, &'a QBrush) {
  fn addEllipse(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsScene::height();
impl /*struct*/ QGraphicsScene {
  pub fn height<RetType, T: QGraphicsScene_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QGraphicsScene_height<RetType> {
  fn height(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  qreal QGraphicsScene::height();
impl<'a> /*trait*/ QGraphicsScene_height<f64> for () {
  fn height(self , rsthis: & QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6heightEv()};
    let mut ret = unsafe {demth_ZNK14QGraphicsScene6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setSelectionArea(const QPainterPath & path, const QTransform & deviceTransform);
impl /*struct*/ QGraphicsScene {
  pub fn setSelectionArea<RetType, T: QGraphicsScene_setSelectionArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelectionArea(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setSelectionArea<RetType> {
  fn setSelectionArea(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setSelectionArea(const QPainterPath & path, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_setSelectionArea<()> for (&'a QPainterPath, &'a QTransform) {
  fn setSelectionArea(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QFont QGraphicsScene::font();
impl /*struct*/ QGraphicsScene {
  pub fn font<RetType, T: QGraphicsScene_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QGraphicsScene_font<RetType> {
  fn font(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QFont QGraphicsScene::font();
impl<'a> /*trait*/ QGraphicsScene_font<QFont> for () {
  fn font(self , rsthis: & QGraphicsScene) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene4fontEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::clearSelection();
impl /*struct*/ QGraphicsScene {
  pub fn clearSelection<RetType, T: QGraphicsScene_clearSelection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearSelection(self);
    // return 1;
  }
}

pub trait QGraphicsScene_clearSelection<RetType> {
  fn clearSelection(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::clearSelection();
impl<'a> /*trait*/ QGraphicsScene_clearSelection<()> for () {
  fn clearSelection(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene14clearSelectionEv()};
     unsafe {_ZN14QGraphicsScene14clearSelectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::QGraphicsScene(const QGraphicsScene & );
impl<'a> /*trait*/ QGraphicsScene_New for (&'a QGraphicsScene) {
  fn New(self) -> QGraphicsScene {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsSceneC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsScene_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QGraphicsSceneC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QGraphicsSceneC1ERKS_(arg0)} as u64;
    let rsthis = QGraphicsScene{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::removeItem(QGraphicsItem * item);
impl /*struct*/ QGraphicsScene {
  pub fn removeItem<RetType, T: QGraphicsScene_removeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeItem(self);
    // return 1;
  }
}

pub trait QGraphicsScene_removeItem<RetType> {
  fn removeItem(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::removeItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_removeItem<()> for (&'a QGraphicsItem) {
  fn removeItem(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10removeItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene10removeItemEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsEllipseItem * QGraphicsScene::addEllipse(qreal x, qreal y, qreal w, qreal h, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addEllipse<()> for (f64, f64, f64, f64, &'a QPen, &'a QBrush) {
  fn addEllipse(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
     unsafe {demth_ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setActiveWindow(QGraphicsWidget * widget);
impl /*struct*/ QGraphicsScene {
  pub fn setActiveWindow<RetType, T: QGraphicsScene_setActiveWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActiveWindow(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setActiveWindow<RetType> {
  fn setActiveWindow(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setActiveWindow(QGraphicsWidget * widget);
impl<'a> /*trait*/ QGraphicsScene_setActiveWindow<()> for (&'a QGraphicsWidget) {
  fn setActiveWindow(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsScene::focusItem();
impl /*struct*/ QGraphicsScene {
  pub fn focusItem<RetType, T: QGraphicsScene_focusItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusItem(self);
    // return 1;
  }
}

pub trait QGraphicsScene_focusItem<RetType> {
  fn focusItem(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsScene::focusItem();
impl<'a> /*trait*/ QGraphicsScene_focusItem<()> for () {
  fn focusItem(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene9focusItemEv()};
     unsafe {_ZNK14QGraphicsScene9focusItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsTextItem * QGraphicsScene::addText(const QString & text, const QFont & font);
impl /*struct*/ QGraphicsScene {
  pub fn addText<RetType, T: QGraphicsScene_addText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addText(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addText<RetType> {
  fn addText(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsTextItem * QGraphicsScene::addText(const QString & text, const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_addText<()> for (&'a QString, &'a QFont) {
  fn addText(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addTextERK7QStringRK5QFont()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addTextERK7QStringRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setSortCacheEnabled(bool enabled);
impl /*struct*/ QGraphicsScene {
  pub fn setSortCacheEnabled<RetType, T: QGraphicsScene_setSortCacheEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSortCacheEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setSortCacheEnabled<RetType> {
  fn setSortCacheEnabled(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setSortCacheEnabled(bool enabled);
impl<'a> /*trait*/ QGraphicsScene_setSortCacheEnabled<()> for (i8) {
  fn setSortCacheEnabled(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene19setSortCacheEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QGraphicsScene19setSortCacheEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsScene::itemAt(const QPointF & pos, const QTransform & deviceTransform);
impl<'a> /*trait*/ QGraphicsScene_itemAt<()> for (&'a QPointF, &'a QTransform) {
  fn itemAt(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::destroyItemGroup(QGraphicsItemGroup * group);
impl /*struct*/ QGraphicsScene {
  pub fn destroyItemGroup<RetType, T: QGraphicsScene_destroyItemGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroyItemGroup(self);
    // return 1;
  }
}

pub trait QGraphicsScene_destroyItemGroup<RetType> {
  fn destroyItemGroup(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::destroyItemGroup(QGraphicsItemGroup * group);
impl<'a> /*trait*/ QGraphicsScene_destroyItemGroup<()> for (&'a QGraphicsItemGroup) {
  fn destroyItemGroup(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsScene::width();
impl /*struct*/ QGraphicsScene {
  pub fn width<RetType, T: QGraphicsScene_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QGraphicsScene_width<RetType> {
  fn width(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  qreal QGraphicsScene::width();
impl<'a> /*trait*/ QGraphicsScene_width<f64> for () {
  fn width(self , rsthis: & QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5widthEv()};
    let mut ret = unsafe {demth_ZNK14QGraphicsScene5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::update(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsScene_update<()> for (f64, f64, f64, f64) {
  fn update(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene6updateEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {demth_ZN14QGraphicsScene6updateEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::addItem(QGraphicsItem * item);
impl /*struct*/ QGraphicsScene {
  pub fn addItem<RetType, T: QGraphicsScene_addItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addItem<RetType> {
  fn addItem(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::addItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsScene_addItem<()> for (&'a QGraphicsItem) {
  fn addItem(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addItemEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setBackgroundBrush(const QBrush & brush);
impl /*struct*/ QGraphicsScene {
  pub fn setBackgroundBrush<RetType, T: QGraphicsScene_setBackgroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setBackgroundBrush<RetType> {
  fn setBackgroundBrush(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setBackgroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_setBackgroundBrush<()> for (&'a QBrush) {
  fn setBackgroundBrush(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene18setBackgroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene18setBackgroundBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsScene::activePanel();
impl /*struct*/ QGraphicsScene {
  pub fn activePanel<RetType, T: QGraphicsScene_activePanel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activePanel(self);
    // return 1;
  }
}

pub trait QGraphicsScene_activePanel<RetType> {
  fn activePanel(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsScene::activePanel();
impl<'a> /*trait*/ QGraphicsScene_activePanel<()> for () {
  fn activePanel(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene11activePanelEv()};
     unsafe {_ZNK14QGraphicsScene11activePanelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScene::sceneRectChanged(const QRectF & rect);
impl /*struct*/ QGraphicsScene {
  pub fn sceneRectChanged<RetType, T: QGraphicsScene_sceneRectChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sceneRectChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScene_sceneRectChanged<RetType> {
  fn sceneRectChanged(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::sceneRectChanged(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsScene_sceneRectChanged<()> for (&'a QRectF) {
  fn sceneRectChanged(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene16sceneRectChangedERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene16sceneRectChangedERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStyle * QGraphicsScene::style();
impl /*struct*/ QGraphicsScene {
  pub fn style<RetType, T: QGraphicsScene_style<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.style(self);
    // return 1;
  }
}

pub trait QGraphicsScene_style<RetType> {
  fn style(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QStyle * QGraphicsScene::style();
impl<'a> /*trait*/ QGraphicsScene_style<QStyle> for () {
  fn style(self , rsthis: & QGraphicsScene) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene5styleEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsScene::setFont(const QFont & font);
impl /*struct*/ QGraphicsScene {
  pub fn setFont<RetType, T: QGraphicsScene_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsScene_setFont<RetType> {
  fn setFont(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  void QGraphicsScene::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsScene_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsPathItem * QGraphicsScene::addPath(const QPainterPath & path, const QPen & pen, const QBrush & brush);
impl /*struct*/ QGraphicsScene {
  pub fn addPath<RetType, T: QGraphicsScene_addPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPath(self);
    // return 1;
  }
}

pub trait QGraphicsScene_addPath<RetType> {
  fn addPath(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  QGraphicsPathItem * QGraphicsScene::addPath(const QPainterPath & path, const QPen & pen, const QBrush & brush);
impl<'a> /*trait*/ QGraphicsScene_addPath<()> for (&'a QPainterPath, &'a QPen, &'a QBrush) {
  fn addPath(self , rsthis: & QGraphicsScene) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QGraphicsScene::stickyFocus();
impl /*struct*/ QGraphicsScene {
  pub fn stickyFocus<RetType, T: QGraphicsScene_stickyFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stickyFocus(self);
    // return 1;
  }
}

pub trait QGraphicsScene_stickyFocus<RetType> {
  fn stickyFocus(self , rsthis: & QGraphicsScene) -> RetType;
}

  // proto:  bool QGraphicsScene::stickyFocus();
impl<'a> /*trait*/ QGraphicsScene_stickyFocus<i8> for () {
  fn stickyFocus(self , rsthis: & QGraphicsScene) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScene11stickyFocusEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScene11stickyFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QGraphicsScene_changed
pub struct QGraphicsScene_changed_signal{poi:u64}
impl /* struct */ QGraphicsScene {
  pub fn changed_1(&self) -> QGraphicsScene_changed_signal {
     return QGraphicsScene_changed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScene_changed_signal {
  pub fn connect<T: QGraphicsScene_changed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScene_changed_signal_connect {
  fn connect(self, sigthis: QGraphicsScene_changed_signal);
}

#[derive(Default)] // for QGraphicsScene_sceneRectChanged
pub struct QGraphicsScene_sceneRectChanged_signal{poi:u64}
impl /* struct */ QGraphicsScene {
  pub fn sceneRectChanged_1(&self) -> QGraphicsScene_sceneRectChanged_signal {
     return QGraphicsScene_sceneRectChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScene_sceneRectChanged_signal {
  pub fn connect<T: QGraphicsScene_sceneRectChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScene_sceneRectChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsScene_sceneRectChanged_signal);
}

#[derive(Default)] // for QGraphicsScene_selectionChanged
pub struct QGraphicsScene_selectionChanged_signal{poi:u64}
impl /* struct */ QGraphicsScene {
  pub fn selectionChanged_1(&self) -> QGraphicsScene_selectionChanged_signal {
     return QGraphicsScene_selectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScene_selectionChanged_signal {
  pub fn connect<T: QGraphicsScene_selectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScene_selectionChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsScene_selectionChanged_signal);
}

#[derive(Default)] // for QGraphicsScene_focusItemChanged
pub struct QGraphicsScene_focusItemChanged_signal{poi:u64}
impl /* struct */ QGraphicsScene {
  pub fn focusItemChanged_1(&self) -> QGraphicsScene_focusItemChanged_signal {
     return QGraphicsScene_focusItemChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScene_focusItemChanged_signal {
  pub fn connect<T: QGraphicsScene_focusItemChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScene_focusItemChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsScene_focusItemChanged_signal);
}

// selectionChanged()
extern fn QGraphicsScene_selectionChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
}
extern fn QGraphicsScene_selectionChanged_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QGraphicsScene_selectionChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsScene_selectionChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScene_selectionChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16selectionChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsScene_selectionChanged_signal_connect for Box<fn()> {
  fn connect(self, sigthis: QGraphicsScene_selectionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScene_selectionChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16selectionChangedEv(arg0, arg1, arg2)};
  }
}
// focusItemChanged(class QGraphicsItem *, class QGraphicsItem *, Qt::FocusReason)
extern fn QGraphicsScene_focusItemChanged_signal_connect_cb_1(rsfptr:fn(QGraphicsItem, QGraphicsItem, i32), arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QGraphicsScene_focusItemChanged_signal_connect_cb_box_1(rsfptr_raw:*mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QGraphicsScene_focusItemChanged_signal_connect for fn(QGraphicsItem, QGraphicsItem, i32) {
  fn connect(self, sigthis: QGraphicsScene_focusItemChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScene_focusItemChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16focusItemChangedEP13QGraphicsItemS1_N2Qt11FocusReasonE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsScene_focusItemChanged_signal_connect for Box<fn(QGraphicsItem, QGraphicsItem, i32)> {
  fn connect(self, sigthis: QGraphicsScene_focusItemChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScene_focusItemChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16focusItemChangedEP13QGraphicsItemS1_N2Qt11FocusReasonE(arg0, arg1, arg2)};
  }
}
// sceneRectChanged(const class QRectF &)
extern fn QGraphicsScene_sceneRectChanged_signal_connect_cb_2(rsfptr:fn(QRectF), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QGraphicsScene_sceneRectChanged_signal_connect_cb_box_2(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QGraphicsScene_sceneRectChanged_signal_connect for fn(QRectF) {
  fn connect(self, sigthis: QGraphicsScene_sceneRectChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScene_sceneRectChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16sceneRectChangedERK6QRectF(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsScene_sceneRectChanged_signal_connect for Box<fn(QRectF)> {
  fn connect(self, sigthis: QGraphicsScene_sceneRectChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScene_sceneRectChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QGraphicsScene_SlotProxy_connect__ZN14QGraphicsScene16sceneRectChangedERK6QRectF(arg0, arg1, arg2)};
  }
}
// <= body block end

