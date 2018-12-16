

// mod ::core::QObject
// package qtcore
// /usr/include/qt/QtCore/qobject.h
// #include <qobject.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// QObject * sender()
// func (this *QObject) InheritSender(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "sender", f)
// }

// int senderSignalIndex()
// func (this *QObject) InheritSenderSignalIndex(f func() int) {
//  qtrt.SetAllInheritCallback(this, "senderSignalIndex", f)
// }

// int receivers(const char *)
// func (this *QObject) InheritReceivers(f func(signal string) int) {
//  qtrt.SetAllInheritCallback(this, "receivers", f)
// }

// bool isSignalConnected(const QMetaMethod &)
// func (this *QObject) InheritIsSignalConnected(f func(signal *QMetaMethod) bool) {
//  qtrt.SetAllInheritCallback(this, "isSignalConnected", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QObject) InheritTimerEvent(f func(event *QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void childEvent(QChildEvent *)
// func (this *QObject) InheritChildEvent(f func(event *QChildEvent/*777 QChildEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "childEvent", f)
// }

// void customEvent(QEvent *)
// func (this *QObject) InheritCustomEvent(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "customEvent", f)
// }

// void connectNotify(const QMetaMethod &)
// func (this *QObject) InheritConnectNotify(f func(signal *QMetaMethod) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "connectNotify", f)
// }

