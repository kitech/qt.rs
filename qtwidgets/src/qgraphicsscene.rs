

// mod ::widgets::QGraphicsScene
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicsscene.h
// #include <qgraphicsscene.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 40
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QGraphicsScene) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool eventFilter(QObject *, QEvent *)
// func (this *QGraphicsScene) InheritEventFilter(f func(watched *qtcore.QObject/*777 QObject **/, event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventFilter", f)
// }

// void contextMenuEvent(QGraphicsSceneContextMenuEvent *)
// func (this *QGraphicsScene) InheritContextMenuEvent(f func(event *QGraphicsSceneContextMenuEvent/*777 QGraphicsSceneContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void dragEnterEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsScene) InheritDragEnterEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragMoveEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsScene) InheritDragMoveEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dragLeaveEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsScene) InheritDragLeaveEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dropEvent(QGraphicsSceneDragDropEvent *)
// func (this *QGraphicsScene) InheritDropEvent(f func(event *QGraphicsSceneDragDropEvent/*777 QGraphicsSceneDragDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QGraphicsScene) InheritFocusInEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QGraphicsScene) InheritFocusOutEvent(f func(event *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void helpEvent(QGraphicsSceneHelpEvent *)
// func (this *QGraphicsScene) InheritHelpEvent(f func(event *QGraphicsSceneHelpEvent/*777 QGraphicsSceneHelpEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "helpEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QGraphicsScene) InheritKeyPressEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QGraphicsScene) InheritKeyReleaseEvent(f func(event *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void mousePressEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsScene) InheritMousePressEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsScene) InheritMouseMoveEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsScene) InheritMouseReleaseEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *)
// func (this *QGraphicsScene) InheritMouseDoubleClickEvent(f func(event *QGraphicsSceneMouseEvent/*777 QGraphicsSceneMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void wheelEvent(QGraphicsSceneWheelEvent *)
// func (this *QGraphicsScene) InheritWheelEvent(f func(event *QGraphicsSceneWheelEvent/*777 QGraphicsSceneWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QGraphicsScene) InheritInputMethodEvent(f func(event *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// void drawBackground(QPainter *, const QRectF &)
// func (this *QGraphicsScene) InheritDrawBackground(f func(painter *qtgui.QPainter/*777 QPainter **/, rect *qtcore.QRectF) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawBackground", f)
// }

// void drawForeground(QPainter *, const QRectF &)
// func (this *QGraphicsScene) InheritDrawForeground(f func(painter *qtgui.QPainter/*777 QPainter **/, rect *qtcore.QRectF) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawForeground", f)
// }

