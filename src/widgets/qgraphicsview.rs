// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qgraphicsview.h
// dst-file: /src/widgets/qgraphicsview.rs
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
use super::qabstractscrollarea::QAbstractScrollArea; // 773
use std::ops::Deref;
use super::super::core::qrect::QRect; // 771
use super::super::gui::qpolygon::QPolygonF; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::gui::qpolygon::QPolygon; // 771
use super::super::core::qpoint::QPointF; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::gui::qmatrix::QMatrix; // 771
use super::super::gui::qbrush::QBrush; // 771
use super::super::core::qpoint::QPoint; // 771
use super::qgraphicsscene::QGraphicsScene; // 773
use super::qwidget::QWidget; // 773
use super::qgraphicsitem::QGraphicsItem; // 773
use super::super::gui::qtransform::QTransform; // 771
use super::super::core::qsize::QSize; // 771
use super::super::gui::qpainterpath::QPainterPath; // 771
use super::super::gui::qpainter::QPainter; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsView_Class_Size() -> c_int;
  // proto:  void QGraphicsView::scale(qreal sx, qreal sy);
  fn C_ZN13QGraphicsView5scaleEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  QPolygonF QGraphicsView::mapToScene(const QRect & rect);
  fn C_ZNK13QGraphicsView10mapToSceneERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QGraphicsView::mapFromScene(const QRectF & rect);
  fn C_ZNK13QGraphicsView12mapFromSceneERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::translate(qreal dx, qreal dy);
  fn C_ZN13QGraphicsView9translateEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  QPointF QGraphicsView::mapToScene(int x, int y);
  fn C_ZNK13QGraphicsView10mapToSceneEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QGraphicsView::metaObject();
  fn C_ZNK13QGraphicsView10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsView::setSceneRect(qreal x, qreal y, qreal w, qreal h);
  fn C_ZN13QGraphicsView12setSceneRectEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  QRect QGraphicsView::rubberBandRect();
  fn C_ZNK13QGraphicsView14rubberBandRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsView::setMatrix(const QMatrix & matrix, bool combine);
  fn C_ZN13QGraphicsView9setMatrixERK7QMatrixb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  bool QGraphicsView::isInteractive();
  fn C_ZNK13QGraphicsView13isInteractiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsView::setBackgroundBrush(const QBrush & brush);
  fn C_ZN13QGraphicsView18setBackgroundBrushERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QGraphicsView::isTransformed();
  fn C_ZNK13QGraphicsView13isTransformedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QGraphicsItem * QGraphicsView::itemAt(int x, int y);
  fn C_ZNK13QGraphicsView6itemAtEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QGraphicsView::centerOn(const QPointF & pos);
  fn C_ZN13QGraphicsView8centerOnERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsView::setForegroundBrush(const QBrush & brush);
  fn C_ZN13QGraphicsView18setForegroundBrushERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsView::shear(qreal sh, qreal sv);
  fn C_ZN13QGraphicsView5shearEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  QBrush QGraphicsView::foregroundBrush();
  fn C_ZNK13QGraphicsView15foregroundBrushEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsView::itemAt(const QPoint & pos);
  fn C_ZNK13QGraphicsView6itemAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsView::updateSceneRect(const QRectF & rect);
  fn C_ZN13QGraphicsView15updateSceneRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsView::mapToScene(const QPoint & point);
  fn C_ZNK13QGraphicsView10mapToSceneERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::setInteractive(bool allowed);
  fn C_ZN13QGraphicsView14setInteractiveEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QMatrix QGraphicsView::matrix();
  fn C_ZNK13QGraphicsView6matrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPolygonF QGraphicsView::mapToScene(int x, int y, int w, int h);
  fn C_ZNK13QGraphicsView10mapToSceneEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  void QGraphicsView::QGraphicsView(QGraphicsScene * scene, QWidget * parent);
  fn C_ZN13QGraphicsViewC2EP14QGraphicsSceneP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QGraphicsView::QGraphicsView(QWidget * parent);
  fn C_ZN13QGraphicsViewC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QGraphicsView::centerOn(const QGraphicsItem * item);
  fn C_ZN13QGraphicsView8centerOnEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTransform QGraphicsView::viewportTransform();
  fn C_ZNK13QGraphicsView17viewportTransformEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsView::resetCachedContent();
  fn C_ZN13QGraphicsView18resetCachedContentEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPolygonF QGraphicsView::mapToScene(const QPolygon & polygon);
  fn C_ZNK13QGraphicsView10mapToSceneERK8QPolygon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::ensureVisible(const QGraphicsItem * item, int xmargin, int ymargin);
  fn C_ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  QRectF QGraphicsView::sceneRect();
  fn C_ZNK13QGraphicsView9sceneRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QGraphicsScene * QGraphicsView::scene();
  fn C_ZNK13QGraphicsView5sceneEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QGraphicsView::sizeHint();
  fn C_ZNK13QGraphicsView8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QBrush QGraphicsView::backgroundBrush();
  fn C_ZNK13QGraphicsView15backgroundBrushEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QGraphicsView::mapFromScene(qreal x, qreal y);
  fn C_ZNK13QGraphicsView12mapFromSceneEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QGraphicsView::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
  fn C_ZN13QGraphicsView13ensureVisibleERK6QRectFii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  QList<QGraphicsItem *> QGraphicsView::items();
  fn C_ZNK13QGraphicsView5itemsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTransform QGraphicsView::transform();
  fn C_ZNK13QGraphicsView9transformEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QGraphicsItem *> QGraphicsView::items(int x, int y);
  fn C_ZNK13QGraphicsView5itemsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QGraphicsView::centerOn(qreal x, qreal y);
  fn C_ZN13QGraphicsView8centerOnEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsView::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
  fn C_ZN13QGraphicsView13ensureVisibleEddddii(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int);
  // proto:  void QGraphicsView::rotate(qreal angle);
  fn C_ZN13QGraphicsView6rotateEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QPolygon QGraphicsView::mapFromScene(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsView12mapFromSceneEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsView::setTransform(const QTransform & matrix, bool combine);
  fn C_ZN13QGraphicsView12setTransformERK10QTransformb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  void QGraphicsView::setSceneRect(const QRectF & rect);
  fn C_ZN13QGraphicsView12setSceneRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPolygon QGraphicsView::mapFromScene(const QPolygonF & polygon);
  fn C_ZNK13QGraphicsView12mapFromSceneERK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPoint QGraphicsView::mapFromScene(const QPointF & point);
  fn C_ZNK13QGraphicsView12mapFromSceneERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QGraphicsView::mapFromScene(const QPainterPath & path);
  fn C_ZNK13QGraphicsView12mapFromSceneERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::~QGraphicsView();
  fn C_ZN13QGraphicsViewD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QGraphicsItem *> QGraphicsView::items(const QPoint & pos);
  fn C_ZNK13QGraphicsView5itemsERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsView::resetMatrix();
  fn C_ZN13QGraphicsView11resetMatrixEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsView::resetTransform();
  fn C_ZN13QGraphicsView14resetTransformEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QGraphicsView::mapToScene(const QPainterPath & path);
  fn C_ZNK13QGraphicsView10mapToSceneERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::setScene(QGraphicsScene * scene);
  fn C_ZN13QGraphicsView8setSceneEP14QGraphicsScene(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QGraphicsView_SlotProxy_connect__ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsView)=1
#[derive(Default)]
pub struct QGraphicsView {
  qbase: QAbstractScrollArea,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _rubberBandChanged: QGraphicsView_rubberBandChanged_signal,
}

impl /*struct*/ QGraphicsView {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsView {
    return QGraphicsView{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsView {
  type Target = QAbstractScrollArea;

  fn deref(&self) -> &QAbstractScrollArea {
    return & self.qbase;
  }
}
impl AsRef<QAbstractScrollArea> for QGraphicsView {
  fn as_ref(& self) -> & QAbstractScrollArea {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsView::scale(qreal sx, qreal sy);
impl /*struct*/ QGraphicsView {
  pub fn scale<RetType, T: QGraphicsView_scale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scale(self);
    // return 1;
  }
}

pub trait QGraphicsView_scale<RetType> {
  fn scale(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QGraphicsView_scale<()> for (f64, f64) {
  fn scale(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN13QGraphicsView5scaleEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsView::mapToScene(const QRect & rect);
impl /*struct*/ QGraphicsView {
  pub fn mapToScene<RetType, T: QGraphicsView_mapToScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToScene(self);
    // return 1;
  }
}

pub trait QGraphicsView_mapToScene<RetType> {
  fn mapToScene(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QPolygonF QGraphicsView::mapToScene(const QRect & rect);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPolygonF> for (&'a QRect) {
  fn mapToScene(self , rsthis: & QGraphicsView) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsView10mapToSceneERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygon QGraphicsView::mapFromScene(const QRectF & rect);
impl /*struct*/ QGraphicsView {
  pub fn mapFromScene<RetType, T: QGraphicsView_mapFromScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene(self);
    // return 1;
  }
}

pub trait QGraphicsView_mapFromScene<RetType> {
  fn mapFromScene(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QPolygon QGraphicsView::mapFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPolygon> for (&'a QRectF) {
  fn mapFromScene(self , rsthis: & QGraphicsView) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsView12mapFromSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::translate(qreal dx, qreal dy);
impl /*struct*/ QGraphicsView {
  pub fn translate<RetType, T: QGraphicsView_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QGraphicsView_translate<RetType> {
  fn translate(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsView_translate<()> for (f64, f64) {
  fn translate(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN13QGraphicsView9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsView::mapToScene(int x, int y);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPointF> for (i32, i32) {
  fn mapToScene(self , rsthis: & QGraphicsView) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK13QGraphicsView10mapToSceneEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsView::metaObject();
impl /*struct*/ QGraphicsView {
  pub fn metaObject<RetType, T: QGraphicsView_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsView_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsView::metaObject();
impl<'a> /*trait*/ QGraphicsView_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsView) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::setSceneRect(qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsView {
  pub fn setSceneRect<RetType, T: QGraphicsView_setSceneRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsView_setSceneRect<RetType> {
  fn setSceneRect(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::setSceneRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsView_setSceneRect<()> for (f64, f64, f64, f64) {
  fn setSceneRect(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setSceneRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {C_ZN13QGraphicsView12setSceneRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QRect QGraphicsView::rubberBandRect();
impl /*struct*/ QGraphicsView {
  pub fn rubberBandRect<RetType, T: QGraphicsView_rubberBandRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rubberBandRect(self);
    // return 1;
  }
}

pub trait QGraphicsView_rubberBandRect<RetType> {
  fn rubberBandRect(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QRect QGraphicsView::rubberBandRect();
impl<'a> /*trait*/ QGraphicsView_rubberBandRect<QRect> for () {
  fn rubberBandRect(self , rsthis: & QGraphicsView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView14rubberBandRectEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView14rubberBandRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::setMatrix(const QMatrix & matrix, bool combine);
impl /*struct*/ QGraphicsView {
  pub fn setMatrix<RetType, T: QGraphicsView_setMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsView_setMatrix<RetType> {
  fn setMatrix(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsView_setMatrix<()> for (&'a QMatrix, i8) {
  fn setMatrix(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN13QGraphicsView9setMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QGraphicsView::isInteractive();
impl /*struct*/ QGraphicsView {
  pub fn isInteractive<RetType, T: QGraphicsView_isInteractive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isInteractive(self);
    // return 1;
  }
}

pub trait QGraphicsView_isInteractive<RetType> {
  fn isInteractive(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  bool QGraphicsView::isInteractive();
impl<'a> /*trait*/ QGraphicsView_isInteractive<i8> for () {
  fn isInteractive(self , rsthis: & QGraphicsView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView13isInteractiveEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView13isInteractiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsView::setBackgroundBrush(const QBrush & brush);
impl /*struct*/ QGraphicsView {
  pub fn setBackgroundBrush<RetType, T: QGraphicsView_setBackgroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsView_setBackgroundBrush<RetType> {
  fn setBackgroundBrush(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::setBackgroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsView_setBackgroundBrush<()> for (&'a QBrush) {
  fn setBackgroundBrush(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18setBackgroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsView18setBackgroundBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsView::isTransformed();
impl /*struct*/ QGraphicsView {
  pub fn isTransformed<RetType, T: QGraphicsView_isTransformed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTransformed(self);
    // return 1;
  }
}

pub trait QGraphicsView_isTransformed<RetType> {
  fn isTransformed(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  bool QGraphicsView::isTransformed();
impl<'a> /*trait*/ QGraphicsView_isTransformed<i8> for () {
  fn isTransformed(self , rsthis: & QGraphicsView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView13isTransformedEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView13isTransformedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsView::itemAt(int x, int y);
impl /*struct*/ QGraphicsView {
  pub fn itemAt<RetType, T: QGraphicsView_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsView_itemAt<RetType> {
  fn itemAt(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsView::itemAt(int x, int y);
impl<'a> /*trait*/ QGraphicsView_itemAt<()> for (i32, i32) {
  fn itemAt(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZNK13QGraphicsView6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::centerOn(const QPointF & pos);
impl /*struct*/ QGraphicsView {
  pub fn centerOn<RetType, T: QGraphicsView_centerOn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.centerOn(self);
    // return 1;
  }
}

pub trait QGraphicsView_centerOn<RetType> {
  fn centerOn(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::centerOn(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsView_centerOn<()> for (&'a QPointF) {
  fn centerOn(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsView8centerOnERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::setForegroundBrush(const QBrush & brush);
impl /*struct*/ QGraphicsView {
  pub fn setForegroundBrush<RetType, T: QGraphicsView_setForegroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setForegroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsView_setForegroundBrush<RetType> {
  fn setForegroundBrush(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::setForegroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsView_setForegroundBrush<()> for (&'a QBrush) {
  fn setForegroundBrush(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18setForegroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsView18setForegroundBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::shear(qreal sh, qreal sv);
impl /*struct*/ QGraphicsView {
  pub fn shear<RetType, T: QGraphicsView_shear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shear(self);
    // return 1;
  }
}

pub trait QGraphicsView_shear<RetType> {
  fn shear(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QGraphicsView_shear<()> for (f64, f64) {
  fn shear(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN13QGraphicsView5shearEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QBrush QGraphicsView::foregroundBrush();
impl /*struct*/ QGraphicsView {
  pub fn foregroundBrush<RetType, T: QGraphicsView_foregroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.foregroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsView_foregroundBrush<RetType> {
  fn foregroundBrush(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QBrush QGraphicsView::foregroundBrush();
impl<'a> /*trait*/ QGraphicsView_foregroundBrush<QBrush> for () {
  fn foregroundBrush(self , rsthis: & QGraphicsView) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView15foregroundBrushEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView15foregroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsView::itemAt(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsView_itemAt<()> for (&'a QPoint) {
  fn itemAt(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QGraphicsView6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::updateSceneRect(const QRectF & rect);
impl /*struct*/ QGraphicsView {
  pub fn updateSceneRect<RetType, T: QGraphicsView_updateSceneRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateSceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsView_updateSceneRect<RetType> {
  fn updateSceneRect(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::updateSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_updateSceneRect<()> for (&'a QRectF) {
  fn updateSceneRect(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView15updateSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsView15updateSceneRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsView::mapToScene(const QPoint & point);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPointF> for (&'a QPoint) {
  fn mapToScene(self , rsthis: & QGraphicsView) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsView10mapToSceneERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::setInteractive(bool allowed);
impl /*struct*/ QGraphicsView {
  pub fn setInteractive<RetType, T: QGraphicsView_setInteractive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInteractive(self);
    // return 1;
  }
}

pub trait QGraphicsView_setInteractive<RetType> {
  fn setInteractive(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::setInteractive(bool allowed);
impl<'a> /*trait*/ QGraphicsView_setInteractive<()> for (i8) {
  fn setInteractive(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView14setInteractiveEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsView14setInteractiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMatrix QGraphicsView::matrix();
impl /*struct*/ QGraphicsView {
  pub fn matrix<RetType, T: QGraphicsView_matrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QGraphicsView_matrix<RetType> {
  fn matrix(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QMatrix QGraphicsView::matrix();
impl<'a> /*trait*/ QGraphicsView_matrix<QMatrix> for () {
  fn matrix(self , rsthis: & QGraphicsView) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6matrixEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsView::mapToScene(int x, int y, int w, int h);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPolygonF> for (i32, i32, i32, i32) {
  fn mapToScene(self , rsthis: & QGraphicsView) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {C_ZNK13QGraphicsView10mapToSceneEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::QGraphicsView(QGraphicsScene * scene, QWidget * parent);
impl /*struct*/ QGraphicsView {
  pub fn new<T: QGraphicsView_new>(value: T) -> QGraphicsView {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsView_new {
  fn new(self) -> QGraphicsView;
}

  // proto:  void QGraphicsView::QGraphicsView(QGraphicsScene * scene, QWidget * parent);
impl<'a> /*trait*/ QGraphicsView_new for (&'a QGraphicsScene, &'a QWidget) {
  fn new(self) -> QGraphicsView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewC2EP14QGraphicsSceneP7QWidget()};
    let ctysz: c_int = unsafe{QGraphicsView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QGraphicsViewC2EP14QGraphicsSceneP7QWidget(arg0, arg1)};
    let rsthis = QGraphicsView{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsView::QGraphicsView(QWidget * parent);
impl<'a> /*trait*/ QGraphicsView_new for (&'a QWidget) {
  fn new(self) -> QGraphicsView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewC2EP7QWidget()};
    let ctysz: c_int = unsafe{QGraphicsView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QGraphicsViewC2EP7QWidget(arg0)};
    let rsthis = QGraphicsView{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsView::centerOn(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsView_centerOn<()> for (&'a QGraphicsItem) {
  fn centerOn(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsView8centerOnEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTransform QGraphicsView::viewportTransform();
impl /*struct*/ QGraphicsView {
  pub fn viewportTransform<RetType, T: QGraphicsView_viewportTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.viewportTransform(self);
    // return 1;
  }
}

pub trait QGraphicsView_viewportTransform<RetType> {
  fn viewportTransform(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QTransform QGraphicsView::viewportTransform();
impl<'a> /*trait*/ QGraphicsView_viewportTransform<QTransform> for () {
  fn viewportTransform(self , rsthis: & QGraphicsView) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView17viewportTransformEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView17viewportTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::resetCachedContent();
impl /*struct*/ QGraphicsView {
  pub fn resetCachedContent<RetType, T: QGraphicsView_resetCachedContent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetCachedContent(self);
    // return 1;
  }
}

pub trait QGraphicsView_resetCachedContent<RetType> {
  fn resetCachedContent(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::resetCachedContent();
impl<'a> /*trait*/ QGraphicsView_resetCachedContent<()> for () {
  fn resetCachedContent(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18resetCachedContentEv()};
     unsafe {C_ZN13QGraphicsView18resetCachedContentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsView::mapToScene(const QPolygon & polygon);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPolygonF> for (&'a QPolygon) {
  fn mapToScene(self , rsthis: & QGraphicsView) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsView10mapToSceneERK8QPolygon(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::ensureVisible(const QGraphicsItem * item, int xmargin, int ymargin);
impl /*struct*/ QGraphicsView {
  pub fn ensureVisible<RetType, T: QGraphicsView_ensureVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible(self);
    // return 1;
  }
}

pub trait QGraphicsView_ensureVisible<RetType> {
  fn ensureVisible(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::ensureVisible(const QGraphicsItem * item, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible<()> for (&'a QGraphicsItem, i32, i32) {
  fn ensureVisible(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsView::sceneRect();
impl /*struct*/ QGraphicsView {
  pub fn sceneRect<RetType, T: QGraphicsView_sceneRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsView_sceneRect<RetType> {
  fn sceneRect(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QRectF QGraphicsView::sceneRect();
impl<'a> /*trait*/ QGraphicsView_sceneRect<QRectF> for () {
  fn sceneRect(self , rsthis: & QGraphicsView) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView9sceneRectEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView9sceneRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsScene * QGraphicsView::scene();
impl /*struct*/ QGraphicsView {
  pub fn scene<RetType, T: QGraphicsView_scene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scene(self);
    // return 1;
  }
}

pub trait QGraphicsView_scene<RetType> {
  fn scene(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QGraphicsScene * QGraphicsView::scene();
impl<'a> /*trait*/ QGraphicsView_scene<()> for () {
  fn scene(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5sceneEv()};
     unsafe {C_ZNK13QGraphicsView5sceneEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QGraphicsView::sizeHint();
impl /*struct*/ QGraphicsView {
  pub fn sizeHint<RetType, T: QGraphicsView_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QGraphicsView_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QSize QGraphicsView::sizeHint();
impl<'a> /*trait*/ QGraphicsView_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QGraphicsView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView8sizeHintEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QBrush QGraphicsView::backgroundBrush();
impl /*struct*/ QGraphicsView {
  pub fn backgroundBrush<RetType, T: QGraphicsView_backgroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backgroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsView_backgroundBrush<RetType> {
  fn backgroundBrush(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QBrush QGraphicsView::backgroundBrush();
impl<'a> /*trait*/ QGraphicsView_backgroundBrush<QBrush> for () {
  fn backgroundBrush(self , rsthis: & QGraphicsView) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView15backgroundBrushEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView15backgroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QGraphicsView::mapFromScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPoint> for (f64, f64) {
  fn mapFromScene(self , rsthis: & QGraphicsView) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsView12mapFromSceneEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible<()> for (&'a QRectF, i32, i32) {
  fn ensureVisible(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN13QGraphicsView13ensureVisibleERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsItem *> QGraphicsView::items();
impl /*struct*/ QGraphicsView {
  pub fn items<RetType, T: QGraphicsView_items<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.items(self);
    // return 1;
  }
}

pub trait QGraphicsView_items<RetType> {
  fn items(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QList<QGraphicsItem *> QGraphicsView::items();
impl<'a> /*trait*/ QGraphicsView_items<()> for () {
  fn items(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsEv()};
     unsafe {C_ZNK13QGraphicsView5itemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTransform QGraphicsView::transform();
impl /*struct*/ QGraphicsView {
  pub fn transform<RetType, T: QGraphicsView_transform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QGraphicsView_transform<RetType> {
  fn transform(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  QTransform QGraphicsView::transform();
impl<'a> /*trait*/ QGraphicsView_transform<QTransform> for () {
  fn transform(self , rsthis: & QGraphicsView) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView9transformEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsView9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QGraphicsItem *> QGraphicsView::items(int x, int y);
impl<'a> /*trait*/ QGraphicsView_items<()> for (i32, i32) {
  fn items(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZNK13QGraphicsView5itemsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::centerOn(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsView_centerOn<()> for (f64, f64) {
  fn centerOn(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN13QGraphicsView8centerOnEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible<()> for (f64, f64, f64, f64, i32, i32) {
  fn ensureVisible(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {C_ZN13QGraphicsView13ensureVisibleEddddii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::rotate(qreal angle);
impl /*struct*/ QGraphicsView {
  pub fn rotate<RetType, T: QGraphicsView_rotate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotate(self);
    // return 1;
  }
}

pub trait QGraphicsView_rotate<RetType> {
  fn rotate(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::rotate(qreal angle);
impl<'a> /*trait*/ QGraphicsView_rotate<()> for (f64) {
  fn rotate(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView6rotateEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QGraphicsView6rotateEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygon QGraphicsView::mapFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPolygon> for (f64, f64, f64, f64) {
  fn mapFromScene(self , rsthis: & QGraphicsView) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsView12mapFromSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::setTransform(const QTransform & matrix, bool combine);
impl /*struct*/ QGraphicsView {
  pub fn setTransform<RetType, T: QGraphicsView_setTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTransform(self);
    // return 1;
  }
}

pub trait QGraphicsView_setTransform<RetType> {
  fn setTransform(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::setTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsView_setTransform<()> for (&'a QTransform, i8) {
  fn setTransform(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN13QGraphicsView12setTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::setSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_setSceneRect<()> for (&'a QRectF) {
  fn setSceneRect(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsView12setSceneRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygon QGraphicsView::mapFromScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPolygon> for (&'a QPolygonF) {
  fn mapFromScene(self , rsthis: & QGraphicsView) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsView12mapFromSceneERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QGraphicsView::mapFromScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPoint> for (&'a QPointF) {
  fn mapFromScene(self , rsthis: & QGraphicsView) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsView12mapFromSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsView::mapFromScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPainterPath> for (&'a QPainterPath) {
  fn mapFromScene(self , rsthis: & QGraphicsView) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsView12mapFromSceneERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::~QGraphicsView();
impl /*struct*/ QGraphicsView {
  pub fn free<RetType, T: QGraphicsView_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsView_free<RetType> {
  fn free(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::~QGraphicsView();
impl<'a> /*trait*/ QGraphicsView_free<()> for () {
  fn free(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewD2Ev()};
     unsafe {C_ZN13QGraphicsViewD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsItem *> QGraphicsView::items(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsView_items<()> for (&'a QPoint) {
  fn items(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QGraphicsView5itemsERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::resetMatrix();
impl /*struct*/ QGraphicsView {
  pub fn resetMatrix<RetType, T: QGraphicsView_resetMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsView_resetMatrix<RetType> {
  fn resetMatrix(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::resetMatrix();
impl<'a> /*trait*/ QGraphicsView_resetMatrix<()> for () {
  fn resetMatrix(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView11resetMatrixEv()};
     unsafe {C_ZN13QGraphicsView11resetMatrixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsView::resetTransform();
impl /*struct*/ QGraphicsView {
  pub fn resetTransform<RetType, T: QGraphicsView_resetTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetTransform(self);
    // return 1;
  }
}

pub trait QGraphicsView_resetTransform<RetType> {
  fn resetTransform(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::resetTransform();
impl<'a> /*trait*/ QGraphicsView_resetTransform<()> for () {
  fn resetTransform(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView14resetTransformEv()};
     unsafe {C_ZN13QGraphicsView14resetTransformEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsView::mapToScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPainterPath> for (&'a QPainterPath) {
  fn mapToScene(self , rsthis: & QGraphicsView) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsView10mapToSceneERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsView::setScene(QGraphicsScene * scene);
impl /*struct*/ QGraphicsView {
  pub fn setScene<RetType, T: QGraphicsView_setScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScene(self);
    // return 1;
  }
}

pub trait QGraphicsView_setScene<RetType> {
  fn setScene(self , rsthis: & QGraphicsView) -> RetType;
}

  // proto:  void QGraphicsView::setScene(QGraphicsScene * scene);
impl<'a> /*trait*/ QGraphicsView_setScene<()> for (&'a QGraphicsScene) {
  fn setScene(self , rsthis: & QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8setSceneEP14QGraphicsScene()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsView8setSceneEP14QGraphicsScene(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QGraphicsView_rubberBandChanged
pub struct QGraphicsView_rubberBandChanged_signal{poi:u64}
impl /* struct */ QGraphicsView {
  pub fn rubberBandChanged(&self) -> QGraphicsView_rubberBandChanged_signal {
     return QGraphicsView_rubberBandChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsView_rubberBandChanged_signal {
  pub fn connect<T: QGraphicsView_rubberBandChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsView_rubberBandChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsView_rubberBandChanged_signal);
}

// rubberBandChanged(class QRect, class QPointF, class QPointF)
extern fn QGraphicsView_rubberBandChanged_signal_connect_cb_0(rsfptr:fn(QRect, QPointF, QPointF), arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  let rsarg1 = QPointF::inheritFrom(arg1 as u64);
  let rsarg2 = QPointF::inheritFrom(arg2 as u64);
  rsfptr(rsarg0,rsarg1,rsarg2);
}
extern fn QGraphicsView_rubberBandChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QRect, QPointF, QPointF)>, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  let rsarg1 = QPointF::inheritFrom(arg1 as u64);
  let rsarg2 = QPointF::inheritFrom(arg2 as u64);
  // rsfptr(rsarg0,rsarg1,rsarg2);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1,rsarg2)};
}
impl /* trait */ QGraphicsView_rubberBandChanged_signal_connect for fn(QRect, QPointF, QPointF) {
  fn connect(self, sigthis: QGraphicsView_rubberBandChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsView_rubberBandChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsView_SlotProxy_connect__ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsView_rubberBandChanged_signal_connect for Box<Fn(QRect, QPointF, QPointF)> {
  fn connect(self, sigthis: QGraphicsView_rubberBandChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsView_rubberBandChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsView_SlotProxy_connect__ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_(arg0, arg1, arg2)};
  }
}
// <= body block end