// void disconnectNotify(const QMetaMethod &)
// func (this *QObject) InheritDisconnectNotify(f func(signal *QMetaMethod) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "disconnectNotify", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QObject)=16
pub struct QObject {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QObject_ITF interface {
//    QObject_PTR() *QObject
//}
//func (ptr *QObject) QObject_PTR() *QObject { return ptr }

impl /*struct*/ QObject {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QObject {
    return QObject{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QObject {
//  type Target = QObjectBASE;
//
//  fn deref(&self) -> &QObjectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QObjectBASE> for QObject {
//  fn as_ref(& self) -> & QObjectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qobject.h:119
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*
Returns a pointer to the meta-object of this object.

A meta-object contains information about a class that inherits QObject, e.g. class name, superclass name, properties, signals and slots. Every QObject subclass that contains the Q_OBJECT macro will have a meta-object.

The meta-object information is required by the signal/slot connection mechanism and the property system. The inherits() function also makes use of the meta-object.

If you have no pointer to an actual object instance but still want to access the meta-object of a class, you can use staticMetaObject.

Example:


  QObject *obj = new QPushButton;
  obj->metaObject()->className();             // returns "QPushButton"

  QPushButton::staticMetaObject.className();  // returns "QPushButton"



See also staticMetaObject.
*/
impl /*struct*/ QObject {
  pub fn metaObject_0<RetType, T: QObject_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QObject_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QObject(QObject *)

/*
Constructs an object with parent object parent.

The parent of an object may be viewed as the object's owner. For instance, a dialog box is the parent of the OK and Cancel buttons it contains.

The destructor of a parent object destroys all child objects.

Setting parent to 0 constructs an object with no parent. If the object is a widget, it will become a top-level window.

See also parent(), findChild(), and findChildren().
*/
// QObject(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QObject {
  pub fn QObject_0<T: QObject_QObject_0>(value: T) -> QObject {
    let rsthis = value.QObject_0();
    return rsthis;
    // return 1;
  }
}

pub trait QObject_QObject_0 {
  fn QObject_0(self) -> QObject;
}
// QObject(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QObject_QObject_0 for (usize) {
  fn QObject_0(self) -> QObject {
    // unsafe{_ZN7QObjectC2EPS_()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QObjectC2EPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:125
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QObject()

/*

*/
pub fn DeleteQObject(this :*mut QObject) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QObjectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qobject.h:127
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
This virtual function receives events to an object and should return true if the event e was recognized and processed.

The event() function can be reimplemented to customize the behavior of an object.

Make sure you call the parent event class implementation for all the events you did not handle.

Example:


  class MyClass : public QWidget
  {
      Q_OBJECT

  public:
      MyClass(QWidget *parent = 0);
      ~MyClass();

      bool event(QEvent* ev)
      {
          if (ev->type() == QEvent::PolishRequest) {
              // overwrite handling of PolishRequest if any
              doThings();
              return true;
          } else  if (ev->type() == QEvent::Show) {
              // complement handling of Show if any
              doThings2();
              QWidget::event(ev);
              return true;
          }
          // Make sure the rest of events are handled
          return QWidget::event(ev);
      }
  };



See also installEventFilter(), timerEvent(), QCoreApplication::sendEvent(), and QCoreApplication::postEvent().
*/
impl /*struct*/ QObject {
  pub fn event_0<RetType, T: QObject_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QObject_event_0<RetType> {
  fn event_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:128
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Filters events if this object has been installed as an event filter for the watched object.

In your reimplementation of this function, if you want to filter the event out, i.e. stop it being handled further, return true; otherwise return false.

Example:


  class MainWindow : public QMainWindow
  {
  public:
      MainWindow();

  protected:
      bool eventFilter(QObject *obj, QEvent *ev);

  private:
      QTextEdit *textEdit;
  };

  MainWindow::MainWindow()
  {
      textEdit = new QTextEdit;
      setCentralWidget(textEdit);

      textEdit->installEventFilter(this);
  }

  bool MainWindow::eventFilter(QObject *obj, QEvent *event)
  {
      if (obj == textEdit) {
          if (event->type() == QEvent::KeyPress) {
              QKeyEvent *keyEvent = static_cast<QKeyEvent*>(event);
              qDebug() << "Ate key press" << keyEvent->key();
              return true;
          } else {
              return false;
          }
      } else {
          // pass the event on to the parent class
          return QMainWindow::eventFilter(obj, event);
      }
  }



Notice in the example above that unhandled events are passed to the base class's eventFilter() function, since the base class might have reimplemented eventFilter() for its own internal purposes.

Warning: If you delete the receiver object in this function, be sure to return true. Otherwise, Qt will forward the event to the deleted object and the program might crash.

See also installEventFilter().
*/
impl /*struct*/ QObject {
  pub fn eventFilter_0<RetType, T: QObject_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QObject_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject11eventFilterEPS_P6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:145
// index:0
// Public Visibility=Default Availability=Available
// [8] QString objectName() const

/*

*/
impl /*struct*/ QObject {
  pub fn objectName_0<RetType, T: QObject_objectName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.objectName_0(self);
    // return 1;
  }
}
pub trait QObject_objectName_0<RetType> {
  fn objectName_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_objectName_0<usize> for () {
  fn objectName_0(self , rsthis: & QObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject10objectNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setObjectName(const QString &)

/*

*/
impl /*struct*/ QObject {
  pub fn setObjectName_0<RetType, T: QObject_setObjectName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setObjectName_0(self);
    // return 1;
  }
}
pub trait QObject_setObjectName_0<RetType> {
  fn setObjectName_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_setObjectName_0<(/*void*/)> for (usize) {
  fn setObjectName_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject13setObjectNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:148
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isWidgetType() const

/*
Returns true if the object is a widget; otherwise returns false.

Calling this function is equivalent to calling inherits("QWidget"), except that it is much faster.
*/
impl /*struct*/ QObject {
  pub fn isWidgetType_0<RetType, T: QObject_isWidgetType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWidgetType_0(self);
    // return 1;
  }
}
pub trait QObject_isWidgetType_0<RetType> {
  fn isWidgetType_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_isWidgetType_0<bool> for () {
  fn isWidgetType_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject12isWidgetTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:149
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isWindowType() const

/*
Returns true if the object is a window; otherwise returns false.

Calling this function is equivalent to calling inherits("QWindow"), except that it is much faster.
*/
impl /*struct*/ QObject {
  pub fn isWindowType_0<RetType, T: QObject_isWindowType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWindowType_0(self);
    // return 1;
  }
}
pub trait QObject_isWindowType_0<RetType> {
  fn isWindowType_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_isWindowType_0<bool> for () {
  fn isWindowType_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject12isWindowTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:151
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool signalsBlocked() const

/*
Returns true if signals are blocked; otherwise returns false.

Signals are not blocked by default.

See also blockSignals() and QSignalBlocker.
*/
impl /*struct*/ QObject {
  pub fn signalsBlocked_0<RetType, T: QObject_signalsBlocked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.signalsBlocked_0(self);
    // return 1;
  }
}
pub trait QObject_signalsBlocked_0<RetType> {
  fn signalsBlocked_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_signalsBlocked_0<bool> for () {
  fn signalsBlocked_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject14signalsBlockedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:152
// index:0
// Public Visibility=Default Availability=Available
// [1] bool blockSignals(bool)

/*
If block is true, signals emitted by this object are blocked (i.e., emitting a signal will not invoke anything connected to it). If block is false, no such blocking will occur.

The return value is the previous value of signalsBlocked().

Note that the destroyed() signal will be emitted even if the signals for this object have been blocked.

Signals emitted while being blocked are not buffered.

See also signalsBlocked() and QSignalBlocker.
*/
impl /*struct*/ QObject {
  pub fn blockSignals_0<RetType, T: QObject_blockSignals_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockSignals_0(self);
    // return 1;
  }
}
pub trait QObject_blockSignals_0<RetType> {
  fn blockSignals_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_blockSignals_0<bool> for (bool) {
  fn blockSignals_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject12blockSignalsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:154
// index:0
// Public Visibility=Default Availability=Available
// [8] QThread * thread() const

/*
Returns the thread in which the object lives.

See also moveToThread().
*/
impl /*struct*/ QObject {
  pub fn thread_0<RetType, T: QObject_thread_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.thread_0(self);
    // return 1;
  }
}
pub trait QObject_thread_0<RetType> {
  fn thread_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_thread_0<usize> for () {
  fn thread_0(self , rsthis: & QObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject6threadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void moveToThread(QThread *)

/*
Changes the thread affinity for this object and its children. The object cannot be moved if it has a parent. Event processing will continue in the targetThread.

To move an object to the main thread, use QApplication::instance() to retrieve a pointer to the current application, and then use QApplication::thread() to retrieve the thread in which the application lives. For example:


  myObject->moveToThread(QApplication::instance()->thread());



If targetThread is zero, all event processing for this object and its children stops.

Note that all active timers for the object will be reset. The timers are first stopped in the current thread and restarted (with the same interval) in the targetThread. As a result, constantly moving an object between threads can postpone timer events indefinitely.

A QEvent::ThreadChange event is sent to this object just before the thread affinity is changed. You can handle this event to perform any special processing. Note that any new events that are posted to this object will be handled in the targetThread.

Warning: This function is not thread-safe; the current thread must be same as the current thread affinity. In other words, this function can only "push" an object from the current thread to another thread, it cannot "pull" an object from any arbitrary thread to the current thread.

See also thread().
*/
impl /*struct*/ QObject {
  pub fn moveToThread_0<RetType, T: QObject_moveToThread_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveToThread_0(self);
    // return 1;
  }
}
pub trait QObject_moveToThread_0<RetType> {
  fn moveToThread_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_moveToThread_0<(/*void*/)> for (usize) {
  fn moveToThread_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject12moveToThreadEP7QThread", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:157
// index:0
// Public Visibility=Default Availability=Available
// [4] int startTimer(int, Qt::TimerType)

/*
Starts a timer and returns a timer identifier, or returns zero if it could not start a timer.

A timer event will occur every interval milliseconds until killTimer() is called. If interval is 0, then the timer event occurs once every time there are no more window system events to process.

The virtual timerEvent() function is called with the QTimerEvent event parameter class when a timer event occurs. Reimplement this function to get timer events.

If multiple timers are running, the QTimerEvent::timerId() can be used to find out which timer was activated.

Example:


  class MyObject : public QObject
  {
      Q_OBJECT

  public:
      MyObject(QObject *parent = 0);

  protected:
      void timerEvent(QTimerEvent *event);
  };

  MyObject::MyObject(QObject *parent)
      : QObject(parent)
  {
      startTimer(50);     // 50-millisecond timer
      startTimer(1000);   // 1-second timer
      startTimer(60000);  // 1-minute timer

      using namespace std::chrono;
      startTimer(milliseconds(50));
      startTimer(seconds(1));
      startTimer(minutes(1));

      // since C++14 we can use std::chrono::duration literals, e.g.:
      startTimer(100ms);
      startTimer(5s);
      startTimer(2min);
      startTimer(1h);
  }

  void MyObject::timerEvent(QTimerEvent *event)
  {
      qDebug() << "Timer ID:" << event->timerId();
  }



Note that QTimer's accuracy depends on the underlying operating system and hardware. The timerType argument allows you to customize the accuracy of the timer. See Qt::TimerType for information on the different timer types. Most platforms support an accuracy of 20 milliseconds; some provide more. If Qt is unable to deliver the requested number of timer events, it will silently discard some.

The QTimer class provides a high-level programming interface with single-shot timers and timer signals instead of events. There is also a QBasicTimer class that is more lightweight than QTimer and less clumsy than using timer IDs directly.

See also timerEvent(), killTimer(), and QTimer::singleShot().
*/
impl /*struct*/ QObject {
  pub fn startTimer_0<RetType, T: QObject_startTimer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startTimer_0(self);
    // return 1;
  }
}
pub trait QObject_startTimer_0<RetType> {
  fn startTimer_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_startTimer_0<i32> for (i32,i32) {
  fn startTimer_0(self , rsthis: & QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject10startTimerEiN2Qt9TimerTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:165
// index:0
// Public Visibility=Default Availability=Available
// [-2] void killTimer(int)

/*
Kills the timer with timer identifier, id.

The timer identifier is returned by startTimer() when a timer event is started.

See also timerEvent() and startTimer().
*/
impl /*struct*/ QObject {
  pub fn killTimer_0<RetType, T: QObject_killTimer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.killTimer_0(self);
    // return 1;
  }
}
pub trait QObject_killTimer_0<RetType> {
  fn killTimer_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_killTimer_0<(/*void*/)> for (i32) {
  fn killTimer_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject9killTimerEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:208
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QObjectList & children() const

/*
Returns a list of child objects. The QObjectList class is defined in the <QObject> header file as the following:


  typedef QList<QObject*> QObjectList;



The first child added is the first object in the list and the last child added is the last object in the list, i.e. new children are appended at the end.

Note that the list order changes when QWidget children are raised or lowered. A widget that is raised becomes the last object in the list, and a widget that is lowered becomes the first object in the list.

See also findChild(), findChildren(), parent(), and setParent().
*/
impl /*struct*/ QObject {
  pub fn children_0<RetType, T: QObject_children_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.children_0(self);
    // return 1;
  }
}
pub trait QObject_children_0<RetType> {
  fn children_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_children_0<usize> for () {
  fn children_0(self , rsthis: & QObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject8childrenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:210
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setParent(QObject *)

/*
Makes the object a child of parent.

See also parent() and children().
*/
impl /*struct*/ QObject {
  pub fn setParent_0<RetType, T: QObject_setParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setParent_0(self);
    // return 1;
  }
}
pub trait QObject_setParent_0<RetType> {
  fn setParent_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_setParent_0<(/*void*/)> for (usize) {
  fn setParent_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject9setParentEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:211
// index:0
// Public Visibility=Default Availability=Available
// [-2] void installEventFilter(QObject *)

/*
Installs an event filter filterObj on this object. For example:


  monitoredObj->installEventFilter(filterObj);



An event filter is an object that receives all events that are sent to this object. The filter can either stop the event or forward it to this object. The event filter filterObj receives events via its eventFilter() function. The eventFilter() function must return true if the event should be filtered, (i.e. stopped); otherwise it must return false.

If multiple event filters are installed on a single object, the filter that was installed last is activated first.

Here's a KeyPressEater class that eats the key presses of its monitored objects:


  class KeyPressEater : public QObject
  {
      Q_OBJECT
      ...

  protected:
      bool eventFilter(QObject *obj, QEvent *event);
  };

  bool KeyPressEater::eventFilter(QObject *obj, QEvent *event)
  {
      if (event->type() == QEvent::KeyPress) {
          QKeyEvent *keyEvent = static_cast<QKeyEvent *>(event);
          qDebug("Ate key press %d", keyEvent->key());
          return true;
      } else {
          // standard event processing
          return QObject::eventFilter(obj, event);
      }
  }



And here's how to install it on two widgets:


  KeyPressEater *keyPressEater = new KeyPressEater(this);
  QPushButton *pushButton = new QPushButton(this);
  QListView *listView = new QListView(this);

  pushButton->installEventFilter(keyPressEater);
  listView->installEventFilter(keyPressEater);



The QShortcut class, for example, uses this technique to intercept shortcut key presses.

Warning: If you delete the receiver object in your eventFilter() function, be sure to return true. If you return false, Qt sends the event to the deleted object and the program will crash.

Note that the filtering object must be in the same thread as this object. If filterObj is in a different thread, this function does nothing. If either filterObj or this object are moved to a different thread after calling this function, the event filter will not be called until both objects have the same thread affinity again (it is not removed).

See also removeEventFilter(), eventFilter(), and event().
*/
impl /*struct*/ QObject {
  pub fn installEventFilter_0<RetType, T: QObject_installEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.installEventFilter_0(self);
    // return 1;
  }
}
pub trait QObject_installEventFilter_0<RetType> {
  fn installEventFilter_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_installEventFilter_0<(/*void*/)> for (usize) {
  fn installEventFilter_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject18installEventFilterEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:212
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeEventFilter(QObject *)

/*
Removes an event filter object obj from this object. The request is ignored if such an event filter has not been installed.

All event filters for this object are automatically removed when this object is destroyed.

It is always safe to remove an event filter, even during event filter activation (i.e. from the eventFilter() function).

See also installEventFilter(), eventFilter(), and event().
*/
impl /*struct*/ QObject {
  pub fn removeEventFilter_0<RetType, T: QObject_removeEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeEventFilter_0(self);
    // return 1;
  }
}
pub trait QObject_removeEventFilter_0<RetType> {
  fn removeEventFilter_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_removeEventFilter_0<(/*void*/)> for (usize) {
  fn removeEventFilter_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject17removeEventFilterEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:214
// index:0
// Public static Visibility=Default Availability=Available
// [8] QMetaObject::Connection connect(const QObject *, const char *, const QObject *, const char *, Qt::ConnectionType)

/*
Creates a connection of the given type from the signal in the sender object to the method in the receiver object. Returns a handle to the connection that can be used to disconnect it later.

You must use the SIGNAL() and SLOT() macros when specifying the signal and the method, for example:


  QLabel *label = new QLabel;
  QScrollBar *scrollBar = new QScrollBar;
  QObject::connect(scrollBar, SIGNAL(valueChanged(int)),
                   label,  SLOT(setNum(int)));



This example ensures that the label always displays the current scroll bar value. Note that the signal and slots parameters must not contain any variable names, only the type. E.g. the following would not work and return false:


  // WRONG
  QObject::connect(scrollBar, SIGNAL(valueChanged(int value)),
                   label, SLOT(setNum(int value)));



A signal can also be connected to another signal:


  class MyWidget : public QWidget
  {
      Q_OBJECT

  public:
      MyWidget();

  signals:
      void buttonClicked();

  private:
      QPushButton *myButton;
  };

  MyWidget::MyWidget()
  {
      myButton = new QPushButton(this);
      connect(myButton, SIGNAL(clicked()),
              this, SIGNAL(buttonClicked()));
  }



In this example, the MyWidget constructor relays a signal from a private member variable, and makes it available under a name that relates to MyWidget.

A signal can be connected to many slots and signals. Many signals can be connected to one slot.

If a signal is connected to several slots, the slots are activated in the same order in which the connections were made, when the signal is emitted.

The function returns a QMetaObject::Connection that represents a handle to a connection if it successfully connects the signal to the slot. The connection handle will be invalid if it cannot create the connection, for example, if QObject is unable to verify the existence of either signal or method, or if their signatures aren't compatible. You can check if the handle is valid by casting it to a bool.

By default, a signal is emitted for every connection you make; two signals are emitted for duplicate connections. You can break all of these connections with a single disconnect() call. If you pass the Qt::UniqueConnection type, the connection will only be made if it is not a duplicate. If there is already a duplicate (exact same signal to the exact same slot on the same objects), the connection will fail and connect will return an invalid QMetaObject::Connection.

Note: Qt::UniqueConnections do not work for lambdas, non-member functions and functors; they only apply to connecting to member functions.

The optional type parameter describes the type of connection to establish. In particular, it determines whether a particular signal is delivered to a slot immediately or queued for delivery at a later time. If the signal is queued, the parameters must be of types that are known to Qt's meta-object system, because Qt needs to copy the arguments to store them in an event behind the scenes. If you try to use a queued connection and get the error message


  QObject::connect: Cannot queue arguments of type 'MyType'
  (Make sure 'MyType' is registered using qRegisterMetaType().)



call qRegisterMetaType() to register the data type before you establish the connection.

Note: This function is thread-safe.

See also disconnect(), sender(), qRegisterMetaType(), Q_DECLARE_METATYPE(), and Differences between String-Based and Functor-Based Connections.
*/
impl /*struct*/ QObject {
  pub fn connect_0<RetType, T: QObject_connect_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.connect_0();
    // return 1;
  }
}
pub trait QObject_connect_0<RetType> {
  fn connect_0(self ) -> RetType;
}
impl<'a> /*trait*/ QObject_connect_0<i32> for (usize,usize,usize,usize,i32) {
  fn connect_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject7connectEPKS_PKcS1_S3_N2Qt14ConnectionTypeE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:217
// index:1
// Public static Visibility=Default Availability=Available
// [8] QMetaObject::Connection connect(const QObject *, const QMetaMethod &, const QObject *, const QMetaMethod &, Qt::ConnectionType)

/*
Creates a connection of the given type from the signal in the sender object to the method in the receiver object. Returns a handle to the connection that can be used to disconnect it later.

You must use the SIGNAL() and SLOT() macros when specifying the signal and the method, for example:


  QLabel *label = new QLabel;
  QScrollBar *scrollBar = new QScrollBar;
  QObject::connect(scrollBar, SIGNAL(valueChanged(int)),
                   label,  SLOT(setNum(int)));



This example ensures that the label always displays the current scroll bar value. Note that the signal and slots parameters must not contain any variable names, only the type. E.g. the following would not work and return false:


  // WRONG
  QObject::connect(scrollBar, SIGNAL(valueChanged(int value)),
                   label, SLOT(setNum(int value)));



A signal can also be connected to another signal:


  class MyWidget : public QWidget
  {
      Q_OBJECT

  public:
      MyWidget();

  signals:
      void buttonClicked();

  private:
      QPushButton *myButton;
  };

  MyWidget::MyWidget()
  {
      myButton = new QPushButton(this);
      connect(myButton, SIGNAL(clicked()),
              this, SIGNAL(buttonClicked()));
  }



In this example, the MyWidget constructor relays a signal from a private member variable, and makes it available under a name that relates to MyWidget.

A signal can be connected to many slots and signals. Many signals can be connected to one slot.

If a signal is connected to several slots, the slots are activated in the same order in which the connections were made, when the signal is emitted.

The function returns a QMetaObject::Connection that represents a handle to a connection if it successfully connects the signal to the slot. The connection handle will be invalid if it cannot create the connection, for example, if QObject is unable to verify the existence of either signal or method, or if their signatures aren't compatible. You can check if the handle is valid by casting it to a bool.

By default, a signal is emitted for every connection you make; two signals are emitted for duplicate connections. You can break all of these connections with a single disconnect() call. If you pass the Qt::UniqueConnection type, the connection will only be made if it is not a duplicate. If there is already a duplicate (exact same signal to the exact same slot on the same objects), the connection will fail and connect will return an invalid QMetaObject::Connection.

Note: Qt::UniqueConnections do not work for lambdas, non-member functions and functors; they only apply to connecting to member functions.

The optional type parameter describes the type of connection to establish. In particular, it determines whether a particular signal is delivered to a slot immediately or queued for delivery at a later time. If the signal is queued, the parameters must be of types that are known to Qt's meta-object system, because Qt needs to copy the arguments to store them in an event behind the scenes. If you try to use a queued connection and get the error message


  QObject::connect: Cannot queue arguments of type 'MyType'
  (Make sure 'MyType' is registered using qRegisterMetaType().)



call qRegisterMetaType() to register the data type before you establish the connection.

Note: This function is thread-safe.

See also disconnect(), sender(), qRegisterMetaType(), Q_DECLARE_METATYPE(), and Differences between String-Based and Functor-Based Connections.
*/
impl /*struct*/ QObject {
  pub fn connect_1<RetType, T: QObject_connect_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.connect_1();
    // return 1;
  }
}
pub trait QObject_connect_1<RetType> {
  fn connect_1(self ) -> RetType;
}
impl<'a> /*trait*/ QObject_connect_1<i32> for (usize,usize,usize,usize,i32) {
  fn connect_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject7connectEPKS_RK11QMetaMethodS1_S4_N2Qt14ConnectionTypeE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:221
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QMetaObject::Connection connect(const QObject *, const char *, const char *, Qt::ConnectionType) const

/*
Creates a connection of the given type from the signal in the sender object to the method in the receiver object. Returns a handle to the connection that can be used to disconnect it later.

You must use the SIGNAL() and SLOT() macros when specifying the signal and the method, for example:


  QLabel *label = new QLabel;
  QScrollBar *scrollBar = new QScrollBar;
  QObject::connect(scrollBar, SIGNAL(valueChanged(int)),
                   label,  SLOT(setNum(int)));



This example ensures that the label always displays the current scroll bar value. Note that the signal and slots parameters must not contain any variable names, only the type. E.g. the following would not work and return false:


  // WRONG
  QObject::connect(scrollBar, SIGNAL(valueChanged(int value)),
                   label, SLOT(setNum(int value)));



A signal can also be connected to another signal:


  class MyWidget : public QWidget
  {
      Q_OBJECT

  public:
      MyWidget();

  signals:
      void buttonClicked();

  private:
      QPushButton *myButton;
  };

  MyWidget::MyWidget()
  {
      myButton = new QPushButton(this);
      connect(myButton, SIGNAL(clicked()),
              this, SIGNAL(buttonClicked()));
  }



In this example, the MyWidget constructor relays a signal from a private member variable, and makes it available under a name that relates to MyWidget.

A signal can be connected to many slots and signals. Many signals can be connected to one slot.

If a signal is connected to several slots, the slots are activated in the same order in which the connections were made, when the signal is emitted.

The function returns a QMetaObject::Connection that represents a handle to a connection if it successfully connects the signal to the slot. The connection handle will be invalid if it cannot create the connection, for example, if QObject is unable to verify the existence of either signal or method, or if their signatures aren't compatible. You can check if the handle is valid by casting it to a bool.

By default, a signal is emitted for every connection you make; two signals are emitted for duplicate connections. You can break all of these connections with a single disconnect() call. If you pass the Qt::UniqueConnection type, the connection will only be made if it is not a duplicate. If there is already a duplicate (exact same signal to the exact same slot on the same objects), the connection will fail and connect will return an invalid QMetaObject::Connection.

Note: Qt::UniqueConnections do not work for lambdas, non-member functions and functors; they only apply to connecting to member functions.

The optional type parameter describes the type of connection to establish. In particular, it determines whether a particular signal is delivered to a slot immediately or queued for delivery at a later time. If the signal is queued, the parameters must be of types that are known to Qt's meta-object system, because Qt needs to copy the arguments to store them in an event behind the scenes. If you try to use a queued connection and get the error message


  QObject::connect: Cannot queue arguments of type 'MyType'
  (Make sure 'MyType' is registered using qRegisterMetaType().)



call qRegisterMetaType() to register the data type before you establish the connection.

Note: This function is thread-safe.

See also disconnect(), sender(), qRegisterMetaType(), Q_DECLARE_METATYPE(), and Differences between String-Based and Functor-Based Connections.
*/
impl /*struct*/ QObject {
  pub fn connect_2<RetType, T: QObject_connect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.connect_2(self);
    // return 1;
  }
}
pub trait QObject_connect_2<RetType> {
  fn connect_2(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_connect_2<i32> for (usize,usize,usize,i32) {
  fn connect_2(self , rsthis: & QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject7connectEPKS_PKcS3_N2Qt14ConnectionTypeE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:343
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool disconnect(const QObject *, const char *, const QObject *, const char *)

/*
Disconnects signal in object sender from method in object receiver. Returns true if the connection is successfully broken; otherwise returns false.

A signal-slot connection is removed when either of the objects involved are destroyed.

disconnect() is typically used in three ways, as the following examples demonstrate.
*/
impl /*struct*/ QObject {
  pub fn disconnect_0<RetType, T: QObject_disconnect_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.disconnect_0();
    // return 1;
  }
}
pub trait QObject_disconnect_0<RetType> {
  fn disconnect_0(self ) -> RetType;
}
impl<'a> /*trait*/ QObject_disconnect_0<bool> for (usize,usize,usize,usize) {
  fn disconnect_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject10disconnectEPKS_PKcS1_S3_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:345
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool disconnect(const QObject *, const QMetaMethod &, const QObject *, const QMetaMethod &)

/*
Disconnects signal in object sender from method in object receiver. Returns true if the connection is successfully broken; otherwise returns false.

A signal-slot connection is removed when either of the objects involved are destroyed.

disconnect() is typically used in three ways, as the following examples demonstrate.
*/
impl /*struct*/ QObject {
  pub fn disconnect_1<RetType, T: QObject_disconnect_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.disconnect_1();
    // return 1;
  }
}
pub trait QObject_disconnect_1<RetType> {
  fn disconnect_1(self ) -> RetType;
}
impl<'a> /*trait*/ QObject_disconnect_1<bool> for (usize,usize,usize,usize) {
  fn disconnect_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject10disconnectEPKS_RK11QMetaMethodS1_S4_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:347
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool disconnect(const char *, const QObject *, const char *) const

/*
Disconnects signal in object sender from method in object receiver. Returns true if the connection is successfully broken; otherwise returns false.

A signal-slot connection is removed when either of the objects involved are destroyed.

disconnect() is typically used in three ways, as the following examples demonstrate.
*/
impl /*struct*/ QObject {
  pub fn disconnect_2<RetType, T: QObject_disconnect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.disconnect_2(self);
    // return 1;
  }
}
pub trait QObject_disconnect_2<RetType> {
  fn disconnect_2(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_disconnect_2<bool> for (usize,usize,usize) {
  fn disconnect_2(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject10disconnectEPKcPKS_S1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:350
// index:3
// Public inline Visibility=Default Availability=Available
// [1] bool disconnect(const QObject *, const char *) const

/*
Disconnects signal in object sender from method in object receiver. Returns true if the connection is successfully broken; otherwise returns false.

A signal-slot connection is removed when either of the objects involved are destroyed.

disconnect() is typically used in three ways, as the following examples demonstrate.
*/
impl /*struct*/ QObject {
  pub fn disconnect_3<RetType, T: QObject_disconnect_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.disconnect_3(self);
    // return 1;
  }
}
pub trait QObject_disconnect_3<RetType> {
  fn disconnect_3(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_disconnect_3<bool> for (usize,usize) {
  fn disconnect_3(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject10disconnectEPKS_PKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:391
// index:0
// Public Visibility=Default Availability=Available
// [-2] void dumpObjectTree()

/*
Dumps a tree of children to the debug output.

Note: before Qt 5.9, this function was not const.

See also dumpObjectInfo().
*/
impl /*struct*/ QObject {
  pub fn dumpObjectTree_0<RetType, T: QObject_dumpObjectTree_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dumpObjectTree_0(self);
    // return 1;
  }
}
pub trait QObject_dumpObjectTree_0<RetType> {
  fn dumpObjectTree_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_dumpObjectTree_0<(/*void*/)> for () {
  fn dumpObjectTree_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QObject14dumpObjectTreeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:394
// index:1
// Public Visibility=Default Availability=Available
// [-2] void dumpObjectTree() const

/*
Dumps a tree of children to the debug output.

Note: before Qt 5.9, this function was not const.

See also dumpObjectInfo().
*/
impl /*struct*/ QObject {
  pub fn dumpObjectTree_1<RetType, T: QObject_dumpObjectTree_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dumpObjectTree_1(self);
    // return 1;
  }
}
pub trait QObject_dumpObjectTree_1<RetType> {
  fn dumpObjectTree_1(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_dumpObjectTree_1<(/*void*/)> for () {
  fn dumpObjectTree_1(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZNK7QObject14dumpObjectTreeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:392
// index:0
// Public Visibility=Default Availability=Available
// [-2] void dumpObjectInfo()

/*
Dumps information about signal connections, etc. for this object to the debug output.

Note: before Qt 5.9, this function was not const.

See also dumpObjectTree().
*/
impl /*struct*/ QObject {
  pub fn dumpObjectInfo_0<RetType, T: QObject_dumpObjectInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dumpObjectInfo_0(self);
    // return 1;
  }
}
pub trait QObject_dumpObjectInfo_0<RetType> {
  fn dumpObjectInfo_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_dumpObjectInfo_0<(/*void*/)> for () {
  fn dumpObjectInfo_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QObject14dumpObjectInfoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:395
// index:1
// Public Visibility=Default Availability=Available
// [-2] void dumpObjectInfo() const

/*
Dumps information about signal connections, etc. for this object to the debug output.

Note: before Qt 5.9, this function was not const.

See also dumpObjectTree().
*/
impl /*struct*/ QObject {
  pub fn dumpObjectInfo_1<RetType, T: QObject_dumpObjectInfo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dumpObjectInfo_1(self);
    // return 1;
  }
}
pub trait QObject_dumpObjectInfo_1<RetType> {
  fn dumpObjectInfo_1(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_dumpObjectInfo_1<(/*void*/)> for () {
  fn dumpObjectInfo_1(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZNK7QObject14dumpObjectInfoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:398
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setProperty(const char *, const QVariant &)

/*
Sets the value of the object's name property to value.

If the property is defined in the class using Q_PROPERTY then true is returned on success and false otherwise. If the property is not defined using Q_PROPERTY, and therefore not listed in the meta-object, it is added as a dynamic property and false is returned.

Information about all available properties is provided through the metaObject() and dynamicPropertyNames().

Dynamic properties can be queried again using property() and can be removed by setting the property value to an invalid QVariant. Changing the value of a dynamic property causes a QDynamicPropertyChangeEvent to be sent to the object.

Note: Dynamic properties starting with "_q_" are reserved for internal purposes.

See also property(), metaObject(), dynamicPropertyNames(), and QMetaProperty::write().
*/
impl /*struct*/ QObject {
  pub fn setProperty_0<RetType, T: QObject_setProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProperty_0(self);
    // return 1;
  }
}
pub trait QObject_setProperty_0<RetType> {
  fn setProperty_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_setProperty_0<bool> for (usize,usize) {
  fn setProperty_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject11setPropertyEPKcRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:399
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant property(const char *) const

/*
Returns the value of the object's name property.

If no such property exists, the returned variant is invalid.

Information about all available properties is provided through the metaObject() and dynamicPropertyNames().

See also setProperty(), QVariant::isValid(), metaObject(), and dynamicPropertyNames().
*/
impl /*struct*/ QObject {
  pub fn property_0<RetType, T: QObject_property_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.property_0(self);
    // return 1;
  }
}
pub trait QObject_property_0<RetType> {
  fn property_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_property_0<usize> for (usize) {
  fn property_0(self , rsthis: & QObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject8propertyEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:404
// index:0
// Public static Visibility=Default Availability=Available
// [4] uint registerUserData()

/*

*/
impl /*struct*/ QObject {
  pub fn registerUserData_0<RetType, T: QObject_registerUserData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerUserData_0();
    // return 1;
  }
}
pub trait QObject_registerUserData_0<RetType> {
  fn registerUserData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QObject_registerUserData_0<u32> for () {
  fn registerUserData_0(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QObject16registerUserDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:405
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUserData(uint, QObjectUserData *)

/*

*/
impl /*struct*/ QObject {
  pub fn setUserData_0<RetType, T: QObject_setUserData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUserData_0(self);
    // return 1;
  }
}
pub trait QObject_setUserData_0<RetType> {
  fn setUserData_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_setUserData_0<(/*void*/)> for (u32,usize) {
  fn setUserData_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject11setUserDataEjP15QObjectUserData", 2,qtrt::FFITY_UINT32,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:406
// index:0
// Public Visibility=Default Availability=Available
// [8] QObjectUserData * userData(uint) const

/*

*/
impl /*struct*/ QObject {
  pub fn userData_0<RetType, T: QObject_userData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.userData_0(self);
    // return 1;
  }
}
pub trait QObject_userData_0<RetType> {
  fn userData_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_userData_0<usize> for (u32) {
  fn userData_0(self , rsthis: & QObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject8userDataEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:410
// index:0
// Public Visibility=Default Availability=Available
// [-2] void destroyed(QObject *)

/*
This signal is emitted immediately before the object obj is destroyed, and can not be blocked.

All the objects's children are destroyed immediately after this signal is emitted.

See also deleteLater() and QPointer.
*/
impl /*struct*/ QObject {
  pub fn destroyed_0<RetType, T: QObject_destroyed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.destroyed_0(self);
    // return 1;
  }
}
pub trait QObject_destroyed_0<RetType> {
  fn destroyed_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_destroyed_0<(/*void*/)> for (usize) {
  fn destroyed_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject9destroyedEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:414
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QObject * parent() const

/*
Returns a pointer to the parent object.

See also setParent() and children().
*/
impl /*struct*/ QObject {
  pub fn parent_0<RetType, T: QObject_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QObject_parent_0<RetType> {
  fn parent_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_parent_0<usize> for () {
  fn parent_0(self , rsthis: & QObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:416
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool inherits(const char *) const

/*
Returns true if this object is an instance of a class that inherits className or a QObject subclass that inherits className; otherwise returns false.

A class is considered to inherit itself.

Example:


  QTimer *timer = new QTimer;         // QTimer inherits QObject
  timer->inherits("QTimer");          // returns true
  timer->inherits("QObject");         // returns true
  timer->inherits("QAbstractButton"); // returns false

  // QVBoxLayout inherits QObject and QLayoutItem
  QVBoxLayout *layout = new QVBoxLayout;
  layout->inherits("QObject");        // returns true
  layout->inherits("QLayoutItem");    // returns true (even though QLayoutItem is not a QObject)



If you need to determine whether an object is an instance of a particular class for the purpose of casting it, consider using qobject_cast<Type *>(object) instead.

See also metaObject() and qobject_cast().
*/
impl /*struct*/ QObject {
  pub fn inherits_0<RetType, T: QObject_inherits_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inherits_0(self);
    // return 1;
  }
}
pub trait QObject_inherits_0<RetType> {
  fn inherits_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_inherits_0<bool> for (usize) {
  fn inherits_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject8inheritsEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:420
// index:0
// Public Visibility=Default Availability=Available
// [-2] void deleteLater()

/*
Schedules this object for deletion.

The object will be deleted when control returns to the event loop. If the event loop is not running when this function is called (e.g. deleteLater() is called on an object before QCoreApplication::exec()), the object will be deleted once the event loop is started. If deleteLater() is called after the main event loop has stopped, the object will not be deleted. Since Qt 4.8, if deleteLater() is called on an object that lives in a thread with no running event loop, the object will be destroyed when the thread finishes.

Note that entering and leaving a new event loop (e.g., by opening a modal dialog) will not perform the deferred deletion; for the object to be deleted, the control must return to the event loop from which deleteLater() was called.

Note: It is safe to call this function more than once; when the first deferred deletion event is delivered, any pending events for the object are removed from the event queue.

See also destroyed() and QPointer.
*/
impl /*struct*/ QObject {
  pub fn deleteLater_0<RetType, T: QObject_deleteLater_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deleteLater_0(self);
    // return 1;
  }
}
pub trait QObject_deleteLater_0<RetType> {
  fn deleteLater_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_deleteLater_0<(/*void*/)> for () {
  fn deleteLater_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QObject11deleteLaterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:423
// index:0
// Protected Visibility=Default Availability=Available
// [8] QObject * sender() const

/*
Returns a pointer to the object that sent the signal, if called in a slot activated by a signal; otherwise it returns 0. The pointer is valid only during the execution of the slot that calls this function from this object's thread context.

The pointer returned by this function becomes invalid if the sender is destroyed, or if the slot is disconnected from the sender's signal.

Warning: This function violates the object-oriented principle of modularity. However, getting access to the sender might be useful when many signals are connected to a single slot.

Warning: As mentioned above, the return value of this function is not valid when the slot is called via a Qt::DirectConnection from a thread different from this object's thread. Do not use this function in this type of scenario.

See also senderSignalIndex().
*/
impl /*struct*/ QObject {
  pub fn sender_0<RetType, T: QObject_sender_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sender_0(self);
    // return 1;
  }
}
pub trait QObject_sender_0<RetType> {
  fn sender_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_sender_0<usize> for () {
  fn sender_0(self , rsthis: & QObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject6senderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:424
// index:0
// Protected Visibility=Default Availability=Available
// [4] int senderSignalIndex() const

/*
Returns the meta-method index of the signal that called the currently executing slot, which is a member of the class returned by sender(). If called outside of a slot activated by a signal, -1 is returned.

For signals with default parameters, this function will always return the index with all parameters, regardless of which was used with connect(). For example, the signal destroyed(QObject *obj = 0) will have two different indexes (with and without the parameter), but this function will always return the index with a parameter. This does not apply when overloading signals with different parameters.

Warning: This function violates the object-oriented principle of modularity. However, getting access to the signal index might be useful when many signals are connected to a single slot.

Warning: The return value of this function is not valid when the slot is called via a Qt::DirectConnection from a thread different from this object's thread. Do not use this function in this type of scenario.

This function was introduced in  Qt 4.8.

See also sender(), QMetaObject::indexOfSignal(), and QMetaObject::method().
*/
impl /*struct*/ QObject {
  pub fn senderSignalIndex_0<RetType, T: QObject_senderSignalIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.senderSignalIndex_0(self);
    // return 1;
  }
}
pub trait QObject_senderSignalIndex_0<RetType> {
  fn senderSignalIndex_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_senderSignalIndex_0<i32> for () {
  fn senderSignalIndex_0(self , rsthis: & QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject17senderSignalIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:425
// index:0
// Protected Visibility=Default Availability=Available
// [4] int receivers(const char *) const

/*
Returns the number of receivers connected to the signal.

Since both slots and signals can be used as receivers for signals, and the same connections can be made many times, the number of receivers is the same as the number of connections made from this signal.

When calling this function, you can use the SIGNAL() macro to pass a specific signal:


  if (receivers(SIGNAL(valueChanged(QByteArray))) > 0) {
      QByteArray data;
      get_the_value(&data);       // expensive operation
      emit valueChanged(data);
  }



Warning: This function violates the object-oriented principle of modularity. However, it might be useful when you need to perform expensive initialization only if something is connected to a signal.

See also isSignalConnected().
*/
impl /*struct*/ QObject {
  pub fn receivers_0<RetType, T: QObject_receivers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.receivers_0(self);
    // return 1;
  }
}
pub trait QObject_receivers_0<RetType> {
  fn receivers_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_receivers_0<i32> for (usize) {
  fn receivers_0(self , rsthis: & QObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject9receiversEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:426
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool isSignalConnected(const QMetaMethod &) const

/*
Returns true if the signal is connected to at least one receiver, otherwise returns false.

signal must be a signal member of this object, otherwise the behaviour is undefined.


  static const QMetaMethod valueChangedSignal = QMetaMethod::fromSignal(&MyObject::valueChanged);
  if (isSignalConnected(valueChangedSignal)) {
      QByteArray data;
      data = get_the_value();       // expensive operation
      emit valueChanged(data);
  }



As the code snippet above illustrates, you can use this function to avoid emitting a signal that nobody listens to.

Warning: This function violates the object-oriented principle of modularity. However, it might be useful when you need to perform expensive initialization only if something is connected to a signal.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QObject {
  pub fn isSignalConnected_0<RetType, T: QObject_isSignalConnected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSignalConnected_0(self);
    // return 1;
  }
}
pub trait QObject_isSignalConnected_0<RetType> {
  fn isSignalConnected_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_isSignalConnected_0<bool> for (usize) {
  fn isSignalConnected_0(self , rsthis: & QObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QObject17isSignalConnectedERK11QMetaMethod", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobject.h:428
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
This event handler can be reimplemented in a subclass to receive timer events for the object.

QTimer provides a higher-level interface to the timer functionality, and also more general information about timers. The timer event is passed in the event parameter.

See also startTimer(), killTimer(), and event().
*/
impl /*struct*/ QObject {
  pub fn timerEvent_0<RetType, T: QObject_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QObject_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:429
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void childEvent(QChildEvent *)

/*
This event handler can be reimplemented in a subclass to receive child events. The event is passed in the event parameter.

QEvent::ChildAdded and QEvent::ChildRemoved events are sent to objects when children are added or removed. In both cases you can only rely on the child being a QObject, or if isWidgetType() returns true, a QWidget. (This is because, in the ChildAdded case, the child is not yet fully constructed, and in the ChildRemoved case it might have been destructed already).

QEvent::ChildPolished events are sent to widgets when children are polished, or when polished children are added. If you receive a child polished event, the child's construction is usually completed. However, this is not guaranteed, and multiple polish events may be delivered during the execution of a widget's constructor.

For every child widget, you receive one ChildAdded event, zero or more ChildPolished events, and one ChildRemoved event.

The ChildPolished event is omitted if a child is removed immediately after it is added. If a child is polished several times during construction and destruction, you may receive several child polished events for the same child, each time with a different virtual table.

See also event().
*/
impl /*struct*/ QObject {
  pub fn childEvent_0<RetType, T: QObject_childEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childEvent_0(self);
    // return 1;
  }
}
pub trait QObject_childEvent_0<RetType> {
  fn childEvent_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_childEvent_0<(/*void*/)> for (usize) {
  fn childEvent_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject10childEventEP11QChildEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:430
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void customEvent(QEvent *)

/*
This event handler can be reimplemented in a subclass to receive custom events. Custom events are user-defined events with a type value at least as large as the QEvent::User item of the QEvent::Type enum, and is typically a QEvent subclass. The event is passed in the event parameter.

See also event() and QEvent.
*/
impl /*struct*/ QObject {
  pub fn customEvent_0<RetType, T: QObject_customEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.customEvent_0(self);
    // return 1;
  }
}
pub trait QObject_customEvent_0<RetType> {
  fn customEvent_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_customEvent_0<(/*void*/)> for (usize) {
  fn customEvent_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject11customEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:432
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void connectNotify(const QMetaMethod &)

/*
This virtual function is called when something has been connected to signal in this object.

If you want to compare signal with a specific signal, you can use QMetaMethod::fromSignal() as follows:


  if (signal == QMetaMethod::fromSignal(&MyObject::valueChanged)) {
      // signal is valueChanged
  }



Warning: This function violates the object-oriented principle of modularity. However, it might be useful when you need to perform expensive initialization only if something is connected to a signal.

Warning: This function is called from the thread which performs the connection, which may be a different thread from the thread in which this object lives.

This function was introduced in  Qt 5.0.

See also connect() and disconnectNotify().
*/
impl /*struct*/ QObject {
  pub fn connectNotify_0<RetType, T: QObject_connectNotify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.connectNotify_0(self);
    // return 1;
  }
}
pub trait QObject_connectNotify_0<RetType> {
  fn connectNotify_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_connectNotify_0<(/*void*/)> for (usize) {
  fn connectNotify_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject13connectNotifyERK11QMetaMethod", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:433
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void disconnectNotify(const QMetaMethod &)

/*
This virtual function is called when something has been disconnected from signal in this object.

See connectNotify() for an example of how to compare signal with a specific signal.

If all signals were disconnected from this object (e.g., the signal argument to disconnect() was 0), disconnectNotify() is only called once, and the signal will be an invalid QMetaMethod (QMetaMethod::isValid() returns false).

Warning: This function violates the object-oriented principle of modularity. However, it might be useful for optimizing access to expensive resources.

Warning: This function is called from the thread which performs the disconnection, which may be a different thread from the thread in which this object lives. This function may also be called with a QObject internal mutex locked. It is therefore not allowed to re-enter any of any QObject functions from your reimplementation and if you lock a mutex in your reimplementation, make sure that you don't call QObject functions with that mutex held in other places or it will result in a deadlock.

This function was introduced in  Qt 5.0.

See also disconnect() and connectNotify().
*/
impl /*struct*/ QObject {
  pub fn disconnectNotify_0<RetType, T: QObject_disconnectNotify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.disconnectNotify_0(self);
    // return 1;
  }
}
pub trait QObject_disconnectNotify_0<RetType> {
  fn disconnectNotify_0(self , rsthis: & QObject) -> RetType;
}
impl<'a> /*trait*/ QObject_disconnectNotify_0<(/*void*/)> for (usize) {
  fn disconnectNotify_0(self , rsthis: & QObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QObject16disconnectNotifyERK11QMetaMethod", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