// void drawItems(QPainter *, int, QGraphicsItem **, const QStyleOptionGraphicsItem *, QWidget *)
// func (this *QGraphicsScene) InheritDrawItems(f func(painter *qtgui.QPainter/*777 QPainter **/, numItems int, items []interface{}, options []interface{}, widget *QWidget/*777 QWidget **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawItems", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QGraphicsScene) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsScene)=16
pub struct QGraphicsScene {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsScene_ITF interface {
//    qtcore.QObject_ITF
//    QGraphicsScene_PTR() *QGraphicsScene
//}
//func (ptr *QGraphicsScene) QGraphicsScene_PTR() *QGraphicsScene { return ptr }

impl /*struct*/ QGraphicsScene {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsScene {
    return QGraphicsScene{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsScene {
//  type Target = QGraphicsSceneBASE;
//
//  fn deref(&self) -> &QGraphicsSceneBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneBASE> for QGraphicsScene {
//  fn as_ref(& self) -> & QGraphicsSceneBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicsscene.h:98
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn metaObject_0<RetType, T: QGraphicsScene_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsScene(QObject *)

/*
Constructs a QGraphicsScene object. The parent parameter is passed to QObject's constructor.
*/
// QGraphicsScene(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsScene {
  pub fn QGraphicsScene_0<T: QGraphicsScene_QGraphicsScene_0>(value: T) -> QGraphicsScene {
    let rsthis = value.QGraphicsScene_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScene_QGraphicsScene_0 {
  fn QGraphicsScene_0(self) -> QGraphicsScene;
}
// QGraphicsScene(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsScene_QGraphicsScene_0 for (usize) {
  fn QGraphicsScene_0(self) -> QGraphicsScene {
    // unsafe{_ZN14QGraphicsSceneC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QGraphicsSceneC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsScene{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:125
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsScene(const QRectF &, QObject *)

/*
Constructs a QGraphicsScene object. The parent parameter is passed to QObject's constructor.
*/
// QGraphicsScene(const QRectF &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsScene {
  pub fn QGraphicsScene_1<T: QGraphicsScene_QGraphicsScene_1>(value: T) -> QGraphicsScene {
    let rsthis = value.QGraphicsScene_1();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScene_QGraphicsScene_1 {
  fn QGraphicsScene_1(self) -> QGraphicsScene;
}
// QGraphicsScene(const QRectF &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsScene_QGraphicsScene_1 for (usize,usize) {
  fn QGraphicsScene_1(self) -> QGraphicsScene {
    // unsafe{_ZN14QGraphicsSceneC2ERK6QRectFP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QGraphicsSceneC2ERK6QRectFP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsScene{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:126
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsScene(qreal, qreal, qreal, qreal, QObject *)

/*
Constructs a QGraphicsScene object. The parent parameter is passed to QObject's constructor.
*/
// QGraphicsScene(qreal, qreal, qreal, qreal, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsScene {
  pub fn QGraphicsScene_2<T: QGraphicsScene_QGraphicsScene_2>(value: T) -> QGraphicsScene {
    let rsthis = value.QGraphicsScene_2();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScene_QGraphicsScene_2 {
  fn QGraphicsScene_2(self) -> QGraphicsScene;
}
// QGraphicsScene(qreal, qreal, qreal, qreal, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsScene_QGraphicsScene_2 for (f64,f64,f64,f64,usize) {
  fn QGraphicsScene_2(self) -> QGraphicsScene {
    // unsafe{_ZN14QGraphicsSceneC2EddddP7QObject()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QGraphicsSceneC2EddddP7QObject", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsScene{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:127
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsScene()

/*

*/
pub fn DeleteQGraphicsScene(this :*mut QGraphicsScene) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QGraphicsSceneD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicsscene.h:129
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF sceneRect() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn sceneRect_0<RetType, T: QGraphicsScene_sceneRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_sceneRect_0<RetType> {
  fn sceneRect_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_sceneRect_0<usize> for () {
  fn sceneRect_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene9sceneRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:130
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal width() const

/*
This convenience function is equivalent to calling sceneRect().width().

See also height().
*/
impl /*struct*/ QGraphicsScene {
  pub fn width_0<RetType, T: QGraphicsScene_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_width_0<RetType> {
  fn width_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_width_0<f64> for () {
  fn width_0(self , rsthis: & QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:131
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal height() const

/*
This convenience function is equivalent to calling sceneRect().height().

See also width().
*/
impl /*struct*/ QGraphicsScene {
  pub fn height_0<RetType, T: QGraphicsScene_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_height_0<RetType> {
  fn height_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_height_0<f64> for () {
  fn height_0(self , rsthis: & QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSceneRect(const QRectF &)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setSceneRect_0<RetType, T: QGraphicsScene_setSceneRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSceneRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setSceneRect_0<RetType> {
  fn setSceneRect_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setSceneRect_0<(/*void*/)> for (usize) {
  fn setSceneRect_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene12setSceneRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:133
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setSceneRect(qreal, qreal, qreal, qreal)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setSceneRect_1<RetType, T: QGraphicsScene_setSceneRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSceneRect_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setSceneRect_1<RetType> {
  fn setSceneRect_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setSceneRect_1<(/*void*/)> for (f64,f64,f64,f64) {
  fn setSceneRect_1(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene12setSceneRectEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void render(QPainter *, const QRectF &, const QRectF &, Qt::AspectRatioMode)

/*
Renders the source rect from scene into target, using painter. This function is useful for capturing the contents of the scene onto a paint device, such as a QImage (e.g., to take a screenshot), or for printing with QPrinter. For example:


  QGraphicsScene scene;
  scene.addItem(...
  ...
  QPrinter printer(QPrinter::HighResolution);
  printer.setPaperSize(QPrinter::A4);

  QPainter painter(&printer);
  scene.render(&painter);



If source is a null rect, this function will use sceneRect() to determine what to render. If target is a null rect, the dimensions of painter's paint device will be used.

The source rect contents will be transformed according to aspectRatioMode to fit into the target rect. By default, the aspect ratio is kept, and source is scaled to fit in target.

See also QGraphicsView::render().
*/
impl /*struct*/ QGraphicsScene {
  pub fn render_0<RetType, T: QGraphicsScene_render_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.render_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_render_0<RetType> {
  fn render_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_render_0<(/*void*/)> for (usize,usize,usize,i32) {
  fn render_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene6renderEP8QPainterRK6QRectFS4_N2Qt15AspectRatioModeE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:140
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsScene::ItemIndexMethod itemIndexMethod() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn itemIndexMethod_0<RetType, T: QGraphicsScene_itemIndexMethod_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemIndexMethod_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_itemIndexMethod_0<RetType> {
  fn itemIndexMethod_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_itemIndexMethod_0<i32> for () {
  fn itemIndexMethod_0(self , rsthis: & QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene15itemIndexMethodEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setItemIndexMethod(QGraphicsScene::ItemIndexMethod)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setItemIndexMethod_0<RetType, T: QGraphicsScene_setItemIndexMethod_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItemIndexMethod_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setItemIndexMethod_0<RetType> {
  fn setItemIndexMethod_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setItemIndexMethod_0<(/*void*/)> for (i32) {
  fn setItemIndexMethod_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene18setItemIndexMethodENS_15ItemIndexMethodE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:143
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSortCacheEnabled() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn isSortCacheEnabled_0<RetType, T: QGraphicsScene_isSortCacheEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSortCacheEnabled_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_isSortCacheEnabled_0<RetType> {
  fn isSortCacheEnabled_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_isSortCacheEnabled_0<bool> for () {
  fn isSortCacheEnabled_0(self , rsthis: & QGraphicsScene) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene18isSortCacheEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSortCacheEnabled(bool)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setSortCacheEnabled_0<RetType, T: QGraphicsScene_setSortCacheEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSortCacheEnabled_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setSortCacheEnabled_0<RetType> {
  fn setSortCacheEnabled_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setSortCacheEnabled_0<(/*void*/)> for (bool) {
  fn setSortCacheEnabled_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene19setSortCacheEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:146
// index:0
// Public Visibility=Default Availability=Available
// [4] int bspTreeDepth() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn bspTreeDepth_0<RetType, T: QGraphicsScene_bspTreeDepth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bspTreeDepth_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_bspTreeDepth_0<RetType> {
  fn bspTreeDepth_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_bspTreeDepth_0<i32> for () {
  fn bspTreeDepth_0(self , rsthis: & QGraphicsScene) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene12bspTreeDepthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:147
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBspTreeDepth(int)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setBspTreeDepth_0<RetType, T: QGraphicsScene_setBspTreeDepth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBspTreeDepth_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setBspTreeDepth_0<RetType> {
  fn setBspTreeDepth_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setBspTreeDepth_0<(/*void*/)> for (i32) {
  fn setBspTreeDepth_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene15setBspTreeDepthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:149
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF itemsBoundingRect() const

/*
Calculates and returns the bounding rect of all items on the scene. This function works by iterating over all items, and because of this, it can be slow for large scenes.

See also sceneRect().
*/
impl /*struct*/ QGraphicsScene {
  pub fn itemsBoundingRect_0<RetType, T: QGraphicsScene_itemsBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemsBoundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_itemsBoundingRect_0<RetType> {
  fn itemsBoundingRect_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_itemsBoundingRect_0<usize> for () {
  fn itemsBoundingRect_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene17itemsBoundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:151
// index:0
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(Qt::SortOrder) const

/*
Returns an ordered list of all items on the scene. order decides the stacking order.

See also addItem(), removeItem(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn items_0<RetType, T: QGraphicsScene_items_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_items_0<RetType> {
  fn items_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_items_0<usize> for (i32) {
  fn items_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene5itemsEN2Qt9SortOrderE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:153
// index:1
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(const QPointF &, Qt::ItemSelectionMode, Qt::SortOrder, const QTransform &) const

/*
Returns an ordered list of all items on the scene. order decides the stacking order.

See also addItem(), removeItem(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn items_1<RetType, T: QGraphicsScene_items_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_items_1<RetType> {
  fn items_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_items_1<usize> for (usize,i32,i32,usize) {
  fn items_1(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene5itemsERK7QPointFN2Qt17ItemSelectionModeENS3_9SortOrderERK10QTransform", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:154
// index:2
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(const QRectF &, Qt::ItemSelectionMode, Qt::SortOrder, const QTransform &) const

/*
Returns an ordered list of all items on the scene. order decides the stacking order.

See also addItem(), removeItem(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn items_2<RetType, T: QGraphicsScene_items_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_2(self);
    // return 1;
  }
}
pub trait QGraphicsScene_items_2<RetType> {
  fn items_2(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_items_2<usize> for (usize,i32,i32,usize) {
  fn items_2(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene5itemsERK6QRectFN2Qt17ItemSelectionModeENS3_9SortOrderERK10QTransform", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:155
// index:3
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(const QPolygonF &, Qt::ItemSelectionMode, Qt::SortOrder, const QTransform &) const

/*
Returns an ordered list of all items on the scene. order decides the stacking order.

See also addItem(), removeItem(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn items_3<RetType, T: QGraphicsScene_items_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_3(self);
    // return 1;
  }
}
pub trait QGraphicsScene_items_3<RetType> {
  fn items_3(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_items_3<usize> for (usize,i32,i32,usize) {
  fn items_3(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene5itemsERK9QPolygonFN2Qt17ItemSelectionModeENS3_9SortOrderERK10QTransform", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:156
// index:4
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(const QPainterPath &, Qt::ItemSelectionMode, Qt::SortOrder, const QTransform &) const

/*
Returns an ordered list of all items on the scene. order decides the stacking order.

See also addItem(), removeItem(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn items_4<RetType, T: QGraphicsScene_items_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_4(self);
    // return 1;
  }
}
pub trait QGraphicsScene_items_4<RetType> {
  fn items_4(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_items_4<usize> for (usize,i32,i32,usize) {
  fn items_4(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene5itemsERK12QPainterPathN2Qt17ItemSelectionModeENS3_9SortOrderERK10QTransform", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:170
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> items(qreal, qreal, qreal, qreal, Qt::ItemSelectionMode, Qt::SortOrder, const QTransform &) const

/*
Returns an ordered list of all items on the scene. order decides the stacking order.

See also addItem(), removeItem(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn items_5<RetType, T: QGraphicsScene_items_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.items_5(self);
    // return 1;
  }
}
pub trait QGraphicsScene_items_5<RetType> {
  fn items_5(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_items_5<usize> for (f64,f64,f64,f64,i32,i32,usize) {
  fn items_5(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene5itemsEddddN2Qt17ItemSelectionModeENS0_9SortOrderERK10QTransform", 7,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:158
// index:0
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> collidingItems(const QGraphicsItem *, Qt::ItemSelectionMode) const

/*
Returns a list of all items that collide with item. Collisions are determined by calling QGraphicsItem::collidesWithItem(); the collision detection is determined by mode. By default, all items whose shape intersects item or is contained inside item's shape are returned.

The items are returned in descending stacking order (i.e., the first item in the list is the uppermost item, and the last item is the lowermost item).

See also items(), itemAt(), QGraphicsItem::collidesWithItem(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn collidingItems_0<RetType, T: QGraphicsScene_collidingItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.collidingItems_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_collidingItems_0<RetType> {
  fn collidingItems_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_collidingItems_0<usize> for (usize,i32) {
  fn collidingItems_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene14collidingItemsEPK13QGraphicsItemN2Qt17ItemSelectionModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:165
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * itemAt(const QPointF &, const QTransform &) const

/*
Returns the topmost visible item at the specified position, or 0 if there are no items at this position.

deviceTransform is the transformation that applies to the view, and needs to be provided if the scene contains items that ignore transformations.

Note: See items() for a definition of which items are considered visible by this function.

This function was introduced in  Qt 4.6.

See also items(), collidingItems(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn itemAt_0<RetType, T: QGraphicsScene_itemAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_itemAt_0<RetType> {
  fn itemAt_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_itemAt_0<usize> for (usize,usize) {
  fn itemAt_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene6itemAtERK7QPointFRK10QTransform", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:179
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QGraphicsItem * itemAt(qreal, qreal, const QTransform &) const

/*
Returns the topmost visible item at the specified position, or 0 if there are no items at this position.

deviceTransform is the transformation that applies to the view, and needs to be provided if the scene contains items that ignore transformations.

Note: See items() for a definition of which items are considered visible by this function.

This function was introduced in  Qt 4.6.

See also items(), collidingItems(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn itemAt_1<RetType, T: QGraphicsScene_itemAt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.itemAt_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_itemAt_1<RetType> {
  fn itemAt_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_itemAt_1<usize> for (f64,f64,usize) {
  fn itemAt_1(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene6itemAtEddRK10QTransform", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] QList<QGraphicsItem *> selectedItems() const

/*
Returns a list of all currently selected items. The items are returned in no particular order.

See also setSelectionArea().
*/
impl /*struct*/ QGraphicsScene {
  pub fn selectedItems_0<RetType, T: QGraphicsScene_selectedItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedItems_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_selectedItems_0<RetType> {
  fn selectedItems_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_selectedItems_0<usize> for () {
  fn selectedItems_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene13selectedItemsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:183
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath selectionArea() const

/*
Returns the selection area that was previously set with setSelectionArea(), or an empty QPainterPath if no selection area has been set.

See also setSelectionArea().
*/
impl /*struct*/ QGraphicsScene {
  pub fn selectionArea_0<RetType, T: QGraphicsScene_selectionArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionArea_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_selectionArea_0<RetType> {
  fn selectionArea_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_selectionArea_0<usize> for () {
  fn selectionArea_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene13selectionAreaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelectionArea(const QPainterPath &, const QTransform &)

/*
Sets the selection area to path. All items within this area are immediately selected, and all items outside are unselected. You can get the list of all selected items by calling selectedItems().

deviceTransform is the transformation that applies to the view, and needs to be provided if the scene contains items that ignore transformations.

For an item to be selected, it must be marked as selectable (QGraphicsItem::ItemIsSelectable).

This function was introduced in  Qt 4.6.

See also clearSelection() and selectionArea().
*/
impl /*struct*/ QGraphicsScene {
  pub fn setSelectionArea_0<RetType, T: QGraphicsScene_setSelectionArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionArea_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setSelectionArea_0<RetType> {
  fn setSelectionArea_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setSelectionArea_0<(/*void*/)> for (usize,usize) {
  fn setSelectionArea_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathRK10QTransform", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:185
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setSelectionArea(const QPainterPath &, Qt::ItemSelectionMode, const QTransform &)

/*
Sets the selection area to path. All items within this area are immediately selected, and all items outside are unselected. You can get the list of all selected items by calling selectedItems().

deviceTransform is the transformation that applies to the view, and needs to be provided if the scene contains items that ignore transformations.

For an item to be selected, it must be marked as selectable (QGraphicsItem::ItemIsSelectable).

This function was introduced in  Qt 4.6.

See also clearSelection() and selectionArea().
*/
impl /*struct*/ QGraphicsScene {
  pub fn setSelectionArea_1<RetType, T: QGraphicsScene_setSelectionArea_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionArea_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setSelectionArea_1<RetType> {
  fn setSelectionArea_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setSelectionArea_1<(/*void*/)> for (usize,i32,usize) {
  fn setSelectionArea_1(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathN2Qt17ItemSelectionModeERK10QTransform", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:186
// index:2
// Public Visibility=Default Availability=Available
// [-2] void setSelectionArea(const QPainterPath &, Qt::ItemSelectionOperation, Qt::ItemSelectionMode, const QTransform &)

/*
Sets the selection area to path. All items within this area are immediately selected, and all items outside are unselected. You can get the list of all selected items by calling selectedItems().

deviceTransform is the transformation that applies to the view, and needs to be provided if the scene contains items that ignore transformations.

For an item to be selected, it must be marked as selectable (QGraphicsItem::ItemIsSelectable).

This function was introduced in  Qt 4.6.

See also clearSelection() and selectionArea().
*/
impl /*struct*/ QGraphicsScene {
  pub fn setSelectionArea_2<RetType, T: QGraphicsScene_setSelectionArea_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelectionArea_2(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setSelectionArea_2<RetType> {
  fn setSelectionArea_2(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setSelectionArea_2<(/*void*/)> for (usize,i32,i32,usize) {
  fn setSelectionArea_2(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16setSelectionAreaERK12QPainterPathN2Qt22ItemSelectionOperationENS3_17ItemSelectionModeERK10QTransform", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void destroyItemGroup(QGraphicsItemGroup *)

/*
Reparents all items in group to group's parent item, then removes group from the scene, and finally deletes it. The items' positions and transformations are mapped from the group to the group's parent.

See also createItemGroup() and QGraphicsItemGroup::removeFromGroup().
*/
impl /*struct*/ QGraphicsScene {
  pub fn destroyItemGroup_0<RetType, T: QGraphicsScene_destroyItemGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.destroyItemGroup_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_destroyItemGroup_0<RetType> {
  fn destroyItemGroup_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_destroyItemGroup_0<(/*void*/)> for (usize) {
  fn destroyItemGroup_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16destroyItemGroupEP18QGraphicsItemGroup", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:192
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addItem(QGraphicsItem *)

/*
Adds or moves the item and all its childen to this scene. This scene takes ownership of the item.

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

If the item is already in a different scene, it will first be removed from its old scene, and then added to this scene as a top-level.

QGraphicsScene will send ItemSceneChange notifications to item while it is added to the scene. If item does not currently belong to a scene, only one notification is sent. If it does belong to scene already (i.e., it is moved to this scene), QGraphicsScene will send an addition notification as the item is removed from its previous scene.

If the item is a panel, the scene is active, and there is no active panel in the scene, then the item will be activated.

See also removeItem(), addEllipse(), addLine(), addPath(), addPixmap(), addRect(), addText(), addWidget(), and Sorting.
*/
impl /*struct*/ QGraphicsScene {
  pub fn addItem_0<RetType, T: QGraphicsScene_addItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addItem_0<RetType> {
  fn addItem_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addItem_0<(/*void*/)> for (usize) {
  fn addItem_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7addItemEP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:193
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsEllipseItem * addEllipse(const QRectF &, const QPen &, const QBrush &)

/*
Creates and adds an ellipse item to the scene, and returns the item pointer. The geometry of the ellipse is defined by rect, and its pen and brush are initialized to pen and brush.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addLine(), addPath(), addPixmap(), addRect(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addEllipse_0<RetType, T: QGraphicsScene_addEllipse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addEllipse_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addEllipse_0<RetType> {
  fn addEllipse_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addEllipse_0<usize> for (usize,usize,usize) {
  fn addEllipse_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10addEllipseERK6QRectFRK4QPenRK6QBrush", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:202
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QGraphicsEllipseItem * addEllipse(qreal, qreal, qreal, qreal, const QPen &, const QBrush &)

/*
Creates and adds an ellipse item to the scene, and returns the item pointer. The geometry of the ellipse is defined by rect, and its pen and brush are initialized to pen and brush.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addLine(), addPath(), addPixmap(), addRect(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addEllipse_1<RetType, T: QGraphicsScene_addEllipse_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addEllipse_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addEllipse_1<RetType> {
  fn addEllipse_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addEllipse_1<usize> for (f64,f64,f64,f64,usize,usize) {
  fn addEllipse_1(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10addEllipseEddddRK4QPenRK6QBrush", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:194
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsLineItem * addLine(const QLineF &, const QPen &)

/*
Creates and adds a line item to the scene, and returns the item pointer. The geometry of the line is defined by line, and its pen is initialized to pen.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addPath(), addPixmap(), addRect(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addLine_0<RetType, T: QGraphicsScene_addLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addLine_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addLine_0<RetType> {
  fn addLine_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addLine_0<usize> for (usize,usize) {
  fn addLine_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7addLineERK6QLineFRK4QPen", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:204
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QGraphicsLineItem * addLine(qreal, qreal, qreal, qreal, const QPen &)

/*
Creates and adds a line item to the scene, and returns the item pointer. The geometry of the line is defined by line, and its pen is initialized to pen.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addPath(), addPixmap(), addRect(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addLine_1<RetType, T: QGraphicsScene_addLine_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addLine_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addLine_1<RetType> {
  fn addLine_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addLine_1<usize> for (f64,f64,f64,f64,usize) {
  fn addLine_1(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7addLineEddddRK4QPen", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:195
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsPathItem * addPath(const QPainterPath &, const QPen &, const QBrush &)

/*
Creates and adds a path item to the scene, and returns the item pointer. The geometry of the path is defined by path, and its pen and brush are initialized to pen and brush.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addLine(), addPixmap(), addRect(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addPath_0<RetType, T: QGraphicsScene_addPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPath_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addPath_0<RetType> {
  fn addPath_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addPath_0<usize> for (usize,usize,usize) {
  fn addPath_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7addPathERK12QPainterPathRK4QPenRK6QBrush", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:196
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsPixmapItem * addPixmap(const QPixmap &)

/*
Creates and adds a pixmap item to the scene, and returns the item pointer. The pixmap is defined by pixmap.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addLine(), addPath(), addRect(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addPixmap_0<RetType, T: QGraphicsScene_addPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPixmap_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addPixmap_0<RetType> {
  fn addPixmap_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addPixmap_0<usize> for (usize) {
  fn addPixmap_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene9addPixmapERK7QPixmap", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:197
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsPolygonItem * addPolygon(const QPolygonF &, const QPen &, const QBrush &)

/*
Creates and adds a polygon item to the scene, and returns the item pointer. The polygon is defined by polygon, and its pen and brush are initialized to pen and brush.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addLine(), addPath(), addRect(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addPolygon_0<RetType, T: QGraphicsScene_addPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPolygon_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addPolygon_0<RetType> {
  fn addPolygon_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addPolygon_0<usize> for (usize,usize,usize) {
  fn addPolygon_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10addPolygonERK9QPolygonFRK4QPenRK6QBrush", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:198
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsRectItem * addRect(const QRectF &, const QPen &, const QBrush &)

/*
Creates and adds a rectangle item to the scene, and returns the item pointer. The geometry of the rectangle is defined by rect, and its pen and brush are initialized to pen and brush.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0). For example, if a QRect(50, 50, 100, 100) is added, its top-left corner will be at (50, 50) relative to the origin in the items coordinate system.

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addLine(), addPixmap(), addPixmap(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addRect_0<RetType, T: QGraphicsScene_addRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addRect_0<RetType> {
  fn addRect_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addRect_0<usize> for (usize,usize,usize) {
  fn addRect_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7addRectERK6QRectFRK4QPenRK6QBrush", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:206
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QGraphicsRectItem * addRect(qreal, qreal, qreal, qreal, const QPen &, const QBrush &)

/*
Creates and adds a rectangle item to the scene, and returns the item pointer. The geometry of the rectangle is defined by rect, and its pen and brush are initialized to pen and brush.

Note that the item's geometry is provided in item coordinates, and its position is initialized to (0, 0). For example, if a QRect(50, 50, 100, 100) is added, its top-left corner will be at (50, 50) relative to the origin in the items coordinate system.

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addLine(), addPixmap(), addPixmap(), addText(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addRect_1<RetType, T: QGraphicsScene_addRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRect_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addRect_1<RetType> {
  fn addRect_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addRect_1<usize> for (f64,f64,f64,f64,usize,usize) {
  fn addRect_1(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7addRectEddddRK4QPenRK6QBrush", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:199
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsTextItem * addText(const QString &, const QFont &)

/*
Creates and adds a text item to the scene, and returns the item pointer. The text string is initialized to text, and its font is initialized to font.

The item's position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addLine(), addPixmap(), addPixmap(), addRect(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addText_0<RetType, T: QGraphicsScene_addText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addText_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addText_0<RetType> {
  fn addText_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addText_0<usize> for (usize,usize) {
  fn addText_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7addTextERK7QStringRK5QFont", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:200
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsSimpleTextItem * addSimpleText(const QString &, const QFont &)

/*
Creates and adds a QGraphicsSimpleTextItem to the scene, and returns the item pointer. The text string is initialized to text, and its font is initialized to font.

The item's position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

See also addEllipse(), addLine(), addPixmap(), addPixmap(), addRect(), addItem(), and addWidget().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addSimpleText_0<RetType, T: QGraphicsScene_addSimpleText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addSimpleText_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addSimpleText_0<RetType> {
  fn addSimpleText_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addSimpleText_0<usize> for (usize,usize) {
  fn addSimpleText_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene13addSimpleTextERK7QStringRK5QFont", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:201
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsProxyWidget * addWidget(QWidget *, Qt::WindowFlags)

/*
Creates a new QGraphicsProxyWidget for widget, adds it to the scene, and returns a pointer to the proxy. wFlags set the default window flags for the embedding proxy widget.

The item's position is initialized to (0, 0).

If the item is visible (i.e., QGraphicsItem::isVisible() returns true), QGraphicsScene will emit changed() once control goes back to the event loop.

Note that widgets with the Qt::WA_PaintOnScreen widget attribute set and widgets that wrap an external application or controller are not supported. Examples are QGLWidget and QAxWidget.

See also addEllipse(), addLine(), addPixmap(), addPixmap(), addRect(), addText(), addSimpleText(), and addItem().
*/
impl /*struct*/ QGraphicsScene {
  pub fn addWidget_0<RetType, T: QGraphicsScene_addWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_addWidget_0<RetType> {
  fn addWidget_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_addWidget_0<usize> for (usize,i32) {
  fn addWidget_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene9addWidgetEP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeItem(QGraphicsItem *)

/*
Removes the item item and all its children from the scene. The ownership of item is passed on to the caller (i.e., QGraphicsScene will no longer delete item when destroyed).

See also addItem().
*/
impl /*struct*/ QGraphicsScene {
  pub fn removeItem_0<RetType, T: QGraphicsScene_removeItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_removeItem_0<RetType> {
  fn removeItem_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_removeItem_0<(/*void*/)> for (usize) {
  fn removeItem_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10removeItemEP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:210
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * focusItem() const

/*
When the scene is active, this functions returns the scene's current focus item, or 0 if no item currently has focus. When the scene is inactive, this functions returns the item that will gain input focus when the scene becomes active.

The focus item receives keyboard input when the scene receives a key event.

See also setFocusItem(), QGraphicsItem::hasFocus(), and isActive().
*/
impl /*struct*/ QGraphicsScene {
  pub fn focusItem_0<RetType, T: QGraphicsScene_focusItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_focusItem_0<RetType> {
  fn focusItem_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_focusItem_0<usize> for () {
  fn focusItem_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene9focusItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:211
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocusItem(QGraphicsItem *, Qt::FocusReason)

/*
Sets the scene's focus item to item, with the focus reason focusReason, after removing focus from any previous item that may have had focus.

If item is 0, or if it either does not accept focus (i.e., it does not have the QGraphicsItem::ItemIsFocusable flag enabled), or is not visible or not enabled, this function only removes focus from any previous focusitem.

If item is not 0, and the scene does not currently have focus (i.e., hasFocus() returns false), this function will call setFocus() automatically.

See also focusItem(), hasFocus(), and setFocus().
*/
impl /*struct*/ QGraphicsScene {
  pub fn setFocusItem_0<RetType, T: QGraphicsScene_setFocusItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocusItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setFocusItem_0<RetType> {
  fn setFocusItem_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setFocusItem_0<(/*void*/)> for (usize,i32) {
  fn setFocusItem_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene12setFocusItemEP13QGraphicsItemN2Qt11FocusReasonE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:212
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFocus() const

/*
Returns true if the scene has focus; otherwise returns false. If the scene has focus, it will will forward key events from QKeyEvent to any item that has focus.

See also setFocus() and setFocusItem().
*/
impl /*struct*/ QGraphicsScene {
  pub fn hasFocus_0<RetType, T: QGraphicsScene_hasFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_hasFocus_0<RetType> {
  fn hasFocus_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_hasFocus_0<bool> for () {
  fn hasFocus_0(self , rsthis: & QGraphicsScene) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene8hasFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:213
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocus(Qt::FocusReason)

/*
Sets focus on the scene by sending a QFocusEvent to the scene, passing focusReason as the reason. If the scene regains focus after having previously lost it while an item had focus, the last focus item will receive focus with focusReason as the reason.

If the scene already has focus, this function does nothing.

See also hasFocus(), clearFocus(), and setFocusItem().
*/
impl /*struct*/ QGraphicsScene {
  pub fn setFocus_0<RetType, T: QGraphicsScene_setFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setFocus_0<RetType> {
  fn setFocus_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setFocus_0<(/*void*/)> for (i32) {
  fn setFocus_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene8setFocusEN2Qt11FocusReasonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:214
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearFocus()

/*
Clears focus from the scene. If any item has focus when this function is called, it will lose focus, and regain focus again once the scene regains focus.

A scene that does not have focus ignores key events.

See also hasFocus(), setFocus(), and setFocusItem().
*/
impl /*struct*/ QGraphicsScene {
  pub fn clearFocus_0<RetType, T: QGraphicsScene_clearFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_clearFocus_0<RetType> {
  fn clearFocus_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_clearFocus_0<(/*void*/)> for () {
  fn clearFocus_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10clearFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:216
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStickyFocus(bool)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setStickyFocus_0<RetType, T: QGraphicsScene_setStickyFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStickyFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setStickyFocus_0<RetType> {
  fn setStickyFocus_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setStickyFocus_0<(/*void*/)> for (bool) {
  fn setStickyFocus_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene14setStickyFocusEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:217
// index:0
// Public Visibility=Default Availability=Available
// [1] bool stickyFocus() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn stickyFocus_0<RetType, T: QGraphicsScene_stickyFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stickyFocus_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_stickyFocus_0<RetType> {
  fn stickyFocus_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_stickyFocus_0<bool> for () {
  fn stickyFocus_0(self , rsthis: & QGraphicsScene) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene11stickyFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:219
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * mouseGrabberItem() const

/*
Returns the current mouse grabber item, or 0 if no item is currently grabbing the mouse. The mouse grabber item is the item that receives all mouse events sent to the scene.

An item becomes a mouse grabber when it receives and accepts a mouse press event, and it stays the mouse grabber until either of the following events occur:


If the item receives a mouse release event when there are no other buttons pressed, it loses the mouse grab.
If the item becomes invisible (i.e., someone calls item->setVisible(false)), or if it becomes disabled (i.e., someone calls item->setEnabled(false)), it loses the mouse grab.
If the item is removed from the scene, it loses the mouse grab.


If the item loses its mouse grab, the scene will ignore all mouse events until a new item grabs the mouse (i.e., until a new item receives a mouse press event).
*/
impl /*struct*/ QGraphicsScene {
  pub fn mouseGrabberItem_0<RetType, T: QGraphicsScene_mouseGrabberItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseGrabberItem_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_mouseGrabberItem_0<RetType> {
  fn mouseGrabberItem_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_mouseGrabberItem_0<usize> for () {
  fn mouseGrabberItem_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene16mouseGrabberItemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:221
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush backgroundBrush() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn backgroundBrush_0<RetType, T: QGraphicsScene_backgroundBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundBrush_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_backgroundBrush_0<RetType> {
  fn backgroundBrush_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_backgroundBrush_0<usize> for () {
  fn backgroundBrush_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene15backgroundBrushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:222
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackgroundBrush(const QBrush &)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setBackgroundBrush_0<RetType, T: QGraphicsScene_setBackgroundBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundBrush_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setBackgroundBrush_0<RetType> {
  fn setBackgroundBrush_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setBackgroundBrush_0<(/*void*/)> for (usize) {
  fn setBackgroundBrush_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene18setBackgroundBrushERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:224
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush foregroundBrush() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn foregroundBrush_0<RetType, T: QGraphicsScene_foregroundBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.foregroundBrush_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_foregroundBrush_0<RetType> {
  fn foregroundBrush_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_foregroundBrush_0<usize> for () {
  fn foregroundBrush_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene15foregroundBrushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:225
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setForegroundBrush(const QBrush &)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setForegroundBrush_0<RetType, T: QGraphicsScene_setForegroundBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setForegroundBrush_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setForegroundBrush_0<RetType> {
  fn setForegroundBrush_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setForegroundBrush_0<(/*void*/)> for (usize) {
  fn setForegroundBrush_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene18setForegroundBrushERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:227
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
This method is used by input methods to query a set of properties of the scene to be able to support complex input method operations as support for surrounding text and reconversions.

The query parameter specifies which property is queried.

See also QWidget::inputMethodQuery().
*/
impl /*struct*/ QGraphicsScene {
  pub fn inputMethodQuery_0<RetType, T: QGraphicsScene_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:231
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void update(qreal, qreal, qreal, qreal)

/*
Schedules a redraw of the area rect on the scene.

See also sceneRect() and changed().
*/
impl /*struct*/ QGraphicsScene {
  pub fn update_0<RetType, T: QGraphicsScene_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_update_0<RetType> {
  fn update_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_update_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn update_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene6updateEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:257
// index:1
// Public Visibility=Default Availability=Available
// [-2] void update(const QRectF &)

/*
Schedules a redraw of the area rect on the scene.

See also sceneRect() and changed().
*/
impl /*struct*/ QGraphicsScene {
  pub fn update_1<RetType, T: QGraphicsScene_update_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_update_1<RetType> {
  fn update_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_update_1<(/*void*/)> for (usize) {
  fn update_1(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene6updateERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:233
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void invalidate(qreal, qreal, qreal, qreal, QGraphicsScene::SceneLayers)

/*
Invalidates and schedules a redraw of the layers in rect on the scene. Any cached content in layers is unconditionally invalidated and redrawn.

You can use this function overload to notify QGraphicsScene of changes to the background or the foreground of the scene. This function is commonly used for scenes with tile-based backgrounds to notify changes when QGraphicsView has enabled CacheBackground.

Example:


  QRectF TileScene::rectForTile(int x, int y) const
  {
      // Return the rectangle for the tile at position (x, y).
      return QRectF(x * tileWidth, y * tileHeight, tileWidth, tileHeight);
  }

  void TileScene::setTile(int x, int y, const QPixmap &pixmap)
  {
      // Sets or replaces the tile at position (x, y) with pixmap.
      if (x >= 0 && x < numTilesH && y >= 0 && y < numTilesV) {
          tiles[y][x] = pixmap;
          invalidate(rectForTile(x, y), BackgroundLayer);
      }
  }

  void TileScene::drawBackground(QPainter *painter, const QRectF &exposed)
  {
      // Draws all tiles that intersect the exposed area.
      for (int y = 0; y < numTilesV; ++y) {
          for (int x = 0; x < numTilesH; ++x) {
              QRectF rect = rectForTile(x, y);
              if (exposed.intersects(rect))
                  painter->drawPixmap(rect.topLeft(), tiles[y][x]);
          }
      }
  }



Note that QGraphicsView currently supports background caching only (see QGraphicsView::CacheBackground). This function is equivalent to calling update() if any layer but BackgroundLayer is passed.

See also QGraphicsView::resetCachedContent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn invalidate_0<RetType, T: QGraphicsScene_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_invalidate_0<(/*void*/)> for (f64,f64,f64,f64,i32) {
  fn invalidate_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10invalidateEdddd6QFlagsINS_10SceneLayerEE", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:258
// index:1
// Public Visibility=Default Availability=Available
// [-2] void invalidate(const QRectF &, QGraphicsScene::SceneLayers)

/*
Invalidates and schedules a redraw of the layers in rect on the scene. Any cached content in layers is unconditionally invalidated and redrawn.

You can use this function overload to notify QGraphicsScene of changes to the background or the foreground of the scene. This function is commonly used for scenes with tile-based backgrounds to notify changes when QGraphicsView has enabled CacheBackground.

Example:


  QRectF TileScene::rectForTile(int x, int y) const
  {
      // Return the rectangle for the tile at position (x, y).
      return QRectF(x * tileWidth, y * tileHeight, tileWidth, tileHeight);
  }

  void TileScene::setTile(int x, int y, const QPixmap &pixmap)
  {
      // Sets or replaces the tile at position (x, y) with pixmap.
      if (x >= 0 && x < numTilesH && y >= 0 && y < numTilesV) {
          tiles[y][x] = pixmap;
          invalidate(rectForTile(x, y), BackgroundLayer);
      }
  }

  void TileScene::drawBackground(QPainter *painter, const QRectF &exposed)
  {
      // Draws all tiles that intersect the exposed area.
      for (int y = 0; y < numTilesV; ++y) {
          for (int x = 0; x < numTilesH; ++x) {
              QRectF rect = rectForTile(x, y);
              if (exposed.intersects(rect))
                  painter->drawPixmap(rect.topLeft(), tiles[y][x]);
          }
      }
  }



Note that QGraphicsView currently supports background caching only (see QGraphicsView::CacheBackground). This function is equivalent to calling update() if any layer but BackgroundLayer is passed.

See also QGraphicsView::resetCachedContent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn invalidate_1<RetType, T: QGraphicsScene_invalidate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_1(self);
    // return 1;
  }
}
pub trait QGraphicsScene_invalidate_1<RetType> {
  fn invalidate_1(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_invalidate_1<(/*void*/)> for (usize,i32) {
  fn invalidate_1(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10invalidateERK6QRectF6QFlagsINS_10SceneLayerEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:236
// index:0
// Public Visibility=Default Availability=Available
// [8] QStyle * style() const

/*
Returns the scene's style, or the same as QApplication::style() if the scene has not been explicitly assigned a style.

This function was introduced in  Qt 4.4.

See also setStyle().
*/
impl /*struct*/ QGraphicsScene {
  pub fn style_0<RetType, T: QGraphicsScene_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_style_0<RetType> {
  fn style_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_style_0<usize> for () {
  fn style_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:237
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyle(QStyle *)

/*
Sets or replaces the style of the scene to style, and reparents the style to this scene. Any previously assigned style is deleted. The scene's style defaults to QApplication::style(), and serves as the default for all QGraphicsWidget items in the scene.

Changing the style, either directly by calling this function, or indirectly by calling QApplication::setStyle(), will automatically update the style for all widgets in the scene that do not have a style explicitly assigned to them.

If style is 0, QGraphicsScene will revert to QApplication::style().

This function was introduced in  Qt 4.4.

See also style().
*/
impl /*struct*/ QGraphicsScene {
  pub fn setStyle_0<RetType, T: QGraphicsScene_setStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyle_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setStyle_0<RetType> {
  fn setStyle_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setStyle_0<(/*void*/)> for (usize) {
  fn setStyle_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene8setStyleEP6QStyle", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:239
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn font_0<RetType, T: QGraphicsScene_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_font_0<RetType> {
  fn font_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_font_0<usize> for () {
  fn font_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:240
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setFont_0<RetType, T: QGraphicsScene_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:242
// index:0
// Public Visibility=Default Availability=Available
// [16] QPalette palette() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn palette_0<RetType, T: QGraphicsScene_palette_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.palette_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_palette_0<RetType> {
  fn palette_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_palette_0<usize> for () {
  fn palette_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene7paletteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:243
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPalette(const QPalette &)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setPalette_0<RetType, T: QGraphicsScene_setPalette_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPalette_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setPalette_0<RetType> {
  fn setPalette_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setPalette_0<(/*void*/)> for (usize) {
  fn setPalette_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10setPaletteERK8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:245
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isActive() const

/*
Returns true if the scene is active (e.g., it's viewed by at least one QGraphicsView that is active); otherwise returns false.

This function was introduced in  Qt 4.6.

See also QGraphicsItem::isActive() and QWidget::isActiveWindow().
*/
impl /*struct*/ QGraphicsScene {
  pub fn isActive_0<RetType, T: QGraphicsScene_isActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActive_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_isActive_0<RetType> {
  fn isActive_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_isActive_0<bool> for () {
  fn isActive_0(self , rsthis: & QGraphicsScene) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:246
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsItem * activePanel() const

/*
Returns the current active panel, or 0 if no panel is currently active.

This function was introduced in  Qt 4.6.

See also QGraphicsScene::setActivePanel().
*/
impl /*struct*/ QGraphicsScene {
  pub fn activePanel_0<RetType, T: QGraphicsScene_activePanel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activePanel_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_activePanel_0<RetType> {
  fn activePanel_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_activePanel_0<usize> for () {
  fn activePanel_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene11activePanelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:247
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActivePanel(QGraphicsItem *)

/*
Activates item, which must be an item in this scene. You can also pass 0 for item, in which case QGraphicsScene will deactivate any currently active panel.

If the scene is currently inactive, item remains inactive until the scene becomes active (or, ir item is 0, no item will be activated).

This function was introduced in  Qt 4.6.

See also activePanel(), isActive(), and QGraphicsItem::isActive().
*/
impl /*struct*/ QGraphicsScene {
  pub fn setActivePanel_0<RetType, T: QGraphicsScene_setActivePanel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActivePanel_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setActivePanel_0<RetType> {
  fn setActivePanel_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setActivePanel_0<(/*void*/)> for (usize) {
  fn setActivePanel_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene14setActivePanelEP13QGraphicsItem", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:248
// index:0
// Public Visibility=Default Availability=Available
// [8] QGraphicsWidget * activeWindow() const

/*
Returns the current active window, or 0 if no window is currently active.

This function was introduced in  Qt 4.4.

See also QGraphicsScene::setActiveWindow().
*/
impl /*struct*/ QGraphicsScene {
  pub fn activeWindow_0<RetType, T: QGraphicsScene_activeWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeWindow_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_activeWindow_0<RetType> {
  fn activeWindow_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_activeWindow_0<usize> for () {
  fn activeWindow_0(self , rsthis: & QGraphicsScene) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene12activeWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:249
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setActiveWindow(QGraphicsWidget *)

/*
Activates widget, which must be a widget in this scene. You can also pass 0 for widget, in which case QGraphicsScene will deactivate any currently active window.

This function was introduced in  Qt 4.4.

See also activeWindow() and QGraphicsWidget::isActiveWindow().
*/
impl /*struct*/ QGraphicsScene {
  pub fn setActiveWindow_0<RetType, T: QGraphicsScene_setActiveWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActiveWindow_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setActiveWindow_0<RetType> {
  fn setActiveWindow_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setActiveWindow_0<(/*void*/)> for (usize) {
  fn setActiveWindow_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene15setActiveWindowEP15QGraphicsWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:251
// index:0
// Public Visibility=Default Availability=Available
// [1] bool sendEvent(QGraphicsItem *, QEvent *)

/*
Sends event event to item item through possible event filters.

The event is sent only if the item is enabled.

Returns false if the event was filtered or if the item is disabled. Otherwise returns the value that was returned from the event handler.

This function was introduced in  Qt 4.6.

See also QGraphicsItem::sceneEvent() and QGraphicsItem::sceneEventFilter().
*/
impl /*struct*/ QGraphicsScene {
  pub fn sendEvent_0<RetType, T: QGraphicsScene_sendEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sendEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_sendEvent_0<RetType> {
  fn sendEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_sendEvent_0<bool> for (usize,usize) {
  fn sendEvent_0(self , rsthis: & QGraphicsScene) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene9sendEventEP13QGraphicsItemP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:253
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal minimumRenderSize() const

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn minimumRenderSize_0<RetType, T: QGraphicsScene_minimumRenderSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumRenderSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_minimumRenderSize_0<RetType> {
  fn minimumRenderSize_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_minimumRenderSize_0<f64> for () {
  fn minimumRenderSize_0(self , rsthis: & QGraphicsScene) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScene17minimumRenderSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:254
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumRenderSize(qreal)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn setMinimumRenderSize_0<RetType, T: QGraphicsScene_setMinimumRenderSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumRenderSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_setMinimumRenderSize_0<RetType> {
  fn setMinimumRenderSize_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_setMinimumRenderSize_0<(/*void*/)> for (f64) {
  fn setMinimumRenderSize_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene20setMinimumRenderSizeEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:259
// index:0
// Public Visibility=Default Availability=Available
// [-2] void advance()

/*
This slot advances the scene by one step, by calling QGraphicsItem::advance() for all items on the scene. This is done in two phases: in the first phase, all items are notified that the scene is about to change, and in the second phase all items are notified that they can move. In the first phase, QGraphicsItem::advance() is called passing a value of 0 as an argument, and 1 is passed in the second phase.

Note that you can also use the Animation Framework for animations.

See also QGraphicsItem::advance() and QTimeLine.
*/
impl /*struct*/ QGraphicsScene {
  pub fn advance_0<RetType, T: QGraphicsScene_advance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.advance_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_advance_0<RetType> {
  fn advance_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_advance_0<(/*void*/)> for () {
  fn advance_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene7advanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:260
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearSelection()

/*
Clears the current selection.

See also setSelectionArea() and selectedItems().
*/
impl /*struct*/ QGraphicsScene {
  pub fn clearSelection_0<RetType, T: QGraphicsScene_clearSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearSelection_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_clearSelection_0<RetType> {
  fn clearSelection_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_clearSelection_0<(/*void*/)> for () {
  fn clearSelection_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene14clearSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:261
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes and deletes all items from the scene, but otherwise leaves the state of the scene unchanged.

This function was introduced in  Qt 4.4.

See also addItem().
*/
impl /*struct*/ QGraphicsScene {
  pub fn clear_0<RetType, T: QGraphicsScene_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_clear_0<RetType> {
  fn clear_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:264
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().

Processes the event event, and dispatches it to the respective event handlers.

In addition to calling the convenience event handlers, this function is responsible for converting mouse move events to hover events for when there is no mouse grabber item. Hover events are delivered directly to items; there is no convenience function for them.

Unlike QWidget, QGraphicsScene does not have the convenience functions enterEvent() and leaveEvent(). Use this function to obtain those events instead.

See also contextMenuEvent(), keyPressEvent(), keyReleaseEvent(), mousePressEvent(), mouseMoveEvent(), mouseReleaseEvent(), mouseDoubleClickEvent(), focusInEvent(), and focusOutEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn event_0<RetType, T: QGraphicsScene_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_event_0<RetType> {
  fn event_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QGraphicsScene) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:265
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().

QGraphicsScene filters QApplication's events to detect palette and font changes.
*/
impl /*struct*/ QGraphicsScene {
  pub fn eventFilter_0<RetType, T: QGraphicsScene_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QGraphicsScene) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:266
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QGraphicsSceneContextMenuEvent *)

/*
This event handler, for event contextMenuEvent, can be reimplemented in a subclass to receive context menu events. The default implementation forwards the event to the topmost visible item that accepts context menu events at the position of the event. If no items accept context menu events at this position, the event is ignored.

Note: See items() for a definition of which items are considered visible by this function.

See also QGraphicsItem::contextMenuEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn contextMenuEvent_0<RetType, T: QGraphicsScene_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16contextMenuEventEP30QGraphicsSceneContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:267
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive drag enter events for the scene.

The default implementation accepts the event and prepares the scene to accept drag move events.

See also QGraphicsItem::dragEnterEvent(), dragMoveEvent(), dragLeaveEvent(), and dropEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn dragEnterEvent_0<RetType, T: QGraphicsScene_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene14dragEnterEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:268
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive drag move events for the scene.

Note: See items() for a definition of which items are considered visible by this function.

See also QGraphicsItem::dragMoveEvent(), dragEnterEvent(), dragLeaveEvent(), and dropEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn dragMoveEvent_0<RetType, T: QGraphicsScene_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene13dragMoveEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:269
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive drag leave events for the scene.

See also QGraphicsItem::dragLeaveEvent(), dragEnterEvent(), dragMoveEvent(), and dropEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn dragLeaveEvent_0<RetType, T: QGraphicsScene_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene14dragLeaveEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:270
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QGraphicsSceneDragDropEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive drop events for the scene.

See also QGraphicsItem::dropEvent(), dragEnterEvent(), dragMoveEvent(), and dragLeaveEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn dropEvent_0<RetType, T: QGraphicsScene_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene9dropEventEP27QGraphicsSceneDragDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:271
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
This event handler, for event focusEvent, can be reimplemented in a subclass to receive focus in events.

The default implementation sets focus on the scene, and then on the last focus item.

See also QGraphicsItem::focusOutEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn focusInEvent_0<RetType, T: QGraphicsScene_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:272
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
This event handler, for event focusEvent, can be reimplemented in a subclass to receive focus out events.

The default implementation removes focus from any focus item, then removes focus from the scene.

See also QGraphicsItem::focusInEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn focusOutEvent_0<RetType, T: QGraphicsScene_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:273
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void helpEvent(QGraphicsSceneHelpEvent *)

/*
This event handler, for event helpEvent, can be reimplemented in a subclass to receive help events. The events are of type QEvent::ToolTip, which are created when a tooltip is requested.

The default implementation shows the tooltip of the topmost visible item, i.e., the item with the highest z-value, at the mouse cursor position. If no item has a tooltip set, this function does nothing.

Note: See items() for a definition of which items are considered visible by this function.

See also QGraphicsItem::toolTip() and QGraphicsSceneHelpEvent.
*/
impl /*struct*/ QGraphicsScene {
  pub fn helpEvent_0<RetType, T: QGraphicsScene_helpEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.helpEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_helpEvent_0<RetType> {
  fn helpEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_helpEvent_0<(/*void*/)> for (usize) {
  fn helpEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene9helpEventEP23QGraphicsSceneHelpEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:274
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
This event handler, for event keyEvent, can be reimplemented in a subclass to receive keypress events. The default implementation forwards the event to current focus item.

See also QGraphicsItem::keyPressEvent() and focusItem().
*/
impl /*struct*/ QGraphicsScene {
  pub fn keyPressEvent_0<RetType, T: QGraphicsScene_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:275
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
This event handler, for event keyEvent, can be reimplemented in a subclass to receive key release events. The default implementation forwards the event to current focus item.

See also QGraphicsItem::keyReleaseEvent() and focusItem().
*/
impl /*struct*/ QGraphicsScene {
  pub fn keyReleaseEvent_0<RetType, T: QGraphicsScene_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:276
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event mouseEvent, can be reimplemented in a subclass to receive mouse press events for the scene.

The default implementation depends on the state of the scene. If there is a mouse grabber item, then the event is sent to the mouse grabber. Otherwise, it is forwarded to the topmost visible item that accepts mouse events at the scene position from the event, and that item promptly becomes the mouse grabber item.

If there is no item at the given position on the scene, the selection area is reset, any focus item loses its input focus, and the event is then ignored.

Note: See items() for a definition of which items are considered visible by this function.

See also QGraphicsItem::mousePressEvent() and QGraphicsItem::setAcceptedMouseButtons().
*/
impl /*struct*/ QGraphicsScene {
  pub fn mousePressEvent_0<RetType, T: QGraphicsScene_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene15mousePressEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:277
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event mouseEvent, can be reimplemented in a subclass to receive mouse move events for the scene.

The default implementation depends on the mouse grabber state. If there is a mouse grabber item, the event is sent to the mouse grabber. If there are any items that accept hover events at the current position, the event is translated into a hover event and accepted; otherwise it's ignored.

See also QGraphicsItem::mousePressEvent(), QGraphicsItem::mouseReleaseEvent(), QGraphicsItem::mouseDoubleClickEvent(), and QGraphicsItem::setAcceptedMouseButtons().
*/
impl /*struct*/ QGraphicsScene {
  pub fn mouseMoveEvent_0<RetType, T: QGraphicsScene_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene14mouseMoveEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:278
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event mouseEvent, can be reimplemented in a subclass to receive mouse release events for the scene.

The default implementation depends on the mouse grabber state. If there is no mouse grabber, the event is ignored. Otherwise, if there is a mouse grabber item, the event is sent to the mouse grabber. If this mouse release represents the last pressed button on the mouse, the mouse grabber item then loses the mouse grab.

See also QGraphicsItem::mousePressEvent(), QGraphicsItem::mouseMoveEvent(), QGraphicsItem::mouseDoubleClickEvent(), and QGraphicsItem::setAcceptedMouseButtons().
*/
impl /*struct*/ QGraphicsScene {
  pub fn mouseReleaseEvent_0<RetType, T: QGraphicsScene_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene17mouseReleaseEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:279
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *)

/*
This event handler, for event mouseEvent, can be reimplemented in a subclass to receive mouse doubleclick events for the scene.

If someone doubleclicks on the scene, the scene will first receive a mouse press event, followed by a release event (i.e., a click), then a doubleclick event, and finally a release event. If the doubleclick event is delivered to a different item than the one that received the first press and release, it will be delivered as a press event. However, tripleclick events are not delivered as doubleclick events in this case.

The default implementation is similar to mousePressEvent().

Note: See items() for a definition of which items are considered visible by this function.

See also QGraphicsItem::mousePressEvent(), QGraphicsItem::mouseMoveEvent(), QGraphicsItem::mouseReleaseEvent(), and QGraphicsItem::setAcceptedMouseButtons().
*/
impl /*struct*/ QGraphicsScene {
  pub fn mouseDoubleClickEvent_0<RetType, T: QGraphicsScene_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene21mouseDoubleClickEventEP24QGraphicsSceneMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:280
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QGraphicsSceneWheelEvent *)

/*
This event handler, for event wheelEvent, can be reimplemented in a subclass to receive mouse wheel events for the scene.

By default, the event is delivered to the topmost visible item under the cursor. If ignored, the event propagates to the item beneath, and again until the event is accepted, or it reaches the scene. If no items accept the event, it is ignored.

Note: See items() for a definition of which items are considered visible by this function.

See also QGraphicsItem::wheelEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn wheelEvent_0<RetType, T: QGraphicsScene_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene10wheelEventEP24QGraphicsSceneWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:281
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
This event handler, for event event, can be reimplemented in a subclass to receive input method events for the scene.

The default implementation forwards the event to the focusItem(). If no item currently has focus or the current focus item does not accept input methods, this function does nothing.

See also QGraphicsItem::inputMethodEvent().
*/
impl /*struct*/ QGraphicsScene {
  pub fn inputMethodEvent_0<RetType, T: QGraphicsScene_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:283
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawBackground(QPainter *, const QRectF &)

/*
Draws the background of the scene using painter, before any items and the foreground are drawn. Reimplement this function to provide a custom background for the scene.

All painting is done in scene coordinates. The rect parameter is the exposed rectangle.

If all you want is to define a color, texture, or gradient for the background, you can call setBackgroundBrush() instead.

See also drawForeground() and drawItems().
*/
impl /*struct*/ QGraphicsScene {
  pub fn drawBackground_0<RetType, T: QGraphicsScene_drawBackground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawBackground_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_drawBackground_0<RetType> {
  fn drawBackground_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_drawBackground_0<(/*void*/)> for (usize,usize) {
  fn drawBackground_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene14drawBackgroundEP8QPainterRK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:284
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawForeground(QPainter *, const QRectF &)

/*
Draws the foreground of the scene using painter, after the background and all items have been drawn. Reimplement this function to provide a custom foreground for the scene.

All painting is done in scene coordinates. The rect parameter is the exposed rectangle.

If all you want is to define a color, texture or gradient for the foreground, you can call setForegroundBrush() instead.

See also drawBackground() and drawItems().
*/
impl /*struct*/ QGraphicsScene {
  pub fn drawForeground_0<RetType, T: QGraphicsScene_drawForeground_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawForeground_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_drawForeground_0<RetType> {
  fn drawForeground_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_drawForeground_0<(/*void*/)> for (usize,usize) {
  fn drawForeground_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene14drawForegroundEP8QPainterRK6QRectF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:285
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawItems(QPainter *, int, QGraphicsItem **, const QStyleOptionGraphicsItem *, QWidget *)

/*

*/
impl /*struct*/ QGraphicsScene {
  pub fn drawItems_0<RetType, T: QGraphicsScene_drawItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawItems_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_drawItems_0<RetType> {
  fn drawItems_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_drawItems_0<(/*void*/)> for (usize,i32,usize,usize,usize) {
  fn drawItems_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene9drawItemsEP8QPainteriPP13QGraphicsItemPK24QStyleOptionGraphicsItemP7QWidget", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:295
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Finds a new widget to give the keyboard focus to, as appropriate for Tab and Shift+Tab, and returns true if it can find a new widget, or false if it cannot. If next is true, this function searches forward; if next is false, it searches backward.

You can reimplement this function in a subclass of QGraphicsScene to provide fine-grained control over how tab focus passes inside your scene. The default implementation is based on the tab focus chain defined by QGraphicsWidget::setTabOrder().

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QGraphicsScene {
  pub fn focusNextPrevChild_0<RetType, T: QGraphicsScene_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QGraphicsScene) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QGraphicsScene18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:299
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sceneRectChanged(const QRectF &)

/*
This signal is emitted by QGraphicsScene whenever the scene rect changes. The rect parameter is the new scene rectangle.

See also QGraphicsView::updateSceneRect().
*/
impl /*struct*/ QGraphicsScene {
  pub fn sceneRectChanged_0<RetType, T: QGraphicsScene_sceneRectChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sceneRectChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_sceneRectChanged_0<RetType> {
  fn sceneRectChanged_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_sceneRectChanged_0<(/*void*/)> for (usize) {
  fn sceneRectChanged_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16sceneRectChangedERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:300
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectionChanged()

/*
This signal is emitted by QGraphicsScene whenever the selection changes. You can call selectedItems() to get the new list of selected items.

The selection changes whenever an item is selected or unselected, a selection area is set, cleared or otherwise changed, if a preselected item is added to the scene, or if a selected item is removed from the scene.

QGraphicsScene emits this signal only once for group selection operations. For example, if you set a selection area, select or unselect a QGraphicsItemGroup, or if you add or remove from the scene a parent item that contains several selected items, selectionChanged() is emitted only once after the operation has completed (instead of once for each item).

This function was introduced in  Qt 4.3.

See also setSelectionArea(), selectedItems(), and QGraphicsItem::setSelected().
*/
impl /*struct*/ QGraphicsScene {
  pub fn selectionChanged_0<RetType, T: QGraphicsScene_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_selectionChanged_0<(/*void*/)> for () {
  fn selectionChanged_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16selectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicsscene.h:301
// index:0
// Public Visibility=Default Availability=Available
// [-2] void focusItemChanged(QGraphicsItem *, QGraphicsItem *, Qt::FocusReason)

/*
This signal is emitted by QGraphicsScene whenever focus changes in the scene (i.e., when an item gains or loses input focus, or when focus passes from one item to another). You can connect to this signal if you need to keep track of when other items gain input focus. It is particularly useful for implementing virtual keyboards, input methods, and cursor items.

oldFocusItem is a pointer to the item that previously had focus, or 0 if no item had focus before the signal was emitted. newFocusItem is a pointer to the item that gained input focus, or 0 if focus was lost. reason is the reason for the focus change (e.g., if the scene was deactivated while an input field had focus, oldFocusItem would point to the input field item, newFocusItem would be 0, and reason would be Qt::ActiveWindowFocusReason.
*/
impl /*struct*/ QGraphicsScene {
  pub fn focusItemChanged_0<RetType, T: QGraphicsScene_focusItemChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusItemChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsScene_focusItemChanged_0<RetType> {
  fn focusItemChanged_0(self , rsthis: & QGraphicsScene) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScene_focusItemChanged_0<(/*void*/)> for (usize,usize,i32) {
  fn focusItemChanged_0(self , rsthis: & QGraphicsScene) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScene16focusItemChangedEP13QGraphicsItemS1_N2Qt11FocusReasonE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the indexing algorithms QGraphicsScene provides for managing positional information about items on the scene.



See also setItemIndexMethod() and bspTreeDepth.

*/
pub type QGraphicsScene__ItemIndexMethod = i32;
// A Binary Space Partitioning tree is applied. All QGraphicsScene's item location algorithms are of an order close to logarithmic complexity, by making use of binary search. Adding, moving and removing items is logarithmic. This approach is best for static scenes (i.e., scenes where most items do not move).
pub const QGraphicsScene__BspTreeIndex :QGraphicsScene__ItemIndexMethod = 0;
// 
pub const QGraphicsScene__NoIndex :QGraphicsScene__ItemIndexMethod = -1;
pub fn QGraphicsScene_ItemIndexMethodItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsScene", val);
}
pub fn QGraphicsScene_ItemIndexMethodItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsScene
  //return nilthis.ItemIndexMethodItemName(val);
  return QGraphicsScene_ItemIndexMethodItemName(val);
}


/*


*/
pub type QGraphicsScene__SceneLayer = i32;
// 
pub const QGraphicsScene__ItemLayer :QGraphicsScene__SceneLayer = 1;
// 
pub const QGraphicsScene__BackgroundLayer :QGraphicsScene__SceneLayer = 2;
// 
pub const QGraphicsScene__ForegroundLayer :QGraphicsScene__SceneLayer = 4;
// 
pub const QGraphicsScene__AllLayers :QGraphicsScene__SceneLayer = 65535;
pub fn QGraphicsScene_SceneLayerItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsScene", val);
}
pub fn QGraphicsScene_SceneLayerItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsScene
  //return nilthis.SceneLayerItemName(val);
  return QGraphicsScene_SceneLayerItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
